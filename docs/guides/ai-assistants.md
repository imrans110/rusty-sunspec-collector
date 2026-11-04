# Connecting an AI assistant (MCP)

`collector-app --mcp URL` is a read-only [MCP](https://modelcontextprotocol.io) server. An AI
assistant such as Claude Desktop or Claude Code starts it and talks to it over stdin and stdout.
Each tool call becomes a GET request to a running collector's `/devices`, `/models` and `/healthz`
endpoints at `URL`, so an operator can ask "why did inverter 2 stop?" and get an answer from live
readings and the official SunSpec definitions.

It cannot change anything. The bridge only sends GET requests to the collector, and the collector
itself only reads devices. It adds no listener to the collector and needs no extra settings there.
The bridge is part of `collector-app` from 0.6.0. It works with collectors from 0.5.0, which
serve `/devices` and `/models`.

## Tools

| Tool | Arguments | Answers with |
|------|-----------|--------------|
| `list_devices` | none | Every supervised device as `host/unit`: whether it is being read, seconds since its last good read, session restarts, and the models it has and polls |
| `get_readings` | `device` (`host/unit`), optional `model` | The device's latest readings: every point with scale factors applied, `labels` naming enum values and set bitfield bits (`St: FAULT`, `Evt1: GROUND_FAULT`), and each reading's age |
| `explain_point` | `model`, optional `point` | A point's label, description, units, type and symbols from the bundled definitions. `point` can be a point name (`W`, `Evt1`, `module[0].DCA`) or a symbol name (`GROUND_FAULT`, `MPPT`); without it, the model's point names |
| `collector_health` | none | The `/healthz` report: whether ingest and the uplink are running, and devices up out of devices known |

When the collector cannot be reached, or a device or model is unknown, the tool returns an error
result that says so, and lists the known devices for an unknown one. The session stays open, so
the assistant can tell you what is wrong.

Questions that work well:

- "Which devices are down, and since when?"
- "What is inverter 3 producing, and are any alarms set?"
- "Why is PF null on 10.0.4.12/2?" (a null value means the device does not implement the point)
- "What does Evt1 bit 5 mean on a three-phase inverter?"

## Setting it up

The bridge runs wherever the assistant runs, and needs to reach the collector's metrics port
(default 9090). Pick the setup that matches where the assistant is.

### On the gateway, over SSH (recommended for remote sites)

The assistant starts the bridge on the gateway through SSH. MCP's stdio passes straight through the
SSH session, the bridge talks to the collector on `localhost`, and the metrics port does not have
to be reachable from your laptop. It needs key-based SSH login that does not prompt
(`BatchMode=yes` makes a prompt fail instead of hanging).

Claude Desktop (`claude_desktop_config.json`, under Settings > Developer > Edit Config):

```json
{
  "mcpServers": {
    "sunspec-site-a": {
      "command": "ssh",
      "args": ["-o", "BatchMode=yes", "admin@gateway-a.example", "collector-app", "--mcp", "http://127.0.0.1:9090"]
    }
  }
}
```

Claude Code:

```sh
claude mcp add sunspec-site-a -- ssh -o BatchMode=yes admin@gateway-a.example collector-app --mcp http://127.0.0.1:9090
```

### On the same machine as the collector

On a Linux machine where `install.sh` installed the collector (or with a binary built from
source), point the bridge at the local port:

```sh
claude mcp add sunspec -- collector-app --mcp http://127.0.0.1:9090
```

### With the Docker image

The image's entrypoint is `collector-app`, so `-i` and `--mcp` run the bridge in a throwaway
container. Use an address the container can reach:

```json
{
  "mcpServers": {
    "sunspec": {
      "command": "docker",
      "args": ["run", "-i", "--rm", "ghcr.io/imrans110/sunspec-collector:latest", "--mcp", "http://10.0.4.2:9090"]
    }
  }
}
```

The release archives are built for Linux. On macOS or Windows, use SSH or Docker, or build the
binary with `cargo build --release -p collector-app` (see [Building & Packaging](../build.md)).

## Security

- **Read-only by design:** the bridge has no way to write to the collector or to a device, and
  probing devices from the assistant is not offered.
- **What the assistant sees:** every device's address, models and latest readings. Serial numbers
  are part of model 1's readings.
- **The metrics port has no authentication.** Anyone who can reach it can read the same data the
  bridge does. Prefer SSH to the gateway over opening the port to other networks, or limit it with a
  firewall.

## Troubleshooting

| Message | Cause |
|---------|-------|
| `could not reach the collector at ...` | Wrong URL or port, the collector is not running, or a firewall blocks the metrics port. Check with `curl http://<host>:9090/healthz` from where the bridge runs. |
| `the collector does not supervise ...` | The device key is not one the collector polls; the message lists the known ones. Keys are `host/unit` exactly as `list_devices` shows them. |
| `there is no bundled definition for model ...` | The model ID is not an official SunSpec model the collector bundles. |
| The assistant shows no tools | It could not start the command. Run the same command in a terminal: it should wait silently for input (stop it with Ctrl+D). Errors go to stderr; set `RUST_LOG=debug` for more. |
