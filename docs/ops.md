# Runtime operations

## systemd

- Install the unit file:

```sh
sudo cp docs/sunspec-collector.service /etc/systemd/system/sunspec-collector.service
sudo systemctl daemon-reload
```

- Provide an environment file (optional):

```sh
sudo cp docs/sunspec-collector.env /etc/sunspec-collector.env
```

- Enable and start:

```sh
sudo systemctl enable --now sunspec-collector
```

- Check status:

```sh
systemctl status sunspec-collector
```

## Logs

- View recent logs:

```sh
journalctl -u sunspec-collector -n 200 --no-pager
```

- Follow logs:

```sh
journalctl -u sunspec-collector -f
```

## Buffer storage

- Default buffer path: `sunspec-buffer.sqlite` (current working directory).
- Recommended persistent path (systemd): set `SUNSPEC_BUFFER_PATH=/var/lib/sunspec-collector/buffer.sqlite` in `/etc/sunspec-collector.env`.

Example directory setup:

```sh
sudo mkdir -p /var/lib/sunspec-collector
sudo chown root:root /var/lib/sunspec-collector
sudo chmod 755 /var/lib/sunspec-collector
```

## Troubleshooting config errors

- "load config failed": Check `SUNSPEC_CONFIG`/`--config` points to a readable TOML/JSON file.
- "config validation failed": Review required fields and ranges in `README.md` under Configuration.
- "discovery.subnet must be CIDR": Ensure the subnet is in IPv4 CIDR form (example: `192.168.1.0/24`).
- "kafka.topic contains invalid characters": Use only letters, digits, `.`, `_`, or `-`.
- "kafka.brokers must be non-empty": Set `SUNSPEC_KAFKA_BROKERS` or remove the kafka section to run in mock mode.

## Monitoring (Prometheus)

The collector exposes a Prometheus-compatible metrics endpoint at `http://localhost:9090/metrics` (port is configurable via `SUNSPEC_METRICS_PORT`).

### Key Metrics

| Metric Name | Type | Description | Labels |
|---|---|---|---|
| `poller_success` | Counter | Number of successful poll cycles | `ip` |
| `poller_error` | Counter | Number of failed poll cycles | `ip`, `type` (modbus, channel) |
| `buffer_size` | Gauge | Current number of messages in SQLite buffer | - |
| `uplink_messages_sent` | Counter | Total messages successfully published to Kafka | `batch_size` |
| `uplink_publish_error` | Counter | Number of failed Kafka publish attempts | - |
| `uplink_publish_latency` | Histogram | Latency of publishing a batch to Kafka | - |

### Alerting Recommendations
- **Zombie Poller**: Rate of `poller_success` == 0 for > 5m for a known IP.
- **Buffer Backpressure**: `buffer_size` > 10,000 (indicates Kafka is down or slow).
- **High Error Rate**: Rate of `poller_error` > 10% of `poller_success`.
