# HTTP endpoints

All four endpoints are served on `metrics.port` (default 9090) on all IPv4 interfaces.

## `/metrics`

Prometheus text format. Every metric has HELP text. Metrics ending in `_seconds` are histograms
with buckets at 0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1, 2.5, 5 and 10 seconds.

```sh
curl -s http://localhost:9090/metrics | grep '^sunspec_'
```

## `/healthz`

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

## `/devices`

The live view of every supervised device: its state, what its latest session discovered, and
the latest decoded reading of each polled model. Use it when commissioning ("is it reading my
inverter, and are the values right?") or to see data without a Kafka consumer.

```sh
curl -s localhost:9090/devices                     # every device
curl -s localhost:9090/devices/192.168.1.50/126    # one device (404 if it is not supervised)
```

```json
{"devices": [{
  "device": "192.168.1.50/126", "host": "192.168.1.50", "unit_id": 126,
  "up": true, "last_success_ms": 1792250000123, "restarts": 0,
  "base_address": 40000, "models": [1, 103, 120, 160], "polled_models": [1, 103, 160],
  "readings": [
    {"model_id": 103, "model_name": "inverter_three_phase", "collected_at_ms": 1792250000123,
     "age_s": 0.4, "points": {"A": 7.98, "PhVphA": 230.3, "W": 5509.0, "Hz": 50.02, "St": 4}}
  ]
}]}
```

| Field | Meaning |
|-------|---------|
| `up`, `last_success_ms`, `restarts` | The same state as `sunspec_device_up`, `sunspec_device_last_success_timestamp_seconds` and `sunspec_poller_restarts_total`. `last_success_ms` is `null` before the first successful read. |
| `base_address`, `models` | Where the `SunS` marker was found and the model chain, from the latest discovery; `null` and empty until a session has discovered the device. |
| `polled_models` | The models the [polling profile](../guides/polling-profiles.md#polling-profiles) selects. |
| `readings` | The latest successful read of each polled model, in device order: decoded points in definition order (scale factors applied, "not implemented" values `null`), `labels` (enum values and set bitfield bits by name, as in records), and how old the read is. A model on a slow interval shows its last read until the next one. |

Only the latest reading of each model is kept, so memory does not grow. A device that has never
answered is listed with empty `models` and `readings`.

The endpoint exposes device data (readings, models, addresses) to anyone who can reach
`metrics.port`, like `/metrics`. Keep the port on a management network or behind a firewall.

## `/models`

The bundled SunSpec model definitions, for describing what records and `/devices` contain:

```sh
curl -s localhost:9090/models          # {"models": [{"id": 1, "name": "common", "label": "Common"}, ...]}
curl -s localhost:9090/models/103      # one model: every point with type, units, label, description, symbols
```

Point names match records, with `[*]` for a repeating group's index (`module[*].DCA`). An unknown
ID returns 404. `collector-app --print-models` prints the same metadata for every model, for
consumers that load it once.
