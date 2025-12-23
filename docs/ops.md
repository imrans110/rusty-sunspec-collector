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
