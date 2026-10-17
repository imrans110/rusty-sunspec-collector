# Operations

This guide covers deploying, running and monitoring the collector: Docker and systemd deployment,
the HTTP endpoints, metrics and alerts, shutdown behaviour, troubleshooting, and upgrade notes for
this release.

Related pages: [Configuration](configuration.md) for every setting,
[Buffer Maintenance](buffer_maintenance.md) for the SQLite buffer,
[Building & Packaging](build.md) for release builds, and [Architecture](ARCHITECTURE.md) for how
the parts fit together.

---

## Deployment options

| Option | Use it for | Buffer location |
|--------|-----------|-----------------|
| Docker image | Gateways and servers that run containers | `/app/data/sunspec-buffer.sqlite` in a volume |
| Docker Compose stack | Local testing with Kafka, a simulator and Prometheus | `collector-data` volume |
| systemd service | Bare-metal or VM Linux hosts | `/var/lib/sunspec-collector/buffer.sqlite` |

Run one collector per buffer file. Two processes on one SQLite file read the same rows, so
records are published twice, and they can fail with "database is locked".

### Docker

#### Images

Releases publish multi-arch (`linux/amd64`, `linux/arm64`) images to GHCR:

```sh
docker pull ghcr.io/imrans110/sunspec-collector:latest     # or a version, e.g. :0.2.0
docker pull ghcr.io/imrans110/sunspec-simulator:latest
```

To build them from source instead:

```sh
docker build -t sunspec-collector .                                 # collector (default target)
docker build --target simulator -t sunspec-simulator .              # SunSpec device simulator
```

The examples below use the local tag `sunspec-collector`; substitute
`ghcr.io/imrans110/sunspec-collector:<version>` for a published image. The Dockerfile builds both binaries in `rust:1.88-slim-bookworm` and copies them into
`debian:bookworm-slim`. The `collector` image:

- runs as the non-root user `collector`;
- runs in the working directory `/app/data`, so the default relative buffer path is
  `/app/data/sunspec-buffer.sqlite`. Mount a volume at `/app/data`. The image does not set
  `SUNSPEC_BUFFER_PATH`, so `buffer.path` from a config file applies; keep it relative or under
  `/app/data` so the buffer stays in the volume;
- exposes port 9090;
- has a `HEALTHCHECK` that runs `curl -fsS "http://localhost:${SUNSPEC_METRICS_PORT:-9090}/healthz"`
  every 30 s (3 s timeout, 20 s start period, 3 retries). It follows `SUNSPEC_METRICS_PORT`, but
  not `metrics.port` from a config file, so set the port with the environment variable in a
  container.

#### Run

```sh
docker run -d \
  --name sunspec-collector \
  --restart unless-stopped \
  --stop-timeout 30 \
  -p 9090:9090 \
  -e SUNSPEC_SUBNET=192.168.1.0/24 \
  -e SUNSPEC_KAFKA_BROKERS=kafka:9092 \
  -v sunspec-data:/app/data \
  sunspec-collector
```

`--stop-timeout 30` gives the graceful shutdown the same 30 s the systemd unit allows. Docker's
default is 10 s; see [Graceful shutdown](#graceful-shutdown).

With an environment file:

```sh
docker run -d --name sunspec-collector -p 9090:9090 \
  --env-file collector.env \
  -v sunspec-data:/app/data \
  sunspec-collector
```

Write one `VAR=value` per line in environment files and put comments on their own lines. Docker's
`--env-file` and systemd's `EnvironmentFile=` both keep text after `#` on a value line as part of
the value, and the collector rejects a value such as `9090   # metrics port` at startup.

To use a config file, mount it and point `SUNSPEC_CONFIG` at it (the image has no entrypoint, so
arguments after the image name replace the command):

```sh
docker run -d --name sunspec-collector -p 9090:9090 \
  -v "$(pwd)/collector.toml:/app/collector.toml:ro" \
  -e SUNSPEC_CONFIG=/app/collector.toml \
  -v sunspec-data:/app/data \
  sunspec-collector
```

#### Check, inspect, stop

```sh
docker inspect --format='{{.State.Health.Status}}' sunspec-collector
docker logs -f sunspec-collector
docker stop sunspec-collector && docker rm sunspec-collector
```

Docker marks a container `unhealthy` when the health check fails but does not restart it. The
`--restart` policy applies only when the process exits.

### Docker Compose (local stack)

`docker compose up --build` starts:

| Service | Image | Notes |
|---------|-------|-------|
| `kafka` | `apache/kafka:3.9.1` (KRaft) | Plaintext from the host at `localhost:29092`; SASL/PLAIN listener on `kafka:9094` inside the network |
| `simulator` | built from the `simulator` target | Units 1 and 2 on port 1502 |
| `collector` | built from the `collector` target | Polls `simulator:1` and `simulator:2`; publishes over SASL/PLAIN with its password from a Docker secret; port 9090; data in volume `collector-data` |
| `prometheus` | `prom/prometheus:v3.5.0` | `http://localhost:9091`, scraping the collector via `docker/prometheus.yml` |

```sh
curl localhost:9090/healthz
curl -s localhost:9090/metrics | grep '^sunspec_'
docker compose down -v        # stop and remove the volumes, including the buffer
```

The collector service has no `stop_grace_period`, so `docker compose stop` allows the default 10 s.

### systemd

The unit file is [`sunspec-collector.service`](sunspec-collector.service) and the environment
file template is [`sunspec-collector.env`](sunspec-collector.env).

#### From a release

On x86_64 or aarch64 Linux with glibc 2.18 or newer and systemd 235 or newer (Debian 10+,
Ubuntu 18.04+, RHEL 8+, Raspberry Pi OS 10+):

```sh
curl -fsSL https://github.com/imrans110/rusty-sunspec-collector/releases/latest/download/install.sh | sudo sh
sudoedit /etc/sunspec-collector.env        # devices, Kafka brokers
sudo systemctl enable --now sunspec-collector
```

`install.sh` downloads `sunspec-collector-<version>-<target>.tar.gz` and `SHA256SUMS` from the
release, checks the archive's checksum, and installs:

| Path | Notes |
|------|-------|
| `/usr/local/bin/collector-app`, `/usr/local/bin/sunspec-sim` | Replaced atomically, so a running service keeps its old binary until restarted |
| `/etc/systemd/system/sunspec-collector.service` | Replaced on upgrade; a locally edited copy is saved as `.service.bak`. Put local changes in a drop-in (`systemctl edit sunspec-collector`) |
| `/etc/sunspec-collector.env` | Installed only if missing (mode 0600, since it may hold Kafka passwords); upgrades keep your settings |
| `/usr/local/share/doc/sunspec-collector/` | README, CHANGELOG, licenses, `config.example.toml` and the shipped environment file |

It runs `systemctl daemon-reload` but does not start or restart the service; after an upgrade, run
`sudo systemctl restart sunspec-collector`. `SUNSPEC_VERSION=0.2.0` installs a specific release.
To read the script first: `curl -fsSLO <url>`, read `install.sh`, then `sudo sh install.sh`.

To install from a downloaded archive by hand, use the archive's files in the steps below instead
of building.

#### From source

```sh
# 1. Build the release binary (see Building & Packaging) and install it
cargo build --release -p collector-app
sudo install -m 0755 target/release/collector-app /usr/local/bin/collector-app

# 2. Install the unit and the environment file, then edit the environment file
sudo install -m 0644 docs/sunspec-collector.service /etc/systemd/system/sunspec-collector.service
sudo install -m 0644 docs/sunspec-collector.env /etc/sunspec-collector.env
sudoedit /etc/sunspec-collector.env

# 3. Optional: a config file, enabled with SUNSPEC_CONFIG=/etc/sunspec-collector.toml
#    in the environment file
sudo install -m 0644 docs/config.example.toml /etc/sunspec-collector.toml

# 4. Start it
sudo systemctl daemon-reload
sudo systemctl enable --now sunspec-collector
systemctl status sunspec-collector
```

Do not create `/var/lib/sunspec-collector` yourself; systemd creates and owns it (see below).

Keep `SUNSPEC_BUFFER_PATH=/var/lib/sunspec-collector/buffer.sqlite` in the environment file. The
unit sets no working directory and the file system is read-only to the service except for its
state directory, so the default relative buffer path fails with `buffer init failed`.

The service reads the config file as an unprivileged dynamic user, so the file must be
world-readable (mode 0644) and must not be under `/home` (`ProtectHome=true`). systemd reads the
environment file itself, before dropping privileges.

#### What the unit does

| Setting | Effect |
|---------|--------|
| `Type=notify` | systemd marks the service started once the collector has opened the buffer and started its tasks (`READY=1`). |
| `WatchdogSec=30` | The collector pings the watchdog every 15 s, but only while the pipeline is healthy (the same check as `/healthz`). If ingest or the uplink stops checking in for 120 s, the pings stop and systemd stops the service within 30 s; `Restart=always` starts it again. The log shows `pipeline unhealthy; withholding systemd watchdog ping`. |
| `Restart=always`, `RestartSec=5` | Restarts after any exit, including a configuration error, every 5 s. |
| `TimeoutStopSec=30` | Time allowed for the graceful stop after SIGTERM before SIGKILL. |
| `DynamicUser=yes` | Runs as a transient unprivileged user; no account needs to be created. |
| `StateDirectory=sunspec-collector`, `StateDirectoryMode=0750` | systemd creates `/var/lib/sunspec-collector`, owned by the service user, and keeps it across restarts and reboots. With `DynamicUser=yes` the data is stored in `/var/lib/private/sunspec-collector` and `/var/lib/sunspec-collector` is a symlink to it. This is the only writable location. |
| `EnvironmentFile=-/etc/sunspec-collector.env` | Loads the environment file; the leading `-` means a missing file is not an error. |
| `LimitNOFILE=4096` | File descriptor limit. |
| Hardening | `NoNewPrivileges`, `ProtectSystem=strict`, `ProtectHome`, `PrivateTmp`, `PrivateDevices`, `ProtectKernelTunables`, `ProtectKernelModules`, `ProtectControlGroups`, `RestrictAddressFamilies=AF_INET AF_INET6 AF_UNIX`, `RestrictNamespaces`, `LockPersonality`, `MemoryDenyWriteExecute`, `SystemCallArchitectures=native`. |

`ExecStart` passes no arguments. Use `SUNSPEC_CONFIG` in the environment file, or a drop-in
(`systemctl edit sunspec-collector`), to change how the binary is started.

#### Logs

```sh
journalctl -u sunspec-collector -n 200 --no-pager     # recent logs
journalctl -u sunspec-collector -f                    # follow
journalctl -u sunspec-collector -p warning --no-pager # warnings and errors only
```

---

## Securing the Kafka connection

The collector supports TLS, mutual TLS and SASL (`PLAIN`, `SCRAM-SHA-256`, `SCRAM-SHA-512`); the
keys are listed in [Configuration](configuration.md#connection-security-tls-and-sasl). Use
`sasl_ssl` or `ssl` whenever the traffic leaves the site: `sasl_plaintext` sends the password and
the telemetry unencrypted, and the collector logs a warning when it is used.

SCRAM over TLS, with the password in a file:

```toml
[kafka]
brokers = "kafka-1.example.com:9093,kafka-2.example.com:9093"
security_protocol = "sasl_ssl"
ssl_ca_location = "/etc/sunspec-collector/kafka-ca.pem"
sasl_mechanism = "SCRAM-SHA-512"
sasl_username = "site-0042"
sasl_password_file = "/run/credentials/sunspec-collector.service/kafka-password"
```

Mutual TLS instead of SASL: set `security_protocol = "ssl"`, `ssl_ca_location`,
`ssl_certificate_location` and `ssl_key_location` (plus `ssl_key_password_file` if the key is
encrypted).

### Keeping secrets out of config and environment files

Prefer the `*_password_file` settings over putting passwords in the config file or environment:

- **systemd:** store the password root-only on the host and pass it as a credential. Add to the
  unit (or a drop-in from `systemctl edit sunspec-collector`):

  ```ini
  [Service]
  LoadCredential=kafka-password:/etc/sunspec-collector/kafka-password
  Environment=SUNSPEC_KAFKA_SASL_PASSWORD_FILE=/run/credentials/sunspec-collector.service/kafka-password
  ```

  The file is readable only by the service, which also works with `DynamicUser=yes`.
- **Docker / Compose:** mount the password as a secret and point the `_FILE` variable at it, as
  the local stack does:

  ```yaml
  services:
    collector:
      environment:
        SUNSPEC_KAFKA_SASL_PASSWORD_FILE: /run/secrets/kafka_sasl_password
      secrets:
        - kafka_sasl_password
  secrets:
    kafka_sasl_password:
      file: ./kafka-sasl-password
  ```

  The secret file must be readable by the container user (`collector`).

The compose stack's password (`docker/dev-secrets/`) is a development-only value.

### Rotating credentials

Certificates, keys and passwords are read when the collector starts. After rotating them, restart
the service (`systemctl restart sunspec-collector` or `docker compose restart collector`). Unsent
samples stay in the buffer and are published after the restart.

## HTTP endpoints

Both endpoints are served on `metrics.port` (default 9090) on all IPv4 interfaces.

### `/metrics`

Prometheus text format. Every metric has HELP text. Metrics ending in `_seconds` are histograms
with buckets at 0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1, 2.5, 5 and 10 seconds.

```sh
curl -s http://localhost:9090/metrics | grep '^sunspec_'
```

### `/healthz`

Returns 200 while both the ingest task and the uplink have checked in within the last 120 s, and
503 otherwise. The body is the same JSON in both cases:

```json
{"healthy":true,"ingest_age_s":0.7,"uplink_age_s":0.3,"devices_up":2,"devices_total":2}
```

| Field | Meaning |
|-------|---------|
| `healthy` | `true` when both ages are below 120 s. |
| `ingest_age_s` | Seconds since the ingest task last checked in (at least every 5 s); `null` before the first check-in. |
| `uplink_age_s` | Seconds since the uplink last checked in (at least every 30 s plus the time of one publish attempt); `null` before the first check-in. |
| `devices_up` | Devices whose current session has read successfully. |
| `devices_total` | Devices being supervised. |

Device outages do not make the collector unhealthy, because restarting the collector would not
bring a device back. A Kafka outage does not either: the uplink keeps checking in while it backs
off. Use the metrics and alerts below to detect both.

The Docker `HEALTHCHECK` calls this endpoint, and the systemd watchdog uses the same check.

If `metrics.port` cannot be bound (for example, it is already in use), the collector exits at
startup with `Error: bind metrics port <port>` and the operating-system error.

---

## Metrics reference

| Metric | Type | Labels | Meaning |
|--------|------|--------|---------|
| `sunspec_poll_success_total` | counter | `device` | Successful model reads. |
| `sunspec_poll_errors_total` | counter | `device`, `kind` | Failed Modbus operations: connects at session start, reads during model discovery, and model reads during polling. `kind`: `exception`, `timeout`, `transport`, `connect`, `address`. |
| `sunspec_poll_duration_seconds` | histogram | `device` | Time to read the models due in one poll cycle; cycles with nothing due are not recorded. |
| `sunspec_device_up` | gauge | `device` | 0 from the moment the device's poller starts; 1 from the first successful read of a session until the session ends, then 0 again. |
| `sunspec_device_last_success_timestamp_seconds` | gauge | `device` | Unix time of the last successful read. |
| `sunspec_poller_restarts_total` | counter | `device` | Device sessions that ended with an error and were scheduled for retry. |
| `sunspec_poller_stalls_total` | counter | `device` | Sessions ended by stall detection (no successful read within `poller.stall_timeout_ms`). |
| `sunspec_devices_known` | gauge | — | Devices being supervised. |
| `sunspec_samples_dropped_total` | counter | `reason` = `channel_full` | Samples dropped before reaching the buffer. |
| `sunspec_buffer_writes_total` | counter | — | Samples written to SQLite. |
| `sunspec_buffer_dropped_total` | counter | `reason` = `max_rows`, `max_age`, `corrupt`, `write_error` | Rows or samples dropped without being published. |
| `sunspec_buffer_rows` | gauge | — | Rows waiting in the buffer. Updated every 10 s. |
| `sunspec_buffer_oldest_age_seconds` | gauge | — | Age of the oldest buffered row; 0 when empty. Updated every 10 s. |
| `sunspec_uplink_records_total` | counter | — | Records acknowledged by Kafka. |
| `sunspec_uplink_errors_total` | counter | — | Delivery attempts Kafka did not acknowledge. The records stay buffered and are retried, so one record can count more than once. |
| `sunspec_uplink_publish_seconds` | histogram | — | Time to publish one batch and receive its acknowledgements. |

Notes:

- The `device` label is `host/unit`, for example `192.168.1.20/1`.
- `sunspec_device_up` is published with the value 0 as soon as a device's poller starts, so a
  device that has never answered has a series with the value 0. Most other series appear when
  they are first recorded; in particular `sunspec_device_last_success_timestamp_seconds` appears
  only after a device's first successful read.
- `sunspec_poll_errors_total` counts a failed connect at the start of a session, Modbus errors
  while discovering the model chain, and failed model reads while polling. Exceptions returned
  while probing the candidate base addresses are not counted, because the collector expects them
  and moves on to the next base. A device with no SunSpec map at any base therefore shows only
  `sunspec_poller_restarts_total` and a `device session ended` log line.
- `kind` values: `exception` (the device returned a Modbus exception), `timeout` (a read or a
  connect timed out), `transport` (I/O or protocol error on the connection), `connect` (the
  connection was refused or failed, or the host could not be resolved), `address` (the read would
  run past register 65535).
- `sunspec_uplink_errors_total` counts per attempt: a record that fails three times counts three
  times. During a Kafka outage it grows by up to `buffer.batch_size` per attempt.
- Without `kafka.brokers`, records handed to the log-only sink count in
  `sunspec_uplink_records_total`.
- `sunspec_uplink_publish_seconds` is recorded only for batches in which at least one record was
  acknowledged.
- `reason="write_error"` counts samples lost because an SQLite write failed (for example, a full
  disk). `reason="corrupt"` counts rows that could not be decoded or encoded and were deleted.

### Scrape configuration

```yaml
scrape_configs:
  - job_name: sunspec-collector
    static_configs:
      - targets: ["collector-host:9090"]
```

The Compose stack uses the same configuration in `docker/prometheus.yml`.

### Recommended alert rules

```yaml
groups:
  - name: sunspec-collector
    rules:
      - alert: SunspecCollectorDown
        expr: up{job="sunspec-collector"} == 0
        for: 2m
        labels:
          severity: critical
        annotations:
          summary: "Prometheus cannot scrape the collector on {{ $labels.instance }}"

      - alert: SunspecDeviceDown
        expr: sunspec_device_up == 0
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "Device {{ $labels.device }} has not been polled successfully for 5 minutes"

      # Devices whose sessions keep failing, including ones that flap between up and down.
      - alert: SunspecDeviceRestarting
        expr: increase(sunspec_poller_restarts_total[15m]) >= 5
        labels:
          severity: warning
        annotations:
          summary: "Device {{ $labels.device }} sessions keep failing"

      - alert: SunspecPollerStalled
        expr: increase(sunspec_poller_stalls_total[15m]) > 0
        labels:
          severity: warning
        annotations:
          summary: "Device {{ $labels.device }} stalled and was restarted"

      - alert: SunspecSamplesDropped
        expr: increase(sunspec_samples_dropped_total[15m]) > 0
        labels:
          severity: warning
        annotations:
          summary: "Samples dropped before the buffer ({{ $labels.reason }})"

      - alert: SunspecUplinkErrors
        expr: increase(sunspec_uplink_errors_total[10m]) > 0
        labels:
          severity: warning
        annotations:
          summary: "Kafka is not acknowledging records from {{ $labels.instance }}"

      # Rows are waiting but nothing was delivered in the last 5 minutes (also fires when
      # nothing has been delivered since startup, when the counter does not exist yet).
      - alert: SunspecUplinkStalled
        expr: (sunspec_buffer_rows > 0) unless (rate(sunspec_uplink_records_total[5m]) > 0)
        for: 10m
        labels:
          severity: critical
        annotations:
          summary: "No records delivered to Kafka from {{ $labels.instance }}"

      - alert: SunspecBufferBacklog
        expr: sunspec_buffer_oldest_age_seconds > 600
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "Oldest buffered row is {{ $value | humanizeDuration }} old"

      - alert: SunspecBufferDataLoss
        expr: increase(sunspec_buffer_dropped_total[1h]) > 0
        labels:
          severity: critical
        annotations:
          summary: "Buffered data dropped without being published ({{ $labels.reason }})"
```

To alert on `/healthz` itself, probe it with the Prometheus blackbox exporter (an `http_2xx`
module) and alert on the probe:

```yaml
      - alert: SunspecCollectorUnhealthy
        expr: probe_success{job="sunspec-healthz"} == 0
        for: 5m
        labels:
          severity: critical
```

---

## Graceful shutdown

SIGINT (Ctrl-C) and SIGTERM (sent by `systemctl stop` and `docker stop`) start a graceful stop:

1. The log shows `shutdown signal received`. Device pollers stop; reads in progress are abandoned.
   Pollers get 10 s to finish before they are aborted.
2. The ingest task writes every sample still queued in the channel to SQLite, then logs
   `ingest stopped`.
3. The uplink finishes the batch it is publishing, then keeps draining the buffer for up to
   `buffer.shutdown_drain_timeout_ms` (default 5 s; `0` skips this step). If time runs out, it
   logs `final uplink drain timed out; remaining rows stay buffered for the next start`.
4. Retention, the HTTP server and the watchdog stop. The log shows `collector stopped` and the
   process exits with status 0.

Rows left in the buffer are published after the next start.

A stop normally takes a few seconds. The worst case is roughly the 10 s poller grace, plus one
in-flight batch (up to `kafka.timeout_ms`), plus `buffer.shutdown_drain_timeout_ms`. The systemd
unit allows 30 s (`TimeoutStopSec`). Docker allows 10 s unless you pass `--stop-timeout` (or set
`stop_grace_period` in Compose). If you raise `buffer.shutdown_drain_timeout_ms`, raise these
limits too.

If the process is killed instead (SIGKILL, power loss):

- rows already committed to SQLite are kept and sent after the restart;
- samples still in the channel, not yet written, are lost;
- rows that Kafka acknowledged but that were not yet deleted are sent again, so consumers see
  duplicates. See [Delivery semantics](buffer_maintenance.md#delivery-semantics).

---

## Logs

Logs go to standard output (journald under systemd, `docker logs` in Docker). Set the level with
`RUST_LOG` (default `info`) and the format with `SUNSPEC_LOG_FORMAT=text|json`; see
[Logging](configuration.md#logging). At `info`, steady-state operation logs almost nothing:
per-cycle and per-batch messages are at debug level.

Messages worth knowing:

| Message | Level | Meaning |
|---------|-------|---------|
| `starting device poller` | info | A device was added (`device=host/unit`). |
| `discovered SunSpec models` | info | Session started; shows the base address and model IDs found. |
| `device session ended` | warn | A session failed; `error` gives the reason, `retry_in_ms` the backoff. |
| `modbus read failed` | warn | One read attempt failed (logged by `modbus_client` and `poller_actor`). |
| `modbus reconnected` | info | The client reconnected after a transport error or timeout. |
| `telemetry channel full, sample dropped` | warn | Ingest is not keeping up. |
| `no devices found` | warn | The device set is empty; subnet mode retries after `rescan_interval_s`. |
| `records not delivered; they stay buffered` | warn | Kafka did not acknowledge some records. |
| `buffer write failed; samples dropped` | warn | SQLite write failed; samples lost. |
| `buffer retention: dropped rows past max age` | warn | Retention deleted rows older than `buffer.max_age_s`. |
| `buffer retention: dropped oldest rows over max rows` | warn | Retention deleted rows beyond `buffer.max_rows`. |
| `corrupt buffered sample, discarding` / `sample cannot be encoded, discarding` | warn | A row could never be published and was deleted. |
| `pipeline unhealthy; withholding systemd watchdog ping` | warn | Ingest or uplink stopped checking in. |
| `final uplink drain timed out; ...` | warn | Shutdown left rows in the buffer. |

librdkafka also logs broker connection errors (for example `Connect to ipv4#... failed`) at error
level while a broker is unreachable.

---

## Troubleshooting

### Startup errors

| Error | Cause | Fix |
|-------|-------|-----|
| `load config failed` ... `unknown field` | Misspelled or removed key in the config file | Compare with the [reference](configuration.md#reference). |
| `load config failed` ... `SUNSPEC_X="..." is invalid` | Unparsable environment value, including inline comments in an env file | Fix the value; put comments on their own lines. |
| `load config failed` ... `read config file` | `--config` / `SUNSPEC_CONFIG` path missing or unreadable | Check the path and permissions (mode 0644 for systemd). |
| `config validation failed` | Value out of range | The message names the key and the allowed values. |
| `config validation failed` ... `kafka.acks must be all (or -1) while kafka.enable_idempotence is true` | `acks` of `0` or `1` with idempotence on (the default) | Set `kafka.enable_idempotence = false` or `acks = "all"`. |
| `config validation failed` ... `modbus.max_batch_size must be between 1 and 125` | Batch size above the Modbus limit | Use a value from 1 to 125. |
| `kafka publisher init failed` | librdkafka rejected a setting when the producer was created | The message gives librdkafka's reason. |
| `bind metrics port <port>` | The port is in use (often a second collector) or not permitted | Stop the other process or change `metrics.port` / `SUNSPEC_METRICS_PORT`. |
| `buffer init failed` | Buffer path not writable | systemd: keep the buffer under `/var/lib/sunspec-collector`. Docker: mount a volume at `/app/data` and keep `buffer.path` relative or under `/app/data`. |
| `SUNSPEC_LOG_FORMAT must be text or json` | Unsupported log format | Use `text` or `json`. |

### A device is not polled

1. Check that the collector knows the device: `sunspec_devices_known`, `devices_total` in
   `/healthz`, and a `starting device poller device=host/unit` log line. In subnet mode the host
   must accept a TCP connection on the Modbus port within `discovery.per_host_timeout_ms`.
2. Check `sunspec_device_up{device="host/unit"}`. A value of 0 means the device is not being
   read successfully; if `sunspec_device_last_success_timestamp_seconds` has no series for it,
   it has not been read successfully since the collector started. If
   `sunspec_poller_restarts_total` keeps increasing, its sessions keep failing;
   `sunspec_poll_errors_total{kind}` shows whether connects or reads fail.
3. Find the reason in the logs: `journalctl -u sunspec-collector | grep 'device session ended'`.

| `error=` in `device session ended` | Likely cause |
|-----------------------------------|--------------|
| `could not resolve host:port` | Hostname does not resolve. |
| `connect to ... failed` / `request timed out after ...ms` | Wrong host or port, firewall, device offline, or `poller.request_timeout_ms` too short. Test with `nc -zv <host> <port>`. |
| `no SunSpec register map at base addresses [...]` | The `SunS` marker was not found at any base. The device may not be SunSpec, may use another base address (set `sunspec.base_address`), or the unit ID is wrong. |
| `SunSpec model chain is malformed: ...` | The model headers do not form a valid chain ending in `0xFFFF`. |
| `10 consecutive poll cycles failed` | Reads keep failing after discovery; check `sunspec_poll_errors_total{kind}`. Devices that reject large reads need a lower `modbus.max_batch_size`. |
| `no successful read for ...` | Stall detection; see [Stalls](#stalls). |
| `none of the discovered models [...] is selected for polling` | `include_models` / `exclude_models` (global or on the device entry) leave nothing to poll on this device. The list shows the models it has. |

If the `modbus read failed` lines before a session ends show
`device returned modbus exception: Gateway target device failed to respond`, a gateway answered
but has no device at that unit ID. Check the unit IDs.

Failed sessions are retried with exponential backoff capped at 60 s, so a device that comes back
online is picked up within about a minute without restarting the collector.

### No records in Kafka

1. Check that `kafka.brokers` / `SUNSPEC_KAFKA_BROKERS` is set. Without it, records are only logged
   at debug level (`record published (log sink)` with `RUST_LOG=info,avro_kafka=debug`) and then
   removed from the buffer.
2. Check `sunspec_uplink_errors_total` and the `records not delivered; they stay buffered` log
   line. Rising errors mean the brokers are unreachable or are rejecting the records (wrong
   address, missing topic with auto-creation disabled, ACLs, or a security mismatch). librdkafka's
   own error lines name the broker; authentication problems show up as `SASL authentication
   error` or `SSL handshake failed`. Check `kafka.security_protocol`, the SASL mechanism and
   credentials, and that `kafka.ssl_ca_location` holds the CA that signed the broker certificate.
3. Check `sunspec_buffer_oldest_age_seconds` and `sunspec_buffer_rows`. If they grow while
   `sunspec_uplink_records_total` is flat, nothing is being delivered.
4. If records arrive but consumers fail to read them, the consumers may still expect the old
   format; see [Upgrade notes](#upgrade-notes-for-this-release).

### The buffer keeps growing

The buffer grows when samples are written faster than they are delivered. Compare:

```promql
rate(sunspec_buffer_writes_total[5m])     # samples in
rate(sunspec_uplink_records_total[5m])    # records out
```

- Kafka down or rejecting records: see the previous section. The buffer holds the data until
  retention starts dropping it (`sunspec_buffer_dropped_total{reason="max_rows"|"max_age"}`).
- Kafka up but slow: check `sunspec_uplink_publish_seconds`. Each batch waits for the producer's
  50 ms linger plus the broker round trip, so a larger `buffer.batch_size` raises throughput.
- More data than expected: every polled model of every device produces one row per poll interval.
  Read slow-changing models less often or skip unneeded ones with a
  [polling profile](configuration.md#polling-profiles); see also
  [Sizing](buffer_maintenance.md#sizing).

### Stalls

`sunspec_poller_stalls_total` increases when a session goes `poller.stall_timeout_ms` (default
max(10 × poll interval, 30 s)) without a successful read. This usually means every read of the
device timed out or failed for that long, for example when a device keeps its TCP connection open
but stops answering. The session is ended and retried with backoff. The `modbus read failed` lines
for the device show the errors. Occasional stalls point to the device, the gateway or the network;
a device that stalls repeatedly also triggers the `SunspecDeviceRestarting` alert.

If `/healthz` returns 503, the ingest task or the uplink itself has stopped checking in. That is
a collector problem rather than a device problem; the systemd watchdog restarts the service. Keep
the logs from before the restart.

### Samples dropped with `reason="channel_full"`

The ingest task could not take a sample within one poll interval, usually because SQLite writes are
slow (slow or failing storage). Check disk health and latency. `runtime.channel_capacity` absorbs
short bursts but does not fix slow storage.

### Poll cycles take longer than the poll interval

Compare `sunspec_poll_duration_seconds` with `poller.poll_interval_ms`. When a cycle overruns, the
next tick is skipped, so samples arrive less often than configured. Increase the poll interval, or
reduce per-request latency (lower `poller.request_timeout_ms`, fewer retries).

### "database is locked"

Two collectors are using the same buffer file, or a manual `sqlite3` session holds a write lock.
Run one collector per buffer file and follow
[Buffer Maintenance](buffer_maintenance.md#safe-maintenance) for manual work.

---

## Upgrade notes for this release

This release contains breaking changes. Read all of this section before upgrading.

### Kafka record format v2 (breaking)

| | Previous format | This release (v2) |
|-|-----------------|-------------------|
| Records | One Avro Object Container File per message, mixing devices | One record per sample |
| Key | None | `device_key`, e.g. `192.168.1.20/1` |
| Value | Container file with the schema embedded in every message | Avro single-object encoding: bytes `C3 01`, the schema's 8-byte CRC-64-AVRO (Rabin) fingerprint (little-endian), then the Avro binary datum |
| Headers | None | `content-type: application/avro; encoding=single-object`, `schema-fingerprint: <16 hex chars>` |
| Schema | Raw registers only | `com.rusty.sunspec.v2.SunspecSample`: decoded `points` and raw `registers` |
| `model_name` | Project names, e.g. `three_phase_inverter` | Official SunSpec names, e.g. `inverter_three_phase` |

`collector-app --print-schema` prints the v2 schema; Rust consumers can use `avro_kafka::decode`.
Consumers must be updated before or together with the collector. To avoid mixing formats on one
topic, consider publishing v2 to a new topic (`kafka.topic`).

Rows buffered by the previous version are published in the v2 format after the upgrade, but with
an empty `points` map and the old model names. To avoid this, let the old version drain its buffer
(`buffer_size` at 0) before you upgrade.

Expect more records per device than before. Models beyond the first 200 registers are now found by
walking the model chain, and every unit ID behind a gateway is polled (previously only one unit per
host was).

### Metric renames (breaking)

Update dashboards and alerts:

| Old metric | New metric | Change |
|------------|------------|--------|
| `buffer_enqueue_success` | `sunspec_buffer_writes_total` | Renamed. |
| `buffer_enqueue_error` | `sunspec_buffer_dropped_total{reason="write_error"}` | Now one counter with a `reason` label. |
| `buffer_size` | `sunspec_buffer_rows` | Renamed; updated every 10 s. |
| `uplink_messages_sent` | `sunspec_uplink_records_total` | `batch_size` label removed. Counts acknowledged records. |
| `uplink_publish_error` | `sunspec_uplink_errors_total` | Counts records, not failed batches. |
| `uplink_publish_latency` | `sunspec_uplink_publish_seconds` | Histogram with buckets. |
| `poller_success{ip}` | `sunspec_poll_success_total{device}` | Label `ip` replaced by `device` (`host/unit`). |
| `poller_error{ip,type}` | `sunspec_poll_errors_total{device,kind}` | `kind` gives the error class. Channel-full drops moved to `sunspec_samples_dropped_total{reason="channel_full"}`. |

New metrics: `sunspec_poll_duration_seconds`, `sunspec_device_up`,
`sunspec_device_last_success_timestamp_seconds`, `sunspec_poller_restarts_total`,
`sunspec_poller_stalls_total`, `sunspec_devices_known`, `sunspec_samples_dropped_total`,
`sunspec_buffer_dropped_total`, `sunspec_buffer_oldest_age_seconds`.

### Strict configuration (breaking)

Unknown keys, unparsable environment values, invalid static devices, unsupported
`kafka.compression` / `kafka.acks` values, `kafka.acks` of `0` or `1` with idempotence enabled,
and `modbus.max_batch_size` above 125 now stop the collector at startup instead of being ignored
or adjusted. Before upgrading, compare your config file with the
[reference](configuration.md#reference) and check your environment file for inline comments and
empty values.

### Deprecated settings

Accepted, ignored, and logged with a warning:

- `sunspec.discovery_register_count` / `SUNSPEC_DISCOVERY_REG_COUNT`: models are found by walking
  the model chain.
- `modbus.timeout_ms` / `SUNSPEC_MODBUS_TIMEOUT_MS`: use `poller.request_timeout_ms` /
  `SUNSPEC_REQUEST_TIMEOUT_MS`.

### Deployment changes

- systemd: the unit now runs as a dynamic unprivileged user with
  `StateDirectory=sunspec-collector`, `Type=notify` with a health-gated watchdog, and
  `TimeoutStopSec=30`. Install the new unit file and run `systemctl daemon-reload`. The previous
  guide had you create `/var/lib/sunspec-collector` as root; after the first start with the new
  unit, check that the log shows `buffer initialized` and no write errors.
- Docker: the `HEALTHCHECK` now calls `/healthz` instead of `/metrics`, on the port in
  `SUNSPEC_METRICS_PORT` (default 9090). The image's working directory is `/app/data`, so without
  `SUNSPEC_BUFFER_PATH` or `buffer.path` the buffer is `/app/data/sunspec-buffer.sqlite`.
  Deployments that set `SUNSPEC_BUFFER_PATH=/app/data/buffer.sqlite`, as the previous guide and
  Compose file did, keep using that file.
- A metrics port that cannot be bound now stops the collector at startup instead of leaving it
  running without `/metrics` and `/healthz`.
- SIGTERM now takes the graceful shutdown path.

### Upgrade procedure

1. Update Kafka consumers to read v2 records (or prepare a new topic).
2. Let the old collector drain its buffer, then stop it.
3. Install the new binary or image, the new unit file and an updated config / environment file.
4. Start the collector and check `/healthz`, `sunspec_device_up` and
   `sunspec_uplink_records_total`.
5. Update dashboards and alert rules to the new metric names.
