# Upgrade notes for this release

This release contains breaking changes. Read all of this section before upgrading.

## Kafka record format v2 (breaking)

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

## Schema 2.1: `labels`

Schema 2.1 adds a `labels` map (enum values and set bitfield bits by name) with an empty default.
Avro readers resolve old and new records with either schema, but the schema's fingerprint
changes. Consumers that look up the writer schema by fingerprint need the 2.1 schema
(`collector-app --print-schema`) before they read records from this version.

Rows buffered by the previous version are published in the v2 format after the upgrade, but with
an empty `points` map and the old model names. To avoid this, let the old version drain its buffer
(`buffer_size` at 0) before you upgrade.

Expect more records per device than before. Models beyond the first 200 registers are now found by
walking the model chain, and every unit ID behind a gateway is polled (previously only one unit per
host was).

## Metric renames (breaking)

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

## Strict configuration (breaking)

Unknown keys, unparsable environment values, invalid static devices, unsupported
`kafka.compression` / `kafka.acks` values, `kafka.acks` of `0` or `1` with idempotence enabled,
and `modbus.max_batch_size` above 125 now stop the collector at startup instead of being ignored
or adjusted. Before upgrading, compare your config file with the
[reference](../configuration.md#reference) and check your environment file for inline comments and
empty values.

## Deprecated settings

Accepted, ignored, and logged with a warning:

- `sunspec.discovery_register_count` / `SUNSPEC_DISCOVERY_REG_COUNT`: models are found by walking
  the model chain.
- `modbus.timeout_ms` / `SUNSPEC_MODBUS_TIMEOUT_MS`: use `poller.request_timeout_ms` /
  `SUNSPEC_REQUEST_TIMEOUT_MS`.

## Deployment changes

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

## Upgrade procedure

1. Update Kafka consumers to read v2 records (or prepare a new topic).
2. Let the old collector drain its buffer, then stop it.
3. Install the new binary or image, the new unit file and an updated config / environment file.
4. Start the collector and check `/healthz`, `sunspec_device_up` and
   `sunspec_uplink_records_total`.
5. Update dashboards and alert rules to the new metric names.
