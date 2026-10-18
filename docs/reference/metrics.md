# Metrics reference

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
