# Solar modules

<!-- Generated from the bundled SunSpec definitions by
     UPDATE_MODEL_DOCS=1 cargo test -p sunspec-parser --test models_docs
     Do not edit by hand. -->

Point names are the ones records use; `[*]` stands for a repeating group's index. See [Supported SunSpec models](README.md) for every group.

| Model | Name in records | Points |
|-------|-----------------|--------|
| [501](#model-501-solar-module) | `solar_module_float` | 17 |
| [502](#model-502-solar-module) | `solar_module` | 17 |

## Model 501: Solar Module

Name in records: `solar_module_float`.

A solar module model supporting DC-DC converter

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `Stat` | enum16 |  | Status | Module Status Code |
| `StatVend` | enum16 |  | Vendor Status | Module Vendor Status Code |
| `Evt` | bitfield32 |  | Events | Module Event Flags |
| `EvtVend` | bitfield32 |  | Vendor Module Event Flags | Vendor specific flags |
| `Ctl` | enum16 |  | Control | Module Control |
| `CtlVend` | enum32 |  | Vendor Control | Vendor Module Control |
| `CtlVal` | int32 |  | Control Value | Module Control Value |
| `Tms` | uint32 | Secs | Timestamp | Time in seconds since 2000 epoch |
| `OutA` | float32 | A | Output Current | Output Current |
| `OutV` | float32 | V | Output Voltage | Output Voltage |
| `OutWh` | float32 | Wh | Output Energy | Output Energy |
| `OutW` | float32 | W | Output Power | Output Power |
| `Tmp` | float32 | C | Temp | Module Temperature |
| `InA` | float32 | A | Input Current | Input Current |
| `InV` | float32 | V | Input Voltage | Input Voltage |
| `InWh` | float32 | Wh | Input Energy | Input Energy |
| `InW` | float32 | W | Input Power | Input Power |

Value names:

- `Stat` (enum16): 1 `OFF`, 2 `SLEEPING`, 3 `STARTING`, 4 `MPPT`, 5 `THROTTLED`, 6 `SHUTTING_DOWN`, 7 `FAULT`, 8 `STANDBY`, 9 `TEST`, 10 `OTHER`
- `Evt` (bitfield32): bit 0 `GROUND_FAULT`, bit 1 `INPUT_OVER_VOLTAGE`, bit 2 `RESERVED_2`, bit 3 `DC_DISCONNECT`, bit 4 `RESERVED_4`, bit 5 `RESERVED_5`, bit 6 `MANUAL_SHUTDOWN`, bit 7 `OVER_TEMPERATURE`, bit 8 `RESERVED_8`, bit 9 `RESERVED_9`, bit 10 `RESERVED_10`, bit 11 `RESERVED_11`, bit 12 `BLOWN_FUSE`, bit 13 `UNDER_TEMPERATURE`, bit 14 `MEMORY_LOSS`, bit 15 `ARC_DETECTION`, bit 16 `THEFT_DETECTION`, bit 17 `OUTPUT_OVER_CURRENT`, bit 18 `OUTPUT_OVER_VOLTAGE`, bit 19 `OUTPUT_UNDER_VOLTAGE`, bit 20 `TEST_FAILED`

## Model 502: Solar Module

Name in records: `solar_module`.

A solar module model supporting DC-DC converter

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `Stat` | enum16 |  | Status | Module Status Code |
| `StatVend` | enum16 |  | Vendor Status | Module Vendor Status Code |
| `Evt` | bitfield32 |  | Events | Module Event Flags |
| `EvtVend` | bitfield32 |  | Vendor Module Event Flags | Vendor specific flags |
| `Ctl` | enum16 |  | Control | Module Control |
| `CtlVend` | enum32 |  | Vendor Control | Vendor Module Control |
| `CtlVal` | int32 |  | Control Value | Module Control Value |
| `Tms` | uint32 | Secs | Timestamp | Time in seconds since 2000 epoch |
| `OutA` | int16 | A | Output Current | Output Current |
| `OutV` | int16 | V | Output Voltage | Output Voltage |
| `OutWh` | acc32 | Wh | Output Energy | Output Energy |
| `OutPw` | int16 | W | Output Power | Output Power |
| `Tmp` | int16 | C | Temp | Module Temperature |
| `InA` | int16 | A | Input Current | Input Current |
| `InV` | int16 | V | Input Voltage | Input Voltage |
| `InWh` | acc32 | Wh | Input Energy | Input Energy |
| `InW` | int16 | W | Input Power | Input Power |

Value names:

- `Stat` (enum16): 1 `OFF`, 2 `SLEEPING`, 3 `STARTING`, 4 `MPPT`, 5 `THROTTLED`, 6 `SHUTTING_DOWN`, 7 `FAULT`, 8 `STANDBY`, 9 `TEST`, 10 `OTHER`
- `Evt` (bitfield32): bit 0 `GROUND_FAULT`, bit 1 `INPUT_OVER_VOLTAGE`, bit 2 `RESERVED_2`, bit 3 `DC_DISCONNECT`, bit 4 `RESERVED_4`, bit 5 `RESERVED_5`, bit 6 `MANUAL_SHUTDOWN`, bit 7 `OVER_TEMPERATURE`, bit 8 `RESERVED_8`, bit 9 `RESERVED_9`, bit 10 `RESERVED_10`, bit 11 `RESERVED_11`, bit 12 `BLOWN_FUSE`, bit 13 `UNDER_TEMPERATURE`, bit 14 `MEMORY_LOSS`, bit 15 `ARC_DETECTION`, bit 16 `THEFT_DETECTION`, bit 17 `OUTPUT_OVER_CURRENT`, bit 18 `OUTPUT_OVER_VOLTAGE`, bit 19 `OUTPUT_UNDER_VOLTAGE`, bit 20 `TEST_FAILED`
