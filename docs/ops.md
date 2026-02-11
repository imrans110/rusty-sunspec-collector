# Operations

This guide covers deploying, running, and monitoring Rusty SunSpec Collector in production.

For configuration options see [Configuration](configuration.md). For architecture details see [Architecture Guide](ARCHITECTURE.md).

---

## systemd Deployment (Linux)

### Install the service

```sh
sudo cp docs/sunspec-collector.service /etc/systemd/system/sunspec-collector.service
sudo systemctl daemon-reload
```

The unit file includes security hardening (`NoNewPrivileges`, `ProtectSystem=strict`, `ProtectHome`, `PrivateTmp`).

### Provide an environment file

```sh
sudo cp docs/sunspec-collector.env /etc/sunspec-collector.env
# Edit to match your deployment
sudo vi /etc/sunspec-collector.env
```

### Enable and start

```sh
sudo systemctl enable --now sunspec-collector
```

### Check status

```sh
systemctl status sunspec-collector
```

### Logs

```sh
# View recent logs
journalctl -u sunspec-collector -n 200 --no-pager

# Follow logs in real time
journalctl -u sunspec-collector -f

# Filter by severity
journalctl -u sunspec-collector -p warning --no-pager
```

---

## Docker Deployment

### Build

```sh
docker build -t sunspec-collector .
```

The Dockerfile uses a multi-stage build (Rust builder on `bookworm-slim`, runtime on `debian:bookworm-slim`). The container runs as a non-root `collector` user with a built-in healthcheck.

### Run

```sh
docker run -d \
  --name sunspec-collector \
  -p 9090:9090 \
  -e SUNSPEC_SUBNET=192.168.1.0/24 \
  -e SUNSPEC_KAFKA_BROKERS=kafka:9092 \
  -e SUNSPEC_BUFFER_PATH=/app/data/buffer.sqlite \
  -v sunspec-data:/app/data \
  sunspec-collector
```

### Run with env file

```sh
docker run -d \
  --name sunspec-collector \
  -p 9090:9090 \
  --env-file .env \
  -v sunspec-data:/app/data \
  sunspec-collector
```

### Health check

The container's `HEALTHCHECK` curls `/metrics` every 30s:

```sh
docker inspect --format='{{.State.Health.Status}}' sunspec-collector
```

### View logs

```sh
docker logs -f sunspec-collector
```

### Stop and remove

```sh
docker stop sunspec-collector && docker rm sunspec-collector
```

---

## Buffer Storage

The SQLite buffer provides durable, on-disk message queuing between pollers and Kafka.

### Storage paths

| Deployment | Default path | How to change |
|------------|-------------|---------------|
| Local dev | `sunspec-buffer.sqlite` (cwd) | `SUNSPEC_BUFFER_PATH=...` |
| Docker | `/app/data/buffer.sqlite` | Mount a volume to `/app/data` |
| systemd | Set in env file | `SUNSPEC_BUFFER_PATH=/var/lib/sunspec-collector/buffer.sqlite` |

### Persistent directory setup (systemd)

```sh
sudo mkdir -p /var/lib/sunspec-collector
sudo chown root:root /var/lib/sunspec-collector
sudo chmod 755 /var/lib/sunspec-collector
```

### Inspect the buffer

```sh
sqlite3 sunspec-buffer.sqlite "SELECT id, topic, length(payload), created_at FROM telemetry_queue ORDER BY id DESC LIMIT 10;"
```

### Check queue depth

```sh
sqlite3 sunspec-buffer.sqlite "SELECT COUNT(*) FROM telemetry_queue;"
```

### Maintenance

The buffer is append-only and deletes rows after successful uplink. Over time, the file can grow due to SQLite freelist pages. Schedule periodic maintenance if buffer size grows beyond expectations.

**Recommended strategy:**

- Keep WAL mode enabled (already configured).
- Run `VACUUM` during low-traffic windows to reclaim disk space.
- Optionally run `PRAGMA wal_checkpoint(TRUNCATE);` to truncate the WAL file.

**Maintenance script:**

```sh
#!/usr/bin/env sh
set -euo pipefail

DB_PATH=${SUNSPEC_BUFFER_PATH:-/var/lib/sunspec-collector/buffer.sqlite}

sqlite3 "$DB_PATH" "PRAGMA wal_checkpoint(TRUNCATE);"
sqlite3 "$DB_PATH" "VACUUM;"
```

**Automated maintenance with systemd timer:**

Create `/etc/systemd/system/sunspec-buffer-maintenance.service`:

```ini
[Unit]
Description=SunSpec buffer maintenance

[Service]
Type=oneshot
Environment=SUNSPEC_BUFFER_PATH=/var/lib/sunspec-collector/buffer.sqlite
ExecStart=/bin/sh -c 'sqlite3 "$SUNSPEC_BUFFER_PATH" "PRAGMA wal_checkpoint(TRUNCATE);" && sqlite3 "$SUNSPEC_BUFFER_PATH" "VACUUM;"'
```

Create `/etc/systemd/system/sunspec-buffer-maintenance.timer`:

```ini
[Unit]
Description=Run SunSpec buffer maintenance weekly

[Timer]
OnCalendar=Sun *-*-* 03:00:00
Persistent=true

[Install]
WantedBy=timers.target
```

Enable the timer:

```sh
sudo systemctl daemon-reload
sudo systemctl enable --now sunspec-buffer-maintenance.timer
```

---

## Monitoring (Prometheus)

The collector exposes a Prometheus-compatible metrics endpoint at `http://localhost:9090/metrics` (port configurable via `SUNSPEC_METRICS_PORT`).

### Metrics Reference

| Metric | Type | Description | Labels |
|--------|------|-------------|--------|
| `poller_success` | Counter | Successful model reads | `ip` |
| `poller_error` | Counter | Failed reads | `ip`, `type` |
| `buffer_enqueue_success` | Counter | Successful buffer writes | — |
| `buffer_enqueue_error` | Counter | Failed buffer writes | — |
| `buffer_size` | Gauge | Current queue depth | — |
| `uplink_messages_sent` | Counter | Messages published to Kafka | `batch_size` |
| `uplink_publish_error` | Counter | Failed Kafka publishes | — |
| `uplink_publish_latency` | Histogram | Batch publish duration | — |

### Verify metrics

```sh
curl -s http://localhost:9090/metrics | head -20
```

### Alerting Recommendations

| Alert | Condition | Severity |
|-------|-----------|----------|
| Zombie Poller | `rate(poller_success{ip="..."}[5m]) == 0` for a known IP | Warning |
| Buffer Backpressure | `buffer_size > 10000` | Critical |
| High Error Rate | `rate(poller_error[5m]) / rate(poller_success[5m]) > 0.1` | Warning |
| Uplink Down | `rate(uplink_messages_sent[10m]) == 0` and `buffer_size > 0` | Critical |
| Publish Failures | `rate(uplink_publish_error[5m]) > 0` | Warning |

---

## Troubleshooting

### Configuration errors

| Error | Cause | Fix |
|-------|-------|-----|
| `load config failed` | File not found or unreadable | Check `SUNSPEC_CONFIG` / `--config` path |
| `config validation failed` | Field out of range | Review [Configuration](configuration.md) |
| `discovery.subnet must be CIDR` | Invalid subnet format | Use IPv4 CIDR, e.g. `192.168.1.0/24` |
| `kafka.topic contains invalid characters` | Special chars in topic | Use only `[a-zA-Z0-9._-]` |
| `kafka.brokers must be non-empty` | Empty string | Set `SUNSPEC_KAFKA_BROKERS` or omit for mock mode |
| `failed to install metrics recorder` | Double install | Ensure one collector instance per process |

### Runtime issues

| Symptom | Likely Cause | Fix |
|---------|-------------|-----|
| No devices discovered | Wrong subnet or port | Verify: `nc -zv <ip> 502` |
| Modbus timeout on all devices | Firewall or wrong port | Check `SUNSPEC_PORT`, increase `SUNSPEC_MODBUS_TIMEOUT_MS` |
| Buffer growing indefinitely | Kafka broker down | Check connectivity, review `uplink_publish_error` |
| High CPU usage | Too many models, short interval | Increase `SUNSPEC_POLL_INTERVAL_MS` |
| SQLite "database is locked" | Two instances on same file | One collector per buffer file. Check: `fuser <path>` |
| Poller respawning repeatedly | Device offline | Check logs; respawn backoff caps at 60s |
