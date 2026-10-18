# Inverters and inverter controls

<!-- Generated from the bundled SunSpec definitions by
     UPDATE_MODEL_DOCS=1 cargo test -p sunspec-parser --test models_docs
     Do not edit by hand. -->

Point names are the ones records use; `[*]` stands for a repeating group's index. See [Supported SunSpec models](README.md) for every group.

| Model | Name in records | Points |
|-------|-----------------|--------|
| [101](#model-101-inverter-single-phase) | `inverter_single_phase` | 31 |
| [102](#model-102-inverter-split-phase) | `inverter_split_phase` | 31 |
| [103](#model-103-inverter-three-phase) | `inverter_three_phase` | 31 |
| [111](#model-111-inverter-single-phase-float) | `inverter_single_phase_float` | 31 |
| [112](#model-112-inverter-split-phase-float) | `inverter_split_phase_float` | 31 |
| [113](#model-113-inverter-three-phase-float) | `inverter_three_phase_float` | 31 |
| [120](#model-120-nameplate) | `nameplate` | 16 |
| [121](#model-121-basic-settings) | `settings` | 20 |
| [122](#model-122-measurements_status) | `status` | 17 |
| [123](#model-123-immediate-controls) | `controls` | 21 |
| [124](#model-124-storage) | `storage_basic` | 16 |
| [125](#model-125-pricing) | `pricing` | 6 |
| [126](#model-126-static-volt-var) | `volt_var` | 54 |
| [127](#model-127-freq-watt-param) | `freq_watt_param` | 6 |
| [128](#model-128-dynamic-reactive-current) | `reactive_current` | 11 |
| [129](#model-129-lvrtd) | `lvrt` | 50 |
| [130](#model-130-hvrtd) | `hvrt` | 50 |
| [131](#model-131-watt-pf) | `watt_pf` | 53 |
| [132](#model-132-volt-watt) | `volt_watt` | 54 |
| [133](#model-133-basic-scheduling) | `schedule` | 34 |
| [134](#model-134-freq-watt-crv) | `freq_watt` | 58 |
| [135](#model-135-lfrt) | `lfrt` | 50 |
| [136](#model-136-hfrt) | `hfrt` | 50 |
| [137](#model-137-lvrtc) | `lvrtc` | 50 |
| [138](#model-138-hvrtc) | `hvrtc` | 50 |
| [139](#model-139-lvrtx) | `lvrtx` | 51 |
| [140](#model-140-hvrtx) | `hvrtx` | 51 |
| [141](#model-141-lfrtc) | `lfrtc` | 50 |
| [142](#model-142-hfrtc) | `hfrtc` | 50 |
| [143](#model-143-lfrtx) | `lfrtx` | 51 |
| [144](#model-144-hfrtx) | `hfrtx` | 51 |
| [145](#model-145-extended-settings) | `ext_settings` | 7 |
| [160](#model-160-multiple-mppt-inverter-extension-model) | `mppt` | 13 |

## Model 101: Inverter (Single Phase)

Name in records: `inverter_single_phase`.

Include this model for single phase inverter monitoring

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `A` | uint16 | A | Amps | AC Current |
| `AphA` | uint16 | A | Amps PhaseA | Phase A Current |
| `AphB` | uint16 | A | Amps PhaseB | Phase B Current |
| `AphC` | uint16 | A | Amps PhaseC | Phase C Current |
| `PPVphAB` | uint16 | V | Phase Voltage AB | Phase Voltage AB |
| `PPVphBC` | uint16 | V | Phase Voltage BC | Phase Voltage BC |
| `PPVphCA` | uint16 | V | Phase Voltage CA | Phase Voltage CA |
| `PhVphA` | uint16 | V | Phase Voltage AN | Phase Voltage AN |
| `PhVphB` | uint16 | V | Phase Voltage BN | Phase Voltage BN |
| `PhVphC` | uint16 | V | Phase Voltage CN | Phase Voltage CN |
| `W` | int16 | W | Watts | AC Power |
| `Hz` | uint16 | Hz | Hz | Line Frequency |
| `VA` | int16 | VA | VA | AC Apparent Power |
| `VAr` | int16 | var | VAr | AC Reactive Power |
| `PF` | int16 | Pct | PF | AC Power Factor |
| `WH` | acc32 | Wh | WattHours | AC Energy |
| `DCA` | uint16 | A | DC Amps | DC Current |
| `DCV` | uint16 | V | DC Voltage | DC Voltage |
| `DCW` | int16 | W | DC Watts | DC Power |
| `TmpCab` | int16 | C | Cabinet Temperature | Cabinet Temperature |
| `TmpSnk` | int16 | C | Heat Sink Temperature | Heat Sink Temperature |
| `TmpTrns` | int16 | C | Transformer Temperature | Transformer Temperature |
| `TmpOt` | int16 | C | Other Temperature | Other Temperature |
| `St` | enum16 |  | Operating State | Operating state |
| `StVnd` | enum16 |  | Vendor Operating State | Vendor specific operating state code |
| `Evt1` | bitfield32 |  | Event1 | Event fields |
| `Evt2` | bitfield32 |  | Event Bitfield 2 | Reserved for future use |
| `EvtVnd1` | bitfield32 |  | Vendor Event Bitfield 1 | Vendor defined events |
| `EvtVnd2` | bitfield32 |  | Vendor Event Bitfield 2 | Vendor defined events |
| `EvtVnd3` | bitfield32 |  | Vendor Event Bitfield 3 | Vendor defined events |
| `EvtVnd4` | bitfield32 |  | Vendor Event Bitfield 4 | Vendor defined events |

Value names:

- `St` (enum16): 1 `OFF`, 2 `SLEEPING`, 3 `STARTING`, 4 `MPPT`, 5 `THROTTLED`, 6 `SHUTTING_DOWN`, 7 `FAULT`, 8 `STANDBY`
- `Evt1` (bitfield32): bit 0 `GROUND_FAULT`, bit 1 `DC_OVER_VOLT`, bit 2 `AC_DISCONNECT`, bit 3 `DC_DISCONNECT`, bit 4 `GRID_DISCONNECT`, bit 5 `CABINET_OPEN`, bit 6 `MANUAL_SHUTDOWN`, bit 7 `OVER_TEMP`, bit 8 `OVER_FREQUENCY`, bit 9 `UNDER_FREQUENCY`, bit 10 `AC_OVER_VOLT`, bit 11 `AC_UNDER_VOLT`, bit 12 `BLOWN_STRING_FUSE`, bit 13 `UNDER_TEMP`, bit 14 `MEMORY_LOSS`, bit 15 `HW_TEST_FAILURE`

## Model 102: Inverter (Split-Phase)

Name in records: `inverter_split_phase`.

Include this model for split phase inverter monitoring

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `A` | uint16 | A | Amps | AC Current |
| `AphA` | uint16 | A | Amps PhaseA | Phase A Current |
| `AphB` | uint16 | A | Amps PhaseB | Phase B Current |
| `AphC` | uint16 | A | Amps PhaseC | Phase C Current |
| `PPVphAB` | uint16 | V | Phase Voltage AB | Phase Voltage AB |
| `PPVphBC` | uint16 | V | Phase Voltage BC | Phase Voltage BC |
| `PPVphCA` | uint16 | V | Phase Voltage CA | Phase Voltage CA |
| `PhVphA` | uint16 | V | Phase Voltage AN | Phase Voltage AN |
| `PhVphB` | uint16 | V | Phase Voltage BN | Phase Voltage BN |
| `PhVphC` | uint16 | V | Phase Voltage CN | Phase Voltage CN |
| `W` | int16 | W | Watts | AC Power |
| `Hz` | uint16 | Hz | Hz | Line Frequency |
| `VA` | int16 | VA | VA | AC Apparent Power |
| `VAr` | int16 | var | VAr | AC Reactive Power |
| `PF` | int16 | Pct | PF | AC Power Factor |
| `WH` | acc32 | Wh | WattHours | AC Energy |
| `DCA` | uint16 | A | DC Amps | DC Current |
| `DCV` | uint16 | V | DC Voltage | DC Voltage |
| `DCW` | int16 | W | DC Watts | DC Power |
| `TmpCab` | int16 | C | Cabinet Temperature | Cabinet Temperature |
| `TmpSnk` | int16 | C | Heat Sink Temperature | Heat Sink Temperature |
| `TmpTrns` | int16 | C | Transformer Temperature | Transformer Temperature |
| `TmpOt` | int16 | C | Other Temperature | Other Temperature |
| `St` | enum16 |  | Operating State | Operating state |
| `StVnd` | enum16 |  | Vendor Operating State | Vendor specific operating state code |
| `Evt1` | bitfield32 |  | Event1 | Event fields |
| `Evt2` | bitfield32 |  | Event Bitfield 2 | Reserved for future use |
| `EvtVnd1` | bitfield32 |  | Vendor Event Bitfield 1 | Vendor defined events |
| `EvtVnd2` | bitfield32 |  | Vendor Event Bitfield 2 | Vendor defined events |
| `EvtVnd3` | bitfield32 |  | Vendor Event Bitfield 3 | Vendor defined events |
| `EvtVnd4` | bitfield32 |  | Vendor Event Bitfield 4 | Vendor defined events |

Value names:

- `St` (enum16): 1 `OFF`, 2 `SLEEPING`, 3 `STARTING`, 4 `MPPT`, 5 `THROTTLED`, 6 `SHUTTING_DOWN`, 7 `FAULT`, 8 `STANDBY`
- `Evt1` (bitfield32): bit 0 `GROUND_FAULT`, bit 1 `DC_OVER_VOLT`, bit 2 `AC_DISCONNECT`, bit 3 `DC_DISCONNECT`, bit 4 `GRID_DISCONNECT`, bit 5 `CABINET_OPEN`, bit 6 `MANUAL_SHUTDOWN`, bit 7 `OVER_TEMP`, bit 8 `OVER_FREQUENCY`, bit 9 `UNDER_FREQUENCY`, bit 10 `AC_OVER_VOLT`, bit 11 `AC_UNDER_VOLT`, bit 12 `BLOWN_STRING_FUSE`, bit 13 `UNDER_TEMP`, bit 14 `MEMORY_LOSS`, bit 15 `HW_TEST_FAILURE`

## Model 103: Inverter (Three Phase)

Name in records: `inverter_three_phase`.

Include this model for three phase inverter monitoring

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `A` | uint16 | A | Amps | AC Current |
| `AphA` | uint16 | A | Amps PhaseA | Phase A Current |
| `AphB` | uint16 | A | Amps PhaseB | Phase B Current |
| `AphC` | uint16 | A | Amps PhaseC | Phase C Current |
| `PPVphAB` | uint16 | V | Phase Voltage AB | Phase Voltage AB |
| `PPVphBC` | uint16 | V | Phase Voltage BC | Phase Voltage BC |
| `PPVphCA` | uint16 | V | Phase Voltage CA | Phase Voltage CA |
| `PhVphA` | uint16 | V | Phase Voltage AN | Phase Voltage AN |
| `PhVphB` | uint16 | V | Phase Voltage BN | Phase Voltage BN |
| `PhVphC` | uint16 | V | Phase Voltage CN | Phase Voltage CN |
| `W` | int16 | W | Watts | AC Power |
| `Hz` | uint16 | Hz | Hz | Line Frequency |
| `VA` | int16 | VA | VA | AC Apparent Power |
| `VAr` | int16 | var | VAr | AC Reactive Power |
| `PF` | int16 | Pct | PF | AC Power Factor |
| `WH` | acc32 | Wh | WattHours | AC Energy |
| `DCA` | uint16 | A | DC Amps | DC Current |
| `DCV` | uint16 | V | DC Voltage | DC Voltage |
| `DCW` | int16 | W | DC Watts | DC Power |
| `TmpCab` | int16 | C | Cabinet Temperature | Cabinet Temperature |
| `TmpSnk` | int16 | C | Heat Sink Temperature | Heat Sink Temperature |
| `TmpTrns` | int16 | C | Transformer Temperature | Transformer Temperature |
| `TmpOt` | int16 | C | Other Temperature | Other Temperature |
| `St` | enum16 |  | Operating State | Operating state |
| `StVnd` | enum16 |  | Vendor Operating State | Vendor specific operating state code |
| `Evt1` | bitfield32 |  | Event1 | Event fields |
| `Evt2` | bitfield32 |  | Event Bitfield 2 | Reserved for future use |
| `EvtVnd1` | bitfield32 |  | Vendor Event Bitfield 1 | Vendor defined events |
| `EvtVnd2` | bitfield32 |  | Vendor Event Bitfield 2 | Vendor defined events |
| `EvtVnd3` | bitfield32 |  | Vendor Event Bitfield 3 | Vendor defined events |
| `EvtVnd4` | bitfield32 |  | Vendor Event Bitfield 4 | Vendor defined events |

Value names:

- `St` (enum16): 1 `OFF`, 2 `SLEEPING`, 3 `STARTING`, 4 `MPPT`, 5 `THROTTLED`, 6 `SHUTTING_DOWN`, 7 `FAULT`, 8 `STANDBY`
- `Evt1` (bitfield32): bit 0 `GROUND_FAULT`, bit 1 `DC_OVER_VOLT`, bit 2 `AC_DISCONNECT`, bit 3 `DC_DISCONNECT`, bit 4 `GRID_DISCONNECT`, bit 5 `CABINET_OPEN`, bit 6 `MANUAL_SHUTDOWN`, bit 7 `OVER_TEMP`, bit 8 `OVER_FREQUENCY`, bit 9 `UNDER_FREQUENCY`, bit 10 `AC_OVER_VOLT`, bit 11 `AC_UNDER_VOLT`, bit 12 `BLOWN_STRING_FUSE`, bit 13 `UNDER_TEMP`, bit 14 `MEMORY_LOSS`, bit 15 `HW_TEST_FAILURE`

## Model 111: Inverter (Single Phase) FLOAT

Name in records: `inverter_single_phase_float`.

Include this model for single phase inverter monitoring using float values

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `A` | float32 | A | Amps | AC Current |
| `AphA` | float32 | A | Amps PhaseA | Phase A Current |
| `AphB` | float32 | A | Amps PhaseB | Phase B Current |
| `AphC` | float32 | A | Amps PhaseC | Phase C Current |
| `PPVphAB` | float32 | V | Phase Voltage AB | Phase Voltage AB |
| `PPVphBC` | float32 | V | Phase Voltage BC | Phase Voltage BC |
| `PPVphCA` | float32 | V | Phase Voltage CA | Phase Voltage CA |
| `PhVphA` | float32 | V | Phase Voltage AN | Phase Voltage AN |
| `PhVphB` | float32 | V | Phase Voltage BN | Phase Voltage BN |
| `PhVphC` | float32 | V | Phase Voltage CN | Phase Voltage CN |
| `W` | float32 | W | Watts | AC Power |
| `Hz` | float32 | Hz | Hz | Line Frequency |
| `VA` | float32 | VA | VA | AC Apparent Power |
| `VAr` | float32 | var | VAr | AC Reactive Power |
| `PF` | float32 | Pct | PF | AC Power Factor |
| `WH` | float32 | Wh | WattHours | AC Energy |
| `DCA` | float32 | A | DC Amps | DC Current |
| `DCV` | float32 | V | DC Voltage | DC Voltage |
| `DCW` | float32 | W | DC Watts | DC Power |
| `TmpCab` | float32 | C | Cabinet Temperature | Cabinet Temperature |
| `TmpSnk` | float32 | C | Heat Sink Temperature | Heat Sink Temperature |
| `TmpTrns` | float32 | C | Transformer Temperature | Transformer Temperature |
| `TmpOt` | float32 | C | Other Temperature | Other Temperature |
| `St` | enum16 |  | Operating State | Operating state |
| `StVnd` | enum16 |  | Vendor Operating State | Vendor specific operating state code |
| `Evt1` | bitfield32 |  | Event1 | Event fields |
| `Evt2` | bitfield32 |  | Event Bitfield 2 | Reserved for future use |
| `EvtVnd1` | bitfield32 |  | Vendor Event Bitfield 1 | Vendor defined events |
| `EvtVnd2` | bitfield32 |  | Vendor Event Bitfield 2 | Vendor defined events |
| `EvtVnd3` | bitfield32 |  | Vendor Event Bitfield 3 | Vendor defined events |
| `EvtVnd4` | bitfield32 |  | Vendor Event Bitfield 4 | Vendor defined events |

Value names:

- `St` (enum16): 1 `ggOFF`, 2 `ggSLEEPING`, 3 `ggSTARTING`, 4 `ggMPPT`, 5 `ggTHROTTLED`, 6 `ggSHUTTING_DOWN`, 7 `ggFAULT`, 8 `ggSTANDBY`
- `Evt1` (bitfield32): bit 0 `GROUND_FAULT`, bit 1 `DC_OVER_VOLT`, bit 2 `AC_DISCONNECT`, bit 3 `DC_DISCONNECT`, bit 4 `GRID_DISCONNECT`, bit 5 `CABINET_OPEN`, bit 6 `MANUAL_SHUTDOWN`, bit 7 `OVER_TEMP`, bit 8 `OVER_FREQUENCY`, bit 9 `UNDER_FREQUENCY`, bit 10 `AC_OVER_VOLT`, bit 11 `AC_UNDER_VOLT`, bit 12 `BLOWN_STRING_FUSE`, bit 13 `UNDER_TEMP`, bit 14 `MEMORY_LOSS`, bit 15 `HW_TEST_FAILURE`

## Model 112: Inverter (Split Phase) FLOAT

Name in records: `inverter_split_phase_float`.

Include this model for split phase inverter monitoring using float values

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `A` | float32 | A | Amps | AC Current |
| `AphA` | float32 | A | Amps PhaseA | Phase A Current |
| `AphB` | float32 | A | Amps PhaseB | Phase B Current |
| `AphC` | float32 | A | Amps PhaseC | Phase C Current |
| `PPVphAB` | float32 | V | Phase Voltage AB | Phase Voltage AB |
| `PPVphBC` | float32 | V | Phase Voltage BC | Phase Voltage BC |
| `PPVphCA` | float32 | V | Phase Voltage CA | Phase Voltage CA |
| `PhVphA` | float32 | V | Phase Voltage AN | Phase Voltage AN |
| `PhVphB` | float32 | V | Phase Voltage BN | Phase Voltage BN |
| `PhVphC` | float32 | V | Phase Voltage CN | Phase Voltage CN |
| `W` | float32 | W | Watts | AC Power |
| `Hz` | float32 | Hz | Hz | Line Frequency |
| `VA` | float32 | VA | VA | AC Apparent Power |
| `VAr` | float32 | var | VAr | AC Reactive Power |
| `PF` | float32 | Pct | PF | AC Power Factor |
| `WH` | float32 | Wh | WattHours | AC Energy |
| `DCA` | float32 | A | DC Amps | DC Current |
| `DCV` | float32 | V | DC Voltage | DC Voltage |
| `DCW` | float32 | W | DC Watts | DC Power |
| `TmpCab` | float32 | C | Cabinet Temperature | Cabinet Temperature |
| `TmpSnk` | float32 | C | Heat Sink Temperature | Heat Sink Temperature |
| `TmpTrns` | float32 | C | Transformer Temperature | Transformer Temperature |
| `TmpOt` | float32 | C | Other Temperature | Other Temperature |
| `St` | enum16 |  | Operating State | Operating state |
| `StVnd` | enum16 |  | Vendor Operating State | Vendor specific operating state code |
| `Evt1` | bitfield32 |  | Event1 | Event fields |
| `Evt2` | bitfield32 |  | Event Bitfield 2 | Reserved for future use |
| `EvtVnd1` | bitfield32 |  | Vendor Event Bitfield 1 | Vendor defined events |
| `EvtVnd2` | bitfield32 |  | Vendor Event Bitfield 2 | Vendor defined events |
| `EvtVnd3` | bitfield32 |  | Vendor Event Bitfield 3 | Vendor defined events |
| `EvtVnd4` | bitfield32 |  | Vendor Event Bitfield 4 | Vendor defined events |

Value names:

- `St` (enum16): 1 `OFF`, 2 `SLEEPING`, 3 `STARTING`, 4 `MPPT`, 5 `THROTTLED`, 6 `SHUTTING_DOWN`, 7 `FAULT`, 8 `STANDBY`
- `Evt1` (bitfield32): bit 0 `GROUND_FAULT`, bit 1 `DC_OVER_VOLT`, bit 2 `AC_DISCONNECT`, bit 3 `DC_DISCONNECT`, bit 4 `GRID_DISCONNECT`, bit 5 `CABINET_OPEN`, bit 6 `MANUAL_SHUTDOWN`, bit 7 `OVER_TEMP`, bit 8 `OVER_FREQUENCY`, bit 9 `UNDER_FREQUENCY`, bit 10 `AC_OVER_VOLT`, bit 11 `AC_UNDER_VOLT`, bit 12 `BLOWN_STRING_FUSE`, bit 13 `UNDER_TEMP`, bit 14 `MEMORY_LOSS`, bit 15 `HW_TEST_FAILURE`

## Model 113: Inverter (Three Phase) FLOAT

Name in records: `inverter_three_phase_float`.

Include this model for three phase inverter monitoring using float values

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `A` | float32 | A | Amps | AC Current |
| `AphA` | float32 | A | Amps PhaseA | Phase A Current |
| `AphB` | float32 | A | Amps PhaseB | Phase B Current |
| `AphC` | float32 | A | Amps PhaseC | Phase C Current |
| `PPVphAB` | float32 | V | Phase Voltage AB | Phase Voltage AB |
| `PPVphBC` | float32 | V | Phase Voltage BC | Phase Voltage BC |
| `PPVphCA` | float32 | V | Phase Voltage CA | Phase Voltage CA |
| `PhVphA` | float32 | V | Phase Voltage AN | Phase Voltage AN |
| `PhVphB` | float32 | V | Phase Voltage BN | Phase Voltage BN |
| `PhVphC` | float32 | V | Phase Voltage CN | Phase Voltage CN |
| `W` | float32 | W | Watts | AC Power |
| `Hz` | float32 | Hz | Hz | Line Frequency |
| `VA` | float32 | VA | VA | AC Apparent Power |
| `VAr` | float32 | var | VAr | AC Reactive Power |
| `PF` | float32 | Pct | PF | AC Power Factor |
| `WH` | float32 | Wh | WattHours | AC Energy |
| `DCA` | float32 | A | DC Amps | DC Current |
| `DCV` | float32 | V | DC Voltage | DC Voltage |
| `DCW` | float32 | W | DC Watts | DC Power |
| `TmpCab` | float32 | C | Cabinet Temperature | Cabinet Temperature |
| `TmpSnk` | float32 | C | Heat Sink Temperature | Heat Sink Temperature |
| `TmpTrns` | float32 | C | Transformer Temperature | Transformer Temperature |
| `TmpOt` | float32 | C | Other Temperature | Other Temperature |
| `St` | enum16 |  | Operating State | Operating state |
| `StVnd` | enum16 |  | Vendor Operating State | Vendor specific operating state code |
| `Evt1` | bitfield32 |  | Event1 | Event fields |
| `Evt2` | bitfield32 |  | Event Bitfield 2 | Reserved for future use |
| `EvtVnd1` | bitfield32 |  | Vendor Event Bitfield 1 | Vendor defined events |
| `EvtVnd2` | bitfield32 |  | Vendor Event Bitfield 2 | Vendor defined events |
| `EvtVnd3` | bitfield32 |  | Vendor Event Bitfield 3 | Vendor defined events |
| `EvtVnd4` | bitfield32 |  | Vendor Event Bitfield 4 | Vendor defined events |

Value names:

- `St` (enum16): 1 `OFF`, 2 `SLEEPING`, 3 `STARTING`, 4 `MPPT`, 5 `THROTTLED`, 6 `SHUTTING_DOWN`, 7 `FAULT`, 8 `STANDBY`
- `Evt1` (bitfield32): bit 0 `GROUND_FAULT`, bit 1 `DC_OVER_VOLT`, bit 2 `AC_DISCONNECT`, bit 3 `DC_DISCONNECT`, bit 4 `GRID_DISCONNECT`, bit 5 `CABINET_OPEN`, bit 6 `MANUAL_SHUTDOWN`, bit 7 `OVER_TEMP`, bit 8 `OVER_FREQUENCY`, bit 9 `UNDER_FREQUENCY`, bit 10 `AC_OVER_VOLT`, bit 11 `AC_UNDER_VOLT`, bit 12 `BLOWN_STRING_FUSE`, bit 13 `UNDER_TEMP`, bit 14 `MEMORY_LOSS`, bit 15 `HW_TEST_FAILURE`

## Model 120: Nameplate

Name in records: `nameplate`.

Inverter Controls Nameplate Ratings

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `DERTyp` | enum16 |  | DERTyp | Type of DER device. Default value is 4 to indicate PV device. |
| `WRtg` | uint16 | W | WRtg | Continuous power output capability of the inverter. |
| `VARtg` | uint16 | VA | VARtg | Continuous Volt-Ampere capability of the inverter. |
| `VArRtgQ1` | int16 | var | VArRtgQ1 | Continuous VAR capability of the inverter in quadrant 1. |
| `VArRtgQ2` | int16 | var | VArRtgQ2 | Continuous VAR capability of the inverter in quadrant 2. |
| `VArRtgQ3` | int16 | var | VArRtgQ3 | Continuous VAR capability of the inverter in quadrant 3. |
| `VArRtgQ4` | int16 | var | VArRtgQ4 | Continuous VAR capability of the inverter in quadrant 4. |
| `ARtg` | uint16 | A | ARtg | Maximum RMS AC current level capability of the inverter. |
| `PFRtgQ1` | int16 | cos() | PFRtgQ1 | Minimum power factor capability of the inverter in quadrant 1. |
| `PFRtgQ2` | int16 | cos() | PFRtgQ2 | Minimum power factor capability of the inverter in quadrant 2. |
| `PFRtgQ3` | int16 | cos() | PFRtgQ3 | Minimum power factor capability of the inverter in quadrant 3. |
| `PFRtgQ4` | int16 | cos() | PFRtgQ4 | Minimum power factor capability of the inverter in quadrant 4. |
| `WHRtg` | uint16 | Wh | WHRtg | Nominal energy rating of storage device. |
| `AhrRtg` | uint16 | AH | AhrRtg | The usable capacity of the battery. Maximum charge minus minimum charge from a technology capability perspective (Amp-hour capacity rating). |
| `MaxChaRte` | uint16 | W | MaxChaRte | Maximum rate of energy transfer into the storage device. |
| `MaxDisChaRte` | uint16 | W | MaxDisChaRte | Maximum rate of energy transfer out of the storage device. |

Value names:

- `DERTyp` (enum16): 4 `PV`, 82 `PV_STOR`

## Model 121: Basic Settings

Name in records: `settings`.

Inverter Controls Basic Settings

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `WMax` | uint16 | W | WMax | Setting for maximum power output. Default to WRtg. |
| `VRef` | uint16 | V | VRef | Voltage at the PCC. |
| `VRefOfs` | int16 | V | VRefOfs | Offset from PCC to inverter. |
| `VMax` | uint16 | V | VMax | Setpoint for maximum voltage. |
| `VMin` | uint16 | V | VMin | Setpoint for minimum voltage. |
| `VAMax` | uint16 | VA | VAMax | Setpoint for maximum apparent power. Default to VARtg. |
| `VArMaxQ1` | int16 | var | VArMaxQ1 | Setting for maximum reactive power in quadrant 1. Default to VArRtgQ1. |
| `VArMaxQ2` | int16 | var | VArMaxQ2 | Setting for maximum reactive power in quadrant 2. Default to VArRtgQ2. |
| `VArMaxQ3` | int16 | var | VArMaxQ3 | Setting for maximum reactive power in quadrant 3. Default to VArRtgQ3. |
| `VArMaxQ4` | int16 | var | VArMaxQ4 | Setting for maximum reactive power in quadrant 4. Default to VArRtgQ4. |
| `WGra` | uint16 | % WMax/sec | WGra | Default ramp rate of change of active power due to command or internal action. |
| `PFMinQ1` | int16 | cos() | PFMinQ1 | Setpoint for minimum power factor value in quadrant 1. Default to PFRtgQ1. |
| `PFMinQ2` | int16 | cos() | PFMinQ2 | Setpoint for minimum power factor value in quadrant 2. Default to PFRtgQ2. |
| `PFMinQ3` | int16 | cos() | PFMinQ3 | Setpoint for minimum power factor value in quadrant 3. Default to PFRtgQ3. |
| `PFMinQ4` | int16 | cos() | PFMinQ4 | Setpoint for minimum power factor value in quadrant 4. Default to PFRtgQ4. |
| `VArAct` | enum16 |  | VArAct | VAR action on change between charging and discharging: 1=switch 2=maintain VAR characterization. |
| `ClcTotVA` | enum16 |  | ClcTotVA | Calculation method for total apparent power. 1=vector 2=arithmetic. |
| `MaxRmpRte` | uint16 | % WGra | MaxRmpRte | Setpoint for maximum ramp rate as percentage of nominal maximum ramp rate. This setting will limit the rate that watts delivery to the grid can increase or decrease in response to intermittent PV generation. |
| `ECPNomHz` | uint16 | Hz | ECPNomHz | Setpoint for nominal frequency at the ECP. |
| `ConnPh` | enum16 |  | ConnPh | Identity of connected phase for single phase inverters. A=1 B=2 C=3. |

Value names:

- `VArAct` (enum16): 1 `SWITCH`, 2 `MAINTAIN`
- `ClcTotVA` (enum16): 1 `VECTOR`, 2 `ARITHMETIC`
- `ConnPh` (enum16): 1 `A`, 2 `B`, 3 `C`

## Model 122: Measurements\_Status

Name in records: `status`.

Inverter Controls Extended Measurements and Status

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `PVConn` | bitfield16 |  | PVConn | PV inverter present/available status. |
| `StorConn` | bitfield16 |  | StorConn | Storage inverter present/available status. |
| `ECPConn` | bitfield16 |  | ECPConn | ECP connection status: disconnected=0 connected=1. |
| `ActWh` | acc64 | Wh | ActWh | AC lifetime active (real) energy output. |
| `ActVAh` | acc64 | VAh | ActVAh | AC lifetime apparent energy output. |
| `ActVArhQ1` | acc64 | varh | ActVArhQ1 | AC lifetime reactive energy output in quadrant 1. |
| `ActVArhQ2` | acc64 | varh | ActVArhQ2 | AC lifetime reactive energy output in quadrant 2. |
| `ActVArhQ3` | acc64 | varh | ActVArhQ3 | AC lifetime negative energy output in quadrant 3. |
| `ActVArhQ4` | acc64 | varh | ActVArhQ4 | AC lifetime reactive energy output in quadrant 4. |
| `VArAval` | int16 | var | VArAval | Amount of VARs available without impacting watts output. |
| `WAval` | uint16 | W | WAval | Amount of Watts available. |
| `StSetLimMsk` | bitfield32 |  | StSetLimMsk | Setpoint limit(s) reached. |
| `StActCtl` | bitfield32 |  | StActCtl | Which inverter controls are currently active. |
| `TmSrc` | string |  | TmSrc | Source of time synchronization. |
| `Tms` | uint32 | Secs | Tms | Seconds since 01-01-2000 00:00 UTC |
| `RtSt` | bitfield16 |  | RtSt | Active ride-through status. |
| `Ris` | uint16 | ohms | Ris | Isolation resistance. |

Value names:

- `PVConn` (bitfield16): bit 0 `CONNECTED`, bit 1 `AVAILABLE`, bit 2 `OPERATING`, bit 3 `TEST`
- `StorConn` (bitfield16): bit 0 `CONNECTED`, bit 1 `AVAILABLE`, bit 2 `OPERATING`, bit 3 `TEST`
- `ECPConn` (bitfield16): bit 0 `DISCONNECTED`, bit 1 `CONNECTED`
- `StSetLimMsk` (bitfield32): bit 0 `WMax`, bit 1 `VAMax`, bit 2 `VArAval`, bit 3 `VArMaxQ1`, bit 4 `VArMaxQ2`, bit 5 `VArMaxQ3`, bit 6 `VArMaxQ4`, bit 7 `PFMinQ1`, bit 8 `PFMinQ2`, bit 9 `PFMinQ3`, bit 10 `PFMinQ4`
- `StActCtl` (bitfield32): bit 0 `FixedW`, bit 1 `FixedVAR`, bit 2 `FixedPF`, bit 3 `Volt-VAr`, bit 4 `Freq-Watt-Param`, bit 5 `Freq-Watt-Curve`, bit 6 `Dyn-Reactive-Current`, bit 7 `LVRT`, bit 8 `HVRT`, bit 9 `Watt-PF`, bit 10 `Volt-Watt`, bit 12 `Scheduled`, bit 13 `LFRT`, bit 14 `HFRT`
- `RtSt` (bitfield16): bit 0 `LVRT_ACTIVE`, bit 1 `HVRT_ACTIVE`, bit 2 `LFRT_ACTIVE`, bit 3 `HFRT_ACTIVE`

## Model 123: Immediate Controls

Name in records: `controls`.

Immediate Inverter Controls

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `Conn_WinTms` | uint16 | Secs | Conn\_WinTms | Time window for connect/disconnect. |
| `Conn_RvrtTms` | uint16 | Secs | Conn\_RvrtTms | Timeout period for connect/disconnect. |
| `Conn` | enum16 |  | Conn | Connection control. |
| `WMaxLimPct` | uint16 | % WMax | WMaxLimPct | Set power output to specified level. |
| `WMaxLimPct_WinTms` | uint16 | Secs | WMaxLimPct\_WinTms | Time window for power limit change. |
| `WMaxLimPct_RvrtTms` | uint16 | Secs | WMaxLimPct\_RvrtTms | Timeout period for power limit. |
| `WMaxLimPct_RmpTms` | uint16 | Secs | WMaxLimPct\_RmpTms | Ramp time for moving from current setpoint to new setpoint. |
| `WMaxLim_Ena` | enum16 |  | WMaxLim\_Ena | Throttle enable/disable control. |
| `OutPFSet` | int16 | cos() | OutPFSet | Set power factor to specific value - cosine of angle. |
| `OutPFSet_WinTms` | uint16 | Secs | OutPFSet\_WinTms | Time window for power factor change. |
| `OutPFSet_RvrtTms` | uint16 | Secs | OutPFSet\_RvrtTms | Timeout period for power factor. |
| `OutPFSet_RmpTms` | uint16 | Secs | OutPFSet\_RmpTms | Ramp time for moving from current setpoint to new setpoint. |
| `OutPFSet_Ena` | enum16 |  | OutPFSet\_Ena | Fixed power factor enable/disable control. |
| `VArWMaxPct` | int16 | % WMax | VArWMaxPct | Reactive power in percent of WMax. |
| `VArMaxPct` | int16 | % VArMax | VArMaxPct | Reactive power in percent of VArMax. |
| `VArAvalPct` | int16 | % VArAval | VArAvalPct | Reactive power in percent of VArAval. |
| `VArPct_WinTms` | uint16 | Secs | VArPct\_WinTms | Time window for VAR limit change. |
| `VArPct_RvrtTms` | uint16 | Secs | VArPct\_RvrtTms | Timeout period for VAR limit. |
| `VArPct_RmpTms` | uint16 | Secs | VArPct\_RmpTms | Ramp time for moving from current setpoint to new setpoint. |
| `VArPct_Mod` | enum16 |  | VArPct\_Mod | VAR percent limit mode. |
| `VArPct_Ena` | enum16 |  | VArPct\_Ena | Percent limit VAr enable/disable control. |

Value names:

- `Conn` (enum16): 0 `DISCONNECT`, 1 `CONNECT`
- `WMaxLim_Ena` (enum16): 0 `DISABLED`, 1 `ENABLED`
- `OutPFSet_Ena` (enum16): 0 `DISABLED`, 1 `ENABLED`
- `VArPct_Mod` (enum16): 0 `NONE`, 1 `WMax`, 2 `VArMax`, 3 `VArAval`
- `VArPct_Ena` (enum16): 0 `DISABLED`, 1 `ENABLED`

## Model 124: Storage

Name in records: `storage_basic`.

Basic Storage Controls

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `WChaMax` | uint16 | W | WChaMax | Setpoint for maximum charge. |
| `WChaGra` | uint16 | % WChaMax/sec | WChaGra | Setpoint for maximum charging rate. Default is MaxChaRte. |
| `WDisChaGra` | uint16 | % WChaMax/sec | WDisChaGra | Setpoint for maximum discharge rate. Default is MaxDisChaRte. |
| `StorCtl_Mod` | bitfield16 |  | StorCtl\_Mod | Activate hold/discharge/charge storage control mode. |
| `VAChaMax` | uint16 | VA | VAChaMax | Setpoint for maximum charging VA. |
| `MinRsvPct` | uint16 | % WChaMax | MinRsvPct | Setpoint for minimum reserve for storage as a percentage of the nominal maximum storage. |
| `ChaState` | uint16 | % AhrRtg | ChaState | Currently available energy as a percent of the capacity rating. |
| `StorAval` | uint16 | AH | StorAval | State of charge (ChaState) minus storage reserve (MinRsvPct) times capacity rating (AhrRtg). |
| `InBatV` | uint16 | V | InBatV | Internal battery voltage. |
| `ChaSt` | enum16 |  | ChaSt | Charge status of storage device. |
| `OutWRte` | int16 | % WDisChaMax | OutWRte | Percent of max discharge rate. |
| `InWRte` | int16 | % WChaMax | InWRte | Percent of max charging rate. |
| `InOutWRte_WinTms` | uint16 | Secs | InOutWRte\_WinTms | Time window for charge/discharge rate change. |
| `InOutWRte_RvrtTms` | uint16 | Secs | InOutWRte\_RvrtTms | Timeout period for charge/discharge rate. |
| `InOutWRte_RmpTms` | uint16 | Secs | InOutWRte\_RmpTms | Ramp time for moving from current setpoint to new setpoint. |
| `ChaGriSet` | enum16 |  |  |  |

Value names:

- `StorCtl_Mod` (bitfield16): bit 0 `CHARGE`, bit 1 `DiSCHARGE`
- `ChaSt` (enum16): 1 `OFF`, 2 `EMPTY`, 3 `DISCHARGING`, 4 `CHARGING`, 5 `FULL`, 6 `HOLDING`, 7 `TESTING`
- `ChaGriSet` (enum16): 0 `PV`, 1 `GRID`

## Model 125: Pricing

Name in records: `pricing`.

Pricing Signal

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `ModEna` | bitfield16 |  | ModEna | Is price-based charge/discharge mode active? |
| `SigType` | enum16 |  | SigType | Meaning of the pricing signal. When a Price schedule is used, type must match the schedule range variable description. |
| `Sig` | int16 |  | Sig | Utility/ESP specific pricing signal. Content depends on pricing signal type. When H/M/L type is specified. Low=0; Med=1; High=2. |
| `WinTms` | uint16 | Secs | WinTms | Time window for charge/discharge pricing change. |
| `RvtTms` | uint16 | Secs | RvtTms | Timeout period for charge/discharge pricing change. |
| `RmpTms` | uint16 | Secs | RmpTms | Ramp time for moving from current charge or discharge level to new level. |

Value names:

- `ModEna` (bitfield16): bit 0 `ENABLE`
- `SigType` (enum16): 0 `UNKNOWN`, 1 `ABSOLUTE`, 2 `RELATIVE`, 3 `MULTIPLIER`, 4 `LEVEL`

## Model 126: Static Volt-VAR

Name in records: `volt_var`.

Static Volt-VAR Arrays

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `ActCrv` | uint16 |  | ActCrv | Index of active curve. 0=no active curve. |
| `ModEna` | bitfield16 |  | ModEna | Is Volt-VAR control active. |
| `WinTms` | uint16 | Secs | WinTms | Time window for volt-VAR change. |
| `RvrtTms` | uint16 | Secs | RvrtTms | Timeout period for volt-VAR curve selection. |
| `RmpTms` | uint16 | Secs | RmpTms | The time of the PT1 in seconds (time to accomplish a change of 95%). |
| `NCrv` | uint16 |  | NCrv | Number of curves supported (recommend 4). |
| `NPt` | uint16 |  | NPt | Number of curve points supported (maximum of 20). |
| `curve[*].ActPt` | uint16 |  | ActPt | Number of active points in array. |
| `curve[*].DeptRef` | enum16 |  | DeptRef | Meaning of dependent variable: 1=%WMax 2=%VArMax 3=%VArAval. |
| `curve[*].V1` | uint16 | % VRef | V1 | Point 1 Volts. |
| `curve[*].VAr1` | int16 |  | VAr1 | Point 1 VARs. |
| `curve[*].V2` | uint16 | % VRef | V2 | Point 2 Volts. |
| `curve[*].VAr2` | int16 |  | VAr2 | Point 2 VARs. |
| `curve[*].V3` | uint16 | % VRef | V3 | Point 3 Volts. |
| `curve[*].VAr3` | int16 |  | VAr3 | Point 3 VARs. |
| `curve[*].V4` | uint16 | % VRef | V4 | Point 4 Volts. |
| `curve[*].VAr4` | int16 |  | VAr4 | Point 4 VARs. |
| `curve[*].V5` | uint16 | % VRef | V5 | Point 5 Volts. |
| `curve[*].VAr5` | int16 |  | VAr5 | Point 5 VARs. |
| `curve[*].V6` | uint16 | % VRef | V6 | Point 6 Volts. |
| `curve[*].VAr6` | int16 |  | VAr6 | Point 6 VARs. |
| `curve[*].V7` | uint16 | % VRef | V7 | Point 7 Volts. |
| `curve[*].VAr7` | int16 |  | VAr7 | Point 7 VARs. |
| `curve[*].V8` | uint16 | % VRef | V8 | Point 8 Volts. |
| `curve[*].VAr8` | int16 |  | VAr8 | Point 8 VARs. |
| `curve[*].V9` | uint16 | % VRef | V9 | Point 9 Volts. |
| `curve[*].VAr9` | int16 |  | VAr9 | Point 9 VARs. |
| `curve[*].V10` | uint16 | % VRef | V10 | Point 10 Volts. |
| `curve[*].VAr10` | int16 |  | VAr10 | Point 10 VARs. |
| `curve[*].V11` | uint16 | % VRef | V11 | Point 11 Volts. |
| `curve[*].VAr11` | int16 |  | VAr11 | Point 11 VARs. |
| `curve[*].V12` | uint16 | % VRef | V12 | Point 12 Volts. |
| `curve[*].VAr12` | int16 |  | VAr12 | Point 12 VARs. |
| `curve[*].V13` | uint16 | % VRef | V13 | Point 13 Volts. |
| `curve[*].VAr13` | int16 |  | VAr13 | Point 13 VARs. |
| `curve[*].V14` | uint16 | % VRef | V14 | Point 14 Volts. |
| `curve[*].VAr14` | int16 |  | VAr14 | Point 14 VARs. |
| `curve[*].V15` | uint16 | % VRef | V15 | Point 15 Volts. |
| `curve[*].VAr15` | int16 |  | VAr15 | Point 15 VARs. |
| `curve[*].V16` | uint16 | % VRef | V16 | Point 16 Volts. |
| `curve[*].VAr16` | int16 |  | VAr16 | Point 16 VARs. |
| `curve[*].V17` | uint16 | % VRef | V17 | Point 17 Volts. |
| `curve[*].VAr17` | int16 |  | VAr17 | Point 17 VARs. |
| `curve[*].V18` | uint16 | % VRef | V18 | Point 18 Volts. |
| `curve[*].VAr18` | int16 |  | VAr18 | Point 18 VARs. |
| `curve[*].V19` | uint16 | % VRef | V19 | Point 19 Volts. |
| `curve[*].VAr19` | int16 |  | VAr19 | Point 19 VARs. |
| `curve[*].V20` | uint16 | % VRef | V20 | Point 20 Volts. |
| `curve[*].VAr20` | int16 |  | VAr20 | Point 20 VARs. |
| `curve[*].CrvNam` | string |  | CrvNam | Optional description for curve. (Max 16 chars) |
| `curve[*].RmpTms` | uint16 | Secs | RmpTms | The time of the PT1 in seconds (time to accomplish a change of 95%). |
| `curve[*].RmpDecTmm` | uint16 | % ref\_value/min | RmpDecTmm | The maximum rate at which the VAR value may be reduced in response to changes in the voltage value. %refVal is %WMax %VArMax or %VArAval depending on value of DeptRef. |
| `curve[*].RmpIncTmm` | uint16 | % ref\_value/min | RmpIncTmm | The maximum rate at which the VAR value may be increased in response to changes in the voltage value. %refVal is %WMax %VArMax or %VArAval depending on value of DeptRef. |
| `curve[*].ReadOnly` | enum16 |  | ReadOnly | Boolean flag indicates if curve is read-only or can be modified. |

Value names:

- `ModEna` (bitfield16): bit 0 `ENABLED`
- `curve[*].DeptRef` (enum16): 1 `WMax`, 2 `VArMax`, 3 `VArAval`
- `curve[*].ReadOnly` (enum16): 0 `READWRITE`, 1 `READONLY`

## Model 127: Freq-Watt Param

Name in records: `freq_watt_param`.

Parameterized Frequency-Watt

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `WGra` | uint16 | % PM/Hz | WGra | The slope of the reduction in the maximum allowed watts output as a function of frequency. |
| `HzStr` | int16 | Hz | HzStr | The frequency deviation from nominal frequency (ECPNomHz) at which a snapshot of the instantaneous power output is taken to act as the CAPPED power level (PM) and above which reduction in power output occurs. |
| `HzStop` | int16 | Hz | HzStop | The frequency deviation from nominal frequency (ECPNomHz) at which curtailed power output may return to normal and the cap on the power level value is removed. |
| `HysEna` | bitfield16 |  | HysEna | Enable hysteresis |
| `ModEna` | bitfield16 |  | ModEna | Is Parameterized Frequency-Watt control active. |
| `HzStopWGra` | uint16 | % WMax/min | HzStopWGra | The maximum time-based rate of change at which power output returns to normal after having been capped by an over frequency event. |

Value names:

- `HysEna` (bitfield16): bit 0 `ENABLED`
- `ModEna` (bitfield16): bit 0 `ENABLED`

## Model 128: Dynamic Reactive Current

Name in records: `reactive_current`.

Dynamic Reactive Current

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `ArGraMod` | enum16 |  | ArGraMod | Indicates if gradients trend toward zero at the edges of the deadband or trend toward zero at the center of the deadband. |
| `ArGraSag` | uint16 | %ARtg/%dV | ArGraSag | The gradient used to increase capacitive dynamic current. A value of 0 indicates no additional reactive current support. |
| `ArGraSwell` | uint16 | %ARtg/%dV | ArGraSwell | The gradient used to increase inductive dynamic current. A value of 0 indicates no additional reactive current support. |
| `ModEna` | bitfield16 |  | ModEna | Activate dynamic reactive current model |
| `FilTms` | uint16 | Secs | FilTms | The time window used to calculate the moving average voltage. |
| `DbVMin` | uint16 | % VRef | DbVMin | The lower delta voltage limit for which negative voltage deviations less than this value no dynamic vars are produced. |
| `DbVMax` | uint16 | % VRef | DbVMax | The upper delta voltage limit for which positive voltage deviations less than this value no dynamic current produced. |
| `BlkZnV` | uint16 | % VRef | BlkZnV | Block zone voltage which defines a lower voltage boundary below which no dynamic current is produced. |
| `HysBlkZnV` | uint16 | % VRef | HysBlkZnV | Hysteresis voltage used with BlkZnV. |
| `BlkZnTmms` | uint16 | mSecs | BlkZnTmms | Block zone time the time before which reactive current support remains active regardless of how low the voltage drops. |
| `HoldTmms` | uint16 | mSecs | HoldTmms | Hold time during which reactive current support continues after the average voltage has entered the dead zone. |

Value names:

- `ArGraMod` (enum16): 0 `EDGE`, 1 `CENTER`
- `ModEna` (bitfield16): bit 0 `ENABLED`

## Model 129: LVRTD

Name in records: `lvrt`.

LVRT Must Disconnect

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `ActCrv` | uint16 |  | ActCrv | Index of active curve. 0=no active curve. |
| `ModEna` | bitfield16 |  | ModEna | LVRT control mode. Enable active curve. |
| `WinTms` | uint16 | Secs | WinTms | Time window for LVRT change. |
| `RvrtTms` | uint16 | Secs | RvrtTms | Timeout period for LVRT curve selection. |
| `RmpTms` | uint16 | Secs | RmpTms | Ramp time for moving from current mode to new mode. |
| `NCrv` | uint16 |  | NCrv | Number of curves supported (recommend 4). |
| `NPt` | uint16 |  | NPt | Number of curve points supported (maximum of 20). |
| `curve[*].ActPt` | uint16 |  | ActPt | Number of active points in array. |
| `curve[*].Tms1` | uint16 | Secs | Tms1 | Point 1 must disconnect duration. |
| `curve[*].V1` | uint16 | % VRef | V1 | Point 1 must disconnect voltage. |
| `curve[*].Tms2` | uint16 | Secs | Tms2 | Point 2 must disconnect duration. |
| `curve[*].V2` | uint16 | % VRef | V2 | Point 2 must disconnect voltage. |
| `curve[*].Tms3` | uint16 | Secs | Tms3 | Point 3 must disconnect duration. |
| `curve[*].V3` | uint16 | % VRef | V3 | Point 3 must disconnect voltage. |
| `curve[*].Tms4` | uint16 | Secs | Tms4 | Point 4 must disconnect duration. |
| `curve[*].V4` | uint16 | % VRef | V4 | Point 4 must disconnect voltage. |
| `curve[*].Tms5` | uint16 | Secs | Tms5 | Point 5 must disconnect duration. |
| `curve[*].V5` | uint16 | % VRef | V5 | Point 5 must disconnect voltage. |
| `curve[*].Tms6` | uint16 | Secs | Tms6 | Point 6 must disconnect duration. |
| `curve[*].V6` | uint16 | % VRef | V6 | Point 6 must disconnect voltage. |
| `curve[*].Tms7` | uint16 | Secs | Tms7 | Point 7 must disconnect duration. |
| `curve[*].V7` | uint16 | % VRef | V7 | Point 7 must disconnect voltage. |
| `curve[*].Tms8` | uint16 | Secs | Tms8 | Point 8 must disconnect duration. |
| `curve[*].V8` | uint16 | % VRef | V8 | Point 8 must disconnect voltage. |
| `curve[*].Tms9` | uint16 | Secs | Tms9 | Point 9 must disconnect duration. |
| `curve[*].V9` | uint16 | % VRef | V9 | Point 9 must disconnect voltage. |
| `curve[*].Tms10` | uint16 | Secs | Tms10 | Point 10 must disconnect duration. |
| `curve[*].V10` | uint16 | % VRef | V10 | Point 10 must disconnect voltage. |
| `curve[*].Tms11` | uint16 | Secs | Tms11 | Point 11 must disconnect duration. |
| `curve[*].V11` | uint16 | % VRef | V11 | Point 11 must disconnect voltage. |
| `curve[*].Tms12` | uint16 | Secs | Tms12 | Point 12 must disconnect duration. |
| `curve[*].V12` | uint16 | % VRef | V12 | Point 12 must disconnect voltage. |
| `curve[*].Tms13` | uint16 | Secs | Tms13 | Point 13 must disconnect duration. |
| `curve[*].V13` | uint16 | % VRef | V13 | Point 13 must disconnect voltage. |
| `curve[*].Tms14` | uint16 | Secs | Tms14 | Point 14 must disconnect duration. |
| `curve[*].V14` | uint16 | % VRef | V14 | Point 14 must disconnect voltage. |
| `curve[*].Tms15` | uint16 | Secs | Tms15 | Point 15 must disconnect duration. |
| `curve[*].V15` | uint16 | % VRef | V15 | Point 15 must disconnect voltage. |
| `curve[*].Tms16` | uint16 | Secs | Tms16 | Point 16 must disconnect duration. |
| `curve[*].V16` | uint16 | % VRef | V16 | Point 16 must disconnect voltage. |
| `curve[*].Tms17` | uint16 | Secs | Tms17 | Point 17 must disconnect duration. |
| `curve[*].V17` | uint16 | % VRef | V17 | Point 17 must disconnect voltage. |
| `curve[*].Tms18` | uint16 | Secs | Tms18 | Point 18 must disconnect duration. |
| `curve[*].V18` | uint16 | % VRef | V18 | Point 18 must disconnect voltage. |
| `curve[*].Tms19` | uint16 | Secs | Tms19 | Point 19 must disconnect duration. |
| `curve[*].V19` | uint16 | % VRef | V19 | Point 19 must disconnect voltage. |
| `curve[*].Tms20` | uint16 | Secs | Tms20 | Point 20 must disconnect duration. |
| `curve[*].V20` | uint16 | % VRef | V20 | Point 20 must disconnect voltage. |
| `curve[*].CrvNam` | string |  | CrvNam | Optional description for curve. |
| `curve[*].ReadOnly` | enum16 |  | ReadOnly | Curve is read-only or can be modified. |

Value names:

- `ModEna` (bitfield16): bit 0 `ENABLED`
- `curve[*].ReadOnly` (enum16): 0 `READWRITE`, 1 `READONLY`

## Model 130: HVRTD

Name in records: `hvrt`.

HVRT Must Disconnect

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `ActCrv` | uint16 |  | ActCrv | Index of active curve. 0=no active curve. |
| `ModEna` | bitfield16 |  | ModEna | HVRT control mode. Enable active curve. |
| `WinTms` | uint16 | Secs | WinTms | Time window for HVRT change. |
| `RvrtTms` | uint16 | Secs | RvrtTms | Timeout period for HVRT curve selection. |
| `RmpTms` | uint16 | Secs | RmpTms | Ramp time for moving from current mode to new mode. |
| `NCrv` | uint16 |  | NCrv | Number of curves supported (recommend 4). |
| `NPt` | uint16 |  | NPt | Number of curve points supported (maximum of 20). |
| `curve[*].ActPt` | uint16 |  | ActPt | Number of active points in array. |
| `curve[*].Tms1` | uint16 | Secs | Tms1 | Point 1 must disconnect duration. |
| `curve[*].V1` | uint16 | % VRef | V1 | Point 1 must disconnect voltage. |
| `curve[*].Tms2` | uint16 | Secs | Tms2 | Point 2 must disconnect duration. |
| `curve[*].V2` | uint16 | % VRef | V2 | Point 2 must disconnect voltage. |
| `curve[*].Tms3` | uint16 | Secs | Tms3 | Point 3 must disconnect duration. |
| `curve[*].V3` | uint16 | % VRef | V3 | Point 3 must disconnect voltage. |
| `curve[*].Tms4` | uint16 | Secs | Tms4 | Point 4 must disconnect duration. |
| `curve[*].V4` | uint16 | % VRef | V4 | Point 4 must disconnect voltage. |
| `curve[*].Tms5` | uint16 | Secs | Tms5 | Point 5 must disconnect duration. |
| `curve[*].V5` | uint16 | % VRef | V5 | Point 5 must disconnect voltage. |
| `curve[*].Tms6` | uint16 | Secs | Tms6 | Point 6 must disconnect duration. |
| `curve[*].V6` | uint16 | % VRef | V6 | Point 6 must disconnect voltage. |
| `curve[*].Tms7` | uint16 | Secs | Tms7 | Point 7 must disconnect duration. |
| `curve[*].V7` | uint16 | % VRef | V7 | Point 7 must disconnect voltage. |
| `curve[*].Tms8` | uint16 | Secs | Tms8 | Point 8 must disconnect duration. |
| `curve[*].V8` | uint16 | % VRef | V8 | Point 8 must disconnect voltage. |
| `curve[*].Tms9` | uint16 | Secs | Tms9 | Point 9 must disconnect duration. |
| `curve[*].V9` | uint16 | % VRef | V9 | Point 9 must disconnect voltage. |
| `curve[*].Tms10` | uint16 | Secs | Tms10 | Point 10 must disconnect duration. |
| `curve[*].V10` | uint16 | % VRef | V10 | Point 10 must disconnect voltage. |
| `curve[*].Tms11` | uint16 | Secs | Tms11 | Point 11 must disconnect duration. |
| `curve[*].V11` | uint16 | % VRef | V11 | Point 11 must disconnect voltage. |
| `curve[*].Tms12` | uint16 | Secs | Tms12 | Point 12 must disconnect duration. |
| `curve[*].V12` | uint16 | % VRef | V12 | Point 12 must disconnect voltage. |
| `curve[*].Tms13` | uint16 | Secs | Tms13 | Point 13 must disconnect duration. |
| `curve[*].V13` | uint16 | % VRef | V13 | Point 13 must disconnect voltage. |
| `curve[*].Tms14` | uint16 | Secs | Tms14 | Point 14 must disconnect duration. |
| `curve[*].V14` | uint16 | % VRef | V14 | Point 14 must disconnect voltage. |
| `curve[*].Tms15` | uint16 | Secs | Tms15 | Point 15 must disconnect duration. |
| `curve[*].V15` | uint16 | % VRef | V15 | Point 15 must disconnect voltage. |
| `curve[*].Tms16` | uint16 | Secs | Tms16 | Point 16 must disconnect duration. |
| `curve[*].V16` | uint16 | % VRef | V16 | Point 16 must disconnect voltage. |
| `curve[*].Tms17` | uint16 | Secs | Tms17 | Point 17 must disconnect duration. |
| `curve[*].V17` | uint16 | % VRef | V17 | Point 17 must disconnect voltage. |
| `curve[*].Tms18` | uint16 | Secs | Tms18 | Point 18 must disconnect duration. |
| `curve[*].V18` | uint16 | % VRef | V18 | Point 18 must disconnect voltage. |
| `curve[*].Tms19` | uint16 | Secs | Tms19 | Point 19 must disconnect duration. |
| `curve[*].V19` | uint16 | % VRef | V19 | Point 19 must disconnect voltage. |
| `curve[*].Tms20` | uint16 | Secs | Tms20 | Point 20 must disconnect duration. |
| `curve[*].V20` | uint16 | % VRef | V20 | Point 20 must disconnect voltage. |
| `curve[*].CrvNam` | string |  | CrvNam | Optional description for curve. |
| `curve[*].ReadOnly` | enum16 |  | ReadOnly | Curve is read-only or can be modified. |

Value names:

- `ModEna` (bitfield16): bit 0 `ENABLED`
- `curve[*].ReadOnly` (enum16): 0 `READWRITE`, 1 `READONLY`

## Model 131: Watt-PF

Name in records: `watt_pf`.

Watt-Power Factor

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `ActCrv` | uint16 |  | ActCrv | Index of active curve. 0=no active curve. |
| `ModEna` | bitfield16 |  | ModEna | Is watt-PF mode active. |
| `WinTms` | uint16 | Secs | WinTms | Time window for watt-PF change. |
| `RvrtTms` | uint16 | Secs | RvrtTms | Timeout period for watt-PF curve selection. |
| `RmpTms` | uint16 | Secs | RmpTms | Ramp time for moving from current mode to new mode. |
| `NCrv` | uint16 |  | NCrv | Number of curves supported (recommend 4). |
| `NPt` | uint16 |  | NPt | Max number of points in array. |
| `curve[*].ActPt` | uint16 |  | ActPt | Number of active points in array. |
| `curve[*].W1` | int16 | % WMax | W1 | Point 1 Watts. |
| `curve[*].PF1` | int16 | cos() | PF1 | Point 1 PF in EEI notation. |
| `curve[*].W2` | int16 | % WMax | W2 | Point 2 Watts. |
| `curve[*].PF2` | int16 | cos() | PF2 | Point 2 PF in EEI notation. |
| `curve[*].W3` | int16 | % WMax | W3 | Point 3 Watts. |
| `curve[*].PF3` | int16 | cos() | PF3 | Point 3 PF in EEI notation. |
| `curve[*].W4` | int16 | % WMax | W4 | Point 4 Watts. |
| `curve[*].PF4` | int16 | cos() | PF4 | Point 4 PF in EEI notation. |
| `curve[*].W5` | int16 | % WMax | W5 | Point 5 Watts. |
| `curve[*].PF5` | int16 | cos() | PF5 | Point 5 PF in EEI notation. |
| `curve[*].W6` | int16 | % WMax | W6 | Point 6 Watts. |
| `curve[*].PF6` | int16 | cos() | PF6 | Point 6 PF in EEI notation. |
| `curve[*].W7` | int16 | % WMax | W7 | Point 7 Watts. |
| `curve[*].PF7` | int16 | cos() | PF7 | Point 7 PF in EEI notation. |
| `curve[*].W8` | int16 | % WMax | W8 | Point 8 Watts. |
| `curve[*].PF8` | int16 | cos() | PF8 | Point 8 PF in EEI notation. |
| `curve[*].W9` | int16 | % WMax | W9 | Point 9 Watts. |
| `curve[*].PF9` | int16 | cos() | PF9 | Point 9 PF in EEI notation. |
| `curve[*].W10` | int16 | % WMax | W10 | Point 10 Watts. |
| `curve[*].PF10` | int16 | cos() | PF10 | Point 10 PF in EEI notation. |
| `curve[*].W11` | int16 | % WMax | W11 | Point 11 Watts. |
| `curve[*].PF11` | int16 | cos() | PF11 | Point 11 PF in EEI notation. |
| `curve[*].W12` | int16 | % WMax | W12 | Point 12 Watts. |
| `curve[*].PF12` | int16 | cos() | PF12 | Point 12 PF in EEI notation. |
| `curve[*].W13` | int16 | % WMax | W13 | Point 13 Watts. |
| `curve[*].PF13` | int16 | cos() | PF13 | Point 13 PF in EEI notation. |
| `curve[*].W14` | int16 | % WMax | W14 | Point 14 Watts. |
| `curve[*].PF14` | int16 | cos() | PF14 | Point 14 PF in EEI notation. |
| `curve[*].W15` | int16 | % WMax | W15 | Point 15 Watts. |
| `curve[*].PF15` | int16 | cos() | PF15 | Point 15 PF in EEI notation. |
| `curve[*].W16` | int16 | % WMax | W16 | Point 16 Watts. |
| `curve[*].PF16` | int16 | cos() | PF16 | Point 16 PF in EEI notation. |
| `curve[*].W17` | int16 | % WMax | W17 | Point 17 Watts. |
| `curve[*].PF17` | int16 | cos() | PF17 | Point 17 PF in EEI notation. |
| `curve[*].W18` | int16 | % WMax | W18 | Point 18 Watts. |
| `curve[*].PF18` | int16 | cos() | PF18 | Point 18 PF in EEI notation. |
| `curve[*].W19` | int16 | % WMax | W19 | Point 19 Watts. |
| `curve[*].PF19` | int16 | cos() | PF19 | Point 19 PF in EEI notation. |
| `curve[*].W20` | int16 | % WMax | W20 | Point 20 Watts. |
| `curve[*].PF20` | int16 | cos() | PF20 | Point 20 PF in EEI notation. |
| `curve[*].CrvNam` | string |  | CrvNam | Optional description for curve. |
| `curve[*].RmpPT1Tms` | uint16 | Secs | RmpPT1Tms | The time of the PT1 in seconds (time to accomplish a change of 95%). |
| `curve[*].RmpDecTmm` | uint16 | % PF/min | RmpDecTmm | The maximum rate at which the power factor may be reduced in response to changes in the power value. |
| `curve[*].RmpIncTmm` | uint16 | % PF/min | RmpIncTmm | The maximum rate at which the power factor may be increased in response to changes in the power value. |
| `curve[*].ReadOnly` | enum16 |  | ReadOnly | Curve is read-only or can be modified. |

Value names:

- `ModEna` (bitfield16): bit 0 `ENABLED`
- `curve[*].ReadOnly` (enum16): 0 `READWRITE`, 1 `READONLY`

## Model 132: Volt-Watt

Name in records: `volt_watt`.

Volt-Watt

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `ActCrv` | uint16 |  | ActCrv | Index of active curve. 0=no active curve. |
| `ModEna` | bitfield16 |  | ModEna | Is Volt-Watt control active. |
| `WinTms` | uint16 | Secs | WinTms | Time window for volt-watt change. |
| `RvrtTms` | uint16 | Secs | RvrtTms | Timeout period for volt-watt curve selection. |
| `RmpTms` | uint16 | Secs | RmpTms | Ramp time for moving from current mode to new mode. |
| `NCrv` | uint16 |  | NCrv | Number of curves supported (recommend min. 4). |
| `NPt` | uint16 |  | NPt | Number of points in array (maximum 20). |
| `curve[*].ActPt` | uint16 |  | ActPt | Number of active points in array. |
| `curve[*].DeptRef` | enum16 |  | DeptRef | Defines the meaning of the Watts DeptRef. 1=% WMax 2=% WAvail |
| `curve[*].V1` | uint16 | % VRef | V1 | Point 1 Volts. |
| `curve[*].W1` | int16 | % VRef | W1 | Point 1 Watts. |
| `curve[*].V2` | uint16 | % VRef | V2 | Point 2 Volts. |
| `curve[*].W2` | int16 | % VRef | W2 | Point 2 Watts. |
| `curve[*].V3` | uint16 | % VRef | V3 | Point 3 Volts. |
| `curve[*].W3` | int16 | % VRef | W3 | Point 3 Watts. |
| `curve[*].V4` | uint16 | % VRef | V4 | Point 4 Volts. |
| `curve[*].W4` | int16 | % VRef | W4 | Point 4 Watts. |
| `curve[*].V5` | uint16 | % VRef | V5 | Point 5 Volts. |
| `curve[*].W5` | int16 | % VRef | W5 | Point 5 Watts. |
| `curve[*].V6` | uint16 | % VRef | V6 | Point 6 Volts. |
| `curve[*].W6` | int16 | % VRef | W6 | Point 6 Watts. |
| `curve[*].V7` | uint16 | % VRef | V7 | Point 7 Volts. |
| `curve[*].W7` | int16 | % VRef | W7 | Point 7 Watts. |
| `curve[*].V8` | uint16 | % VRef | V8 | Point 8 Volts. |
| `curve[*].W8` | int16 | % VRef | W8 | Point 8 Watts. |
| `curve[*].V9` | uint16 | % VRef | V9 | Point 9 Volts. |
| `curve[*].W9` | int16 | % VRef | W9 | Point 9 Watts. |
| `curve[*].V10` | uint16 | % VRef | V10 | Point 10 Volts. |
| `curve[*].W10` | int16 | % VRef | W10 | Point 10 Watts. |
| `curve[*].V11` | uint16 | % VRef | V11 | Point 11 Volts. |
| `curve[*].W11` | int16 | % VRef | W11 | Point 11 Watts. |
| `curve[*].V12` | uint16 | % VRef | V12 | Point 12 Volts. |
| `curve[*].W12` | int16 | % VRef | W12 | Point 12 Watts. |
| `curve[*].V13` | uint16 | % VRef | V13 | Point 13 Volts. |
| `curve[*].W13` | int16 | % VRef | W13 | Point 13 Watts. |
| `curve[*].V14` | uint16 | % VRef | V14 | Point 14 Volts. |
| `curve[*].W14` | int16 | % VRef | W14 | Point 14 Watts. |
| `curve[*].V15` | uint16 | % VRef | V15 | Point 15 Volts. |
| `curve[*].W15` | int16 | % VRef | W15 | Point 15 Watts. |
| `curve[*].V16` | uint16 | % VRef | V16 | Point 16 Volts. |
| `curve[*].W16` | int16 | % VRef | W16 | Point 16 Watts. |
| `curve[*].V17` | uint16 | % VRef | V17 | Point 17 Volts. |
| `curve[*].W17` | int16 | % VRef | W17 | Point 17 Watts. |
| `curve[*].V18` | uint16 | % VRef | V18 | Point 18 Volts. |
| `curve[*].W18` | int16 | % VRef | W18 | Point 18 Watts. |
| `curve[*].V19` | uint16 | % VRef | V19 | Point 19 Volts. |
| `curve[*].W19` | int16 | % VRef | W19 | Point 19 Watts. |
| `curve[*].V20` | uint16 | % VRef | V20 | Point 20 Volts. |
| `curve[*].W20` | int16 | % VRef | W20 | Point 20 Watts. |
| `curve[*].CrvNam` | string |  | CrvNam | Optional description for curve. |
| `curve[*].RmpPt1Tms` | uint16 | Secs | RmpPt1Tms | The time of the PT1 in seconds (time to accomplish a change of 95%). |
| `curve[*].RmpDecTmm` | uint16 | % WMax/min | RmpDecTmm | The maximum rate at which the watt value may be reduced in response to changes in the voltage value. |
| `curve[*].RmpIncTmm` | uint16 | % WMax/min | RmpIncTmm | The maximum rate at which the watt value may be increased in response to changes in the voltage value. |
| `curve[*].ReadOnly` | enum16 |  | ReadOnly | Curve is read-only or can be modified. |

Value names:

- `ModEna` (bitfield16): bit 0 `ENABLED`
- `curve[*].DeptRef` (enum16): 1 `%WMax`, 2 `%WAval`
- `curve[*].ReadOnly` (enum16): 0 `READWRITE`, 1 `READONLY`

## Model 133: Basic Scheduling

Name in records: `schedule`.

Basic Scheduling

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `ActSchd` | bitfield32 |  | ActSchd | Bitfield of active schedules |
| `ModEna` | bitfield16 |  | ModEna | Is basic scheduling active. |
| `NSchd` | uint16 |  | NSchd | Number of schedules supported (recommend min. 4, max 32) |
| `NPts` | uint16 |  | NPts | Number of schedule entries supported (maximum of 10). |
| `repeating[*].ActPts` | uint16 |  | ActPts | Number of active entries in schedule. |
| `repeating[*].StrTms` | uint32 | Secs | StrTms | Schedule start in seconds since 2000 JAN 01 00:00:00 UTC. |
| `repeating[*].RepPer` | uint16 |  | RepPer | The repetition count for time-based schedules (0=repeat forever) |
| `repeating[*].IntvTyp` | enum16 |  | SchdTyp | The repetition frequency for time-based schedules: no repeat=0 |
| `repeating[*].XTyp` | enum16 |  | XTyp | The meaning of the X-values in the array. |
| `repeating[*].YTyp` | enum16 |  | YTyp | The meaning of the Y-values in the array. |
| `repeating[*].X1` | int32 |  | X1 | Entry 1 range. |
| `repeating[*].Y1` | int32 |  | Y1 | Entry 1 target. |
| `repeating[*].X2` | int32 |  | X2 | Entry 2 range. |
| `repeating[*].Y2` | int32 |  | Y2 | Entry 2 target. |
| `repeating[*].X3` | int32 |  | X3 | Entry 3 range. |
| `repeating[*].Y3` | int32 |  | Y3 | Entry 3 target. |
| `repeating[*].X4` | int32 |  | X4 | Entry 4 range. |
| `repeating[*].Y4` | int32 |  | Y4 | Entry 4 target. |
| `repeating[*].X5` | int32 |  | X5 | Entry 15range. |
| `repeating[*].Y5` | int32 |  | Y5 | Entry 5 target. |
| `repeating[*].X6` | int32 |  | X6 | Entry 6 range. |
| `repeating[*].Y6` | int32 |  | Y6 | Entry 6 target. |
| `repeating[*].X7` | int32 |  | X7 | Entry 7 range. |
| `repeating[*].Y7` | int32 |  | Y7 | Entry 7 target. |
| `repeating[*].X8` | int32 |  | X8 | Entry 8 range. |
| `repeating[*].Y8` | int32 |  | Y8 | Entry 8 target. |
| `repeating[*].X9` | int32 |  | X9 | Entry 9 range. |
| `repeating[*].Y9` | int32 |  | Y9 | Entry 9 target. |
| `repeating[*].X10` | int32 |  | X10 | Entry 10 range. |
| `repeating[*].Y10` | int32 |  | Y10 | Entry 10 target. |
| `repeating[*].Nam` | string |  | Nam | Optional description for schedule. |
| `repeating[*].WinTms` | uint16 | Secs | WinTms | Time window for schedule entry change. |
| `repeating[*].RmpTms` | uint16 | Secs | RmpTms | Ramp time for moving from current target to new target. |
| `repeating[*].ActIndx` | uint16 |  | ActIndx | Index of active entry in the active schedule. |

Value names:

- `ActSchd` (bitfield32): bit 0 `SCHED1`, bit 1 `SCHED2`, bit 2 `SCHED3`, bit 3 `SCHED4`, bit 4 `SCHED5`, bit 5 `SCHED6`, bit 6 `SCHED7`, bit 7 `SCHED8`, bit 8 `SCHED9`, bit 9 `SCHED10`, bit 10 `SCHED12`, bit 11 `SCHED13`, bit 12 `SCHED14`, bit 13 `SCHED15`, bit 14 `SCHED16`, bit 15 `SCHED17`, bit 16 `SCHED18`, bit 17 `SCHED19`, bit 18 `SCHED20`, bit 19 `SCHED21`, bit 21 `SCHED22`, bit 22 `SCHED23`, bit 23 `SCHED24`, bit 24 `SCHED25`, bit 25 `SCHED26`, bit 26 `SCHED27`, bit 27 `SCHED28`, bit 28 `SCHED29`, bit 29 `SCHED30`, bit 30 `SCHED31`, bit 31 `SCHED32`
- `ModEna` (bitfield16): bit 0 `ENABLED`
- `repeating[*].IntvTyp` (enum16): 0 `ONETIME`, 1 `DAILY`, 2 `WEEKLY`, 3 `MONTHLY`, 4 `WEEKDAY`, 5 `HOLIDAY`, 6 `WEEKEND`, 7 `YEARLY`
- `repeating[*].XTyp` (enum16): 0 `UNSET`, 1 `TIME`, 2 `TEMP`, 3 `PRICE`, 99 `OTHER`
- `repeating[*].YTyp` (enum16): 0 `UNSET`, 1 `WMax`, 2 `RSRVD2`, 3 `PF`, 4 `RSRVD4`, 5 `WATT_PRICE`, 6 `VAR_PRICE`, 7 `RSRVD7`, 8 `VOLT_VAR_ARRAY`, 9 `WChaGra`, 10 `WDisChaGra`, 11 `VArAval`, 12 `Schedule`, 99 `OTHER`

## Model 134: Freq-Watt Crv

Name in records: `freq_watt`.

Curve-Based Frequency-Watt

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `ActCrv` | uint16 |  | ActCrv | Index of active curve. 0=no active curve. |
| `ModEna` | bitfield16 |  | ModEna | Is curve-based Frequency-Watt control active. |
| `WinTms` | uint16 | Secs | WinTms | Time window for freq-watt change. |
| `RvrtTms` | uint16 | Secs | RvrtTms | Timeout period for freq-watt curve selection. |
| `RmpTms` | uint16 | Secs | RmpTms | Ramp time for moving from current mode to new mode. |
| `NCrv` | uint16 |  | NCrv | Number of curves supported (recommend min. 4). |
| `NPt` | uint16 |  | NPt | Number of curve points supported (maximum of 20). |
| `curve[*].ActPt` | uint16 |  | ActPt | Number of active points in array. |
| `curve[*].Hz1` | uint16 | Hz | Hz1 | Point 1 Hertz. |
| `curve[*].W1` | int16 | % WRef | W1 | Point 1 Watts. |
| `curve[*].Hz2` | uint16 | Hz | Hz2 | Point 2 Hertz. |
| `curve[*].W2` | int16 | % WRef | W2 | Point 2 Watts. |
| `curve[*].Hz3` | uint16 | Hz | Hz3 | Point 3 Hertz. |
| `curve[*].W3` | int16 | % WRef | W3 | Point 3 Watts. |
| `curve[*].Hz4` | uint16 | Hz | Hz4 | Point 4 Hertz. |
| `curve[*].W4` | int16 | % WRef | W4 | Point 4 Watts. |
| `curve[*].Hz5` | uint16 | Hz | Hz5 | Point 5 Hertz. |
| `curve[*].W5` | int16 | % WRef | W5 | Point 5 Watts. |
| `curve[*].Hz6` | uint16 | Hz | Hz6 | Point 6 Hertz. |
| `curve[*].W6` | int16 | % WRef | W6 | Point 6 Watts. |
| `curve[*].Hz7` | uint16 | Hz | Hz7 | Point 7 Hertz. |
| `curve[*].W7` | int16 | % WRef | W7 | Point 7 Watts. |
| `curve[*].Hz8` | uint16 | Hz | Hz8 | Point 8 Hertz. |
| `curve[*].W8` | int16 | % WRef | W8 | Point 8 Watts. |
| `curve[*].Hz9` | uint16 | Hz | Hz9 | Point 9 Hertz. |
| `curve[*].W9` | int16 | % WRef | W9 | Point 9 Watts. |
| `curve[*].Hz10` | uint16 | Hz | Hz10 | Point 10 Hertz. |
| `curve[*].W10` | int16 | % WRef | W10 | Point 10 Watts. |
| `curve[*].Hz11` | uint16 | Hz | Hz11 | Point 11 Hertz. |
| `curve[*].W11` | int16 | % WRef | W11 | Point 11 Watts. |
| `curve[*].Hz12` | uint16 | Hz | Hz12 | Point 12 Hertz. |
| `curve[*].W12` | int16 | % WRef | W12 | Point 12 Watts. |
| `curve[*].Hz13` | uint16 | Hz | Hz13 | Point 13 Hertz. |
| `curve[*].W13` | int16 | % WRef | W13 | Point 13 Watts. |
| `curve[*].Hz14` | uint16 | Hz | Hz14 | Point 14 Hertz. |
| `curve[*].W14` | int16 | % WRef | W14 | Point 14 Watts. |
| `curve[*].Hz15` | uint16 | Hz | Hz15 | Point 15 Hertz. |
| `curve[*].W15` | int16 | % WRef | W15 | Point 15 Watts. |
| `curve[*].Hz16` | uint16 | Hz | Hz16 | Point 16 Hertz. |
| `curve[*].W16` | int16 | % WRef | W16 | Point 16 Watts. |
| `curve[*].Hz17` | uint16 | Hz | Hz17 | Point 17 Hertz. |
| `curve[*].W17` | int16 | % WRef | W17 | Point 17 Watts. |
| `curve[*].Hz18` | uint16 | Hz | Hz18 | Point 18 Hertz. |
| `curve[*].W18` | int16 | % WRef | W18 | Point 18 Watts. |
| `curve[*].Hz19` | uint16 | Hz | Hz19 | Point 19 Hertz. |
| `curve[*].W19` | int16 | % WRef | W19 | Point 19 Watts. |
| `curve[*].Hz20` | uint16 | Hz | Hz20 | Point 20 Hertz. |
| `curve[*].W20` | int16 | % WRef | W20 | Point 20 Watts. |
| `curve[*].CrvNam` | string |  | CrvNam | Optional description for curve. (Max 16 chars) |
| `curve[*].RmpPT1Tms` | uint16 | Secs | RmpPT1Tms | The time of the PT1 in seconds (time to accomplish a change of 95%). |
| `curve[*].RmpDecTmm` | uint16 | % WMax/min | RmpDecTmm | The maximum rate at which the power value may be reduced in response to changes in the frequency value. |
| `curve[*].RmpIncTmm` | uint16 | % WMax/min | RmpIncTmm | The maximum rate at which the power value may be increased in response to changes in the frequency value. |
| `curve[*].RmpRsUp` | uint16 | % WMax/min | RmpRsUp | The maximum rate at which the power may be increased after releasing the frozen value of snap shot function. |
| `curve[*].SnptW` | bitfield16 |  | SnptW | 1=enable snapshot/capture mode |
| `curve[*].WRef` | uint16 | W | WRef | Reference active power (default = WMax). |
| `curve[*].WRefStrHz` | uint16 | Hz | WRefStrHz | Frequency deviation from nominal frequency at the time of the snapshot to start constraining power output. |
| `curve[*].WRefStopHz` | uint16 | Hz | WRefStopHz | Frequency deviation from nominal frequency at which to release the power output. |
| `curve[*].ReadOnly` | enum16 |  | ReadOnly | Curve is read-only or can be modified. |

Value names:

- `ModEna` (bitfield16): bit 0 `ENABLED`
- `curve[*].ReadOnly` (enum16): 0 `READWRITE`, 1 `READONLY`

## Model 135: LFRT

Name in records: `lfrt`.

Low Frequency Ride-through

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `ActCrv` | uint16 |  | ActCrv | Index of active curve. 0=no active curve. |
| `ModEna` | bitfield16 |  | ModEna | LHzRT control mode. Enable active curve. |
| `WinTms` | uint16 | Secs | WinTms | Time window for LFRT change. |
| `RvrtTms` | uint16 | Secs | RvrtTms | Timeout period for LFRT curve selection. |
| `RmpTms` | uint16 | Secs | RmpTms | Ramp time for moving from current mode to new mode. |
| `NCrv` | uint16 |  | NCrv | Number of curves supported (recommend 4). |
| `NPt` | uint16 |  | NPt | Number of curve points supported (maximum of 20). |
| `curve[*].ActPt` | uint16 |  | ActPt | Number of active points in array. |
| `curve[*].Tms1` | uint16 | Secs | Tms1 | Point 1 must disconnect duration. |
| `curve[*].Hz1` | uint16 | Hz | Hz1 | Point 1 must disconnect frequency. |
| `curve[*].Tms2` | uint16 | Secs | Tms2 | Point 2 must disconnect duration. |
| `curve[*].Hz2` | uint16 | Hz | Hz2 | Point 2 must disconnect frequency. |
| `curve[*].Tms3` | uint16 | Secs | Tms3 | Point 3 must disconnect duration. |
| `curve[*].Hz3` | uint16 | Hz | Hz3 | Point 3 must disconnect frequency. |
| `curve[*].Tms4` | uint16 | Secs | Tms4 | Point 4 must disconnect duration. |
| `curve[*].Hz4` | uint16 | Hz | Hz4 | Point 4 must disconnect frequency. |
| `curve[*].Tms5` | uint16 | Secs | Tms5 | Point 5 must disconnect duration. |
| `curve[*].Hz5` | uint16 | Hz | Hz5 | Point 5 must disconnect frequency. |
| `curve[*].Tms6` | uint16 | Secs | Tms6 | Point 6 must disconnect duration. |
| `curve[*].Hz6` | uint16 | Hz | Hz6 | Point 6 must disconnect frequency. |
| `curve[*].Tms7` | uint16 | Secs | Tms7 | Point 7 must disconnect duration. |
| `curve[*].Hz7` | uint16 | Hz | Hz7 | Point 7 must disconnect frequency. |
| `curve[*].Tms8` | uint16 | Secs | Tms8 | Point 8 must disconnect duration. |
| `curve[*].Hz8` | uint16 | Hz | Hz8 | Point 8 must disconnect frequency. |
| `curve[*].Tms9` | uint16 | Secs | Tms9 | Point 9 must disconnect duration. |
| `curve[*].Hz9` | uint16 | Hz | Hz9 | Point 9 must disconnect frequency. |
| `curve[*].Tms10` | uint16 | Secs | Tms10 | Point 10 must disconnect duration. |
| `curve[*].Hz10` | uint16 | Hz | Hz10 | Point 10 must disconnect frequency. |
| `curve[*].Tms11` | uint16 | Secs | Tms11 | Point 11 must disconnect duration. |
| `curve[*].Hz11` | uint16 | Hz | Hz11 | Point 11 must disconnect frequency. |
| `curve[*].Tms12` | uint16 | Secs | Tms12 | Point 12 must disconnect duration. |
| `curve[*].Hz12` | uint16 | Hz | Hz12 | Point 12 must disconnect frequency. |
| `curve[*].Tms13` | uint16 | Secs | Tms13 | Point 13 must disconnect duration. |
| `curve[*].Hz13` | uint16 | Hz | Hz13 | Point 13 must disconnect frequency. |
| `curve[*].Tms14` | uint16 | Secs | Tms14 | Point 14 must disconnect duration. |
| `curve[*].Hz14` | uint16 | Hz | Hz14 | Point 14 must disconnect frequency. |
| `curve[*].Tms15` | uint16 | Secs | Tms15 | Point 15 must disconnect duration. |
| `curve[*].Hz15` | uint16 | Hz | Hz15 | Point 15 must disconnect frequency. |
| `curve[*].Tms16` | uint16 | Secs | Tms16 | Point 16 must disconnect duration. |
| `curve[*].Hz16` | uint16 | Hz | Hz16 | Point 16 must disconnect frequency. |
| `curve[*].Tms17` | uint16 | Secs | Tms17 | Point 17 must disconnect duration. |
| `curve[*].Hz17` | uint16 | Hz | Hz17 | Point 17 must disconnect frequency. |
| `curve[*].Tms18` | uint16 | Secs | Tms18 | Point 18 must disconnect duration. |
| `curve[*].Hz18` | uint16 | Hz | Hz18 | Point 18 must disconnect frequency. |
| `curve[*].Tms19` | uint16 | Secs | Tms19 | Point 19 must disconnect duration. |
| `curve[*].Hz19` | uint16 | Hz | Hz19 | Point 19 must disconnect frequency. |
| `curve[*].Tms20` | uint16 | Secs | Tms20 | Point 20 must disconnect duration. |
| `curve[*].Hz20` | uint16 | Hz | Hz20 | Point 20 must disconnect frequency. |
| `curve[*].CrvNam` | string |  | CrvNam | Optional description for curve. |
| `curve[*].ReadOnly` | enum16 |  | ReadOnly | Curve is read-only or can be modified. |

Value names:

- `ModEna` (bitfield16): bit 0 `ENABLED`
- `curve[*].ReadOnly` (enum16): 0 `READWRITE`, 1 `READONLY`

## Model 136: HFRT

Name in records: `hfrt`.

High Frequency Ride-through

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `ActCrv` | uint16 |  | ActCrv | Index of active curve. 0=no active curve. |
| `ModEna` | bitfield16 |  | ModEna | HFRT control mode. Enable active curve. |
| `WinTms` | uint16 | Secs | WinTms | Time window for HFRT change. |
| `RvrtTms` | uint16 | Secs | RvrtTms | Timeout period for HFRT curve selection. |
| `RmpTms` | uint16 | Secs | RmpTms | Ramp time for moving from current mode to new mode. |
| `NCrv` | uint16 |  | NCrv | Number of curves supported (recommend 4). |
| `NPt` | uint16 |  | NPt | Number of curve points supported (maximum of 20). |
| `curve[*].ActPt` | uint16 |  | ActPt | Number of active points in array. |
| `curve[*].Tms1` | uint16 | Secs | Tms1 | Point 1 must disconnect duration. |
| `curve[*].Hz1` | uint16 | Hz | Hz1 | Point 1 must disconnect frequency. |
| `curve[*].Tms2` | uint16 | Secs | Tms2 | Point 2 must disconnect duration. |
| `curve[*].Hz2` | uint16 | Hz | Hz2 | Point 2 must disconnect frequency. |
| `curve[*].Tms3` | uint16 | Secs | Tms3 | Point 3 must disconnect duration. |
| `curve[*].Hz3` | uint16 | Hz | Hz3 | Point 3 must disconnect frequency. |
| `curve[*].Tms4` | uint16 | Secs | Tms4 | Point 4 must disconnect duration. |
| `curve[*].Hz4` | uint16 | Hz | Hz4 | Point 4 must disconnect frequency. |
| `curve[*].Tms5` | uint16 | Secs | Tms5 | Point 5 must disconnect duration. |
| `curve[*].Hz5` | uint16 | Hz | Hz5 | Point 5 must disconnect frequency. |
| `curve[*].Tms6` | uint16 | Secs | Tms6 | Point 6 must disconnect duration. |
| `curve[*].Hz6` | uint16 | Hz | Hz6 | Point 6 must disconnect frequency. |
| `curve[*].Tms7` | uint16 | Secs | Tms7 | Point 7 must disconnect duration. |
| `curve[*].Hz7` | uint16 | Hz | Hz7 | Point 7 must disconnect frequency. |
| `curve[*].Tms8` | uint16 | Secs | Tms8 | Point 8 must disconnect duration. |
| `curve[*].Hz8` | uint16 | Hz | Hz8 | Point 8 must disconnect frequency. |
| `curve[*].Tms9` | uint16 | Secs | Tms9 | Point 9 must disconnect duration. |
| `curve[*].Hz9` | uint16 | Hz | Hz9 | Point 9 must disconnect frequency. |
| `curve[*].Tms10` | uint16 | Secs | Tms10 | Point 10 must disconnect duration. |
| `curve[*].Hz10` | uint16 | Hz | Hz10 | Point 10 must disconnect frequency. |
| `curve[*].Tms11` | uint16 | Secs | Tms11 | Point 11 must disconnect duration. |
| `curve[*].Hz11` | uint16 | Hz | Hz11 | Point 11 must disconnect frequency. |
| `curve[*].Tms12` | uint16 | Secs | Tms12 | Point 12 must disconnect duration. |
| `curve[*].Hz12` | uint16 | Hz | Hz12 | Point 12 must disconnect frequency. |
| `curve[*].Tms13` | uint16 | Secs | Tms13 | Point 13 must disconnect duration. |
| `curve[*].Hz13` | uint16 | Hz | Hz13 | Point 13 must disconnect frequency. |
| `curve[*].Tms14` | uint16 | Secs | Tms14 | Point 14 must disconnect duration. |
| `curve[*].Hz14` | uint16 | Hz | Hz14 | Point 14 must disconnect frequency. |
| `curve[*].Tms15` | uint16 | Secs | Tms15 | Point 15 must disconnect duration. |
| `curve[*].Hz15` | uint16 | Hz | Hz15 | Point 15 must disconnect frequency. |
| `curve[*].Tms16` | uint16 | Secs | Tms16 | Point 16 must disconnect duration. |
| `curve[*].Hz16` | uint16 | Hz | Hz16 | Point 16 must disconnect frequency. |
| `curve[*].Tms17` | uint16 | Secs | Tms17 | Point 17 must disconnect duration. |
| `curve[*].Hz17` | uint16 | Hz | Hz17 | Point 17 must disconnect frequency. |
| `curve[*].Tms18` | uint16 | Secs | Tms18 | Point 18 must disconnect duration. |
| `curve[*].Hz18` | uint16 | Hz | Hz18 | Point 18 must disconnect frequency. |
| `curve[*].Tms19` | uint16 | Secs | Tms19 | Point 19 must disconnect duration. |
| `curve[*].Hz19` | uint16 | Hz | Hz19 | Point 19 must disconnect frequency. |
| `curve[*].Tms20` | uint16 | Secs | Tms20 | Point 20 must disconnect duration. |
| `curve[*].Hz20` | uint16 | Hz | Hz20 | Point 20 must disconnect frequency. |
| `curve[*].CrvNam` | string |  | CrvNam | Optional description for curve. |
| `curve[*].ReadOnly` | enum16 |  | ReadOnly | Curve is read-only or can be modified. |

Value names:

- `ModEna` (bitfield16): bit 0 `ENABLED`
- `curve[*].ReadOnly` (enum16): 0 `READWRITE`, 1 `READONLY`

## Model 137: LVRTC

Name in records: `lvrtc`.

LVRT must remain connected

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `ActCrv` | uint16 |  | ActCrv | Index of active curve. 0=no active curve. |
| `ModEna` | bitfield16 |  | ModEna | LVRT control mode. Enable active curve. |
| `WinTms` | uint16 | Secs | WinTms | Time window for LVRT change. |
| `RvrtTms` | uint16 | Secs | RvrtTms | Timeout period for LVRT curve selection. |
| `RmpTms` | uint16 | Secs | RmpTms | Ramp time for moving from current mode to new mode. |
| `NCrv` | uint16 |  | NCrv | Number of curves supported (recommend 4). |
| `NPt` | uint16 |  | NPt | Number of curve points supported (maximum of 20). |
| `curve[*].ActPt` | uint16 |  | ActPt | Number of active points in array. |
| `curve[*].Tms1` | uint16 | Secs | Tms1 | Point 1 must remain connected duration. |
| `curve[*].V1` | uint16 | % VRef | V1 | Point 1 must remain connected voltage. |
| `curve[*].Tms2` | uint16 | Secs | Tms2 | Point 2 must remain connected duration. |
| `curve[*].V2` | uint16 | % VRef | V2 | Point 2 must remain connected voltage. |
| `curve[*].Tms3` | uint16 | Secs | Tms3 | Point 3 must remain connected duration. |
| `curve[*].V3` | uint16 | % VRef | V3 | Point 3 must remain connected voltage. |
| `curve[*].Tms4` | uint16 | Secs | Tms4 | Point 4 must remain connected duration. |
| `curve[*].V4` | uint16 | % VRef | V4 | Point 4 must remain connected voltage. |
| `curve[*].Tms5` | uint16 | Secs | Tms5 | Point 5 must remain connected duration. |
| `curve[*].V5` | uint16 | % VRef | V5 | Point 5 must remain connected voltage. |
| `curve[*].Tms6` | uint16 | Secs | Tms6 | Point 6 must remain connected duration. |
| `curve[*].V6` | uint16 | % VRef | V6 | Point 6 must remain connected voltage. |
| `curve[*].Tms7` | uint16 | Secs | Tms7 | Point 7 must remain connected duration. |
| `curve[*].V7` | uint16 | % VRef | V7 | Point 7 must remain connected voltage. |
| `curve[*].Tms8` | uint16 | Secs | Tms8 | Point 8 must remain connected duration. |
| `curve[*].V8` | uint16 | % VRef | V8 | Point 8 must remain connected voltage. |
| `curve[*].Tms9` | uint16 | Secs | Tms9 | Point 9 must remain connected duration. |
| `curve[*].V9` | uint16 | % VRef | V9 | Point 9 must remain connected voltage. |
| `curve[*].Tms10` | uint16 | Secs | Tms10 | Point 10 must remain connected duration. |
| `curve[*].V10` | uint16 | % VRef | V10 | Point 10 must remain connected voltage. |
| `curve[*].Tms11` | uint16 | Secs | Tms11 | Point 11 must remain connected duration. |
| `curve[*].V11` | uint16 | % VRef | V11 | Point 11 must remain connected voltage. |
| `curve[*].Tms12` | uint16 | Secs | Tms12 | Point 12 must remain connected duration. |
| `curve[*].V12` | uint16 | % VRef | V12 | Point 12 must remain connected voltage. |
| `curve[*].Tms13` | uint16 | Secs | Tms13 | Point 13 must remain connected duration. |
| `curve[*].V13` | uint16 | % VRef | V13 | Point 13 must remain connected voltage. |
| `curve[*].Tms14` | uint16 | Secs | Tms14 | Point 14 must remain connected duration. |
| `curve[*].V14` | uint16 | % VRef | V14 | Point 14 must remain connected voltage. |
| `curve[*].Tms15` | uint16 | Secs | Tms15 | Point 15 must remain connected duration. |
| `curve[*].V15` | uint16 | % VRef | V15 | Point 15 must remain connected voltage. |
| `curve[*].Tms16` | uint16 | Secs | Tms16 | Point 16 must remain connected duration. |
| `curve[*].V16` | uint16 | % VRef | V16 | Point 16 must remain connected voltage. |
| `curve[*].Tms17` | uint16 | Secs | Tms17 | Point 17 must remain connected duration. |
| `curve[*].V17` | uint16 | % VRef | V17 | Point 17 must remain connected voltage. |
| `curve[*].Tms18` | uint16 | Secs | Tms18 | Point 18 must remain connected duration. |
| `curve[*].V18` | uint16 | % VRef | V18 | Point 18 must remain connected voltage. |
| `curve[*].Tms19` | uint16 | Secs | Tms19 | Point 19 must remain connected duration. |
| `curve[*].V19` | uint16 | % VRef | V19 | Point 19 must remain connected voltage. |
| `curve[*].Tms20` | uint16 | Secs | Tms20 | Point 20 must remain connected duration. |
| `curve[*].V20` | uint16 | % VRef | V20 | Point 20 must remain connected voltage. |
| `curve[*].CrvNam` | string |  | CrvNam | Optional description for curve. |
| `curve[*].ReadOnly` | enum16 |  | ReadOnly | Curve is read-only or can be modified. |

Value names:

- `ModEna` (bitfield16): bit 0 `ENABLED`
- `curve[*].ReadOnly` (enum16): 0 `READWRITE`, 1 `READONLY`

## Model 138: HVRTC

Name in records: `hvrtc`.

HVRT must remain connected

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `ActCrv` | uint16 |  | ActCrv | Index of active curve. 0=no active curve. |
| `ModEna` | bitfield16 |  | ModEna | HVRT control mode. Enable active curve. |
| `WinTms` | uint16 | Secs | WinTms | Time window for HVRT change. |
| `RvrtTms` | uint16 | Secs | RvrtTms | Timeout period for HVRT curve selection. |
| `RmpTms` | uint16 | Secs | RmpTms | Ramp time for moving from current mode to new mode. |
| `NCrv` | uint16 |  | NCrv | Number of curves supported (recommend 4). |
| `NPt` | uint16 |  | NPt | Number of curve points supported (maximum of 20). |
| `curve[*].ActPt` | uint16 |  | ActPt | Number of active points in array. |
| `curve[*].Tms1` | uint16 | Secs | Tms1 | Point 1 must remain connected duration. |
| `curve[*].V1` | uint16 | % VRef | V1 | Point 1 must remain connected voltage. |
| `curve[*].Tms2` | uint16 | Secs | Tms2 | Point 2 must remain connected duration. |
| `curve[*].V2` | uint16 | % VRef | V2 | Point 2 must remain connected voltage. |
| `curve[*].Tms3` | uint16 | Secs | Tms3 | Point 3 must remain connected duration. |
| `curve[*].V3` | uint16 | % VRef | V3 | Point 3 must remain connected voltage. |
| `curve[*].Tms4` | uint16 | Secs | Tms4 | Point 4 must remain connected duration. |
| `curve[*].V4` | uint16 | % VRef | V4 | Point 4 must remain connected voltage. |
| `curve[*].Tms5` | uint16 | Secs | Tms5 | Point 5 must remain connected duration. |
| `curve[*].V5` | uint16 | % VRef | V5 | Point 5 must remain connected voltage. |
| `curve[*].Tms6` | uint16 | Secs | Tms6 | Point 6 must remain connected duration. |
| `curve[*].V6` | uint16 | % VRef | V6 | Point 6 must remain connected voltage. |
| `curve[*].Tms7` | uint16 | Secs | Tms7 | Point 7 must remain connected duration. |
| `curve[*].V7` | uint16 | % VRef | V7 | Point 7 must remain connected voltage. |
| `curve[*].Tms8` | uint16 | Secs | Tms8 | Point 8 must remain connected duration. |
| `curve[*].V8` | uint16 | % VRef | V8 | Point 8 must remain connected voltage. |
| `curve[*].Tms9` | uint16 | Secs | Tms9 | Point 9 must remain connected duration. |
| `curve[*].V9` | uint16 | % VRef | V9 | Point 9 must remain connected voltage. |
| `curve[*].Tms10` | uint16 | Secs | Tms10 | Point 10 must remain connected duration. |
| `curve[*].V10` | uint16 | % VRef | V10 | Point 10 must remain connected voltage. |
| `curve[*].Tms11` | uint16 | Secs | Tms11 | Point 11 must remain connected duration. |
| `curve[*].V11` | uint16 | % VRef | V11 | Point 11 must remain connected voltage. |
| `curve[*].Tms12` | uint16 | Secs | Tms12 | Point 12 must remain connected duration. |
| `curve[*].V12` | uint16 | % VRef | V12 | Point 12 must remain connected voltage. |
| `curve[*].Tms13` | uint16 | Secs | Tms13 | Point 13 must remain connected duration. |
| `curve[*].V13` | uint16 | % VRef | V13 | Point 13 must remain connected voltage. |
| `curve[*].Tms14` | uint16 | Secs | Tms14 | Point 14 must remain connected duration. |
| `curve[*].V14` | uint16 | % VRef | V14 | Point 14 must remain connected voltage. |
| `curve[*].Tms15` | uint16 | Secs | Tms15 | Point 15 must remain connected duration. |
| `curve[*].V15` | uint16 | % VRef | V15 | Point 15 must remain connected voltage. |
| `curve[*].Tms16` | uint16 | Secs | Tms16 | Point 16 must remain connected duration. |
| `curve[*].V16` | uint16 | % VRef | V16 | Point 16 must remain connected voltage. |
| `curve[*].Tms17` | uint16 | Secs | Tms17 | Point 17 must remain connected duration. |
| `curve[*].V17` | uint16 | % VRef | V17 | Point 17 must remain connected voltage. |
| `curve[*].Tms18` | uint16 | Secs | Tms18 | Point 18 must remain connected duration. |
| `curve[*].V18` | uint16 | % VRef | V18 | Point 18 must remain connected voltage. |
| `curve[*].Tms19` | uint16 | Secs | Tms19 | Point 19 must remain connected duration. |
| `curve[*].V19` | uint16 | % VRef | V19 | Point 19 must remain connected voltage. |
| `curve[*].Tms20` | uint16 | Secs | Tms20 | Point 20 must remain connected duration. |
| `curve[*].V20` | uint16 | % VRef | V20 | Point 20 must remain connected voltage. |
| `curve[*].CrvNam` | string |  | CrvNam | Optional description for curve. |
| `curve[*].ReadOnly` | enum16 |  | ReadOnly | Curve is read-only or can be modified. |

Value names:

- `ModEna` (bitfield16): bit 0 `ENABLED`
- `curve[*].ReadOnly` (enum16): 0 `READWRITE`, 1 `READONLY`

## Model 139: LVRTX

Name in records: `lvrtx`.

LVRT extended curve

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `ActCrv` | uint16 |  | ActCrv | Index of active curve. 0=no active curve. |
| `ModEna` | bitfield16 |  | ModEna | LVRT control mode. Enable active curve. |
| `WinTms` | uint16 | Secs | WinTms | Time window for LVRT change. |
| `RvrtTms` | uint16 | Secs | RvrtTms | Timeout period for LVRT curve selection. |
| `RmpTms` | uint16 | Secs | RmpTms | Ramp time for moving from current mode to new mode. |
| `NCrv` | uint16 |  | NCrv | Number of curves supported (recommend 4). |
| `NPt` | uint16 |  | NPt | Number of curve points supported (maximum of 20). |
| `CrvType` | enum16 |  |  |  |
| `curve[*].ActPt` | uint16 |  | ActPt | Number of active points in array. |
| `curve[*].Tms1` | uint16 | Secs | Tms1 | Point 1 duration. |
| `curve[*].V1` | uint16 | % VRef | V1 | Point 1 voltage. |
| `curve[*].Tms2` | uint16 | Secs | Tms2 | Point 2 duration. |
| `curve[*].V2` | uint16 | % VRef | V2 | Point 2 voltage. |
| `curve[*].Tms3` | uint16 | Secs | Tms3 | Point 3 duration. |
| `curve[*].V3` | uint16 | % VRef | V3 | Point 3 voltage. |
| `curve[*].Tms4` | uint16 | Secs | Tms4 | Point 4 duration. |
| `curve[*].V4` | uint16 | % VRef | V4 | Point 4 voltage. |
| `curve[*].Tms5` | uint16 | Secs | Tms5 | Point 5 duration. |
| `curve[*].V5` | uint16 | % VRef | V5 | Point 5 voltage. |
| `curve[*].Tms6` | uint16 | Secs | Tms6 | Point 6 duration. |
| `curve[*].V6` | uint16 | % VRef | V6 | Point 6 voltage. |
| `curve[*].Tms7` | uint16 | Secs | Tms7 | Point 7 duration. |
| `curve[*].V7` | uint16 | % VRef | V7 | Point 7 voltage. |
| `curve[*].Tms8` | uint16 | Secs | Tms8 | Point 8 duration. |
| `curve[*].V8` | uint16 | % VRef | V8 | Point 8 voltage. |
| `curve[*].Tms9` | uint16 | Secs | Tms9 | Point 9 duration. |
| `curve[*].V9` | uint16 | % VRef | V9 | Point 9 voltage. |
| `curve[*].Tms10` | uint16 | Secs | Tms10 | Point 10 duration. |
| `curve[*].V10` | uint16 | % VRef | V10 | Point 10 voltage. |
| `curve[*].Tms11` | uint16 | Secs | Tms11 | Point 11 duration. |
| `curve[*].V11` | uint16 | % VRef | V11 | Point 11 voltage. |
| `curve[*].Tms12` | uint16 | Secs | Tms12 | Point 12 duration. |
| `curve[*].V12` | uint16 | % VRef | V12 | Point 12 voltage. |
| `curve[*].Tms13` | uint16 | Secs | Tms13 | Point 13 duration. |
| `curve[*].V13` | uint16 | % VRef | V13 | Point 13 voltage. |
| `curve[*].Tms14` | uint16 | Secs | Tms14 | Point 14 duration. |
| `curve[*].V14` | uint16 | % VRef | V14 | Point 14 voltage. |
| `curve[*].Tms15` | uint16 | Secs | Tms15 | Point 15 duration. |
| `curve[*].V15` | uint16 | % VRef | V15 | Point 15 voltage. |
| `curve[*].Tms16` | uint16 | Secs | Tms16 | Point 16 duration. |
| `curve[*].V16` | uint16 | % VRef | V16 | Point 16 voltage. |
| `curve[*].Tms17` | uint16 | Secs | Tms17 | Point 17 duration. |
| `curve[*].V17` | uint16 | % VRef | V17 | Point 17 voltage. |
| `curve[*].Tms18` | uint16 | Secs | Tms18 | Point 18 duration. |
| `curve[*].V18` | uint16 | % VRef | V18 | Point 18 voltage. |
| `curve[*].Tms19` | uint16 | Secs | Tms19 | Point 19 duration. |
| `curve[*].V19` | uint16 | % VRef | V19 | Point 19 voltage. |
| `curve[*].Tms20` | uint16 | Secs | Tms20 | Point 20 duration. |
| `curve[*].V20` | uint16 | % VRef | V20 | Point 20 voltage. |
| `curve[*].CrvNam` | string |  | CrvNam | Optional description for curve. |
| `curve[*].ReadOnly` | enum16 |  | ReadOnly | Curve is read-only or can be modified. |

Value names:

- `ModEna` (bitfield16): bit 0 `ENABLED`
- `CrvType` (enum16): 1 `CEASE_TO_ENERGIZE`
- `curve[*].ReadOnly` (enum16): 0 `READWRITE`, 1 `READONLY`

## Model 140: HVRTX

Name in records: `hvrtx`.

HVRT extended curve

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `ActCrv` | uint16 |  | ActCrv | Index of active curve. 0=no active curve. |
| `ModEna` | bitfield16 |  | ModEna | LVRT control mode. Enable active curve. |
| `WinTms` | uint16 | Secs | WinTms | Time window for LVRT change. |
| `RvrtTms` | uint16 | Secs | RvrtTms | Timeout period for LVRT curve selection. |
| `RmpTms` | uint16 | Secs | RmpTms | Ramp time for moving from current mode to new mode. |
| `NCrv` | uint16 |  | NCrv | Number of curves supported (recommend 4). |
| `NPt` | uint16 |  | NPt | Number of curve points supported (maximum of 20). |
| `CrvType` | enum16 |  |  |  |
| `curve[*].ActPt` | uint16 |  | ActPt | Number of active points in array. |
| `curve[*].Tms1` | uint16 | Secs | Tms1 | Point 1 duration. |
| `curve[*].V1` | uint16 | % VRef | V1 | Point 1 voltage. |
| `curve[*].Tms2` | uint16 | Secs | Tms2 | Point 2 duration. |
| `curve[*].V2` | uint16 | % VRef | V2 | Point 2 voltage. |
| `curve[*].Tms3` | uint16 | Secs | Tms3 | Point 3 duration. |
| `curve[*].V3` | uint16 | % VRef | V3 | Point 3 voltage. |
| `curve[*].Tms4` | uint16 | Secs | Tms4 | Point 4 duration. |
| `curve[*].V4` | uint16 | % VRef | V4 | Point 4 voltage. |
| `curve[*].Tms5` | uint16 | Secs | Tms5 | Point 5 duration. |
| `curve[*].V5` | uint16 | % VRef | V5 | Point 5 voltage. |
| `curve[*].Tms6` | uint16 | Secs | Tms6 | Point 6 duration. |
| `curve[*].V6` | uint16 | % VRef | V6 | Point 6 voltage. |
| `curve[*].Tms7` | uint16 | Secs | Tms7 | Point 7 duration. |
| `curve[*].V7` | uint16 | % VRef | V7 | Point 7 voltage. |
| `curve[*].Tms8` | uint16 | Secs | Tms8 | Point 8 duration. |
| `curve[*].V8` | uint16 | % VRef | V8 | Point 8 voltage. |
| `curve[*].Tms9` | uint16 | Secs | Tms9 | Point 9 duration. |
| `curve[*].V9` | uint16 | % VRef | V9 | Point 9 voltage. |
| `curve[*].Tms10` | uint16 | Secs | Tms10 | Point 10 duration. |
| `curve[*].V10` | uint16 | % VRef | V10 | Point 10 voltage. |
| `curve[*].Tms11` | uint16 | Secs | Tms11 | Point 11 duration. |
| `curve[*].V11` | uint16 | % VRef | V11 | Point 11 voltage. |
| `curve[*].Tms12` | uint16 | Secs | Tms12 | Point 12 duration. |
| `curve[*].V12` | uint16 | % VRef | V12 | Point 12 voltage. |
| `curve[*].Tms13` | uint16 | Secs | Tms13 | Point 13 duration. |
| `curve[*].V13` | uint16 | % VRef | V13 | Point 13 voltage. |
| `curve[*].Tms14` | uint16 | Secs | Tms14 | Point 14 duration. |
| `curve[*].V14` | uint16 | % VRef | V14 | Point 14 voltage. |
| `curve[*].Tms15` | uint16 | Secs | Tms15 | Point 15 duration. |
| `curve[*].V15` | uint16 | % VRef | V15 | Point 15 voltage. |
| `curve[*].Tms16` | uint16 | Secs | Tms16 | Point 16 duration. |
| `curve[*].V16` | uint16 | % VRef | V16 | Point 16 voltage. |
| `curve[*].Tms17` | uint16 | Secs | Tms17 | Point 17 duration. |
| `curve[*].V17` | uint16 | % VRef | V17 | Point 17 voltage. |
| `curve[*].Tms18` | uint16 | Secs | Tms18 | Point 18 duration. |
| `curve[*].V18` | uint16 | % VRef | V18 | Point 18 voltage. |
| `curve[*].Tms19` | uint16 | Secs | Tms19 | Point 19 duration. |
| `curve[*].V19` | uint16 | % VRef | V19 | Point 19 voltage. |
| `curve[*].Tms20` | uint16 | Secs | Tms20 | Point 20 duration. |
| `curve[*].V20` | uint16 | % VRef | V20 | Point 20 voltage. |
| `curve[*].CrvNam` | string |  | CrvNam | Optional description for curve. |
| `curve[*].ReadOnly` | enum16 |  | ReadOnly | Curve is read-only or can be modified. |

Value names:

- `ModEna` (bitfield16): bit 0 `ENABLED`
- `CrvType` (enum16): 1 `CEASE_TO_ENERGIZE`
- `curve[*].ReadOnly` (enum16): 0 `READWRITE`, 1 `READONLY`

## Model 141: LFRTC

Name in records: `lfrtc`.

LFRT must remain connected

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `ActCrv` | uint16 |  | ActCrv | Index of active curve. 0=no active curve. |
| `ModEna` | bitfield16 |  | ModEna | LHzRT control mode. Enable active curve. |
| `WinTms` | uint16 | Secs | WinTms | Time window for LFRT change. |
| `RvrtTms` | uint16 | Secs | RvrtTms | Timeout period for LFRT curve selection. |
| `RmpTms` | uint16 | Secs | RmpTms | Ramp time for moving from current mode to new mode. |
| `NCrv` | uint16 |  | NCrv | Number of curves supported (recommend 4). |
| `NPt` | uint16 |  | NPt | Number of curve points supported (maximum of 20). |
| `curve[*].ActPt` | uint16 |  | ActPt | Number of active points in array. |
| `curve[*].Tms1` | uint16 | Secs | Tms1 | Point 1 must remain connected duration. |
| `curve[*].Hz1` | uint16 | Hz | Hz1 | Point 1 must remain connected frequency. |
| `curve[*].Tms2` | uint16 | Secs | Tms2 | Point 2 must remain connected duration. |
| `curve[*].Hz2` | uint16 | Hz | Hz2 | Point 2 must remain connected frequency. |
| `curve[*].Tms3` | uint16 | Secs | Tms3 | Point 3 must remain connected duration. |
| `curve[*].Hz3` | uint16 | Hz | Hz3 | Point 3 must remain connected frequency. |
| `curve[*].Tms4` | uint16 | Secs | Tms4 | Point 4 must remain connected duration. |
| `curve[*].Hz4` | uint16 | Hz | Hz4 | Point 4 must remain connected frequency. |
| `curve[*].Tms5` | uint16 | Secs | Tms5 | Point 5 must remain connected duration. |
| `curve[*].Hz5` | uint16 | Hz | Hz5 | Point 5 must remain connected frequency. |
| `curve[*].Tms6` | uint16 | Secs | Tms6 | Point 6 must remain connected duration. |
| `curve[*].Hz6` | uint16 | Hz | Hz6 | Point 6 must remain connected frequency. |
| `curve[*].Tms7` | uint16 | Secs | Tms7 | Point 7 must remain connected duration. |
| `curve[*].Hz7` | uint16 | Hz | Hz7 | Point 7 must remain connected frequency. |
| `curve[*].Tms8` | uint16 | Secs | Tms8 | Point 8 must remain connected duration. |
| `curve[*].Hz8` | uint16 | Hz | Hz8 | Point 8 must remain connected frequency. |
| `curve[*].Tms9` | uint16 | Secs | Tms9 | Point 9 must remain connected duration. |
| `curve[*].Hz9` | uint16 | Hz | Hz9 | Point 9 must remain connected frequency. |
| `curve[*].Tms10` | uint16 | Secs | Tms10 | Point 10 must remain connected duration. |
| `curve[*].Hz10` | uint16 | Hz | Hz10 | Point 10 must remain connected frequency. |
| `curve[*].Tms11` | uint16 | Secs | Tms11 | Point 11 must remain connected duration. |
| `curve[*].Hz11` | uint16 | Hz | Hz11 | Point 11 must remain connected frequency. |
| `curve[*].Tms12` | uint16 | Secs | Tms12 | Point 12 must remain connected duration. |
| `curve[*].Hz12` | uint16 | Hz | Hz12 | Point 12 must remain connected frequency. |
| `curve[*].Tms13` | uint16 | Secs | Tms13 | Point 13 must remain connected duration. |
| `curve[*].Hz13` | uint16 | Hz | Hz13 | Point 13 must remain connected frequency. |
| `curve[*].Tms14` | uint16 | Secs | Tms14 | Point 14 must remain connected duration. |
| `curve[*].Hz14` | uint16 | Hz | Hz14 | Point 14 must remain connected frequency. |
| `curve[*].Tms15` | uint16 | Secs | Tms15 | Point 15 must remain connected duration. |
| `curve[*].Hz15` | uint16 | Hz | Hz15 | Point 15 must remain connected frequency. |
| `curve[*].Tms16` | uint16 | Secs | Tms16 | Point 16 must remain connected duration. |
| `curve[*].Hz16` | uint16 | Hz | Hz16 | Point 16 must remain connected frequency. |
| `curve[*].Tms17` | uint16 | Secs | Tms17 | Point 17 must remain connected duration. |
| `curve[*].Hz17` | uint16 | Hz | Hz17 | Point 17 must remain connected frequency. |
| `curve[*].Tms18` | uint16 | Secs | Tms18 | Point 18 must remain connected duration. |
| `curve[*].Hz18` | uint16 | Hz | Hz18 | Point 18 must remain connected frequency. |
| `curve[*].Tms19` | uint16 | Secs | Tms19 | Point 19 must remain connected duration. |
| `curve[*].Hz19` | uint16 | Hz | Hz19 | Point 19 must remain connected frequency. |
| `curve[*].Tms20` | uint16 | Secs | Tms20 | Point 20 must remain connected duration. |
| `curve[*].Hz20` | uint16 | Hz | Hz20 | Point 20 must remain connected frequency. |
| `curve[*].CrvNam` | string |  | CrvNam | Optional description for curve. |
| `curve[*].ReadOnly` | enum16 |  | ReadOnly | Curve is read-only or can be modified. |

Value names:

- `ModEna` (bitfield16): bit 0 `ENABLED`
- `curve[*].ReadOnly` (enum16): 0 `READWRITE`, 1 `READONLY`

## Model 142: HFRTC

Name in records: `hfrtc`.

HFRT must remain connected

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `ActCrv` | uint16 |  | ActCrv | Index of active curve. 0=no active curve. |
| `ModEna` | bitfield16 |  | ModEna | LHzRT control mode. Enable active curve. |
| `WinTms` | uint16 | Secs | WinTms | Time window for LFRT change. |
| `RvrtTms` | uint16 | Secs | RvrtTms | Timeout period for LFRT curve selection. |
| `RmpTms` | uint16 | Secs | RmpTms | Ramp time for moving from current mode to new mode. |
| `NCrv` | uint16 |  | NCrv | Number of curves supported (recommend 4). |
| `NPt` | uint16 |  | NPt | Number of curve points supported (maximum of 20). |
| `curve[*].ActPt` | uint16 |  | ActPt | Number of active points in array. |
| `curve[*].Tms1` | uint16 | Secs | Tms1 | Point 1 must remain connected duration. |
| `curve[*].Hz1` | uint16 | Hz | Hz1 | Point 1 must remain connected frequency. |
| `curve[*].Tms2` | uint16 | Secs | Tms2 | Point 2 must remain connected duration. |
| `curve[*].Hz2` | uint16 | Hz | Hz2 | Point 2 must remain connected frequency. |
| `curve[*].Tms3` | uint16 | Secs | Tms3 | Point 3 must remain connected duration. |
| `curve[*].Hz3` | uint16 | Hz | Hz3 | Point 3 must remain connected frequency. |
| `curve[*].Tms4` | uint16 | Secs | Tms4 | Point 4 must remain connected duration. |
| `curve[*].Hz4` | uint16 | Hz | Hz4 | Point 4 must remain connected frequency. |
| `curve[*].Tms5` | uint16 | Secs | Tms5 | Point 5 must remain connected duration. |
| `curve[*].Hz5` | uint16 | Hz | Hz5 | Point 5 must remain connected frequency. |
| `curve[*].Tms6` | uint16 | Secs | Tms6 | Point 6 must remain connected duration. |
| `curve[*].Hz6` | uint16 | Hz | Hz6 | Point 6 must remain connected frequency. |
| `curve[*].Tms7` | uint16 | Secs | Tms7 | Point 7 must remain connected duration. |
| `curve[*].Hz7` | uint16 | Hz | Hz7 | Point 7 must remain connected frequency. |
| `curve[*].Tms8` | uint16 | Secs | Tms8 | Point 8 must remain connected duration. |
| `curve[*].Hz8` | uint16 | Hz | Hz8 | Point 8 must remain connected frequency. |
| `curve[*].Tms9` | uint16 | Secs | Tms9 | Point 9 must remain connected duration. |
| `curve[*].Hz9` | uint16 | Hz | Hz9 | Point 9 must remain connected frequency. |
| `curve[*].Tms10` | uint16 | Secs | Tms10 | Point 10 must remain connected duration. |
| `curve[*].Hz10` | uint16 | Hz | Hz10 | Point 10 must remain connected frequency. |
| `curve[*].Tms11` | uint16 | Secs | Tms11 | Point 11 must remain connected duration. |
| `curve[*].Hz11` | uint16 | Hz | Hz11 | Point 11 must remain connected frequency. |
| `curve[*].Tms12` | uint16 | Secs | Tms12 | Point 12 must remain connected duration. |
| `curve[*].Hz12` | uint16 | Hz | Hz12 | Point 12 must remain connected frequency. |
| `curve[*].Tms13` | uint16 | Secs | Tms13 | Point 13 must remain connected duration. |
| `curve[*].Hz13` | uint16 | Hz | Hz13 | Point 13 must remain connected frequency. |
| `curve[*].Tms14` | uint16 | Secs | Tms14 | Point 14 must remain connected duration. |
| `curve[*].Hz14` | uint16 | Hz | Hz14 | Point 14 must remain connected frequency. |
| `curve[*].Tms15` | uint16 | Secs | Tms15 | Point 15 must remain connected duration. |
| `curve[*].Hz15` | uint16 | Hz | Hz15 | Point 15 must remain connected frequency. |
| `curve[*].Tms16` | uint16 | Secs | Tms16 | Point 16 must remain connected duration. |
| `curve[*].Hz16` | uint16 | Hz | Hz16 | Point 16 must remain connected frequency. |
| `curve[*].Tms17` | uint16 | Secs | Tms17 | Point 17 must remain connected duration. |
| `curve[*].Hz17` | uint16 | Hz | Hz17 | Point 17 must remain connected frequency. |
| `curve[*].Tms18` | uint16 | Secs | Tms18 | Point 18 must remain connected duration. |
| `curve[*].Hz18` | uint16 | Hz | Hz18 | Point 18 must remain connected frequency. |
| `curve[*].Tms19` | uint16 | Secs | Tms19 | Point 19 must remain connected duration. |
| `curve[*].Hz19` | uint16 | Hz | Hz19 | Point 19 must remain connected frequency. |
| `curve[*].Tms20` | uint16 | Secs | Tms20 | Point 20 must remain connected duration. |
| `curve[*].Hz20` | uint16 | Hz | Hz20 | Point 20 must remain connected frequency. |
| `curve[*].CrvNam` | string |  | CrvNam | Optional description for curve. |
| `curve[*].ReadOnly` | enum16 |  | ReadOnly | Curve is read-only or can be modified. |

Value names:

- `ModEna` (bitfield16): bit 0 `ENABLED`
- `curve[*].ReadOnly` (enum16): 0 `READWRITE`, 1 `READONLY`

## Model 143: LFRTX

Name in records: `lfrtx`.

LFRT extended curve

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `ActCrv` | uint16 |  | ActCrv | Index of active curve. 0=no active curve. |
| `ModEna` | bitfield16 |  | ModEna | LHzRT control mode. Enable active curve. |
| `WinTms` | uint16 | Secs | WinTms | Time window for LFRT change. |
| `RvrtTms` | uint16 | Secs | RvrtTms | Timeout period for LFRT curve selection. |
| `RmpTms` | uint16 | Secs | RmpTms | Ramp time for moving from current mode to new mode. |
| `NCrv` | uint16 |  | NCrv | Number of curves supported (recommend 4). |
| `NPt` | uint16 |  | NPt | Number of curve points supported (maximum of 20). |
| `CrvType` | enum16 |  |  |  |
| `curve[*].ActPt` | uint16 |  | ActPt | Number of active points in array. |
| `curve[*].Tms1` | uint16 | Secs | Tms1 | Point 1 duration. |
| `curve[*].Hz1` | uint16 | Hz | Hz1 | Point 1 frequency. |
| `curve[*].Tms2` | uint16 | Secs | Tms2 | Point 2 duration. |
| `curve[*].Hz2` | uint16 | Hz | Hz2 | Point 2 frequency. |
| `curve[*].Tms3` | uint16 | Secs | Tms3 | Point 3 duration. |
| `curve[*].Hz3` | uint16 | Hz | Hz3 | Point 3 frequency. |
| `curve[*].Tms4` | uint16 | Secs | Tms4 | Point 4 duration. |
| `curve[*].Hz4` | uint16 | Hz | Hz4 | Point 4 frequency. |
| `curve[*].Tms5` | uint16 | Secs | Tms5 | Point 5 duration. |
| `curve[*].Hz5` | uint16 | Hz | Hz5 | Point 5 frequency. |
| `curve[*].Tms6` | uint16 | Secs | Tms6 | Point 6 duration. |
| `curve[*].Hz6` | uint16 | Hz | Hz6 | Point 6 frequency. |
| `curve[*].Tms7` | uint16 | Secs | Tms7 | Point 7 duration. |
| `curve[*].Hz7` | uint16 | Hz | Hz7 | Point 7 frequency. |
| `curve[*].Tms8` | uint16 | Secs | Tms8 | Point 8 duration. |
| `curve[*].Hz8` | uint16 | Hz | Hz8 | Point 8 frequency. |
| `curve[*].Tms9` | uint16 | Secs | Tms9 | Point 9 duration. |
| `curve[*].Hz9` | uint16 | Hz | Hz9 | Point 9 frequency. |
| `curve[*].Tms10` | uint16 | Secs | Tms10 | Point 10 duration. |
| `curve[*].Hz10` | uint16 | Hz | Hz10 | Point 10 frequency. |
| `curve[*].Tms11` | uint16 | Secs | Tms11 | Point 11 duration. |
| `curve[*].Hz11` | uint16 | Hz | Hz11 | Point 11 frequency. |
| `curve[*].Tms12` | uint16 | Secs | Tms12 | Point 12 duration. |
| `curve[*].Hz12` | uint16 | Hz | Hz12 | Point 12 frequency. |
| `curve[*].Tms13` | uint16 | Secs | Tms13 | Point 13 duration. |
| `curve[*].Hz13` | uint16 | Hz | Hz13 | Point 13 frequency. |
| `curve[*].Tms14` | uint16 | Secs | Tms14 | Point 14 duration. |
| `curve[*].Hz14` | uint16 | Hz | Hz14 | Point 14 frequency. |
| `curve[*].Tms15` | uint16 | Secs | Tms15 | Point 15 duration. |
| `curve[*].Hz15` | uint16 | Hz | Hz15 | Point 15 frequency. |
| `curve[*].Tms16` | uint16 | Secs | Tms16 | Point 16 duration. |
| `curve[*].Hz16` | uint16 | Hz | Hz16 | Point 16 frequency. |
| `curve[*].Tms17` | uint16 | Secs | Tms17 | Point 17 duration. |
| `curve[*].Hz17` | uint16 | Hz | Hz17 | Point 17 frequency. |
| `curve[*].Tms18` | uint16 | Secs | Tms18 | Point 18 duration. |
| `curve[*].Hz18` | uint16 | Hz | Hz18 | Point 18 frequency. |
| `curve[*].Tms19` | uint16 | Secs | Tms19 | Point 19 duration. |
| `curve[*].Hz19` | uint16 | Hz | Hz19 | Point 19 frequency. |
| `curve[*].Tms20` | uint16 | Secs | Tms20 | Point 20 duration. |
| `curve[*].Hz20` | uint16 | Hz | Hz20 | Point 20 frequency. |
| `curve[*].CrvNam` | string |  | CrvNam | Optional description for curve. |
| `curve[*].ReadOnly` | enum16 |  | ReadOnly | Curve is read-only or can be modified. |

Value names:

- `ModEna` (bitfield16): bit 0 `ENABLED`
- `CrvType` (enum16): 1 `CEASE_TO_ENERGIZE`
- `curve[*].ReadOnly` (enum16): 0 `READWRITE`, 1 `READONLY`

## Model 144: HFRTX

Name in records: `hfrtx`.

HFRT extended curve

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `ActCrv` | uint16 |  | ActCrv | Index of active curve. 0=no active curve. |
| `ModEna` | bitfield16 |  | ModEna | LHzRT control mode. Enable active curve. |
| `WinTms` | uint16 | Secs | WinTms | Time window for LFRT change. |
| `RvrtTms` | uint16 | Secs | RvrtTms | Timeout period for LFRT curve selection. |
| `RmpTms` | uint16 | Secs | RmpTms | Ramp time for moving from current mode to new mode. |
| `NCrv` | uint16 |  | NCrv | Number of curves supported (recommend 4). |
| `NPt` | uint16 |  | NPt | Number of curve points supported (maximum of 20). |
| `CrvType` | enum16 |  |  |  |
| `curve[*].ActPt` | uint16 |  | ActPt | Number of active points in array. |
| `curve[*].Tms1` | uint16 | Secs | Tms1 | Point 1 duration. |
| `curve[*].Hz1` | uint16 | Hz | Hz1 | Point 1 frequency. |
| `curve[*].Tms2` | uint16 | Secs | Tms2 | Point 2 duration. |
| `curve[*].Hz2` | uint16 | Hz | Hz2 | Point 2 frequency. |
| `curve[*].Tms3` | uint16 | Secs | Tms3 | Point 3 duration. |
| `curve[*].Hz3` | uint16 | Hz | Hz3 | Point 3 frequency. |
| `curve[*].Tms4` | uint16 | Secs | Tms4 | Point 4 duration. |
| `curve[*].Hz4` | uint16 | Hz | Hz4 | Point 4 frequency. |
| `curve[*].Tms5` | uint16 | Secs | Tms5 | Point 5 duration. |
| `curve[*].Hz5` | uint16 | Hz | Hz5 | Point 5 frequency. |
| `curve[*].Tms6` | uint16 | Secs | Tms6 | Point 6 duration. |
| `curve[*].Hz6` | uint16 | Hz | Hz6 | Point 6 frequency. |
| `curve[*].Tms7` | uint16 | Secs | Tms7 | Point 7 duration. |
| `curve[*].Hz7` | uint16 | Hz | Hz7 | Point 7 frequency. |
| `curve[*].Tms8` | uint16 | Secs | Tms8 | Point 8 duration. |
| `curve[*].Hz8` | uint16 | Hz | Hz8 | Point 8 frequency. |
| `curve[*].Tms9` | uint16 | Secs | Tms9 | Point 9 duration. |
| `curve[*].Hz9` | uint16 | Hz | Hz9 | Point 9 frequency. |
| `curve[*].Tms10` | uint16 | Secs | Tms10 | Point 10 duration. |
| `curve[*].Hz10` | uint16 | Hz | Hz10 | Point 10 frequency. |
| `curve[*].Tms11` | uint16 | Secs | Tms11 | Point 11 duration. |
| `curve[*].Hz11` | uint16 | Hz | Hz11 | Point 11 frequency. |
| `curve[*].Tms12` | uint16 | Secs | Tms12 | Point 12 duration. |
| `curve[*].Hz12` | uint16 | Hz | Hz12 | Point 12 frequency. |
| `curve[*].Tms13` | uint16 | Secs | Tms13 | Point 13 duration. |
| `curve[*].Hz13` | uint16 | Hz | Hz13 | Point 13 frequency. |
| `curve[*].Tms14` | uint16 | Secs | Tms14 | Point 14 duration. |
| `curve[*].Hz14` | uint16 | Hz | Hz14 | Point 14 frequency. |
| `curve[*].Tms15` | uint16 | Secs | Tms15 | Point 15 duration. |
| `curve[*].Hz15` | uint16 | Hz | Hz15 | Point 15 frequency. |
| `curve[*].Tms16` | uint16 | Secs | Tms16 | Point 16 duration. |
| `curve[*].Hz16` | uint16 | Hz | Hz16 | Point 16 frequency. |
| `curve[*].Tms17` | uint16 | Secs | Tms17 | Point 17 duration. |
| `curve[*].Hz17` | uint16 | Hz | Hz17 | Point 17 frequency. |
| `curve[*].Tms18` | uint16 | Secs | Tms18 | Point 18 duration. |
| `curve[*].Hz18` | uint16 | Hz | Hz18 | Point 18 frequency. |
| `curve[*].Tms19` | uint16 | Secs | Tms19 | Point 19 duration. |
| `curve[*].Hz19` | uint16 | Hz | Hz19 | Point 19 frequency. |
| `curve[*].Tms20` | uint16 | Secs | Tms20 | Point 20 duration. |
| `curve[*].Hz20` | uint16 | Hz | Hz20 | Point 20 frequency. |
| `curve[*].CrvNam` | string |  | CrvNam | Optional description for curve. |
| `curve[*].ReadOnly` | enum16 |  | ReadOnly | Curve is read-only or can be modified. |

Value names:

- `ModEna` (bitfield16): bit 0 `ENABLED`
- `CrvType` (enum16): 1 `CEASE_TO_ENERGIZE`
- `curve[*].ReadOnly` (enum16): 0 `READWRITE`, 1 `READONLY`

## Model 145: Extended Settings

Name in records: `ext_settings`.

Inverter controls extended settings

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `NomRmpUpRte` | uint16 | Pct | Ramp Up Rate | Ramp up rate as a percentage of max current. |
| `NomRmpDnRte` | uint16 | Pct | NomRmpDnRte | Ramp down rate as a percentage of max current. |
| `EmgRmpUpRte` | uint16 | Pct | Emergency Ramp Up Rate | Emergency ramp up rate as a percentage of max current. |
| `EmgRmpDnRte` | uint16 | Pct | Emergency Ramp Down Rate | Emergency ramp down rate as a percentage of max current. |
| `ConnRmpUpRte` | uint16 | Pct | Connect Ramp Up Rate | Connect ramp up rate as a percentage of max current. |
| `ConnRmpDnRte` | uint16 | Pct | Connect Ramp Down Rate | Connect ramp down rate as a percentage of max current. |
| `AGra` | uint16 | Pct | Default Ramp Rate | Ramp rate specified in percent of max current. |

## Model 160: Multiple MPPT Inverter Extension Model

Name in records: `mppt`.

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `Evt` | bitfield32 |  | Global Events |  |
| `N` | count |  | Number of Modules |  |
| `TmsPer` | uint16 |  | Timestamp Period |  |
| `module[*].ID` | uint16 |  | Input ID |  |
| `module[*].IDStr` | string |  | Input ID String |  |
| `module[*].DCA` | uint16 | A | DC Current |  |
| `module[*].DCV` | uint16 | V | DC Voltage |  |
| `module[*].DCW` | uint16 | W | DC Power |  |
| `module[*].DCWH` | acc32 | Wh | Lifetime Energy |  |
| `module[*].Tms` | uint32 | Secs | Timestamp |  |
| `module[*].Tmp` | int16 | C | Temperature |  |
| `module[*].DCSt` | enum16 |  | Operating State |  |
| `module[*].DCEvt` | bitfield32 |  | Module Events |  |

Value names:

- `Evt` (bitfield32): bit 0 `GROUND_FAULT`, bit 1 `INPUT_OVER_VOLTAGE`, bit 2 `RESERVED_2`, bit 3 `DC_DISCONNECT`, bit 4 `RESERVED_4`, bit 5 `CABINET_OPEN`, bit 6 `MANUAL_SHUTDOWN`, bit 7 `OVER_TEMP`, bit 8 `RESERVED_8`, bit 9 `RESERVED_9`, bit 10 `RESERVED_10`, bit 11 `RESERVED_11`, bit 12 `BLOWN_FUSE`, bit 13 `UNDER_TEMP`, bit 14 `MEMORY_LOSS`, bit 15 `ARC_DETECTION`, bit 16 `RESERVED_16`, bit 17 `RESERVED_17`, bit 18 `RESERVED_18`, bit 19 `RESERVED_19`, bit 20 `TEST_FAILED`, bit 21 `INPUT_UNDER_VOLTAGE`, bit 22 `INPUT_OVER_CURRENT`
- `module[*].DCSt` (enum16): 1 `OFF`, 2 `SLEEPING`, 3 `STARTING`, 4 `MPPT`, 5 `THROTTLED`, 6 `SHUTTING_DOWN`, 7 `FAULT`, 8 `STANDBY`, 9 `TEST`, 10 `RESERVED_10`
- `module[*].DCEvt` (bitfield32): bit 0 `GROUND_FAULT`, bit 1 `INPUT_OVER_VOLTAGE`, bit 2 `RESERVED_2`, bit 3 `DC_DISCONNECT`, bit 4 `RESERVED_4`, bit 5 `CABINET_OPEN`, bit 6 `MANUAL_SHUTDOWN`, bit 7 `OVER_TEMP`, bit 8 `RESERVED_8`, bit 9 `RESERVED_9`, bit 10 `RESERVED_10`, bit 11 `RESERVED_11`, bit 12 `BLOWN_FUSE`, bit 13 `UNDER_TEMP`, bit 14 `MEMORY_LOSS`, bit 15 `ARC_DETECTION`, bit 16 `RESERVED_16`, bit 17 `RESERVED_17`, bit 18 `RESERVED_18`, bit 19 `RESERVED_19`, bit 20 `TEST_FAILED`, bit 21 `INPUT_UNDER_VOLTAGE`, bit 22 `INPUT_OVER_CURRENT`
