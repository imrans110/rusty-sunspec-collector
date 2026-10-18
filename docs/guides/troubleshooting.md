# Troubleshooting

## Startup errors

| Error | Cause | Fix |
|-------|-------|-----|
| `load config failed` ... `unknown field` | Misspelled or removed key in the config file | Compare with the [reference](../configuration.md#reference). |
| `load config failed` ... `SUNSPEC_X="..." is invalid` | Unparsable environment value, including inline comments in an env file | Fix the value; put comments on their own lines. |
| `load config failed` ... `read config file` | `--config` / `SUNSPEC_CONFIG` path missing or unreadable | Check the path and permissions (mode 0644 for systemd). |
| `config validation failed` | Value out of range | The message names the key and the allowed values. |
| `config validation failed` ... `kafka.acks must be all (or -1) while kafka.enable_idempotence is true` | `acks` of `0` or `1` with idempotence on (the default) | Set `kafka.enable_idempotence = false` or `acks = "all"`. |
| `config validation failed` ... `modbus.max_batch_size must be between 1 and 125` | Batch size above the Modbus limit | Use a value from 1 to 125. |
| `kafka publisher init failed` | librdkafka rejected a setting when the producer was created | The message gives librdkafka's reason. |
| `bind metrics port <port>` | The port is in use (often a second collector) or not permitted | Stop the other process or change `metrics.port` / `SUNSPEC_METRICS_PORT`. |
| `buffer init failed` | Buffer path not writable | systemd: keep the buffer under `/var/lib/sunspec-collector`. Docker: mount a volume at `/app/data` and keep `buffer.path` relative or under `/app/data`. |
| `SUNSPEC_LOG_FORMAT must be text or json` | Unsupported log format | Use `text` or `json`. |

## Serial devices through a converter

See [Serial devices through a converter](serial-devices.md).

## Probing a device

See [Probing a device](probing-a-device.md).

## A device is not polled

1. Check that the collector knows the device: `curl localhost:9090/devices` lists every
   supervised device with its state and latest readings; `sunspec_devices_known`,
   `devices_total` in `/healthz`, and a `starting device poller device=host/unit` log line say the
   same. In subnet mode the host
   must accept a TCP connection on the Modbus port within `discovery.per_host_timeout_ms`.
2. Check `sunspec_device_up{device="host/unit"}`. A value of 0 means the device is not being
   read successfully; if `sunspec_device_last_success_timestamp_seconds` has no series for it,
   it has not been read successfully since the collector started. If
   `sunspec_poller_restarts_total` keeps increasing, its sessions keep failing;
   `sunspec_poll_errors_total{kind}` shows whether connects or reads fail.
3. Find the reason in the logs: `journalctl -u sunspec-collector | grep 'device session ended'`,
   or probe the device directly with `collector-app --probe host:unit` (see
   [Probing a device](#probing-a-device)).

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

## No records in Kafka

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
   format; see [Upgrade notes](upgrading.md#upgrade-notes-for-this-release).

## The buffer keeps growing

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
  [polling profile](polling-profiles.md#polling-profiles); see also
  [Sizing](../buffer_maintenance.md#sizing).

## Stalls

`sunspec_poller_stalls_total` increases when a session goes `poller.stall_timeout_ms` (default
max(10 × poll interval, 30 s)) without a successful read. This usually means every read of the
device timed out or failed for that long, for example when a device keeps its TCP connection open
but stops answering. The session is ended and retried with backoff. The `modbus read failed` lines
for the device show the errors. Occasional stalls point to the device, the gateway or the network;
a device that stalls repeatedly also triggers the `SunspecDeviceRestarting` alert.

If `/healthz` returns 503, the ingest task or the uplink itself has stopped checking in. That is
a collector problem rather than a device problem; the systemd watchdog restarts the service. Keep
the logs from before the restart.

## Samples dropped with `reason="channel_full"`

The ingest task could not take a sample within one poll interval, usually because SQLite writes are
slow (slow or failing storage). Check disk health and latency. `runtime.channel_capacity` absorbs
short bursts but does not fix slow storage.

## Poll cycles take longer than the poll interval

Compare `sunspec_poll_duration_seconds` with `poller.poll_interval_ms`. When a cycle overruns, the
next tick is skipped, so samples arrive less often than configured. Increase the poll interval, or
reduce per-request latency (lower `poller.request_timeout_ms`, fewer retries).

## "database is locked"

Two collectors are using the same buffer file, or a manual `sqlite3` session holds a write lock.
Run one collector per buffer file and follow
[Buffer Maintenance](../buffer_maintenance.md#safe-maintenance) for manual work.
