# Deployment

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
docker pull ghcr.io/imrans110/sunspec-collector:latest     # or a version, e.g. :0.5.0
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

The unit file is [`sunspec-collector.service`](../sunspec-collector.service) and the environment
file template is [`sunspec-collector.env`](../sunspec-collector.env).

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
`sudo systemctl restart sunspec-collector`. `SUNSPEC_VERSION=0.5.0` installs a specific release.
To read the script first: `curl -fsSLO <url>`, read `install.sh`, then `sudo sh install.sh`.
From 0.5.0 the release files and images are attested, so a downloaded `install.sh`, archive or
`SHA256SUMS` can be checked with `gh attestation verify` first; see
[Verifying a release](../guides/verifying-a-release.md#verifying-a-release).

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
  duplicates. See [Delivery semantics](../buffer_maintenance.md#delivery-semantics).

---

## Logs

Logs go to standard output (journald under systemd, `docker logs` in Docker). Set the level with
`RUST_LOG` (default `info`) and the format with `SUNSPEC_LOG_FORMAT=text|json`; see
[Logging](../configuration.md#logging). At `info`, steady-state operation logs almost nothing:
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
