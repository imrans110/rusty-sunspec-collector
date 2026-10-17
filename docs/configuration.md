# Configuration

The collector reads its settings from built-in defaults, an optional config file (TOML or JSON)
and `SUNSPEC_*` environment variables. Every setting has a default, so an empty configuration is
valid: it scans `192.168.1.0/24` on port 502 and, with no Kafka brokers set, only logs records.

Two example files ship with the documentation:

- [`config.example.toml`](config.example.toml): a config file with every section.
- [`sunspec-collector.env`](sunspec-collector.env): an environment file, used by the systemd unit
  as `/etc/sunspec-collector.env`.

---

## Load order

Values are resolved in this order; a later source overrides an earlier one:

```
1. Built-in defaults
2. Config file (TOML or JSON)
3. Environment variables (SUNSPEC_*)
```

A typical deployment keeps a config file per site and overrides a few values per host or container
with environment variables.

### Choosing the config file

```sh
collector-app --config /etc/sunspec-collector.toml
collector-app --config=/etc/sunspec-collector.toml
SUNSPEC_CONFIG=/etc/sunspec-collector.toml collector-app
```

`--config` takes precedence over `SUNSPEC_CONFIG`. A file whose name ends in `.json` is parsed as
JSON; any other file is parsed as TOML. With neither option set, no file is read and only the
defaults and environment variables apply. A file that is named but cannot be read is an error.

---

## Strict validation

Configuration errors stop the collector at startup with exit status 1. Nothing falls back to a
default silently. The following are errors:

- Unknown keys or sections in the config file, and values of the wrong type (for example a string
  where a number is expected, or `unit_id = 300`).
- A `SUNSPEC_*` variable whose value cannot be parsed. The error names the variable.
- An invalid static device entry (empty host, host containing whitespace, unit ID outside 0–255).
- `kafka.compression` or `kafka.acks` values outside the supported lists, and `kafka.acks` of `0`
  or `1` while `kafka.enable_idempotence` is true (the default).
- Out-of-range values: zero ports, intervals, timeouts and sizes where at least 1 is required,
  `modbus.max_batch_size` outside 1–125, an invalid CIDR subnet, an empty unit ID list, an invalid
  Kafka topic name, an empty broker list.
- Model settings (see [Polling profiles](#polling-profiles)): a model ID outside 1–65534, an empty
  `include_models`, a model in both `include_models` and `exclude_models`, a `model_intervals_s`
  entry of 0, and a static device listed twice.
- A `framing` other than `tcp` or `rtu`, and static devices behind the same RTU converter (same
  host and port, `framing = "rtu"`) with different `request_timeout_ms` or `max_batch_size`: they
  share one connection.

A misspelled key in the file:

```toml
[poller]
poll_intervall_ms = 1000
```

```text
Error: load config failed

Caused by:
    0: parse toml config /etc/sunspec-collector.toml
    1: TOML parse error at line 2, column 1
         |
       2 | poll_intervall_ms = 1000
         | ^^^^^^^^^^^^^^^^^
       unknown field `poll_intervall_ms`, expected one of `poll_interval_ms`, `request_timeout_ms`, `jitter_ms`, `stall_timeout_ms`, `include_models`, `exclude_models`, `model_intervals_s`
```

An unparsable environment variable:

```text
$ SUNSPEC_POLL_INTERVAL_MS=1s collector-app
Error: load config failed

Caused by:
    SUNSPEC_POLL_INTERVAL_MS="1s" is invalid: invalid digit found in string
```

A value that parses but is not allowed:

```text
$ SUNSPEC_KAFKA_COMPRESSION=brotli collector-app
Error: config validation failed

Caused by:
    kafka.compression must be one of none, gzip, snappy, lz4, zstd, got "brotli"
```

`load config failed` covers reading and parsing (file syntax, unknown keys, types, environment
values, static devices). `config validation failed` covers range and value checks on the merged
result.

Settings are also checked against each other. An idempotent producer needs `acks=all`, so this
is rejected before the Kafka client is created:

```text
$ SUNSPEC_KAFKA_ACKS=1 collector-app
Error: config validation failed

Caused by:
    kafka.acks must be all (or -1) while kafka.enable_idempotence is true
```

Two startup errors come after validation: `kafka publisher init failed` if librdkafka rejects a
setting when the producer is created, and `bind metrics port <port>` if the HTTP port cannot be
bound (see [`[metrics]`](#metrics)).

### Empty environment variables

A variable that is set to an empty string counts as set. `SUNSPEC_POLL_INTERVAL_MS=` is a parse
error and `SUNSPEC_KAFKA_BROKERS=` fails validation. Comment out variables you do not want to set.
The exceptions are lists: an empty `SUNSPEC_STATIC_DEVICES` is an empty list, which selects subnet
mode; an empty `SUNSPEC_INCLUDE_MODELS` means every model; and an empty `SUNSPEC_EXCLUDE_MODELS` or
`SUNSPEC_MODEL_INTERVALS_S` means none.

### Deprecated keys

These keys are still accepted so that existing files and environments keep loading. They have no
effect; the collector logs a warning at startup when one is set.

| File key | Env var | Replacement |
|----------|---------|-------------|
| `sunspec.discovery_register_count` | `SUNSPEC_DISCOVERY_REG_COUNT` | None. Models are found by walking the SunSpec model chain one header at a time. |
| `modbus.timeout_ms` | `SUNSPEC_MODBUS_TIMEOUT_MS` | `poller.request_timeout_ms` / `SUNSPEC_REQUEST_TIMEOUT_MS`, which sets the connect and read timeout for device sessions. |

---

## Reference

Durations are integers in the unit given by the key name (`_ms`, `_s`). "≥ 1" means 0 is rejected.
A dash in the env var column means the setting can only be set in the config file.

### `[discovery]`

The device set comes from either a static list or a subnet scan. When `static_devices` is
non-empty, no scan runs and `subnet`, `max_concurrency`, `per_host_timeout_ms`, `unit_ids` and
`rescan_interval_s` are not used (they are still validated).

| File key | Env var | Default | Notes |
|----------|---------|---------|-------|
| `discovery.subnet` | `SUNSPEC_SUBNET` | `192.168.1.0/24` | IPv4 CIDR, prefix 0–32. Every host address is probed with a TCP connect on the Modbus port; network and broadcast addresses are skipped for prefixes up to /30. |
| `discovery.port` | `SUNSPEC_PORT` | `502` | Modbus TCP port for probing and polling, 1–65535. Same setting as `modbus.port`. |
| `discovery.max_concurrency` | — | `64` | Probes in flight at once during a scan. ≥ 1. |
| `discovery.per_host_timeout_ms` | — | `200` | TCP connect timeout for each probe. ≥ 1. Raise it on high-latency links. |
| `discovery.unit_ids` | `SUNSPEC_DISCOVERY_UNIT_IDS` | `[1]` | Unit IDs polled on every host the scan finds; one device per host and unit ID. Each 0–255, at least one. Env form: comma list, e.g. `1,2,3`. |
| `discovery.rescan_interval_s` | `SUNSPEC_RESCAN_INTERVAL_S` | `600` | Time between scans. New hosts get pollers; hosts that disappear keep theirs and are retried. ≥ 1. |
| `discovery.static_devices` | `SUNSPEC_STATIC_DEVICES` | empty | Fixed device list; see [Static devices](#static-devices). Started once at startup. |

### `[poller]`

| File key | Env var | Default | Notes |
|----------|---------|---------|-------|
| `poller.poll_interval_ms` | `SUNSPEC_POLL_INTERVAL_MS` | `1000` | Time between poll cycles of one device. A cycle reads every polled model that is due, which by default is every discovered model. If a cycle overruns, missed ticks are skipped. ≥ 1. |
| `poller.request_timeout_ms` | `SUNSPEC_REQUEST_TIMEOUT_MS` | `1000` | TCP connect timeout and per-request Modbus read timeout for device sessions. ≥ 1. |
| `poller.jitter_ms` | `SUNSPEC_JITTER_MS` | `0` | Each device's schedule starts after a fixed, device-specific offset between 0 and this value, so devices are not all read at the same moment. `0` = no offset. |
| `poller.stall_timeout_ms` | `SUNSPEC_STALL_TIMEOUT_MS` | max(10 × poll interval, 30000) | A session with no successful read for this long is ended and restarted (counted in `sunspec_poller_stalls_total`). Cycles in which no model is due do not count. ≥ 1 when set. |
| `poller.include_models` | `SUNSPEC_INCLUDE_MODELS` | unset (every model) | Poll only these models, e.g. `[1, 103, 160]`; the rest of the chain is discovered but never read. Model IDs 1–65534, at least one. Env form: comma list; empty means every model. |
| `poller.exclude_models` | `SUNSPEC_EXCLUDE_MODELS` | empty | Never poll these models. A model cannot also be in `include_models`. Env form: comma list. |
| `poller.model_intervals_s` | `SUNSPEC_MODEL_INTERVALS_S` | empty | Read these models at most once per interval instead of every cycle: a table of model ID = seconds (≥ 1). Env form: `1=3600,120=3600`. See [Polling profiles](#polling-profiles). |

A sample that cannot be queued for ingest within one poll interval is dropped and counted in
`sunspec_samples_dropped_total{reason="channel_full"}`, so a slow buffer never blocks polling.

A session also ends after 10 consecutive poll cycles in which at least one read failed. Ended
sessions are retried with the backoff described under `runtime.respawn_delay_ms`.

### `[modbus]`

| File key | Env var | Default | Notes |
|----------|---------|---------|-------|
| `modbus.port` | `SUNSPEC_PORT` | `502` | Same setting as `discovery.port`. If both are in the file, `modbus.port` wins; `SUNSPEC_PORT` overrides both. |
| `modbus.max_batch_size` | `SUNSPEC_MAX_BATCH_SIZE` | `125` | Most registers per read request; longer models are split into several requests. 1–125 (125 is the Modbus protocol limit); other values are rejected with `modbus.max_batch_size must be between 1 and 125 (Modbus protocol limit)`. Lower it for devices that reject large reads. |
| `modbus.timeout_ms` | `SUNSPEC_MODBUS_TIMEOUT_MS` | — | **Deprecated, ignored** with a warning. Use `poller.request_timeout_ms`. |
| `modbus.retry_count` | — | `2` | Retries per request after the first attempt. Timeouts, transport and connect errors are retried; Modbus exceptions are not, except "server device busy". `0` = no retries. |
| `modbus.retry_backoff_ms` | — | `100` | Delay before the first retry; doubles on each further retry. ≥ 1. |
| `modbus.retry_max_backoff_ms` | — | `2000` | Upper bound for the retry delay. ≥ 1. |
| `modbus.inter_read_delay_ms` | — | unset | Pause between the requests of one split read, for devices that need it. ≥ 1 when set. |
| `modbus.framing` | `SUNSPEC_MODBUS_FRAMING` | `tcp` | `tcp` for Modbus TCP, or `rtu` for Modbus RTU frames over TCP, as a transparent serial-to-Ethernet converter in front of an RS-485 bus passes them. Units on the same host and port then share one connection; see [Serial devices through a converter](ops.md#serial-devices-through-a-converter). |

With the defaults, one request to an unresponsive device takes up to three timeouts plus 300 ms of
backoff before it fails. Keep `poller.request_timeout_ms × (retry_count + 1)` well below the poll
interval if you want failed cycles to be detected quickly.

After a transport error or timeout the client drops the connection and reconnects on the next
read. A Modbus exception keeps the connection.

### `[sunspec]`

| File key | Env var | Default | Notes |
|----------|---------|---------|-------|
| `sunspec.base_address` | `SUNSPEC_BASE_ADDRESS` | `40000` | Register address probed first for the `SunS` marker. The other standard bases (40000, 0, 50000) are probed after it. 0–65535. |
| `sunspec.discovery_register_count` | `SUNSPEC_DISCOVERY_REG_COUNT` | — | **Deprecated, ignored** with a warning. |

Models are rediscovered every time a device session starts, so a firmware change that alters the
model chain is picked up after the next reconnect.

### `[buffer]`

See [Buffer Maintenance](buffer_maintenance.md) for how the buffer works and how to size it.

| File key | Env var | Default | Notes |
|----------|---------|---------|-------|
| `buffer.path` | `SUNSPEC_BUFFER_PATH` | `sunspec-buffer.sqlite` | SQLite file, created if missing. A relative path is relative to the working directory. The Docker image's working directory is `/app/data`, so the default there is `/app/data/sunspec-buffer.sqlite`; the systemd env file sets `/var/lib/sunspec-collector/buffer.sqlite`. |
| `buffer.batch_size` | `SUNSPEC_BUFFER_BATCH_SIZE` | `100` | Rows read and published per uplink batch. ≥ 1. |
| `buffer.drain_interval_ms` | `SUNSPEC_BUFFER_DRAIN_MS` | `500` | Pause before the next batch once the backlog is empty (the last batch was not full). Also the minimum delay after a failed batch. ≥ 1. |
| `buffer.max_rows` | `SUNSPEC_BUFFER_MAX_ROWS` | `1000000` | Retention: the oldest rows beyond this count are deleted. Checked every 10 s. ≥ 1. |
| `buffer.max_age_s` | `SUNSPEC_BUFFER_MAX_AGE_S` | `604800` (7 days) | Retention: rows written longer ago than this are deleted. Checked every 10 s. ≥ 1. |
| `buffer.shutdown_drain_timeout_ms` | `SUNSPEC_SHUTDOWN_DRAIN_TIMEOUT_MS` | `5000` | On stop, keep publishing buffered rows for up to this long. `0` = stop at once. |

In Docker, the image runs in `/app/data`, the directory to mount a volume on. The image does not
set `SUNSPEC_BUFFER_PATH`, so `buffer.path` from a config file applies. A relative `buffer.path`
lands in `/app/data`; an absolute path outside `/app/data` is not in the volume and is lost when
the container is removed.

### `[kafka]`

| File key | Env var | Default | Notes |
|----------|---------|---------|-------|
| `kafka.brokers` | `SUNSPEC_KAFKA_BROKERS` | unset | Comma-separated bootstrap servers, e.g. `kafka-1:9092,kafka-2:9092`. Must be non-empty when set. **Unset means no Kafka:** each record is encoded, logged at debug level, and removed from the buffer as if delivered. |
| `kafka.topic` | `SUNSPEC_KAFKA_TOPIC` | `sunspec.telemetry` | 1–249 characters from `A-Z a-z 0-9 . _ -`, not `.` or `..`. |
| `kafka.client_id` | `SUNSPEC_KAFKA_CLIENT_ID` | `sunspec-collector` | librdkafka `client.id`. |
| `kafka.acks` | `SUNSPEC_KAFKA_ACKS` | `all` | One of `0`, `1`, `-1`, `all`. `0` and `1` require `enable_idempotence = false`; otherwise validation fails. |
| `kafka.compression` | `SUNSPEC_KAFKA_COMPRESSION` | `zstd` | One of `none`, `gzip`, `snappy`, `lz4`, `zstd`. |
| `kafka.timeout_ms` | `SUNSPEC_KAFKA_TIMEOUT_MS` | `5000` | librdkafka `message.timeout.ms`: how long the producer tries to deliver a record before it is reported as undelivered. Undelivered rows stay in the buffer. ≥ 1. |
| `kafka.enable_idempotence` | `SUNSPEC_KAFKA_IDEMPOTENCE` | `true` | librdkafka `enable.idempotence`. The env value must be exactly `true` or `false`. |

The producer also sets `linger.ms=50`; this is not configurable.

#### Connection security (TLS and SASL)

| File key | Env var | Default | Notes |
|----------|---------|---------|-------|
| `kafka.security_protocol` | `SUNSPEC_KAFKA_SECURITY_PROTOCOL` | `plaintext` | One of `plaintext`, `ssl`, `sasl_plaintext`, `sasl_ssl` (lowercase). `sasl_plaintext` logs a warning: credentials and data are not encrypted. |
| `kafka.ssl_ca_location` | `SUNSPEC_KAFKA_SSL_CA_LOCATION` | system CAs | PEM file or directory used to verify the broker. Must exist. |
| `kafka.ssl_certificate_location` | `SUNSPEC_KAFKA_SSL_CERTIFICATE_LOCATION` | unset | Client certificate (PEM) for mutual TLS. Must exist; set together with the key. |
| `kafka.ssl_key_location` | `SUNSPEC_KAFKA_SSL_KEY_LOCATION` | unset | Client private key (PEM) for mutual TLS. Must exist; set together with the certificate. |
| `kafka.ssl_key_password` / `kafka.ssl_key_password_file` | `SUNSPEC_KAFKA_SSL_KEY_PASSWORD` / `SUNSPEC_KAFKA_SSL_KEY_PASSWORD_FILE` | unset | Passphrase of an encrypted private key, given directly or as a file. |
| `kafka.sasl_mechanism` | `SUNSPEC_KAFKA_SASL_MECHANISM` | unset | `PLAIN`, `SCRAM-SHA-256` or `SCRAM-SHA-512`. Required with a `sasl_*` protocol. |
| `kafka.sasl_username` | `SUNSPEC_KAFKA_SASL_USERNAME` | unset | Required with a `sasl_*` protocol. |
| `kafka.sasl_password` / `kafka.sasl_password_file` | `SUNSPEC_KAFKA_SASL_PASSWORD` / `SUNSPEC_KAFKA_SASL_PASSWORD_FILE` | unset | Required with a `sasl_*` protocol. Prefer the `_file` form. |

Rules checked at startup:

- `ssl_*` settings need `ssl` or `sasl_ssl`; `sasl_*` settings need `sasl_plaintext` or
  `sasl_ssl`. Setting them with another protocol is an error, so a half-applied security
  configuration cannot silently fall back to plaintext.
- `*_file` settings read a file that contains only the secret (a trailing newline is removed).
  Setting both a secret and its `_file` form is an error, as is an empty file.
- Secrets never appear in logs: they are redacted in debug output.
- librdkafka loads the CA and client certificate when the producer is created, so an unreadable
  or invalid PEM file stops the collector at startup (`kafka publisher init failed`).

GSSAPI (Kerberos) and OAUTHBEARER are not supported. Certificates are read at startup; restart the
collector after rotating them (the buffer keeps unsent data across the restart). See
[Securing the Kafka connection](ops.md#securing-the-kafka-connection) for deployment examples.

### `[metrics]`

| File key | Env var | Default | Notes |
|----------|---------|---------|-------|
| `metrics.port` | `SUNSPEC_METRICS_PORT` | `9090` | Port for `/metrics` and `/healthz`, bound on all IPv4 interfaces (`0.0.0.0`). `0` lets the OS pick a free port; the chosen address is logged. If the port cannot be bound (for example, it is in use), the collector exits at startup with `bind metrics port <port>`. |

The Docker image's `HEALTHCHECK` calls `http://localhost:${SUNSPEC_METRICS_PORT:-9090}/healthz`,
so it follows the port when you set it with `SUNSPEC_METRICS_PORT`. If you set `metrics.port` in a
config file instead, or use `0`, the health check still probes 9090 and fails; set the port with
the environment variable in containers.

### `[runtime]`

| File key | Env var | Default | Notes |
|----------|---------|---------|-------|
| `runtime.channel_capacity` | `SUNSPEC_CHANNEL_CAPACITY` | `256` | Samples that can wait between the pollers and the ingest task. ≥ 1. |
| `runtime.respawn_delay_ms` | `SUNSPEC_RESPAWN_DELAY_MS` | `1000` | Base delay before a failed device session is retried. The delay doubles with each consecutive failure, is capped at 60 s, and gets up to 10 % device-specific jitter. It resets after a session that read successfully. ≥ 1. |

### Environment-only settings

| Env var | Default | Notes |
|---------|---------|-------|
| `SUNSPEC_CONFIG` | unset | Path of the config file, if `--config` is not given. |
| `RUST_LOG` | `info` | Log filter; see [Logging](#logging). |
| `SUNSPEC_LOG_FORMAT` | `text` | `text` or `json`; see [Logging](#logging). |

---

## Static devices

A static device is a host and a Modbus unit ID. Each entry becomes one supervised poller, keyed
`host/unit` (for example `192.168.1.20/1`). That key is used in logs, in the `device` metric label
and as the Kafka record key.

In a config file, use an array of tables (TOML) or an array of objects (JSON). `unit_id` is
optional and defaults to 1:

```toml
[[discovery.static_devices]]
ip = "192.168.1.20"

[[discovery.static_devices]]
ip = "modbus-gw.example.net"
unit_id = 3
```

In the environment, use a comma-separated list of `host[:unit]`:

```sh
SUNSPEC_STATIC_DEVICES=192.168.1.20,192.168.1.21:1,modbus-gw.example.net:3
```

Rules for both forms:

- The host is an IPv4 address or a hostname. Hostnames are resolved on every connect, and each
  resolved address is tried in turn.
- The unit ID is 0–255 and defaults to 1.
- The host must not be empty or contain whitespace.
- In the environment form, spaces around entries and empty entries are ignored. The host and unit
  are split at the last `:`, so this form cannot express an IPv6 address literal.

Gateways expose several units on one host. List each unit as its own entry:

```sh
SUNSPEC_STATIC_DEVICES=192.168.1.100:1,192.168.1.100:2,192.168.1.100:3
```

`discovery.unit_ids` / `SUNSPEC_DISCOVERY_UNIT_IDS` apply only to hosts found by a subnet scan,
not to static entries.

Each host and unit ID may be listed only once. Entries in a config file can also carry
[per-device settings](#per-device-settings).

---

## Polling profiles

By default every discovered model is read on every poll cycle. Models that rarely change, such as
common (1), nameplate (120) and settings (121), can be read less often, and models you do not need
can be skipped. Both cut Modbus traffic and Kafka volume.

```toml
[poller]
poll_interval_ms = 1000
exclude_models = [123]            # never read the immediate controls model

[poller.model_intervals_s]        # model ID = seconds
1 = 3600                          # common: hourly
120 = 3600                        # nameplate: hourly
121 = 600                         # settings: every 10 minutes
```

The same in the environment (for the systemd environment file or Docker):

```sh
SUNSPEC_MODEL_INTERVALS_S=1=3600,120=3600,121=600
SUNSPEC_EXCLUDE_MODELS=123
```

How the schedule works:

- Every polled model is read on the first cycle of a session, so a device's identity and settings
  are published as soon as it connects. After a successful read, a model with an interval is read
  again on the first cycle at least that long later. A failed read is retried on the next cycle.
- Intervals are counted in whole poll cycles: with `poll_interval_ms = 1000`, `121 = 600` reads
  model 121 every 600th cycle.
- A new session (after a reconnect, a stall or repeated failures) reads every model again, so a
  slow model can be read more often than its interval while a device is unstable.
- A cycle in which nothing is due does not contact the device and does not count towards the stall
  timeout.
- `include_models` and `exclude_models` filter the discovered chain. Listing a model that a device
  does not have is not an error. If nothing on a device is selected, its session ends with
  `none of the discovered models [...] is selected for polling` and is retried with the usual
  backoff; the chain is rediscovered each time, so a firmware update that adds a model is noticed.
- Discovery still reads every model header. The `discovered SunSpec models` log line lists the
  whole chain (`models`), the models that are polled (`polled`) and the slow ones (`slow`).

### Per-device settings

A `[[discovery.static_devices]]` entry in a config file can override these settings for that
device:

| Key | Overrides | Notes |
|-----|-----------|-------|
| `poll_interval_ms` | `poller.poll_interval_ms` | ≥ 1. The device's default stall timeout follows it. |
| `request_timeout_ms` | `poller.request_timeout_ms` | ≥ 1. |
| `base_address` | `sunspec.base_address` | Probed first; the standard bases follow. |
| `max_batch_size` | `modbus.max_batch_size` | 1–125. |
| `framing` | `modbus.framing` | `tcp` or `rtu`. |
| `include_models` | `poller.include_models` | Replaces the global list. |
| `exclude_models` | `poller.exclude_models` | Replaces the global list; `[]` clears it for this device. |
| `model_intervals_s` | `poller.model_intervals_s` | Merged model by model; the device's entries win. |

```toml
[poller.model_intervals_s]
1 = 3600
120 = 3600

[[discovery.static_devices]]
ip = "192.168.1.30"               # a slow meter behind a serial gateway
unit_id = 2
poll_interval_ms = 5000
request_timeout_ms = 3000
max_batch_size = 60

[[discovery.static_devices]]
ip = "192.168.1.40"               # an inverter at unit 126 with MPPT data every 10 s
unit_id = 126
include_models = [1, 120, 160]
model_intervals_s = { 1 = 86400, 160 = 10 }   # merged: 1 daily, 120 hourly, 160 every 10 s
```

- Per-device settings exist only in the config file. `SUNSPEC_STATIC_DEVICES` replaces the file's
  device list together with these settings, and logs a warning when it does.
- Devices found by a subnet scan use the global settings.
- After its overrides are applied, a device's `include_models` and `exclude_models` may not share a
  model; the error names the device.

---

## Logging

Logging is configured with environment variables only.

| Variable | Values | Notes |
|----------|--------|-------|
| `RUST_LOG` | [`tracing` filter directives](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html) | Default `info`. Examples: `warn`, `info,poller_actor=debug`, `info,modbus_client=debug,avro_kafka=debug`. If unset or unparsable, `info` is used. |
| `SUNSPEC_LOG_FORMAT` | `text`, `json` | Default `text`. `json` writes one JSON object per line for log shippers. Any other value is a startup error. |

Logs go to standard output. ANSI colours are used only when standard output is a terminal.

Steady-state operation is quiet at `info`: per-cycle and per-batch messages are logged at debug.
Useful debug targets:

- `poller_actor`: poll cycle timings.
- `modbus_client`: every read.
- `collector_app::uplink`: every uplink batch.
- `avro_kafka`: records handled by the log-only sink when no brokers are set.
- `discovery`: every probe during a subnet scan.

---

## Command-line options

| Option | Effect |
|--------|--------|
| `--config PATH`, `--config=PATH` | Read this config file. Overrides `SUNSPEC_CONFIG`. |
| `--print-schema` | Print the Avro schema of the published records as JSON and exit. No configuration is loaded. |
| `--probe HOST[:UNIT]` | Read one device's SunSpec map once, print it as JSON and exit; see [Probing a device](ops.md#probing-a-device). Uses the port, timeouts, base address and static-device overrides from the configuration. |
| `--keep-identifiers` | With `--probe`: keep serial numbers and network addresses instead of blanking them. |
| `--version`, `-V` | Print `collector-app <version>` and exit. |
| `--help`, `-h` | Print a short usage summary and exit. |

Other arguments are ignored.

---

## Complete examples

Both examples below pass strict validation.

### TOML

```toml
# /etc/sunspec-collector.toml
# A non-empty static_devices list selects static mode, so the subnet scan
# settings below are validated but not used.

[discovery]
subnet = "192.168.1.0/24"
port = 502                        # also sets modbus.port
max_concurrency = 64
per_host_timeout_ms = 200
unit_ids = [1]
rescan_interval_s = 600

[[discovery.static_devices]]
ip = "192.168.1.20"               # unit_id defaults to 1

[[discovery.static_devices]]
ip = "modbus-gw.example.net"
unit_id = 2
poll_interval_ms = 2000           # per-device settings: see Polling profiles
max_batch_size = 60

[poller]
poll_interval_ms = 1000
request_timeout_ms = 1000
jitter_ms = 250
stall_timeout_ms = 30000
exclude_models = [123]

[poller.model_intervals_s]
1 = 3600
120 = 3600
121 = 600

[modbus]
max_batch_size = 125
retry_count = 2
retry_backoff_ms = 100
retry_max_backoff_ms = 2000
# inter_read_delay_ms = 5         # unset by default: no pause between split reads

[sunspec]
base_address = 40000

[buffer]
path = "/var/lib/sunspec-collector/buffer.sqlite"
batch_size = 100
drain_interval_ms = 500
max_rows = 1000000
max_age_s = 604800
shutdown_drain_timeout_ms = 5000

[kafka]
brokers = "kafka-1.example.net:9092,kafka-2.example.net:9092"
topic = "sunspec.telemetry"
client_id = "sunspec-collector"
acks = "all"
compression = "zstd"
timeout_ms = 5000
enable_idempotence = true

[metrics]
port = 9090

[runtime]
channel_capacity = 256
respawn_delay_ms = 1000
```

### JSON

Subnet mode, scanning two unit IDs per host, with a slower poll interval and a larger uplink
batch:

```json
{
  "discovery": {
    "subnet": "10.20.0.0/24",
    "port": 502,
    "max_concurrency": 64,
    "per_host_timeout_ms": 200,
    "unit_ids": [1, 2],
    "rescan_interval_s": 600
  },
  "poller": {
    "poll_interval_ms": 5000,
    "request_timeout_ms": 1500,
    "jitter_ms": 1000,
    "model_intervals_s": { "1": 3600, "120": 3600, "121": 600 }
  },
  "modbus": {
    "max_batch_size": 100,
    "retry_count": 2,
    "retry_backoff_ms": 100,
    "retry_max_backoff_ms": 2000,
    "inter_read_delay_ms": 10
  },
  "sunspec": {
    "base_address": 40000
  },
  "buffer": {
    "path": "/var/lib/sunspec-collector/buffer.sqlite",
    "batch_size": 500,
    "drain_interval_ms": 500,
    "max_rows": 2000000,
    "max_age_s": 172800,
    "shutdown_drain_timeout_ms": 5000
  },
  "kafka": {
    "brokers": "kafka.example.net:9092",
    "topic": "site-a.sunspec.telemetry",
    "client_id": "sunspec-collector-site-a",
    "acks": "all",
    "compression": "zstd",
    "timeout_ms": 10000,
    "enable_idempotence": true
  },
  "metrics": {
    "port": 9090
  },
  "runtime": {
    "channel_capacity": 512,
    "respawn_delay_ms": 1000
  }
}
```

Save it with a `.json` extension so it is parsed as JSON.

---

## Common setups

### No Kafka (records are only logged)

```sh
SUNSPEC_STATIC_DEVICES=192.168.1.20:1 RUST_LOG=info,avro_kafka=debug collector-app
```

### Subnet scan with Kafka

```sh
SUNSPEC_SUBNET=10.0.0.0/24 \
SUNSPEC_KAFKA_BROKERS=kafka.internal:9092 \
SUNSPEC_KAFKA_TOPIC=solar.telemetry \
collector-app
```

### Docker with a config file

Mount the file and point `SUNSPEC_CONFIG` at it (or pass `--config /app/collector.toml` after
the image name, since the image's entrypoint is `/app/collector-app`):

```sh
docker run -d --name sunspec-collector \
  -p 9090:9090 \
  -v "$(pwd)/collector.toml:/app/collector.toml:ro" \
  -v sunspec-data:/app/data \
  -e SUNSPEC_CONFIG=/app/collector.toml \
  sunspec-collector
```

### Simulator on a non-standard port

```sh
cargo run -p sunspec-sim -- --bind 127.0.0.1:1502 --units 1,2
SUNSPEC_STATIC_DEVICES=127.0.0.1:1,127.0.0.1:2 SUNSPEC_PORT=1502 cargo run -p collector-app
```
