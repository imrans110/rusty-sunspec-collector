# String combiners

<!-- Generated from the bundled SunSpec definitions by
     UPDATE_MODEL_DOCS=1 cargo test -p sunspec-parser --test models_docs
     Do not edit by hand. -->

Point names are the ones records use; `[*]` stands for a repeating group's index. See [Supported SunSpec models](README.md) for every group.

| Model | Name in records | Points |
|-------|-----------------|--------|
| [401](#model-401-string-combiner-current) | `string_combiner_current` | 13 |
| [402](#model-402-string-combiner-advanced) | `string_combiner_advanced` | 21 |
| [403](#model-403-string-combiner-current) | `string_combiner_current_input` | 13 |
| [404](#model-404-string-combiner-advanced) | `string_combiner_advanced_inputs` | 21 |

## Model 401: String Combiner (Current)

Name in records: `string_combiner_current`.

A basic string combiner

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `DCAMax` | uint16 | A | Rating | Maximum DC Current Rating |
| `N` | count |  | N | Number of Inputs |
| `Evt` | bitfield32 |  | Event | Events |
| `EvtVnd` | bitfield32 |  | Vendor Event | Vendor defined events |
| `DCA` | int16 | A | Amps | Total measured current |
| `DCAhr` | uint32 | Ah | Amp-hours | Total metered Amp-hours |
| `DCV` | uint16 | V | Voltage | Output Voltage |
| `Tmp` | int16 | C | Temp | Internal operating temperature |
| `string[*].InID` | uint16 |  | ID | Uniquely identifies this input set |
| `string[*].InEvt` | bitfield32 |  | Input Event | String Input Event Flags |
| `string[*].InEvtVnd` | bitfield32 |  | Input Event Vendor | String Input Vendor Event Flags |
| `string[*].InDCA` | int16 | A | Amps | String Input Current |
| `string[*].InDCAhr` | uint32 | Ah | Amp-hours | String Input Amp-Hours |

Value names:

- `Evt` (bitfield32): bit 0 `LOW_VOLTAGE`, bit 1 `LOW_POWER`, bit 2 `LOW_EFFICIENCY`, bit 3 `CURRENT`, bit 4 `VOLTAGE`, bit 5 `POWER`, bit 6 `PR`, bit 7 `DISCONNECTED`, bit 8 `FUSE_FAULT`, bit 9 `COMBINER_FUSE_FAULT`, bit 10 `COMBINER_CABINET_OPEN`, bit 11 `TEMP`, bit 12 `GROUNDFAULT`, bit 13 `REVERSED_POLARITY`, bit 14 `INCOMPATIBLE`, bit 15 `COMM_ERROR`, bit 16 `INTERNAL_ERROR`, bit 17 `THEFT`, bit 18 `ARC_DETECTED`
- `string[*].InEvt` (bitfield32): bit 0 `LOW_VOLTAGE`, bit 1 `LOW_POWER`, bit 2 `LOW_EFFICIENCY`, bit 3 `CURRENT`, bit 4 `VOLTAGE`, bit 5 `POWER`, bit 6 `PR`, bit 7 `DISCONNECTED`, bit 8 `FUSE_FAULT`, bit 9 `COMBINER_FUSE_FAULT`, bit 10 `COMBINER_CABINET_OPEN`, bit 11 `TEMP`, bit 12 `GROUNDFAULT`, bit 13 `REVERSED_POLARITY`, bit 14 `INCOMPATIBLE`, bit 15 `COMM_ERROR`, bit 16 `INTERNAL_ERROR`, bit 17 `THEFT`, bit 18 `ARC_DETECTED`

## Model 402: String Combiner (Advanced)

Name in records: `string_combiner_advanced`.

An advanced string combiner

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `DCAMax` | uint16 | A | Rating | Maximum DC Current Rating |
| `N` | count |  | N | Number of Inputs |
| `Evt` | bitfield32 |  | Event | Events |
| `EvtVnd` | bitfield32 |  | Vendor Event | Vendor defined events |
| `DCA` | int16 | A | Amps | Total measured current |
| `DCAhr` | uint32 | Ah | Amp-hours | Total metered Amp-hours |
| `DCV` | uint16 | V | Voltage | Output Voltage |
| `Tmp` | int16 | C | Temp | Internal operating temperature |
| `DCW` | int16 | W | Watts | Output power |
| `DCPR` | uint16 | Pct | PR | DC Performance ratio value |
| `DCWh` | uint32 | Wh | Watt-hours | Output energy |
| `string[*].InID` | uint16 |  | ID | Uniquely identifies this input set |
| `string[*].InEvt` | bitfield32 |  | Input Event | String Input Event Flags |
| `string[*].EvtVnd` | bitfield32 |  | Vendor Event | Vendor defined events |
| `string[*].InDCA` | int16 | A | Amps | String Input Current |
| `string[*].InDCAhr` | uint32 | Ah | Amp-hours | String Input Amp-Hours |
| `string[*].InDCV` | uint16 | V | Voltage | String Input Voltage |
| `string[*].InDCW` | int16 | W | Watts | String Input Power |
| `string[*].InDCWh` | uint32 | Wh | Watt-hours | String Input Energy |
| `string[*].InDCPR` | uint16 | Pct | PR | String Performance Ratio |
| `string[*].InN` | uint16 |  | N | Number of modules in this input string |

Value names:

- `Evt` (bitfield32): bit 0 `LOW_VOLTAGE`, bit 1 `LOW_POWER`, bit 2 `LOW_EFFICIENCY`, bit 3 `CURRENT`, bit 4 `VOLTAGE`, bit 5 `POWER`, bit 6 `PR`, bit 7 `DISCONNECTED`, bit 8 `FUSE_FAULT`, bit 9 `COMBINER_FUSE_FAULT`, bit 10 `COMBINER_CABINET_OPEN`, bit 11 `TEMP`, bit 12 `GROUNDFAULT`, bit 13 `REVERSED_POLARITY`, bit 14 `INCOMPATIBLE`, bit 15 `COMM_ERROR`, bit 16 `INTERNAL_ERROR`, bit 17 `THEFT`, bit 18 `ARC_DETECTED`
- `string[*].InEvt` (bitfield32): bit 0 `LOW_VOLTAGE`, bit 1 `LOW_POWER`, bit 2 `LOW_EFFICIENCY`, bit 3 `CURRENT`, bit 4 `VOLTAGE`, bit 5 `POWER`, bit 6 `PR`, bit 7 `DISCONNECTED`, bit 8 `FUSE_FAULT`, bit 9 `COMBINER_FUSE_FAULT`, bit 10 `COMBINER_CABINET_OPEN`, bit 11 `TEMP`, bit 12 `GROUNDFAULT`, bit 13 `REVERSED_POLARITY`, bit 14 `INCOMPATIBLE`, bit 15 `COMM_ERROR`, bit 16 `INTERNAL_ERROR`, bit 17 `THEFT`, bit 18 `ARC_DETECTED`

## Model 403: String Combiner (Current)

Name in records: `string_combiner_current_input`.

A basic string combiner model

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `DCAMax` | uint16 | A | Rating | Maximum DC Current Rating |
| `N` | count |  | N | Number of Inputs |
| `Evt` | bitfield32 |  | Event | Events |
| `EvtVnd` | bitfield32 |  | Vendor Event | Vendor defined events |
| `DCA` | int16 | A | Amps | Total measured current |
| `DCAhr` | acc32 | Ah | Amp-hours | Total metered Amp-hours |
| `DCV` | int16 | V | Voltage | Output Voltage |
| `Tmp` | int16 | C | Temp | Internal operating temperature |
| `string[*].InID` | uint16 |  | ID | Uniquely identifies this input set |
| `string[*].InEvt` | bitfield32 |  | Input Event | String Input Event Flags |
| `string[*].InEvtVnd` | bitfield32 |  | Input Event Vendor | String Input Vendor Event Flags |
| `string[*].InDCA` | int16 | A | Amps | String Input Current |
| `string[*].InDCAhr` | acc32 | Ah | Amp-hours | String Input Amp-Hours |

Value names:

- `Evt` (bitfield32): bit 0 `LOW_VOLTAGE`, bit 1 `LOW_POWER`, bit 2 `LOW_EFFICIENCY`, bit 3 `CURRENT`, bit 4 `VOLTAGE`, bit 5 `POWER`, bit 6 `PR`, bit 7 `DISCONNECTED`, bit 8 `FUSE_FAULT`, bit 9 `COMBINER_FUSE_FAULT`, bit 10 `COMBINER_CABINET_OPEN`, bit 11 `TEMP`, bit 12 `GROUNDFAULT`, bit 13 `REVERSED_POLARITY`, bit 14 `INCOMPATIBLE`, bit 15 `COMM_ERROR`, bit 16 `INTERNAL_ERROR`, bit 17 `THEFT`, bit 18 `ARC_DETECTED`
- `string[*].InEvt` (bitfield32): bit 0 `LOW_VOLTAGE`, bit 1 `LOW_POWER`, bit 2 `LOW_EFFICIENCY`, bit 3 `CURRENT`, bit 4 `VOLTAGE`, bit 5 `POWER`, bit 6 `PR`, bit 7 `DISCONNECTED`, bit 8 `FUSE_FAULT`, bit 9 `COMBINER_FUSE_FAULT`, bit 10 `COMBINER_CABINET_OPEN`, bit 11 `TEMP`, bit 12 `GROUNDFAULT`, bit 13 `REVERSED_POLARITY`, bit 14 `INCOMPATIBLE`, bit 15 `COMM_ERROR`, bit 16 `INTERNAL_ERROR`, bit 17 `THEFT`, bit 18 `ARC_DETECTED`

## Model 404: String Combiner (Advanced)

Name in records: `string_combiner_advanced_inputs`.

An advanced string combiner including voltage and energy measurements

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `DCAMax` | uint16 | A | Rating | Maximum DC Current Rating |
| `N` | count |  | N | Number of Inputs |
| `Evt` | bitfield32 |  | Event | Events |
| `EvtVnd` | bitfield32 |  | Vendor Event | Vendor defined events |
| `DCA` | int16 | A | Amps | Total measured current |
| `DCAhr` | acc32 | Ah | Amp-hours | Total metered Amp-hours |
| `DCV` | int16 | V | Voltage | Output Voltage |
| `Tmp` | int16 | C | Temp | Internal operating temperature |
| `DCW` | int16 | W | Watts | Output power |
| `DCPR` | int16 | Pct | PR | DC Performance ratio value |
| `DCWh` | acc32 | Wh | Watt-hours | Output energy |
| `string[*].InID` | uint16 |  | ID | Uniquely identifies this input set |
| `string[*].InEvt` | bitfield32 |  | Input Event | String Input Event Flags |
| `string[*].InEvtVnd` | bitfield32 |  | Input Event Vendor | String Input Vendor Event Flags |
| `string[*].InDCA` | int16 | A | Amps | String Input Current |
| `string[*].InDCAhr` | acc32 | Ah | Amp-hours | String Input Amp-Hours |
| `string[*].InDCV` | int16 | V | Voltage | String Input Voltage |
| `string[*].InDCW` | int16 | W | Watts | String Input Power |
| `string[*].InDCWh` | acc32 | Wh | Watt-hours | String Input Energy |
| `string[*].InDCPR` | uint16 | Pct | PR | String Performance Ratio |
| `string[*].InN` | uint16 |  | N | Number of modules in this input string |

Value names:

- `Evt` (bitfield32): bit 0 `LOW_VOLTAGE`, bit 1 `LOW_POWER`, bit 2 `LOW_EFFICIENCY`, bit 3 `CURRENT`, bit 4 `VOLTAGE`, bit 5 `POWER`, bit 6 `PR`, bit 7 `DISCONNECTED`, bit 8 `FUSE_FAULT`, bit 9 `COMBINER_FUSE_FAULT`, bit 10 `COMBINER_CABINET_OPEN`, bit 11 `TEMP`, bit 12 `GROUNDFAULT`, bit 13 `REVERSED_POLARITY`, bit 14 `INCOMPATIBLE`, bit 15 `COMM_ERROR`, bit 16 `INTERNAL_ERROR`, bit 17 `THEFT`, bit 18 `ARC_DETECTED`
- `string[*].InEvt` (bitfield32): bit 0 `LOW_VOLTAGE`, bit 1 `LOW_POWER`, bit 2 `LOW_EFFICIENCY`, bit 3 `CURRENT`, bit 4 `VOLTAGE`, bit 5 `POWER`, bit 6 `PR`, bit 7 `DISCONNECTED`, bit 8 `FUSE_FAULT`, bit 9 `COMBINER_FUSE_FAULT`, bit 10 `COMBINER_CABINET_OPEN`, bit 11 `TEMP`, bit 12 `GROUNDFAULT`, bit 13 `REVERSED_POLARITY`, bit 14 `INCOMPATIBLE`, bit 15 `COMM_ERROR`, bit 16 `INTERNAL_ERROR`, bit 17 `THEFT`, bit 18 `ARC_DETECTED`
