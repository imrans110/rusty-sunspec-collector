# Kafka record contract

Schema version 2. This format is not compatible with earlier versions of the collector (see
[Earlier format](#earlier-format)).

## Record

- **Topic:** `kafka.topic` (default `sunspec.telemetry`).
- **One record per sample**, that is per model read from one device.
- **Key:** `device_key`, `host/unit_id`, for example `192.168.1.20/1`. With librdkafka's default
  partitioner (CRC32 of the key) all records of a device go to the same partition.
- **Value:** Avro [single-object encoding](https://avro.apache.org/docs/current/specification/#single-object-encoding):

  ```
  C3 01 | 8-byte CRC-64-AVRO (Rabin) fingerprint of the schema, little-endian | Avro binary datum
  ```

- **Headers:**
  - `content-type: application/avro; encoding=single-object`
  - `schema-fingerprint: <16 hex characters>`: the 8 fingerprint bytes in the order they appear in
    the value.

Records are published in buffer order, but a record that failed delivery is retried in a later
batch, after records that were acknowledged. Order consumers by `collected_at_ms` when order
matters.

## Schema

`com.rusty.sunspec.v2.SunspecSample`, in `crates/avro-kafka/schemas/sunspec_sample.avsc`;
`collector-app --print-schema` prints it.

| Field | Avro type | Meaning |
|-------|-----------|---------|
| `device_key` | `string` | `host/unit_id`; also the record key |
| `ip` | `string` | Device host (IP address or hostname as configured) |
| `unit_id` | `int` | Modbus unit ID |
| `model_id` | `int` | SunSpec model ID, e.g. `103` |
| `model_name` | `string` | SunSpec model name, e.g. `inverter_three_phase`, or `model_<id>` |
| `start` | `int` | Register address of the model header |
| `collected_at_ms` | `long` (`timestamp-millis`) | When the registers were read |
| `points` | `map` of `["null", "double", "long", "string"]` | Decoded points by SunSpec name |
| `labels` | `map` of `string`, default `{}` | Enum values and set bitfield bits by name (schema 2.1) |
| `registers` | `array` of `int` | Raw registers, including the two header registers |

Point semantics, as described in [SunSpec decoding](../ARCHITECTURE.md#sunspec-decoding):

- names are the SunSpec point names; repeating groups are `group[i].Point` (e.g. `module[0].DCA`)
  and fixed sub-groups `group.Point`;
- scaled values are `double`; enums, bitfields and unscaled integers are `long`; strings are
  `string`, trimmed;
- "not implemented" values, and points whose scale factor is not implemented, are `null`;
- the model header (`ID`, `L`), scale-factor points and padding are omitted;
- models without a bundled definition have an empty `points` map; only `registers` is set.

`points` and `labels` are Avro maps, so their order is not preserved. Point units, labels and
descriptions are in the [model catalog](../ARCHITECTURE.md#model-catalog), by model ID and point name.

## Schema evolution

Changes that old readers can resolve, such as a new field with a default, keep the
`com.rusty.sunspec.v2` namespace and get a minor schema version in the CHANGELOG (2.1 added
`labels`). They still change the schema's fingerprint, so a consumer that resolves the writer
schema by fingerprint needs the new schema (`collector-app --print-schema`) before it reads new
records. A change that old readers cannot resolve would get a new namespace (`v3`) and a breaking
CHANGELOG entry.

A model 103 record, shown as JSON for readability (unions unwrapped, lists shortened):

```text
{
  "device_key": "192.168.1.20/1",
  "ip": "192.168.1.20",
  "unit_id": 1,
  "model_id": 103,
  "model_name": "inverter_three_phase",
  "start": 40070,
  "collected_at_ms": 1760000000123,
  "points": { "A": 12.34, "PhVphA": 230.1, "W": 4321.0, "Hz": 50.0, "St": 4, "VA": null, ... },
  "labels": { "St": "MPPT", "Evt1": "", ... },
  "registers": [103, 50, 1234, ...]
}
```

## Reading records

- **Rust:** `avro_kafka::decode(bytes)` checks the header and fingerprint and returns a `Sample`
  (points sorted by name).
- **Other languages:** check that the value starts with `C3 01` and that the next 8 bytes match the
  fingerprint of the schema you hold, then decode the rest as an Avro binary datum with that schema.
  Most Avro libraries implement single-object decoding directly (for example `BinaryMessageDecoder`
  in Java).

## Producer settings

`acks=all`, `enable.idempotence=true`, `compression.type=zstd`, `linger.ms=50` (fixed) and
`message.timeout.ms=5000` by default. Every setting except `linger.ms` can be changed in the
`[kafka]` section. An idempotent producer requires `acks=all`, so configuration validation rejects
`kafka.acks` `0` or `1` while `kafka.enable_idempotence` is true (the default).

Connection security is librdkafka's: `security.protocol` (`plaintext`, `ssl`, `sasl_plaintext`,
`sasl_ssl`), the `ssl.*` CA, client certificate and key for (mutual) TLS, and SASL `PLAIN` or
`SCRAM-SHA-256/512` credentials. Passwords are held in a `Secret` type whose debug output is
redacted and can be read from files at startup. Validation rejects settings that do not match the
protocol, so a half-configured TLS or SASL setup fails instead of falling back to plaintext.

## Earlier format

Earlier versions sent one Avro Object Container File per Kafka message, mixing samples from several
devices, with the schema embedded in every message, no key, raw registers only, and model names
such as `three_phase_inverter`. Consumers of that format must be updated.
