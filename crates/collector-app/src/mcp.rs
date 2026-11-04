//! Read-only MCP server for AI assistants (`collector-app --mcp URL`).
//!
//! An assistant such as Claude Desktop or Claude Code starts this process and talks to it
//! over stdio. Every tool call becomes a GET request to a running collector's `/devices`,
//! `/models` and `/healthz` endpoints, so the bridge has no write path, adds no listener to
//! the collector, and works with a collector on a remote gateway.

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::{anyhow, bail, Context, Result};
use http_body_util::{BodyExt, Empty};
use hyper::body::Bytes;
use hyper::header::{ACCEPT, HOST};
use hyper::{Request, Uri};
use hyper_util::rt::TokioIo;
use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{CallToolResult, ContentBlock, Implementation, ServerCapabilities, ServerConfig};
use rmcp::{tool, tool_handler, tool_router, ErrorData, ServerHandler, ServiceExt};
use serde::Deserialize;
use serde_json::{json, Value};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::TcpStream;
use tokio::time::timeout;

/// How long one request to the collector may take: connect, send and read the body.
const REQUEST_TIMEOUT: Duration = Duration::from_secs(5);

const INSTRUCTIONS: &str = "Read-only access to a Rusty SunSpec Collector, which polls \
SunSpec inverters, meters and batteries over Modbus. Devices are named host/unit, for example \
10.0.4.12/2. Readings are decoded with the official SunSpec definitions: scale factors are \
already applied, unimplemented values are null, and labels give the names of enum values and \
set bitfield bits. Use explain_point for a point's units, meaning and symbols. Nothing here can \
change a device setting.";

/// A collector's HTTP address, `http://host:port` with an optional path prefix.
#[derive(Debug, Clone)]
pub struct CollectorUrl {
    host: String,
    port: u16,
    authority: String,
    prefix: String,
    display: String,
}

impl CollectorUrl {
    /// Accepts `http://host:port[/prefix]` or a bare `host:port`.
    pub fn parse(input: &str) -> Result<Self> {
        let input = input.trim().trim_end_matches('/');
        let with_scheme = if input.contains("://") {
            input.to_string()
        } else {
            format!("http://{input}")
        };
        let uri: Uri = with_scheme
            .parse()
            .with_context(|| format!("{input:?} is not a URL"))?;
        match uri.scheme_str() {
            Some("http") => {}
            Some(other) => {
                bail!("{input:?}: the collector serves plain http on its metrics port, not {other}")
            }
            None => bail!("{input:?} has no scheme"),
        }
        let host = uri
            .host()
            .ok_or_else(|| anyhow!("{input:?} has no host"))?
            .trim_matches(|c| c == '[' || c == ']')
            .to_string();
        let port = uri.port_u16().unwrap_or(80);
        let authority = uri
            .authority()
            .map(|a| a.as_str().to_string())
            .unwrap_or_else(|| host.clone());
        let prefix = uri.path().trim_end_matches('/').to_string();
        Ok(Self {
            host,
            port,
            authority,
            prefix,
            display: with_scheme,
        })
    }
}

impl std::fmt::Display for CollectorUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.display)
    }
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct DeviceArgs {
    /// The device as host/unit, for example 10.0.4.12/2 (list_devices shows them).
    device: String,
    /// Only this SunSpec model, for example 103 (three-phase inverter). Leave out for all
    /// polled models.
    #[serde(default)]
    model: Option<u16>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct PointArgs {
    /// SunSpec model ID, for example 103.
    model: u16,
    /// A point name such as W, Evt1 or module[0].DCA, or a symbol name such as
    /// GROUND_FAULT or MPPT. Leave out to list the model's points.
    #[serde(default)]
    point: Option<String>,
}

/// The MCP server: four read-only tools over a collector's HTTP endpoints.
#[derive(Clone)]
pub struct CollectorMcp {
    url: CollectorUrl,
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl CollectorMcp {
    pub fn new(url: CollectorUrl) -> Self {
        Self {
            url,
            tool_router: Self::tool_router(),
        }
    }

    #[tool(
        description = "List the devices the collector supervises: whether each is being read \
        successfully, seconds since its last good read, session restarts, and which SunSpec \
        models it has and polls."
    )]
    async fn list_devices(&self) -> Result<CallToolResult, ErrorData> {
        let body = match self.get("/devices").await {
            Ok((200, body)) => body,
            Ok((status, body)) => return Ok(http_error(status, &body)),
            Err(err) => return Ok(self.unreachable(err)),
        };
        let now = now_ms();
        let devices: Vec<Value> = body["devices"]
            .as_array()
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .map(|d| {
                json!({
                    "device": d["device"],
                    "up": d["up"],
                    "seconds_since_last_success": d["last_success_ms"]
                        .as_u64()
                        .map(|ms| now.saturating_sub(ms) as f64 / 1000.0),
                    "restarts": d["restarts"],
                    "base_address": d["base_address"],
                    "models": d["models"],
                    "polled_models": d["polled_models"],
                })
            })
            .collect();
        Ok(success(&json!({ "devices": devices })))
    }

    #[tool(
        description = "Latest decoded readings of one device: every point of each polled model, \
        with scale factors applied. labels names enum values and set bitfield bits, for example \
        St: FAULT or Evt1: GROUND_FAULT. age_s is the age of each reading in seconds."
    )]
    async fn get_readings(
        &self,
        Parameters(args): Parameters<DeviceArgs>,
    ) -> Result<CallToolResult, ErrorData> {
        let Some((host, unit)) = split_device(&args.device) else {
            return Ok(failure(format!(
                "{:?} is not a device: use host/unit, for example 10.0.4.12/2",
                args.device
            )));
        };
        let mut body = match self.get(&format!("/devices/{host}/{unit}")).await {
            Ok((200, body)) => body,
            Ok((404, _)) => return Ok(self.unknown_device(&args.device).await),
            Ok((status, body)) => return Ok(http_error(status, &body)),
            Err(err) => return Ok(self.unreachable(err)),
        };
        if let Some(model) = args.model {
            if let Some(readings) = body["readings"].as_array_mut() {
                readings.retain(|r| r["model_id"].as_u64() == Some(u64::from(model)));
                if readings.is_empty() {
                    return Ok(failure(format!(
                        "{} has no reading of model {model}; it polls {}",
                        args.device, body["polled_models"]
                    )));
                }
            }
        }
        Ok(success(&body))
    }

    #[tool(
        description = "Explain a point of a SunSpec model from the official definitions: its \
        label, description, units and type, and for enums and bitfields every symbol with its \
        value and meaning. Accepts a point name (W, Evt1, module[0].DCA) or a symbol name \
        (GROUND_FAULT, MPPT). Without a point, lists the model's points."
    )]
    async fn explain_point(
        &self,
        Parameters(args): Parameters<PointArgs>,
    ) -> Result<CallToolResult, ErrorData> {
        let model = match self.get(&format!("/models/{}", args.model)).await {
            Ok((200, body)) => body,
            Ok((404, _)) => {
                return Ok(failure(format!(
                    "there is no bundled definition for model {}",
                    args.model
                )))
            }
            Ok((status, body)) => return Ok(http_error(status, &body)),
            Err(err) => return Ok(self.unreachable(err)),
        };
        let points = model["points"].as_array().cloned().unwrap_or_default();
        let Some(wanted) = args.point else {
            let names: Vec<&Value> = points.iter().map(|p| &p["name"]).collect();
            return Ok(success(&json!({
                "id": model["id"],
                "name": model["name"],
                "label": model["label"],
                "description": model["description"],
                "points": names,
            })));
        };
        match find_point(&points, &wanted) {
            Some((point, None)) => Ok(success(&json!({ "model": args.model, "point": point }))),
            Some((point, Some(symbol))) => Ok(success(&json!({
                "model": args.model,
                "matched_symbol": symbol,
                "point": point,
            }))),
            None => Ok(failure(format!(
                "model {} has no point or symbol named {wanted:?}; call explain_point with only \
                 the model to list its points",
                args.model
            ))),
        }
    }

    #[tool(
        description = "The collector's own health: healthy is true while its ingest and uplink \
        tasks are running, with the number of devices up out of devices known. Device outages \
        do not make the collector unhealthy."
    )]
    async fn collector_health(&self) -> Result<CallToolResult, ErrorData> {
        match self.get("/healthz").await {
            // 503 still carries the report: it is how the collector says it is unhealthy.
            Ok((200 | 503, body)) => Ok(success(&body)),
            Ok((status, body)) => Ok(http_error(status, &body)),
            Err(err) => Ok(self.unreachable(err)),
        }
    }
}

impl CollectorMcp {
    /// One GET request, returning the status and the JSON body.
    async fn get(&self, path: &str) -> Result<(u16, Value), String> {
        let url = &self.url;
        let request = async {
            let stream = TcpStream::connect((url.host.as_str(), url.port))
                .await
                .map_err(|err| format!("connect: {err}"))?;
            let (mut sender, conn) = hyper::client::conn::http1::handshake(TokioIo::new(stream))
                .await
                .map_err(|err| format!("handshake: {err}"))?;
            tokio::spawn(conn);
            let request = Request::get(format!("{}{path}", url.prefix))
                .header(HOST, &url.authority)
                .header(ACCEPT, "application/json")
                .body(Empty::<Bytes>::new())
                .map_err(|err| err.to_string())?;
            let response = sender
                .send_request(request)
                .await
                .map_err(|err| format!("request: {err}"))?;
            let status = response.status().as_u16();
            let body = response
                .into_body()
                .collect()
                .await
                .map_err(|err| format!("response: {err}"))?
                .to_bytes();
            let value = serde_json::from_slice(&body)
                .map_err(|err| format!("the response is not JSON ({err})"))?;
            Ok((status, value))
        };
        timeout(REQUEST_TIMEOUT, request)
            .await
            .map_err(|_| format!("no answer within {} s", REQUEST_TIMEOUT.as_secs()))?
    }

    fn unreachable(&self, err: String) -> CallToolResult {
        failure(format!(
            "could not reach the collector at {}: {err}. Check that collector-app is running and \
             that its metrics port is reachable from here.",
            self.url
        ))
    }

    async fn unknown_device(&self, device: &str) -> CallToolResult {
        let known: Vec<String> = match self.get("/devices").await {
            Ok((200, body)) => body["devices"]
                .as_array()
                .map(|ds| {
                    ds.iter()
                        .filter_map(|d| d["device"].as_str().map(str::to_string))
                        .collect()
                })
                .unwrap_or_default(),
            _ => Vec::new(),
        };
        failure(format!(
            "the collector does not supervise {device}; known devices: {}",
            if known.is_empty() {
                "none".to_string()
            } else {
                known.join(", ")
            }
        ))
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for CollectorMcp {
    fn get_info(&self) -> ServerConfig {
        let mut server = Implementation::new("rusty-sunspec-collector", env!("CARGO_PKG_VERSION"));
        server.title = Some("Rusty SunSpec Collector".to_string());
        ServerConfig::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(server)
            .with_instructions(INSTRUCTIONS)
    }
}

/// Serves MCP on `reader`/`writer` until the client disconnects.
pub async fn serve<R, W>(url: CollectorUrl, reader: R, writer: W) -> Result<()>
where
    R: AsyncRead + Send + Unpin + 'static,
    W: AsyncWrite + Send + Unpin + 'static,
{
    let service = CollectorMcp::new(url)
        .serve((reader, writer))
        .await
        .context("MCP handshake failed")?;
    service.waiting().await.context("MCP session failed")?;
    Ok(())
}

/// Serves MCP on stdin and stdout, as assistants expect from a local server.
pub async fn serve_stdio(url: CollectorUrl) -> Result<()> {
    serve(url, tokio::io::stdin(), tokio::io::stdout()).await
}

fn success(value: &Value) -> CallToolResult {
    let text = serde_json::to_string_pretty(value).unwrap_or_else(|_| value.to_string());
    CallToolResult::success(vec![ContentBlock::text(text)])
}

fn failure(message: String) -> CallToolResult {
    CallToolResult::error(vec![ContentBlock::text(message)])
}

fn http_error(status: u16, body: &Value) -> CallToolResult {
    let detail = body["error"]
        .as_str()
        .map(str::to_string)
        .unwrap_or_else(|| body.to_string());
    failure(format!("the collector answered HTTP {status}: {detail}"))
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

/// `host/unit` into its parts. Hosts may not contain `/` or whitespace.
fn split_device(device: &str) -> Option<(&str, u8)> {
    let (host, unit) = device.trim().rsplit_once('/')?;
    if host.is_empty() || host.contains(|c: char| c == '/' || c.is_whitespace()) {
        return None;
    }
    Some((host, unit.parse().ok()?))
}

/// Finds a point by name (repeating-group indices like `module[2]` match `module[*]`), or
/// the point whose symbols include `wanted`. Exact case first, then any case.
fn find_point(points: &[Value], wanted: &str) -> Option<(Value, Option<Value>)> {
    let pattern = normalize_point(wanted.trim());
    let by_name = |exact: bool| {
        points.iter().find(|p| {
            let name = p["name"].as_str().unwrap_or_default();
            if exact {
                name == pattern
            } else {
                name.eq_ignore_ascii_case(&pattern)
            }
        })
    };
    if let Some(point) = by_name(true).or_else(|| by_name(false)) {
        return Some((point.clone(), None));
    }
    for exact in [true, false] {
        for point in points {
            for symbol in point["symbols"].as_array().into_iter().flatten() {
                let name = symbol["name"].as_str().unwrap_or_default();
                let hit = if exact {
                    name == wanted.trim()
                } else {
                    name.eq_ignore_ascii_case(wanted.trim())
                };
                if hit {
                    return Some((point.clone(), Some(symbol.clone())));
                }
            }
        }
    }
    None
}

/// `module[2].DCA` into `module[*].DCA`, the catalog's spelling of repeating groups.
fn normalize_point(name: &str) -> String {
    let mut out = String::with_capacity(name.len());
    let mut chars = name.chars().peekable();
    while let Some(c) = chars.next() {
        out.push(c);
        if c == '[' && chars.peek().is_some_and(|d| d.is_ascii_digit()) {
            while chars.peek().is_some_and(|d| d.is_ascii_digit()) {
                chars.next();
            }
            out.push('*');
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_collector_urls() {
        let url = CollectorUrl::parse("http://10.0.4.2:9090").unwrap();
        assert_eq!(
            (url.host.as_str(), url.port, url.prefix.as_str()),
            ("10.0.4.2", 9090, "")
        );
        let url = CollectorUrl::parse("gateway.local:9090/").unwrap();
        assert_eq!((url.host.as_str(), url.port), ("gateway.local", 9090));
        let url = CollectorUrl::parse("http://proxy/collector/").unwrap();
        assert_eq!((url.port, url.prefix.as_str()), (80, "/collector"));
        assert!(CollectorUrl::parse("https://gateway:9090").is_err());
    }

    #[test]
    fn splits_device_keys() {
        assert_eq!(split_device("10.0.4.12/2"), Some(("10.0.4.12", 2)));
        assert_eq!(
            split_device("inverter-1.local/1"),
            Some(("inverter-1.local", 1))
        );
        assert_eq!(split_device("10.0.4.12"), None);
        assert_eq!(split_device("10.0.4.12/300"), None);
        assert_eq!(split_device("/2"), None);
    }

    #[test]
    fn repeating_group_indices_match_the_catalog_spelling() {
        assert_eq!(normalize_point("module[2].DCA"), "module[*].DCA");
        assert_eq!(normalize_point("module[*].DCA"), "module[*].DCA");
        assert_eq!(normalize_point("W"), "W");
    }

    #[test]
    fn finds_points_by_name_or_symbol() {
        let points = vec![
            json!({ "name": "W", "units": "W" }),
            json!({ "name": "Evt1", "symbols": [{ "name": "GROUND_FAULT", "value": 0 }] }),
        ];
        assert_eq!(find_point(&points, "w").unwrap().0["name"], "W");
        let (point, symbol) = find_point(&points, "ground_fault").unwrap();
        assert_eq!(point["name"], "Evt1");
        assert_eq!(symbol.unwrap()["name"], "GROUND_FAULT");
        assert!(find_point(&points, "nope").is_none());
    }
}
