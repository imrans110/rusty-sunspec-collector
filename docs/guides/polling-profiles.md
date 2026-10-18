# Polling profiles

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

## Per-device settings

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
