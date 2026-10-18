# DER (IEEE 1547-2018)

<!-- Generated from the bundled SunSpec definitions by
     UPDATE_MODEL_DOCS=1 cargo test -p sunspec-parser --test models_docs
     Do not edit by hand. -->

Point names are the ones records use; `[*]` stands for a repeating group's index. See [Supported SunSpec models](README.md) for every group.

| Model | Name in records | Points |
|-------|-----------------|--------|
| [701](#model-701-der-ac-measurement) | `DERMeasureAC` | 60 |
| [702](#model-702-der-capacity) | `DERCapacity` | 42 |
| [703](#model-703-enter-service) | `DEREnterService` | 9 |
| [704](#model-704-der-ac-controls) | `DERCtlAC` | 45 |
| [705](#model-705-der-volt-var) | `DERVoltVar` | 19 |
| [706](#model-706-der-volt-watt) | `DERVoltWatt` | 14 |
| [707](#model-707-der-trip-lv) | `DERTripLV` | 15 |
| [708](#model-708-der-trip-hv) | `DERTripHV` | 15 |
| [709](#model-709-der-trip-lf) | `DERTripLF` | 15 |
| [710](#model-710-der-trip-hf) | `DERTripHF` | 15 |
| [711](#model-711-der-frequency-droop) | `DERFreqDroop` | 14 |
| [712](#model-712-der-watt-var) | `DERWattVar` | 14 |
| [713](#model-713-der-storage-capacity) | `DERStorageCapacity` | 5 |
| [714](#model-714-der-dc-measurement) | `DERMeasureDC` | 17 |
| [715](#model-715-derctl) | `DERCtl` | 5 |

## Model 701: DER AC Measurement

Name in records: `DERMeasureAC`.

DER AC measurement model.

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `ACType` | enum16 |  | AC Wiring Type | AC wiring type. |
| `St` | enum16 |  | Operating State | Operating state of the DER. |
| `InvSt` | enum16 |  | Inverter State | Inverter state. |
| `ConnSt` | enum16 |  | Grid Connection State | Grid connection state of the DER. |
| `Alrm` | bitfield32 |  | Alarm Bitfield | Active alarms for the DER. |
| `DERMode` | bitfield32 |  | DER Operational Characteristics | Current operational characteristics of the DER. |
| `W` | int16 | W | Active Power | Total active power. Active power is positive for DER generation and negative for absorption. |
| `VA` | int16 | VA | Apparent Power | Total apparent power. |
| `Var` | int16 | Var | Reactive Power | Total reactive power. |
| `PF` | int16 |  | Power Factor | Power factor. The sign of power factor should be the sign of active power. |
| `A` | int16 | A | Total AC Current | Total AC current. |
| `LLV` | uint16 | V | Voltage LL | Line to line AC voltage as an average of active phases. |
| `LNV` | uint16 | V | Voltage LN | Line to neutral AC voltage as an average of active phases. |
| `Hz` | uint32 | Hz | Frequency | AC frequency. |
| `TotWhInj` | uint64 | Wh | Total Energy Injected | Total active energy injected (Quadrants 1 & 4). |
| `TotWhAbs` | uint64 | Wh | Total Energy Absorbed | Total active energy absorbed (Quadrants 2 & 3). |
| `TotVarhInj` | uint64 | Varh | Total Reactive Energy Inj | Total reactive energy injected (Quadrants 1 & 2). |
| `TotVarhAbs` | uint64 | Varh | Total Reactive Energy Abs | Total reactive energy absorbed (Quadrants 3 & 4). |
| `TmpAmb` | int16 | C | Ambient Temperature | Ambient temperature. |
| `TmpCab` | int16 | C | Cabinet Temperature | Cabinet temperature. |
| `TmpSnk` | int16 | C | Heat Sink Temperature | Heat sink temperature. |
| `TmpTrns` | int16 | C | Transformer Temperature | Transformer temperature. |
| `TmpSw` | int16 | C | IGBT/MOSFET Temperature | IGBT/MOSFET temperature. |
| `TmpOt` | int16 | C | Other Temperature | Other temperature. |
| `WL1` | int16 | W | Watts L1 | Active power L1. |
| `VAL1` | int16 | VA | VA L1 | Apparent power L1. |
| `VarL1` | int16 | Var | Var L1 | Reactive power L1. |
| `PFL1` | int16 |  | PF L1 | Power factor phase L1. |
| `AL1` | int16 | A | Amps L1 | Current phase L1. |
| `VL1L2` | uint16 | V | Phase Voltage L1-L2 | Phase voltage L1-L2. |
| `VL1` | uint16 | V | Phase Voltage L1-N | Phase voltage L1-N. |
| `TotWhInjL1` | uint64 | Wh | Total Watt-Hours Inj L1 | Total active energy injected L1. |
| `TotWhAbsL1` | uint64 | Wh | Total Watt-Hours Abs L1 | Total active energy absorbed L1. |
| `TotVarhInjL1` | uint64 | Varh | Total Var-Hours Inj L1 | Total reactive energy injected L1. |
| `TotVarhAbsL1` | uint64 | Varh | Total Var-Hours Abs L1 | Total reactive energy absorbed L1. |
| `WL2` | int16 | W | Watts L2 | Active power L2. |
| `VAL2` | int16 | VA | VA L2 | Apparent power L2. |
| `VarL2` | int16 | Var | Var L2 | Reactive power L2. |
| `PFL2` | int16 |  | PF L2 | Power factor L2. |
| `AL2` | int16 | A | Amps L2 | Current L2. |
| `VL2L3` | uint16 | V | Phase Voltage L2-L3 | Phase voltage L2-L3. |
| `VL2` | uint16 | V | Phase Voltage L2-N | Phase voltage L2-N. |
| `TotWhInjL2` | uint64 | Wh | Total Watt-Hours Inj L2 | Total active energy injected L2. |
| `TotWhAbsL2` | uint64 | Wh | Total Watt-Hours Abs L2 | Total active energy absorbed L2. |
| `TotVarhInjL2` | uint64 | Varh | Total Var-Hours Inj L2 | Total reactive energy injected L2. |
| `TotVarhAbsL2` | uint64 | Varh | Total Var-Hours Abs L2 | Total reactive energy absorbed L2. |
| `WL3` | int16 | W | Watts L3 | Active power L3. |
| `VAL3` | int16 | VA | VA L3 | Apparent power L3. |
| `VarL3` | int16 | Var | Var L3 | Reactive power L3. |
| `PFL3` | int16 |  | PF L3 | Power factor L3. |
| `AL3` | int16 | A | Amps L3 | Current L3. |
| `VL3L1` | uint16 | V | Phase Voltage L3-L1 | Phase voltage L3-L1. |
| `VL3` | uint16 | V | Phase Voltage L3-N | Phase voltage L3-N. |
| `TotWhInjL3` | uint64 | Wh | Total Watt-Hours Inj L3 | Total active energy injected L3. |
| `TotWhAbsL3` | uint64 | Wh | Total Watt-Hours Abs L3 | Total active energy absorbed L3. |
| `TotVarhInjL3` | uint64 | Varh | Total Var-Hours Inj L3 | Total reactive energy injected L3. |
| `TotVarhAbsL3` | uint64 | Varh | Total Var-Hours Abs L3 | Total reactive energy absorbed L3. |
| `ThrotPct` | uint16 | Pct | Throttling In Pct | Throttling in pct of maximum active power. |
| `ThrotSrc` | bitfield32 |  | Throttle Source Information | Active throttling source. |
| `MnAlrmInfo` | string |  | Manufacturer Alarm Info | Manufacturer alarm information. Valid if MANUFACTURER\_ALRM indication is active. |

Value names:

- `ACType` (enum16): 0 `SINGLE_PHASE`, 1 `SPLIT_PHASE`, 2 `THREE_PHASE`
- `St` (enum16): 0 `OFF`, 1 `ON`
- `InvSt` (enum16): 0 `OFF`, 1 `SLEEPING`, 2 `STARTING`, 3 `RUNNING`, 4 `THROTTLED`, 5 `SHUTTING_DOWN`, 6 `FAULT`, 7 `STANDBY`
- `ConnSt` (enum16): 0 `DISCONNECTED`, 1 `CONNECTED`
- `Alrm` (bitfield32): bit 0 `GROUND_FAULT`, bit 1 `DC_OVER_VOLT`, bit 2 `AC_DISCONNECT`, bit 3 `DC_DISCONNECT`, bit 4 `GRID_DISCONNECT`, bit 5 `CABINET_OPEN`, bit 6 `MANUAL_SHUTDOWN`, bit 7 `OVER_TEMP`, bit 8 `OVER_FREQUENCY`, bit 9 `UNDER_FREQUENCY`, bit 10 `AC_OVER_VOLT`, bit 11 `AC_UNDER_VOLT`, bit 12 `BLOWN_STRING_FUSE`, bit 13 `UNDER_TEMP`, bit 14 `MEMORY_LOSS`, bit 15 `HW_TEST_FAILURE`, bit 16 `MANUFACTURER_ALRM`
- `DERMode` (bitfield32): bit 0 `GRID_FOLLOWING`, bit 1 `GRID_FORMING`, bit 2 `PV_CLIPPED`
- `ThrotSrc` (bitfield32): bit 0 `MAX_W`, bit 1 `FIXED_W`, bit 2 `FIXED_VAR`, bit 3 `FIXED_PF`, bit 4 `VOLT_VAR`, bit 5 `FREQ_WATT`, bit 6 `DYN_REACT_CURR`, bit 7 `LVRT`, bit 8 `HVRT`, bit 9 `WATT_VAR`, bit 10 `VOLT_WATT`, bit 11 `SCHEDULED`, bit 12 `LFRT`, bit 13 `HFRT`, bit 14 `DERATED`

## Model 702: DER Capacity

Name in records: `DERCapacity`.

DER capacity model.

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `WMaxRtg` | uint16 | W | Active Power Max Rating | Maximum active power rating at unity power factor in watts. |
| `WOvrExtRtg` | uint16 | W | Active Power (Over-Excited) Rating | Active power rating at specified over-excited power factor in watts. |
| `WOvrExtRtgPF` | uint16 |  | Specified Over-Excited PF | Specified over-excited power factor. |
| `WUndExtRtg` | uint16 | W | Active Power (Under-Excited) Rating | Active power rating at specified under-excited power factor in watts. |
| `WUndExtRtgPF` | uint16 |  | Specified Under-Excited PF | Specified under-excited power factor. |
| `VAMaxRtg` | uint16 | VA | Apparent Power Max Rating | Maximum apparent power rating in voltamperes. |
| `VarMaxInjRtg` | uint16 | Var | Reactive Power Injected Rating | Maximum injected reactive power rating in vars. |
| `VarMaxAbsRtg` | uint16 | Var | Reactive Power Absorbed Rating | Maximum absorbed reactive power rating in vars. |
| `WChaRteMaxRtg` | uint16 | W | Charge Rate Max Rating | Maximum active power charge rate in watts. |
| `WDisChaRteMaxRtg` | uint16 | W | Discharge Rate Max Rating | Maximum active power discharge rate in watts. |
| `VAChaRteMaxRtg` | uint16 | VA | Charge Rate Max VA Rating | Maximum apparent power charge rate in voltamperes. |
| `VADisChaRteMaxRtg` | uint16 | VA | Discharge Rate Max VA Rating | Maximum apparent power discharge rate in voltamperes. |
| `VNomRtg` | uint16 | V | AC Voltage Nominal Rating | AC voltage nominal rating. |
| `VMaxRtg` | uint16 | V | AC Voltage Max Rating | AC voltage maximum rating. |
| `VMinRtg` | uint16 | V | AC Voltage Min Rating | AC voltage minimum rating. |
| `AMaxRtg` | uint16 | A | AC Current Max Rating | AC current maximum rating in amps. |
| `PFOvrExtRtg` | uint16 |  | PF Over-Excited Rating (Unused) | Unused. Please use WOvrExtRtgPF. |
| `PFUndExtRtg` | uint16 |  | PF Under-Excited Rating (Unused) | Unused. Please use WUndExtRtgPF. |
| `ReactSusceptRtg` | uint16 | S | Reactive Susceptance | Reactive susceptance that remains connected to the Area EPS in the cease to energize and trip state. |
| `NorOpCatRtg` | enum16 |  | Normal Operating Category | Normal operating performance category as specified in IEEE 1547-2018. |
| `AbnOpCatRtg` | enum16 |  | Abnormal Operating Category | Abnormal operating performance category as specified in IEEE 1547-2018. |
| `CtrlModes` | bitfield32 |  | Supported Control Modes | Supported control mode functions. |
| `IntIslandCatRtg` | bitfield16 |  | Intentional Island Categories | Intentional island categories. |
| `WMax` | uint16 | W | Active Power Max Setting | Maximum active power setting used to adjust maximum active power setting. |
| `WMaxOvrExt` | uint16 | W | Active Power (Over-Excited) Setting | Active power setting at specified over-excited power factor in watts. |
| `WOvrExtPF` | uint16 |  | Specified Over-Excited PF | Specified over-excited power factor. |
| `WMaxUndExt` | uint16 | W | Active Power (Under-Excited) Setting | Active power setting at specified under-excited power factor in watts. |
| `WUndExtPF` | uint16 |  | Specified Under-Excited PF | Specified under-excited power factor. |
| `VAMax` | uint16 | VA | Apparent Power Max Setting | Maximum apparent power setting used to adjust maximum apparent power rating. |
| `VarMaxInj` | uint16 | Var | Reactive Power Injected Setting | Maximum injected reactive power setting used to adjust maximum injected reactive power rating. |
| `VarMaxAbs` | uint16 | Var | Reactive Power Absorbed Setting | Maximum absorbed reactive power setting used to adjust maximum absorbed reactive power rating. |
| `WChaRteMax` | uint16 | W | Charge Rate Max Setting | Maximum active power charge rate setting used to adjust maximum active power charge rate rating. |
| `WDisChaRteMax` | uint16 | W | Discharge Rate Max Setting | Maximum active power discharge rate setting used to adjust maximum active power discharge rate rating. |
| `VAChaRteMax` | uint16 | VA | Charge Rate Max VA Setting | Maximum apparent power charge rate setting used to adjust maximum apparent power charge rate rating. |
| `VADisChaRteMax` | uint16 | VA | Discharge Rate Max VA Setting | Maximum apparent power discharge rate setting used to adjust maximum apparent power discharge rate rating. |
| `VNom` | uint16 | V | Nominal AC Voltage Setting | Nominal AC voltage setting. |
| `VMax` | uint16 | V | AC Voltage Max Setting | AC voltage maximum setting used to adjust AC voltage maximum rating. |
| `VMin` | uint16 | V | AC Voltage Min Setting | AC voltage minimum setting used to adjust AC voltage minimum rating. |
| `AMax` | uint16 | A | AC Current Max Setting | Maximum AC current setting used to adjust maximum AC current rating. |
| `PFOvrExt` | uint16 |  | PF Over-Excited Setting (Unused) | Unused. Please use WOvrExtPF. |
| `PFUndExt` | uint16 |  | PF Under-Excited Setting (Unused) | Unused. Please use WUndExtPF. |
| `IntIslandCat` | bitfield16 |  | Intentional Island Categories | Intentional island categories. |

Value names:

- `NorOpCatRtg` (enum16): 0 `CAT_A`, 1 `CAT_B`
- `AbnOpCatRtg` (enum16): 0 `CAT_1`, 1 `CAT_2`, 2 `CAT_3`
- `CtrlModes` (bitfield32): bit 0 `MAX_W`, bit 1 `FIXED_W`, bit 2 `FIXED_VAR`, bit 3 `FIXED_PF`, bit 4 `VOLT_VAR`, bit 5 `FREQ_WATT`, bit 6 `DYN_REACT_CURR`, bit 7 `LV_TRIP`, bit 8 `HV_TRIP`, bit 9 `WATT_VAR`, bit 10 `VOLT_WATT`, bit 11 `SCHEDULED`, bit 12 `LF_TRIP`, bit 13 `HF_TRIP`
- `IntIslandCatRtg` (bitfield16): bit 0 `UNCATEGORIZED`, bit 1 `INT_ISL_CAPABLE`, bit 2 `BLACK_START_CAPABLE`, bit 3 `ISOCH_CAPABLE`
- `IntIslandCat` (bitfield16): bit 0 `UNCATEGORIZED`, bit 1 `INT_ISL_CAPABLE`, bit 2 `BLACK_START_CAPABLE`, bit 3 `ISOCH_CAPABLE`

## Model 703: Enter Service

Name in records: `DEREnterService`.

Enter service model.

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `ES` | enum16 |  | Permit Enter Service | Permit enter service. |
| `ESVHi` | uint16 | Pct | Enter Service Voltage High | Enter service voltage high threshold as percent of normal voltage. |
| `ESVLo` | uint16 | Pct | Enter Service Voltage Low | Enter service voltage low threshold as percent of normal voltage. |
| `ESHzHi` | uint32 | Hz | Enter Service Frequency High | Enter service frequency high threshold. |
| `ESHzLo` | uint32 | Hz | Enter Service Frequency Low | Enter service frequency low threshold. |
| `ESDlyTms` | uint32 | Secs | Enter Service Delay Time | Enter service delay time in seconds. |
| `ESRndTms` | uint32 | Secs | Enter Service Random Delay | Enter service random delay in seconds. |
| `ESRmpTms` | uint32 | Secs | Enter Service Ramp Time | Enter service ramp time in seconds. |
| `ESDlyRemTms` | uint32 | Secs | Enter Service Delay Remaining | Enter service delay time remaining in seconds. |

Value names:

- `ES` (enum16): 0 `DISABLED`, 1 `ENABLED`

## Model 704: DER AC Controls

Name in records: `DERCtlAC`.

DER AC controls model.

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `PFWInjEna` | enum16 |  | Power Factor Enable (W Inj) Enable | Power factor enable when injecting active power. |
| `PFWInjEnaRvrt` | enum16 |  | Power Factor Reversion Enable (W Inj) | Power factor reversion timer when injecting active power enable. |
| `PFWInjRvrtTms` | uint32 | Secs | PF Reversion Time (W Inj) | Power factor reversion timer when injecting active power. |
| `PFWInjRvrtRem` | uint32 | Secs | PF Reversion Time Rem (W Inj) | Power factor reversion time remaining when injecting active power. |
| `PFWAbsEna` | enum16 |  | Power Factor Enable (W Abs) Enable | Power factor enable when absorbing active power. |
| `PFWAbsEnaRvrt` | enum16 |  | Power Factor Reversion Enable (W Abs) | Power factor reversion timer when absorbing active power enable. |
| `PFWAbsRvrtTms` | uint32 | Secs | PF Reversion Time (W Abs) | Power factor reversion timer when absorbing active power. |
| `PFWAbsRvrtRem` | uint32 | Secs | PF Reversion Time Rem (W Abs) | Power factor reversion time remaining when absorbing active power. |
| `WMaxLimPctEna` | enum16 |  | Limit Max Power Pct Enable | Limit maximum active power percent enable. |
| `WMaxLimPct` | uint16 | Pct | Limit Max Power Pct Setpoint | Limit maximum active power percent value. |
| `WMaxLimPctRvrt` | uint16 | Pct | Reversion Limit Max Power Pct | Reversion limit maximum active power percent value. |
| `WMaxLimPctEnaRvrt` | enum16 |  | Reversion Limit Max Power Pct Enable | Reversion limit maximum active power percent value enable. |
| `WMaxLimPctRvrtTms` | uint32 | Secs | Limit Max Power Pct Reversion Time | Limit maximum active power percent reversion time. |
| `WMaxLimPctRvrtRem` | uint32 | Secs | Limit Max Power Pct Rev Time Rem | Limit maximum active power percent reversion time remaining. |
| `WSetEna` | enum16 |  | Set Active Power Enable | Set active power enable. |
| `WSetMod` | enum16 |  | Set Active Power Mode | Set active power mode. |
| `WSet` | int32 | W | Active Power Setpoint (W) | Active power setting value in watts. |
| `WSetRvrt` | int32 | W | Reversion Active Power (W) | Reversion active power setting value in watts. |
| `WSetPct` | int16 | Pct | Active Power Setpoint (Pct) | Active power setting value as percent. |
| `WSetPctRvrt` | int16 | Pct | Reversion Active Power (Pct) | Reversion active power setting value as percent. |
| `WSetEnaRvrt` | enum16 |  | Reversion Active Power Enable | Reversion active power function enable. |
| `WSetRvrtTms` | uint32 | Secs | Active Power Reversion Time | Set active power reversion time. |
| `WSetRvrtRem` | uint32 | Secs | Active Power Rev Time Rem | Set active power reversion time remaining. |
| `VarSetEna` | enum16 |  | Set Reactive Power Enable | Set reactive power enable. |
| `VarSetMod` | enum16 |  | Set Reactive Power Mode | Set reactive power mode. |
| `VarSetPri` | enum16 |  | Reactive Power Priority | Reactive power priority. |
| `VarSet` | int32 | Var | Reactive Power Setpoint (Vars) | Reactive power setting value in vars. |
| `VarSetRvrt` | int32 | Var | Reversion Reactive Power (Vars) | Reversion reactive power setting value in vars. |
| `VarSetPct` | int16 | Pct | Reactive Power Setpoint (Pct) | Reactive power setting value as percent. |
| `VarSetPctRvrt` | int16 | Pct | Reversion Reactive Power (Pct) | Reversion reactive power setting value as percent. |
| `VarSetEnaRvrt` | enum16 |  | Reversion Reactive Power Enable | Reversion reactive power function enable. |
| `VarSetRvrtTms` | uint32 | Secs | Reactive Power Reversion Time | Set reactive power reversion time. |
| `VarSetRvrtRem` | uint32 | Secs | Reactive Power Rev Time Rem | Set reactive power reversion time remaining. |
| `WRmp` | uint16 | %Max/Sec | Normal Ramp Rate | Ramp rate for increases in active power during normal generation. |
| `WRmpRef` | enum16 |  | Normal Ramp Rate Reference | Ramp rate reference unit for increases in active power or current during normal generation. |
| `VarRmp` | uint16 | %Max/Sec | Reactive Power Ramp Rate | Ramp rate based on max reactive power per second. |
| `AntiIslEna` | enum16 |  | Anti-Islanding Enable | Anti-islanding enable. |
| `PFWInj.PF` | uint16 |  | Power Factor (W Inj) | Power factor setpoint when injecting active power. |
| `PFWInj.Ext` | enum16 |  | Power Factor Excitation (W Inj) | Power factor excitation setpoint when injecting active power. |
| `PFWInjRvrt.PF` | uint16 |  | Reversion Power Factor (W Inj) | Reversion power factor setpoint when injecting active power. |
| `PFWInjRvrt.Ext` | enum16 |  | Reversion PF Excitation (W Inj) | Reversion power factor excitation setpoint when injecting active power. |
| `PFWAbs.PF` | uint16 |  | Power Factor (W Abs) | Power factor setpoint when absorbing active power. |
| `PFWAbs.Ext` | enum16 |  | Power Factor Excitation (W Abs) | Power factor excitation setpoint when absorbing active power. |
| `PFWAbsRvrt.PF` | uint16 |  | Reversion Power Factor (W Abs) | Reversion power factor setpoint when absorbing active power. |
| `PFWAbsRvrt.Ext` | enum16 |  | Reversion PF Excitation (W Abs) | Reversion power factor excitation setpoint when absorbing active power. |

Value names:

- `PFWInjEna` (enum16): 0 `DISABLED`, 1 `ENABLED`
- `PFWInjEnaRvrt` (enum16): 0 `DISABLED`, 1 `ENABLED`
- `PFWAbsEna` (enum16): 0 `DISABLED`, 1 `ENABLED`
- `PFWAbsEnaRvrt` (enum16): 0 `DISABLED`, 1 `ENABLED`
- `WMaxLimPctEna` (enum16): 0 `DISABLED`, 1 `ENABLED`
- `WMaxLimPctEnaRvrt` (enum16): 0 `DISABLED`, 1 `ENABLED`
- `WSetEna` (enum16): 0 `DISABLED`, 1 `ENABLED`
- `WSetMod` (enum16): 0 `W_MAX_PCT`, 1 `WATTS`
- `WSetEnaRvrt` (enum16): 0 `DISABLED`, 1 `ENABLED`
- `VarSetEna` (enum16): 0 `DISABLED`, 1 `ENABLED`
- `VarSetMod` (enum16): 0 `W_MAX_PCT`, 1 `VAR_MAX_PCT`, 2 `VAR_AVAIL_PCT`, 3 `VA_MAX_PCT`, 4 `VARS`
- `VarSetPri` (enum16): 0 `ACTIVE`, 1 `REACTIVE`, 2 `VENDOR`
- `VarSetEnaRvrt` (enum16): 0 `DISABLED`, 1 `ENABLED`
- `WRmpRef` (enum16): 0 `A_MAX`, 1 `W_MAX`
- `AntiIslEna` (enum16): 0 `DISABLED`, 1 `ENABLED`
- `PFWInj.Ext` (enum16): 0 `OVER_EXCITED`, 1 `UNDER_EXCITED`
- `PFWInjRvrt.Ext` (enum16): 0 `OVER_EXCITED`, 1 `UNDER_EXCITED`
- `PFWAbs.Ext` (enum16): 0 `OVER_EXCITED`, 1 `UNDER_EXCITED`
- `PFWAbsRvrt.Ext` (enum16): 0 `OVER_EXCITED`, 1 `UNDER_EXCITED`

## Model 705: DER Volt-Var

Name in records: `DERVoltVar`.

DER Volt-Var model.

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `Ena` | enum16 |  | DER Volt-Var Module Enable | Volt-Var control enable. |
| `AdptCrvReq` | uint16 |  | Adopt Curve Request | Index of curve points to adopt. First curve index is 1. |
| `AdptCrvRslt` | enum16 |  | Adopt Curve Result | Result of last adopt curve operation. |
| `NPt` | uint16 |  | Number Of Points | Number of curve points supported. |
| `NCrv` | uint16 |  | Stored Curve Count | Number of stored curves supported. |
| `RvrtTms` | uint32 | Secs | Reversion Timeout | Reversion time in seconds. 0 = No reversion time. |
| `RvrtRem` | uint32 | Secs | Reversion Time Remaining | Reversion time remaining in seconds. |
| `RvrtCrv` | uint16 |  | Reversion Curve | Default curve after reversion timeout. |
| `Crv[*].ActPt` | uint16 |  | Active Points | Number of active points. |
| `Crv[*].DeptRef` | enum16 |  | Dependent Reference | Curve dependent reference. |
| `Crv[*].Pri` | enum16 |  | Power Priority | Power priority. |
| `Crv[*].VRef` | uint16 | VNomPct | Vref Adjustment | Vref adjustment as percentage of nominal voltage. |
| `Crv[*].VRefAuto` | uint16 | VNomPct | Current Autonomous Vref | Autonomous vref value as a percentage of nominal voltage. |
| `Crv[*].VRefAutoEna` | enum16 |  | Autonomous Vref Enable | Enable autonomous vref. |
| `Crv[*].VRefAutoTms` | uint16 | Secs | Auto Vref Time Constant | Autonomous vref time constant. |
| `Crv[*].RspTms` | uint32 | Secs | Open Loop Response Time | Open loop response time. |
| `Crv[*].ReadOnly` | enum16 |  | Curve Access | Curve read-write access. |
| `Crv[*].Pt[*].V` | uint16 | VNomPct | Voltage Point | Curve voltage point as percentage. |
| `Crv[*].Pt[*].Var` | int16 | DeptRef | Reactive Power Point | Curve reactive power point as set in DeptRef point. |

Value names:

- `Ena` (enum16): 0 `DISABLED`, 1 `ENABLED`
- `AdptCrvRslt` (enum16): 0 `IN_PROGRESS`, 1 `COMPLETED`, 2 `FAILED`
- `Crv[*].DeptRef` (enum16): 0 `W_MAX_PCT`, 1 `VAR_MAX_PCT`, 2 `VAR_AVAL_PCT`, 3 `VA_MAX_PCT`
- `Crv[*].Pri` (enum16): 0 `ACTIVE`, 1 `REACTIVE`, 2 `VENDOR`
- `Crv[*].VRefAutoEna` (enum16): 0 `DISABLED`, 1 `ENABLED`
- `Crv[*].ReadOnly` (enum16): 0 `RW`, 1 `R`

## Model 706: DER Volt-Watt

Name in records: `DERVoltWatt`.

DER Volt-Watt model.

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `Ena` | enum16 |  | DER Volt-Watt Module Enable | Volt-Watt control enable. |
| `AdptCrvReq` | uint16 |  | Adopt Curve Request | Index of curve points to adopt. First curve index is 1. |
| `AdptCrvRslt` | enum16 |  | Adopt Curve Result | Result of last adopt curve operation. |
| `NPt` | uint16 |  | Number Of Points | Number of curve points supported. |
| `NCrv` | uint16 |  | Stored Curve Count | Number of stored curves supported. |
| `RvrtTms` | uint32 | Secs | Reversion Timeout | Reversion time in seconds. 0 = No reversion time. |
| `RvrtRem` | uint32 | Secs | Reversion Time Remaining | Reversion time remaining in seconds. |
| `RvrtCrv` | uint16 |  | Reversion Curve | Default curve after reversion timeout. |
| `Crv[*].ActPt` | uint16 |  | Active Points | Number of active points. |
| `Crv[*].DeptRef` | enum16 |  | Dependent Reference | Curve dependent reference. |
| `Crv[*].RspTms` | uint32 | Secs | Open Loop Response Time | Open loop response time. |
| `Crv[*].ReadOnly` | enum16 |  | Curve Access | Curve read-write access. |
| `Crv[*].Pt[*].V` | uint16 | VNomPct | Voltage Point | Curve voltage point as percentage. |
| `Crv[*].Pt[*].W` | int16 | DeptRef | Dependent Reference | Active power in percent of rated active power. |

Value names:

- `Ena` (enum16): 0 `DISABLED`, 1 `ENABLED`
- `AdptCrvRslt` (enum16): 0 `IN_PROGRESS`, 1 `COMPLETED`, 2 `FAILED`
- `Crv[*].DeptRef` (enum16): 0 `W_MAX_PCT`, 1 `W_AVAL_PCT`
- `Crv[*].ReadOnly` (enum16): 0 `RW`, 1 `R`

## Model 707: DER Trip LV

Name in records: `DERTripLV`.

DER low voltage trip model.

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `Ena` | enum16 |  | DER Trip LV Module Enable | DER low voltage trip control enable. |
| `AdptCrvReq` | uint16 |  | Adopt Curve Request | Index of curve points to adopt. First curve index is 1. |
| `AdptCrvRslt` | enum16 |  | Adopt Curve Result | Result of last adopt curve operation. |
| `NPt` | uint16 |  | Number Of Points | Number of curve points supported. |
| `NCrvSet` | uint16 |  | Stored Curve Count | Number of stored curves supported. |
| `Crv[*].ReadOnly` | enum16 |  | Curve Access | Curve read-write access. |
| `Crv[*].MustTrip.ActPt` | uint16 |  | Number Of Active Points | Number of active points in must trip curve. |
| `Crv[*].MustTrip.Pt[*].V` | uint16 | VNomPct | Voltage Point | Curve voltage point as percentage. |
| `Crv[*].MustTrip.Pt[*].Tms` | uint32 | Secs | Time Point | Curve time point in seconds. |
| `Crv[*].MayTrip.ActPt` | uint16 |  | Number Of Active Points | Number of active points in may trip curve. |
| `Crv[*].MayTrip.Pt[*].V` | uint16 | VNomPct | Voltage Point | Curve voltage point as percentage. |
| `Crv[*].MayTrip.Pt[*].Tms` | uint32 | Secs | Time Point | Curve time point in seconds. |
| `Crv[*].MomCess.ActPt` | uint16 |  | Number Of Active Points | Number of active points in the momentary cessation curve. |
| `Crv[*].MomCess.Pt[*].V` | uint16 | VNomPct | Voltage Point | Curve voltage point as percentage. |
| `Crv[*].MomCess.Pt[*].Tms` | uint32 | Secs | Time Point | Curve time point in seconds. |

Value names:

- `Ena` (enum16): 0 `DISABLED`, 1 `ENABLED`
- `AdptCrvRslt` (enum16): 0 `IN_PROGRESS`, 1 `COMPLETED`, 2 `FAILED`
- `Crv[*].ReadOnly` (enum16): 0 `RW`, 1 `R`

## Model 708: DER Trip HV

Name in records: `DERTripHV`.

DER high voltage trip model.

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `Ena` | enum16 |  | DER Trip HV Module Enable | DER high voltage trip control enable. |
| `AdptCrvReq` | uint16 |  | Adopt Curve Request | Index of curve points to adopt. First curve index is 1. |
| `AdptCrvRslt` | enum16 |  | Adopt Curve Result | Result of last adopt curve operation. |
| `NPt` | uint16 |  | Number Of Points | Number of curve points supported. |
| `NCrvSet` | uint16 |  | Stored Curve Count | Number of stored curves supported. |
| `Crv[*].ReadOnly` | enum16 |  | Curve Access | Curve read-write access. |
| `Crv[*].MustTrip.ActPt` | uint16 |  | Number Of Active Points | Number of active points in must trip curve. |
| `Crv[*].MustTrip.Pt[*].V` | uint16 | VNomPct | Voltage Point | Curve voltage point as percentage. |
| `Crv[*].MustTrip.Pt[*].Tms` | uint32 | Secs | Time Point | Curve time point in seconds. |
| `Crv[*].MayTrip.ActPt` | uint16 |  | Number Of Active Points | Number of active points in may trip curve. |
| `Crv[*].MayTrip.Pt[*].V` | uint16 | VNomPct | Voltage Point | Curve voltage point as percentage. |
| `Crv[*].MayTrip.Pt[*].Tms` | uint32 | Secs | Time Point | Curve time point in seconds. |
| `Crv[*].MomCess.ActPt` | uint16 |  | Number Of Active Points | Number of active points in the momentary cessation curve. |
| `Crv[*].MomCess.Pt[*].V` | uint16 | VNomPct | Voltage Point | Curve voltage point as percentage. |
| `Crv[*].MomCess.Pt[*].Tms` | uint32 | Secs | Time Point | Curve time point in seconds. |

Value names:

- `Ena` (enum16): 0 `DISABLED`, 1 `ENABLED`
- `AdptCrvRslt` (enum16): 0 `IN_PROGRESS`, 1 `COMPLETED`, 2 `FAILED`
- `Crv[*].ReadOnly` (enum16): 0 `RW`, 1 `R`

## Model 709: DER Trip LF

Name in records: `DERTripLF`.

DER low frequency trip model.

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `Ena` | enum16 |  | DER Trip LF Module Enable | DER low frequency trip control enable. |
| `AdptCrvReq` | uint16 |  | Adopt Curve Request | Index of curve points to adopt. First curve index is 1. |
| `AdptCrvRslt` | enum16 |  | Adopt Curve Result | Result of last adopt curve operation. |
| `NPt` | uint16 |  | Number Of Points | Number of curve points supported. |
| `NCrvSet` | uint16 |  | Stored Curve Count | Number of stored curves supported. |
| `Crv[*].ReadOnly` | enum16 |  | Curve Access | Curve read-write access. |
| `Crv[*].MustTrip.ActPt` | uint16 |  | Number Of Active Points | Number of active points in must trip curve. |
| `Crv[*].MustTrip.Pt[*].Hz` | uint32 | Hz | Frequency Point | Curve frequency point. |
| `Crv[*].MustTrip.Pt[*].Tms` | uint32 | Secs | Time Point | Curve time point in seconds. |
| `Crv[*].MayTrip.ActPt` | uint16 |  | Number Of Active Points | Number of active points in may trip curve. |
| `Crv[*].MayTrip.Pt[*].Hz` | uint32 | Hz | Frequency Point | Curve frequency point. |
| `Crv[*].MayTrip.Pt[*].Tms` | uint32 | Secs | Time Point | Curve time point in seconds. |
| `Crv[*].MomCess.ActPt` | uint16 |  | Number Of Active Points | Number of active points in the momentary cessation curve. |
| `Crv[*].MomCess.Pt[*].Hz` | uint32 | Hz | Frequency Point | Curve frequency point. |
| `Crv[*].MomCess.Pt[*].Tms` | uint32 | Secs | Time Point | Curve time point in seconds. |

Value names:

- `Ena` (enum16): 0 `DISABLED`, 1 `ENABLED`
- `AdptCrvRslt` (enum16): 0 `IN_PROGRESS`, 1 `COMPLETED`, 2 `FAILED`
- `Crv[*].ReadOnly` (enum16): 0 `RW`, 1 `R`

## Model 710: DER Trip HF

Name in records: `DERTripHF`.

DER high frequency trip model.

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `Ena` | enum16 |  | DER Trip HF Module Enable | DER high frequency trip control enable. |
| `AdptCrvReq` | uint16 |  | Adopt Curve Request | Index of curve points to adopt. First curve index is 1. |
| `AdptCrvRslt` | enum16 |  | Adopt Curve Result | Result of last adopt curve operation. |
| `NPt` | uint16 |  | Number Of Points | Number of curve points supported. |
| `NCrvSet` | uint16 |  | Stored Curve Count | Number of stored curves supported. |
| `Crv[*].ReadOnly` | enum16 |  | Curve Access | Curve read-write access. |
| `Crv[*].MustTrip.ActPt` | uint16 |  | Number Of Active Points | Number of active points in must trip curve. |
| `Crv[*].MustTrip.Pt[*].Hz` | uint32 | Hz | Frequency Point | Curve frequency point. |
| `Crv[*].MustTrip.Pt[*].Tms` | uint32 | Secs | Time Point | Curve time point in seconds. |
| `Crv[*].MayTrip.ActPt` | uint16 |  | Number Of Active Points | Number of active points in may trip curve. |
| `Crv[*].MayTrip.Pt[*].Hz` | uint32 | Hz | Frequency Point | Curve frequency point. |
| `Crv[*].MayTrip.Pt[*].Tms` | uint32 | Secs | Time Point | Curve time point in seconds. |
| `Crv[*].MomCess.ActPt` | uint16 |  | Number Of Active Points | Number of active points in the momentary cessation curve. |
| `Crv[*].MomCess.Pt[*].Hz` | uint32 | Hz | Frequency Point | Curve frequency point. |
| `Crv[*].MomCess.Pt[*].Tms` | uint32 | Secs | Time Point | Curve time point in seconds. |

Value names:

- `Ena` (enum16): 0 `DISABLED`, 1 `ENABLED`
- `AdptCrvRslt` (enum16): 0 `IN_PROGRESS`, 1 `COMPLETED`, 2 `FAILED`
- `Crv[*].ReadOnly` (enum16): 0 `RW`, 1 `R`

## Model 711: DER Frequency Droop

Name in records: `DERFreqDroop`.

DER Frequency Droop model.

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `Ena` | enum16 |  | DER Frequency Droop Module Enable | DER Frequency-Watt (Frequency-Droop) control enable. |
| `AdptCtlReq` | uint16 |  | Set Active Control Request | Set active control. 0 = No active control. |
| `AdptCtlRslt` | enum16 |  | Set Active Control Result | Result of last set active control operation. |
| `NCtl` | uint16 |  | Stored Control Count | Number of stored controls supported. |
| `RvrtTms` | uint32 | Secs | Reversion Timeout | Reversion time in seconds. 0 = No reversion time. |
| `RvrtRem` | uint32 | Secs | Reversion Time Left | Reversion time remaining in seconds. |
| `RvrtCtl` | uint16 |  | Reversion Control | Default control after reversion timeout. |
| `Ctl[*].DbOf` | uint32 | Hz | Over-Frequency Deadband | The deadband value for over-frequency conditions in Hz. |
| `Ctl[*].DbUf` | uint32 | Hz | Under-Frequency Deadband | The deadband value for under-frequency conditions in Hz. |
| `Ctl[*].KOf` | uint16 |  | Over-Frequency Change Ratio | Frequency droop per-unit frequency change for over-frequency conditions corresponding to 1 per-unit power output change. |
| `Ctl[*].KUf` | uint16 |  | Under-Frequency Change Ratio | Frequency droop per-unit frequency change for under-frequency conditions corresponding to 1 per-unit power output change. |
| `Ctl[*].RspTms` | uint32 | Secs | Open-Loop Response Time | The open-loop response time in seconds. |
| `Ctl[*].PMin` | int16 | Pct | Minimum Active Power | The minimum active power output due to DER prime mover constraints, in percent of the DER active power rating. The valid range is -100 to 100. This setting applies only to the frequency droop control. |
| `Ctl[*].ReadOnly` | enum16 |  | Control Access | Control read-write access. |

Value names:

- `Ena` (enum16): 0 `DISABLED`, 1 `ENABLED`
- `AdptCtlRslt` (enum16): 0 `IN_PROGRESS`, 1 `COMPLETED`, 2 `FAILED`
- `Ctl[*].ReadOnly` (enum16): 0 `RW`, 1 `R`

## Model 712: DER Watt-Var

Name in records: `DERWattVar`.

DER Watt-Var model.

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `Ena` | enum16 |  | DER Watt-Var Module Enable | DER Watt-Var control enable. |
| `AdptCrvReq` | uint16 |  | Set Active Curve Request | Set active curve. 0 = No active curve. |
| `AdptCrvRslt` | enum16 |  | Set Active Curve Result | Result of last set active curve operation. |
| `NPt` | uint16 |  | Number Of Points | Number of curve points supported. |
| `NCrv` | uint16 |  | Stored Curve Count | Number of stored curves supported. |
| `RvrtTms` | uint32 | Secs | Reversion Timeout | Reversion time in seconds. 0 = No reversion time. |
| `RvrtRem` | uint32 | Secs | Reversion Time Left | Reversion time remaining in seconds. |
| `RvrtCrv` | uint16 |  | Reversion Curve | Default curve after reversion timeout. |
| `Crv[*].ActPt` | uint16 |  | Active Points | Number of active points. |
| `Crv[*].DeptRef` | enum16 |  | Dependent Reference | Curve dependent reference. |
| `Crv[*].Pri` | enum16 |  | Power Priority | Power priority. |
| `Crv[*].ReadOnly` | enum16 |  | Curve Access | Curve read-write access. |
| `Crv[*].Pt[*].W` | int16 | WMaxPct | Active Power Point | Curve active power point as percentage. |
| `Crv[*].Pt[*].Var` | int16 | VarPct | Reactive Power Point | Curve reactive power point as set in DeptRef point. |

Value names:

- `Ena` (enum16): 0 `DISABLED`, 1 `ENABLED`
- `AdptCrvRslt` (enum16): 0 `IN_PROGRESS`, 1 `COMPLETED`, 2 `FAILED`
- `Crv[*].DeptRef` (enum16): 0 `W_MAX_PCT`, 1 `VAR_MAX_PCT`, 2 `VAR_AVAL_PCT`, 3 `VA_MAX_PCT`
- `Crv[*].Pri` (enum16): 0 `ACTIVE`, 1 `REACTIVE`
- `Crv[*].ReadOnly` (enum16): 0 `RW`, 1 `R`

## Model 713: DER Storage Capacity

Name in records: `DERStorageCapacity`.

DER storage capacity.

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `WHRtg` | uint16 | WH | Energy Rating | Energy rating of the DER storage. |
| `WHAvail` | uint16 | WH | Energy Available | Energy available of the DER storage (WHAvail = WHRtg \* SoC \* SoH) |
| `SoC` | uint16 | Pct | State of Charge | State of charge of the DER storage. |
| `SoH` | uint16 | Pct | State of Health | State of health of the DER storage. |
| `Sta` | enum16 |  | Status | Storage status. |

Value names:

- `Sta` (enum16): 0 `OK`, 1 `WARNING`, 2 `ERROR`

## Model 714: DER DC Measurement

Name in records: `DERMeasureDC`.

DER DC measurement.

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `PrtAlrms` | bitfield32 |  | Port Alarms | Bitfield of ports with active alarms. Bit is 1 if port has an active alarm. Bit 0 is first port. |
| `NPrt` | uint16 |  | Number Of Ports | Number of DC ports. |
| `DCA` | int16 | A | DC Current | Total DC current for all ports. |
| `DCW` | int16 | W | DC Power | Total DC power for all ports. |
| `DCWhInj` | uint64 | Wh | DC Energy Injected | Total cumulative DC energy injected for all ports. |
| `DCWhAbs` | uint64 | Wh | DC Energy Absorbed | Total cumulative DC energy absorbed for all ports. |
| `Prt[*].PrtTyp` | enum16 |  | Port Type | Port type. |
| `Prt[*].ID` | uint16 |  | Port ID | Port ID. |
| `Prt[*].IDStr` | string |  | Port ID String | Port ID string. |
| `Prt[*].DCA` | int16 | A | DC Current | DC current for the port. |
| `Prt[*].DCV` | uint16 | V | DC Voltage | DC voltage for the port. |
| `Prt[*].DCW` | int16 | W | DC Power | DC power for the port. |
| `Prt[*].DCWhInj` | uint64 | Wh | DC Energy Injected | Total cumulative DC energy injected for the port. |
| `Prt[*].DCWhAbs` | uint64 | Wh | DC Energy Absorbed | Total cumulative DC energy absorbed for the port. |
| `Prt[*].Tmp` | int16 | C | DC Port Temperature | DC port temperature. |
| `Prt[*].DCSta` | enum16 |  | DC Port Status | DC port status. |
| `Prt[*].DCAlrm` | bitfield32 |  | DC Port Alarm | DC port alarm. |

Value names:

- `Prt[*].PrtTyp` (enum16): 0 `PV`, 1 `ESS`, 2 `EV`, 3 `INJ`, 4 `ABS`, 5 `BIDIR`, 6 `DC_DC`
- `Prt[*].DCSta` (enum16): 0 `OFF`, 1 `ON`, 2 `WARNING`, 3 `ERROR`
- `Prt[*].DCAlrm` (bitfield32): bit 0 `GROUND_FAULT`, bit 1 `INPUT_OVER_VOLTAGE`, bit 3 `DC_DISCONNECT`, bit 5 `CABINET_OPEN`, bit 6 `MANUAL_SHUTDOWN`, bit 7 `OVER_TEMP`, bit 12 `BLOWN_FUSE`, bit 13 `UNDER_TEMP`, bit 14 `MEMORY_LOSS`, bit 15 `ARC_DETECTION`, bit 19 `RESERVED`, bit 20 `TEST_FAILED`, bit 21 `INPUT_UNDER_VOLTAGE`, bit 22 `INPUT_OVER_CURRENT`

## Model 715: DERCtl

Name in records: `DERCtl`.

DER Control

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `LocRemCtl` | enum16 |  | Control Mode | DER control mode. Enumeration. |
| `DERHb` | uint32 |  | DER Heartbeat | Value is incremented every second by the DER with periodic resets to zero. |
| `ControllerHb` | uint32 |  | Controller Heartbeat | Value is incremented every second by the controller with periodic resets to zero. |
| `AlarmReset` | uint16 |  | Alarm Reset | Used to reset any latched alarms. 1 = Reset. |
| `OpCtl` | enum16 |  | Set Operation | Commands to PCS. |

Value names:

- `LocRemCtl` (enum16): 0 `REMOTE`, 1 `LOCAL`
- `OpCtl` (enum16): 0 `STOP`, 1 `START`, 2 `ENTER_STANDBY`, 3 `EXIT_STANDBY`
