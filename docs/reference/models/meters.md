# Meters

<!-- Generated from the bundled SunSpec definitions by
     UPDATE_MODEL_DOCS=1 cargo test -p sunspec-parser --test models_docs
     Do not edit by hand. -->

Point names are the ones records use; `[*]` stands for a repeating group's index. See [Supported SunSpec models](README.md) for every group.

| Model | Name in records | Points |
|-------|-----------------|--------|
| [201](#model-201-meter-single-phase-single-phase-an-or-ab-meter) | `ac_meter_an_or_ab` | 62 |
| [202](#model-202-split-single-phase-abn-meter) | `ac_meter_abn` | 62 |
| [203](#model-203-wye-connect-three-phase-abcn-meter) | `ac_meter_abcn` | 62 |
| [204](#model-204-delta-connect-three-phase-abc-meter) | `ac_meter_abc` | 62 |
| [211](#model-211-single-phase-an-or-ab-meter) | `ac_meter_an_or_ab_float` | 62 |
| [212](#model-212-split-single-phase-abn-meter) | `ac_meter_abn_float` | 62 |
| [213](#model-213-wye-connect-three-phase-abcn-meter) | `ac_meter_abcn_float` | 62 |
| [214](#model-214-delta-connect-three-phase-abc-meter) | `ac_meter_abc_float` | 62 |
| [220](#model-220-secure-ac-meter-selected-readings) | `ac_meter_secure` | 22 |

## Model 201: Meter (Single Phase) single phase (AN or AB) meter

Name in records: `ac_meter_an_or_ab`.

Include this model for single phase (AN or AB) metering

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `A` | int16 | A | Amps | Total AC Current |
| `AphA` | int16 | A | Amps PhaseA | Phase A Current |
| `AphB` | int16 | A | Amps PhaseB | Phase B Current |
| `AphC` | int16 | A | Amps PhaseC | Phase C Current |
| `PhV` | int16 | V | Voltage LN | Line to Neutral AC Voltage (average of active phases) |
| `PhVphA` | int16 | V | Phase Voltage AN | Phase Voltage AN |
| `PhVphB` | int16 | V | Phase Voltage BN | Phase Voltage BN |
| `PhVphC` | int16 | V | Phase Voltage CN | Phase Voltage CN |
| `PPV` | int16 | V | Voltage LL | Line to Line AC Voltage (average of active phases) |
| `PPVphAB` | int16 | V | Phase Voltage AB | Phase Voltage AB |
| `PPVphBC` | int16 | V | Phase Voltage BC | Phase Voltage BC |
| `PPVphCA` | int16 | V | Phase Voltage CA | Phase Voltage CA |
| `Hz` | int16 | Hz | Hz | Frequency |
| `W` | int16 | W | Watts | Total Real Power |
| `WphA` | int16 | W | Watts phase A |  |
| `WphB` | int16 | W | Watts phase B |  |
| `WphC` | int16 | W | Watts phase C |  |
| `VA` | int16 | VA | VA | AC Apparent Power |
| `VAphA` | int16 | VA | VA phase A |  |
| `VAphB` | int16 | VA | VA phase B |  |
| `VAphC` | int16 | VA | VA phase C |  |
| `VAR` | int16 | var | VAR | Reactive Power |
| `VARphA` | int16 | var | VAR phase A |  |
| `VARphB` | int16 | var | VAR phase B |  |
| `VARphC` | int16 | var | VAR phase C |  |
| `PF` | int16 | Pct | PF | Power Factor |
| `PFphA` | int16 | Pct | PF phase A |  |
| `PFphB` | int16 | Pct | PF phase B |  |
| `PFphC` | int16 | Pct | PF phase C |  |
| `TotWhExp` | acc32 | Wh | Total Watt-hours Exported | Total Real Energy Exported |
| `TotWhExpPhA` | acc32 | Wh | Total Watt-hours Exported phase A |  |
| `TotWhExpPhB` | acc32 | Wh | Total Watt-hours Exported phase B |  |
| `TotWhExpPhC` | acc32 | Wh | Total Watt-hours Exported phase C |  |
| `TotWhImp` | acc32 | Wh | Total Watt-hours Imported | Total Real Energy Imported |
| `TotWhImpPhA` | acc32 | Wh | Total Watt-hours Imported phase A |  |
| `TotWhImpPhB` | acc32 | Wh | Total Watt-hours Imported phase B |  |
| `TotWhImpPhC` | acc32 | Wh | Total Watt-hours Imported phase C |  |
| `TotVAhExp` | acc32 | VAh | Total VA-hours Exported | Total Apparent Energy Exported |
| `TotVAhExpPhA` | acc32 | VAh | Total VA-hours Exported phase A |  |
| `TotVAhExpPhB` | acc32 | VAh | Total VA-hours Exported phase B |  |
| `TotVAhExpPhC` | acc32 | VAh | Total VA-hours Exported phase C |  |
| `TotVAhImp` | acc32 | VAh | Total VA-hours Imported | Total Apparent Energy Imported |
| `TotVAhImpPhA` | acc32 | VAh | Total VA-hours Imported phase A |  |
| `TotVAhImpPhB` | acc32 | VAh | Total VA-hours Imported phase B |  |
| `TotVAhImpPhC` | acc32 | VAh | Total VA-hours Imported phase C |  |
| `TotVArhImpQ1` | acc32 | varh | Total VAR-hours Imported Q1 | Total Reactive Energy Imported Quadrant 1 |
| `TotVArhImpQ1PhA` | acc32 | varh | Total VAr-hours Imported Q1 phase A |  |
| `TotVArhImpQ1PhB` | acc32 | varh | Total VAr-hours Imported Q1 phase B |  |
| `TotVArhImpQ1PhC` | acc32 | varh | Total VAr-hours Imported Q1 phase C |  |
| `TotVArhImpQ2` | acc32 | varh | Total VAr-hours Imported Q2 | Total Reactive Power Imported Quadrant 2 |
| `TotVArhImpQ2PhA` | acc32 | varh | Total VAr-hours Imported Q2 phase A |  |
| `TotVArhImpQ2PhB` | acc32 | varh | Total VAr-hours Imported Q2 phase B |  |
| `TotVArhImpQ2PhC` | acc32 | varh | Total VAr-hours Imported Q2 phase C |  |
| `TotVArhExpQ3` | acc32 | varh | Total VAr-hours Exported Q3 | Total Reactive Power Exported Quadrant 3 |
| `TotVArhExpQ3PhA` | acc32 | varh | Total VAr-hours Exported Q3 phase A |  |
| `TotVArhExpQ3PhB` | acc32 | varh | Total VAr-hours Exported Q3 phase B |  |
| `TotVArhExpQ3PhC` | acc32 | varh | Total VAr-hours Exported Q3 phase C |  |
| `TotVArhExpQ4` | acc32 | varh | Total VAr-hours Exported Q4 | Total Reactive Power Exported Quadrant 4 |
| `TotVArhExpQ4PhA` | acc32 | varh | Total VAr-hours Exported Q4 Imported phase A |  |
| `TotVArhExpQ4PhB` | acc32 | varh | Total VAr-hours Exported Q4 Imported phase B |  |
| `TotVArhExpQ4PhC` | acc32 | varh | Total VAr-hours Exported Q4 Imported phase C |  |
| `Evt` | bitfield32 |  | Events | Meter Event Flags |

Value names:

- `Evt` (bitfield32): bit 2 `Power_Failure`, bit 3 `Under_Voltage`, bit 4 `Low_PF`, bit 5 `Over_Current`, bit 6 `Over_Voltage`, bit 7 `Missing_Sensor`, bit 16 `OEM01`, bit 17 `OEM02`, bit 18 `OEM03`, bit 19 `OEM04`, bit 20 `OEM05`, bit 21 `OEM06`, bit 22 `OEM07`, bit 23 `OEM08`, bit 24 `OEM09`, bit 25 `OEM10`, bit 26 `OEM11`, bit 27 `OEM12`, bit 28 `OEM13`, bit 29 `OEM14`, bit 30 `OEM15`

## Model 202: split single phase (ABN) meter

Name in records: `ac_meter_abn`.

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `A` | int16 | A | Amps | Total AC Current |
| `AphA` | int16 | A | Amps PhaseA | Phase A Current |
| `AphB` | int16 | A | Amps PhaseB | Phase B Current |
| `AphC` | int16 | A | Amps PhaseC | Phase C Current |
| `PhV` | int16 | V | Voltage LN | Line to Neutral AC Voltage (average of active phases) |
| `PhVphA` | int16 | V | Phase Voltage AN | Phase Voltage AN |
| `PhVphB` | int16 | V | Phase Voltage BN | Phase Voltage BN |
| `PhVphC` | int16 | V | Phase Voltage CN | Phase Voltage CN |
| `PPV` | int16 | V | Voltage LL | Line to Line AC Voltage (average of active phases) |
| `PhVphAB` | int16 | V | Phase Voltage AB | Phase Voltage AB |
| `PhVphBC` | int16 | V | Phase Voltage BC | Phase Voltage BC |
| `PhVphCA` | int16 | V | Phase Voltage CA | Phase Voltage CA |
| `Hz` | int16 | Hz | Hz | Frequency |
| `W` | int16 | W | Watts | Total Real Power |
| `WphA` | int16 | W | Watts phase A |  |
| `WphB` | int16 | W | Watts phase B |  |
| `WphC` | int16 | W | Watts phase C |  |
| `VA` | int16 | VA | VA | AC Apparent Power |
| `VAphA` | int16 | VA | VA phase A |  |
| `VAphB` | int16 | VA | VA phase B |  |
| `VAphC` | int16 | VA | VA phase C |  |
| `VAR` | int16 | var | VAR | Reactive Power |
| `VARphA` | int16 | var | VAR phase A |  |
| `VARphB` | int16 | var | VAR phase B |  |
| `VARphC` | int16 | var | VAR phase C |  |
| `PF` | int16 | Pct | PF | Power Factor |
| `PFphA` | int16 | Pct | PF phase A |  |
| `PFphB` | int16 | Pct | PF phase B |  |
| `PFphC` | int16 | Pct | PF phase C |  |
| `TotWhExp` | acc32 | Wh | Total Watt-hours Exported | Total Real Energy Exported |
| `TotWhExpPhA` | acc32 | Wh | Total Watt-hours Exported phase A |  |
| `TotWhExpPhB` | acc32 | Wh | Total Watt-hours Exported phase B |  |
| `TotWhExpPhC` | acc32 | Wh | Total Watt-hours Exported phase C |  |
| `TotWhImp` | acc32 | Wh | Total Watt-hours Imported | Total Real Energy Imported |
| `TotWhImpPhA` | acc32 | Wh | Total Watt-hours Imported phase A |  |
| `TotWhImpPhB` | acc32 | Wh | Total Watt-hours Imported phase B |  |
| `TotWhImpPhC` | acc32 | Wh | Total Watt-hours Imported phase C |  |
| `TotVAhExp` | acc32 | VAh | Total VA-hours Exported | Total Apparent Energy Exported |
| `TotVAhExpPhA` | acc32 | VAh | Total VA-hours Exported phase A |  |
| `TotVAhExpPhB` | acc32 | VAh | Total VA-hours Exported phase B |  |
| `TotVAhExpPhC` | acc32 | VAh | Total VA-hours Exported phase C |  |
| `TotVAhImp` | acc32 | VAh | Total VA-hours Imported | Total Apparent Energy Imported |
| `TotVAhImpPhA` | acc32 | VAh | Total VA-hours Imported phase A |  |
| `TotVAhImpPhB` | acc32 | VAh | Total VA-hours Imported phase B |  |
| `TotVAhImpPhC` | acc32 | VAh | Total VA-hours Imported phase C |  |
| `TotVArhImpQ1` | acc32 | varh | Total VAR-hours Imported Q1 | Total Reactive Energy Imported Quadrant 1 |
| `TotVArhImpQ1PhA` | acc32 | varh | Total VAr-hours Imported Q1 phase A |  |
| `TotVArhImpQ1PhB` | acc32 | varh | Total VAr-hours Imported Q1 phase B |  |
| `TotVArhImpQ1PhC` | acc32 | varh | Total VAr-hours Imported Q1 phase C |  |
| `TotVArhImpQ2` | acc32 | varh | Total VAr-hours Imported Q2 | Total Reactive Power Imported Quadrant 2 |
| `TotVArhImpQ2PhA` | acc32 | varh | Total VAr-hours Imported Q2 phase A |  |
| `TotVArhImpQ2PhB` | acc32 | varh | Total VAr-hours Imported Q2 phase B |  |
| `TotVArhImpQ2PhC` | acc32 | varh | Total VAr-hours Imported Q2 phase C |  |
| `TotVArhExpQ3` | acc32 | varh | Total VAr-hours Exported Q3 | Total Reactive Power Exported Quadrant 3 |
| `TotVArhExpQ3PhA` | acc32 | varh | Total VAr-hours Exported Q3 phase A |  |
| `TotVArhExpQ3PhB` | acc32 | varh | Total VAr-hours Exported Q3 phase B |  |
| `TotVArhExpQ3PhC` | acc32 | varh | Total VAr-hours Exported Q3 phase C |  |
| `TotVArhExpQ4` | acc32 | varh | Total VAr-hours Exported Q4 | Total Reactive Power Exported Quadrant 4 |
| `TotVArhExpQ4PhA` | acc32 | varh | Total VAr-hours Exported Q4 Imported phase A |  |
| `TotVArhExpQ4PhB` | acc32 | varh | Total VAr-hours Exported Q4 Imported phase B |  |
| `TotVArhExpQ4PhC` | acc32 | varh | Total VAr-hours Exported Q4 Imported phase C |  |
| `Evt` | bitfield32 |  | Events | Meter Event Flags |

Value names:

- `Evt` (bitfield32): bit 2 `M_EVENT_Power_Failure`, bit 3 `M_EVENT_Under_Voltage`, bit 4 `M_EVENT_Low_PF`, bit 5 `M_EVENT_Over_Current`, bit 6 `M_EVENT_Over_Voltage`, bit 7 `M_EVENT_Missing_Sensor`, bit 8 `M_EVENT_Reserved1`, bit 9 `M_EVENT_Reserved2`, bit 10 `M_EVENT_Reserved3`, bit 11 `M_EVENT_Reserved4`, bit 12 `M_EVENT_Reserved5`, bit 13 `M_EVENT_Reserved6`, bit 14 `M_EVENT_Reserved7`, bit 15 `M_EVENT_Reserved8`, bit 16 `M_EVENT_OEM01`, bit 17 `M_EVENT_OEM02`, bit 18 `M_EVENT_OEM03`, bit 19 `M_EVENT_OEM04`, bit 20 `M_EVENT_OEM05`, bit 21 `M_EVENT_OEM06`, bit 22 `M_EVENT_OEM07`, bit 23 `M_EVENT_OEM08`, bit 24 `M_EVENT_OEM09`, bit 25 `M_EVENT_OEM10`, bit 26 `M_EVENT_OEM11`, bit 27 `M_EVENT_OEM12`, bit 28 `M_EVENT_OEM13`, bit 29 `M_EVENT_OEM14`, bit 30 `M_EVENT_OEM15`

## Model 203: wye-connect three phase (abcn) meter

Name in records: `ac_meter_abcn`.

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `A` | int16 | A | Amps | Total AC Current |
| `AphA` | int16 | A | Amps PhaseA | Phase A Current |
| `AphB` | int16 | A | Amps PhaseB | Phase B Current |
| `AphC` | int16 | A | Amps PhaseC | Phase C Current |
| `PhV` | int16 | V | Voltage LN | Line to Neutral AC Voltage (average of active phases) |
| `PhVphA` | int16 | V | Phase Voltage AN | Phase Voltage AN |
| `PhVphB` | int16 | V | Phase Voltage BN | Phase Voltage BN |
| `PhVphC` | int16 | V | Phase Voltage CN | Phase Voltage CN |
| `PPV` | int16 | V | Voltage LL | Line to Line AC Voltage (average of active phases) |
| `PhVphAB` | int16 | V | Phase Voltage AB | Phase Voltage AB |
| `PhVphBC` | int16 | V | Phase Voltage BC | Phase Voltage BC |
| `PhVphCA` | int16 | V | Phase Voltage CA | Phase Voltage CA |
| `Hz` | int16 | Hz | Hz | Frequency |
| `W` | int16 | W | Watts | Total Real Power |
| `WphA` | int16 | W | Watts phase A |  |
| `WphB` | int16 | W | Watts phase B |  |
| `WphC` | int16 | W | Watts phase C |  |
| `VA` | int16 | VA | VA | AC Apparent Power |
| `VAphA` | int16 | VA | VA phase A |  |
| `VAphB` | int16 | VA | VA phase B |  |
| `VAphC` | int16 | VA | VA phase C |  |
| `VAR` | int16 | var | VAR | Reactive Power |
| `VARphA` | int16 | var | VAR phase A |  |
| `VARphB` | int16 | var | VAR phase B |  |
| `VARphC` | int16 | var | VAR phase C |  |
| `PF` | int16 | Pct | PF | Power Factor |
| `PFphA` | int16 | Pct | PF phase A |  |
| `PFphB` | int16 | Pct | PF phase B |  |
| `PFphC` | int16 | Pct | PF phase C |  |
| `TotWhExp` | acc32 | Wh | Total Watt-hours Exported | Total Real Energy Exported |
| `TotWhExpPhA` | acc32 | Wh | Total Watt-hours Exported phase A |  |
| `TotWhExpPhB` | acc32 | Wh | Total Watt-hours Exported phase B |  |
| `TotWhExpPhC` | acc32 | Wh | Total Watt-hours Exported phase C |  |
| `TotWhImp` | acc32 | Wh | Total Watt-hours Imported | Total Real Energy Imported |
| `TotWhImpPhA` | acc32 | Wh | Total Watt-hours Imported phase A |  |
| `TotWhImpPhB` | acc32 | Wh | Total Watt-hours Imported phase B |  |
| `TotWhImpPhC` | acc32 | Wh | Total Watt-hours Imported phase C |  |
| `TotVAhExp` | acc32 | VAh | Total VA-hours Exported | Total Apparent Energy Exported |
| `TotVAhExpPhA` | acc32 | VAh | Total VA-hours Exported phase A |  |
| `TotVAhExpPhB` | acc32 | VAh | Total VA-hours Exported phase B |  |
| `TotVAhExpPhC` | acc32 | VAh | Total VA-hours Exported phase C |  |
| `TotVAhImp` | acc32 | VAh | Total VA-hours Imported | Total Apparent Energy Imported |
| `TotVAhImpPhA` | acc32 | VAh | Total VA-hours Imported phase A |  |
| `TotVAhImpPhB` | acc32 | VAh | Total VA-hours Imported phase B |  |
| `TotVAhImpPhC` | acc32 | VAh | Total VA-hours Imported phase C |  |
| `TotVArhImpQ1` | acc32 | varh | Total VAR-hours Imported Q1 | Total Reactive Energy Imported Quadrant 1 |
| `TotVArhImpQ1PhA` | acc32 | varh | Total VAr-hours Imported Q1 phase A |  |
| `TotVArhImpQ1PhB` | acc32 | varh | Total VAr-hours Imported Q1 phase B |  |
| `TotVArhImpQ1PhC` | acc32 | varh | Total VAr-hours Imported Q1 phase C |  |
| `TotVArhImpQ2` | acc32 | varh | Total VAr-hours Imported Q2 | Total Reactive Power Imported Quadrant 2 |
| `TotVArhImpQ2PhA` | acc32 | varh | Total VAr-hours Imported Q2 phase A |  |
| `TotVArhImpQ2PhB` | acc32 | varh | Total VAr-hours Imported Q2 phase B |  |
| `TotVArhImpQ2PhC` | acc32 | varh | Total VAr-hours Imported Q2 phase C |  |
| `TotVArhExpQ3` | acc32 | varh | Total VAr-hours Exported Q3 | Total Reactive Power Exported Quadrant 3 |
| `TotVArhExpQ3PhA` | acc32 | varh | Total VAr-hours Exported Q3 phase A |  |
| `TotVArhExpQ3PhB` | acc32 | varh | Total VAr-hours Exported Q3 phase B |  |
| `TotVArhExpQ3PhC` | acc32 | varh | Total VAr-hours Exported Q3 phase C |  |
| `TotVArhExpQ4` | acc32 | varh | Total VAr-hours Exported Q4 | Total Reactive Power Exported Quadrant 4 |
| `TotVArhExpQ4PhA` | acc32 | varh | Total VAr-hours Exported Q4 Imported phase A |  |
| `TotVArhExpQ4PhB` | acc32 | varh | Total VAr-hours Exported Q4 Imported phase B |  |
| `TotVArhExpQ4PhC` | acc32 | varh | Total VAr-hours Exported Q4 Imported phase C |  |
| `Evt` | bitfield32 |  | Events | Meter Event Flags |

Value names:

- `Evt` (bitfield32): bit 2 `M_EVENT_Power_Failure`, bit 3 `M_EVENT_Under_Voltage`, bit 4 `M_EVENT_Low_PF`, bit 5 `M_EVENT_Over_Current`, bit 6 `M_EVENT_Over_Voltage`, bit 7 `M_EVENT_Missing_Sensor`, bit 8 `M_EVENT_Reserved1`, bit 9 `M_EVENT_Reserved2`, bit 10 `M_EVENT_Reserved3`, bit 11 `M_EVENT_Reserved4`, bit 12 `M_EVENT_Reserved5`, bit 13 `M_EVENT_Reserved6`, bit 14 `M_EVENT_Reserved7`, bit 15 `M_EVENT_Reserved8`, bit 16 `M_EVENT_OEM01`, bit 17 `M_EVENT_OEM02`, bit 18 `M_EVENT_OEM03`, bit 19 `M_EVENT_OEM04`, bit 20 `M_EVENT_OEM05`, bit 21 `M_EVENT_OEM06`, bit 22 `M_EVENT_OEM07`, bit 23 `M_EVENT_OEM08`, bit 24 `M_EVENT_OEM09`, bit 25 `M_EVENT_OEM10`, bit 26 `M_EVENT_OEM11`, bit 27 `M_EVENT_OEM12`, bit 28 `M_EVENT_OEM13`, bit 29 `M_EVENT_OEM14`, bit 30 `M_EVENT_OEM15`

## Model 204: delta-connect three phase (abc) meter

Name in records: `ac_meter_abc`.

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `A` | int16 | A | Amps | Total AC Current |
| `AphA` | int16 | A | Amps PhaseA | Phase A Current |
| `AphB` | int16 | A | Amps PhaseB | Phase B Current |
| `AphC` | int16 | A | Amps PhaseC | Phase C Current |
| `PhV` | int16 | V | Voltage LN | Line to Neutral AC Voltage (average of active phases) |
| `PhVphA` | int16 | V | Phase Voltage AN | Phase Voltage AN |
| `PhVphB` | int16 | V | Phase Voltage BN | Phase Voltage BN |
| `PhVphC` | int16 | V | Phase Voltage CN | Phase Voltage CN |
| `PPV` | int16 | V | Voltage LL | Line to Line AC Voltage (average of active phases) |
| `PhVphAB` | int16 | V | Phase Voltage AB | Phase Voltage AB |
| `PhVphBC` | int16 | V | Phase Voltage BC | Phase Voltage BC |
| `PhVphCA` | int16 | V | Phase Voltage CA | Phase Voltage CA |
| `Hz` | int16 | Hz | Hz | Frequency |
| `W` | int16 | W | Watts | Total Real Power |
| `WphA` | int16 | W | Watts phase A |  |
| `WphB` | int16 | W | Watts phase B |  |
| `WphC` | int16 | W | Watts phase C |  |
| `VA` | int16 | VA | VA | AC Apparent Power |
| `VAphA` | int16 | VA | VA phase A |  |
| `VAphB` | int16 | VA | VA phase B |  |
| `VAphC` | int16 | VA | VA phase C |  |
| `VAR` | int16 | var | VAR | Reactive Power |
| `VARphA` | int16 | var | VAR phase A |  |
| `VARphB` | int16 | var | VAR phase B |  |
| `VARphC` | int16 | var | VAR phase C |  |
| `PF` | int16 | Pct | PF | Power Factor |
| `PFphA` | int16 | Pct | PF phase A |  |
| `PFphB` | int16 | Pct | PF phase B |  |
| `PFphC` | int16 | Pct | PF phase C |  |
| `TotWhExp` | acc32 | Wh | Total Watt-hours Exported | Total Real Energy Exported |
| `TotWhExpPhA` | acc32 | Wh | Total Watt-hours Exported phase A |  |
| `TotWhExpPhB` | acc32 | Wh | Total Watt-hours Exported phase B |  |
| `TotWhExpPhC` | acc32 | Wh | Total Watt-hours Exported phase C |  |
| `TotWhImp` | acc32 | Wh | Total Watt-hours Imported | Total Real Energy Imported |
| `TotWhImpPhA` | acc32 | Wh | Total Watt-hours Imported phase A |  |
| `TotWhImpPhB` | acc32 | Wh | Total Watt-hours Imported phase B |  |
| `TotWhImpPhC` | acc32 | Wh | Total Watt-hours Imported phase C |  |
| `TotVAhExp` | acc32 | VAh | Total VA-hours Exported | Total Apparent Energy Exported |
| `TotVAhExpPhA` | acc32 | VAh | Total VA-hours Exported phase A |  |
| `TotVAhExpPhB` | acc32 | VAh | Total VA-hours Exported phase B |  |
| `TotVAhExpPhC` | acc32 | VAh | Total VA-hours Exported phase C |  |
| `TotVAhImp` | acc32 | VAh | Total VA-hours Imported | Total Apparent Energy Imported |
| `TotVAhImpPhA` | acc32 | VAh | Total VA-hours Imported phase A |  |
| `TotVAhImpPhB` | acc32 | VAh | Total VA-hours Imported phase B |  |
| `TotVAhImpPhC` | acc32 | VAh | Total VA-hours Imported phase C |  |
| `TotVArhImpQ1` | acc32 | varh | Total VAR-hours Imported Q1 | Total Reactive Energy Imported Quadrant 1 |
| `TotVArhImpQ1PhA` | acc32 | varh | Total VAr-hours Imported Q1 phase A |  |
| `TotVArhImpQ1PhB` | acc32 | varh | Total VAr-hours Imported Q1 phase B |  |
| `TotVArhImpQ1PhC` | acc32 | varh | Total VAr-hours Imported Q1 phase C |  |
| `TotVArhImpQ2` | acc32 | varh | Total VAr-hours Imported Q2 | Total Reactive Power Imported Quadrant 2 |
| `TotVArhImpQ2PhA` | acc32 | varh | Total VAr-hours Imported Q2 phase A |  |
| `TotVArhImpQ2PhB` | acc32 | varh | Total VAr-hours Imported Q2 phase B |  |
| `TotVArhImpQ2PhC` | acc32 | varh | Total VAr-hours Imported Q2 phase C |  |
| `TotVArhExpQ3` | acc32 | varh | Total VAr-hours Exported Q3 | Total Reactive Power Exported Quadrant 3 |
| `TotVArhExpQ3PhA` | acc32 | varh | Total VAr-hours Exported Q3 phase A |  |
| `TotVArhExpQ3PhB` | acc32 | varh | Total VAr-hours Exported Q3 phase B |  |
| `TotVArhExpQ3PhC` | acc32 | varh | Total VAr-hours Exported Q3 phase C |  |
| `TotVArhExpQ4` | acc32 | varh | Total VAr-hours Exported Q4 | Total Reactive Power Exported Quadrant 4 |
| `TotVArhExpQ4PhA` | acc32 | varh | Total VAr-hours Exported Q4 Imported phase A |  |
| `TotVArhExpQ4PhB` | acc32 | varh | Total VAr-hours Exported Q4 Imported phase B |  |
| `TotVArhExpQ4PhC` | acc32 | varh | Total VAr-hours Exported Q4 Imported phase C |  |
| `Evt` | bitfield32 |  | Events | Meter Event Flags |

Value names:

- `Evt` (bitfield32): bit 2 `M_EVENT_Power_Failure`, bit 3 `M_EVENT_Under_Voltage`, bit 4 `M_EVENT_Low_PF`, bit 5 `M_EVENT_Over_Current`, bit 6 `M_EVENT_Over_Voltage`, bit 7 `M_EVENT_Missing_Sensor`, bit 8 `M_EVENT_Reserved1`, bit 9 `M_EVENT_Reserved2`, bit 10 `M_EVENT_Reserved3`, bit 11 `M_EVENT_Reserved4`, bit 12 `M_EVENT_Reserved5`, bit 13 `M_EVENT_Reserved6`, bit 14 `M_EVENT_Reserved7`, bit 15 `M_EVENT_Reserved8`, bit 16 `M_EVENT_OEM01`, bit 17 `M_EVENT_OEM02`, bit 18 `M_EVENT_OEM03`, bit 19 `M_EVENT_OEM04`, bit 20 `M_EVENT_OEM05`, bit 21 `M_EVENT_OEM06`, bit 22 `M_EVENT_OEM07`, bit 23 `M_EVENT_OEM08`, bit 24 `M_EVENT_OEM09`, bit 25 `M_EVENT_OEM10`, bit 26 `M_EVENT_OEM11`, bit 27 `M_EVENT_OEM12`, bit 28 `M_EVENT_OEM13`, bit 29 `M_EVENT_OEM14`, bit 30 `M_EVENT_OEM15`

## Model 211: single phase (AN or AB) meter

Name in records: `ac_meter_an_or_ab_float`.

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `A` | float32 | A | Amps | Total AC Current |
| `AphA` | float32 | A | Amps PhaseA | Phase A Current |
| `AphB` | float32 | A | Amps PhaseB | Phase B Current |
| `AphC` | float32 | A | Amps PhaseC | Phase C Current |
| `PhV` | float32 | V | Voltage LN | Line to Neutral AC Voltage (average of active phases) |
| `PhVphA` | float32 | V | Phase Voltage AN | Phase Voltage AN |
| `PhVphB` | float32 | V | Phase Voltage BN | Phase Voltage BN |
| `PhVphC` | float32 | V | Phase Voltage CN | Phase Voltage CN |
| `PPV` | float32 | V | Voltage LL | Line to Line AC Voltage (average of active phases) |
| `PPVphAB` | float32 | V | Phase Voltage AB | Phase Voltage AB |
| `PPVphBC` | float32 | V | Phase Voltage BC | Phase Voltage BC |
| `PPVphCA` | float32 | V | Phase Voltage CA | Phase Voltage CA |
| `Hz` | float32 | Hz | Hz | Frequency |
| `W` | float32 | W | Watts | Total Real Power |
| `WphA` | float32 | W | Watts phase A |  |
| `WphB` | float32 | W | Watts phase B |  |
| `WphC` | float32 | W | Watts phase C |  |
| `VA` | float32 | VA | VA | AC Apparent Power |
| `VAphA` | float32 | VA | VA phase A |  |
| `VAphB` | float32 | VA | VA phase B |  |
| `VAphC` | float32 | VA | VA phase C |  |
| `VAR` | float32 | var | VAR | Reactive Power |
| `VARphA` | float32 | var | VAR phase A |  |
| `VARphB` | float32 | var | VAR phase B |  |
| `VARphC` | float32 | var | VAR phase C |  |
| `PF` | float32 | PF | PF | Power Factor |
| `PFphA` | float32 | PF | PF phase A |  |
| `PFphB` | float32 | PF | PF phase B |  |
| `PFphC` | float32 | PF | PF phase C |  |
| `TotWhExp` | float32 | Wh | Total Watt-hours Exported | Total Real Energy Exported |
| `TotWhExpPhA` | float32 | Wh | Total Watt-hours Exported phase A |  |
| `TotWhExpPhB` | float32 | Wh | Total Watt-hours Exported phase B |  |
| `TotWhExpPhC` | float32 | Wh | Total Watt-hours Exported phase C |  |
| `TotWhImp` | float32 | Wh | Total Watt-hours Imported | Total Real Energy Imported |
| `TotWhImpPhA` | float32 | Wh | Total Watt-hours Imported phase A |  |
| `TotWhImpPhB` | float32 | Wh | Total Watt-hours Imported phase B |  |
| `TotWhImpPhC` | float32 | Wh | Total Watt-hours Imported phase C |  |
| `TotVAhExp` | float32 | VAh | Total VA-hours Exported | Total Apparent Energy Exported |
| `TotVAhExpPhA` | float32 | VAh | Total VA-hours Exported phase A |  |
| `TotVAhExpPhB` | float32 | VAh | Total VA-hours Exported phase B |  |
| `TotVAhExpPhC` | float32 | VAh | Total VA-hours Exported phase C |  |
| `TotVAhImp` | float32 | VAh | Total VA-hours Imported | Total Apparent Energy Imported |
| `TotVAhImpPhA` | float32 | VAh | Total VA-hours Imported phase A |  |
| `TotVAhImpPhB` | float32 | VAh | Total VA-hours Imported phase B |  |
| `TotVAhImpPhC` | float32 | VAh | Total VA-hours Imported phase C |  |
| `TotVArhImpQ1` | float32 | varh | Total VAR-hours Imported Q1 | Total Reactive Energy Imported Quadrant 1 |
| `TotVArhImpQ1phA` | float32 | varh | Total VAr-hours Imported Q1 phase A |  |
| `TotVArhImpQ1phB` | float32 | varh | Total VAr-hours Imported Q1 phase B |  |
| `TotVArhImpQ1phC` | float32 | varh | Total VAr-hours Imported Q1 phase C |  |
| `TotVArhImpQ2` | float32 | varh | Total VAr-hours Imported Q2 | Total Reactive Power Imported Quadrant 2 |
| `TotVArhImpQ2phA` | float32 | varh | Total VAr-hours Imported Q2 phase A |  |
| `TotVArhImpQ2phB` | float32 | varh | Total VAr-hours Imported Q2 phase B |  |
| `TotVArhImpQ2phC` | float32 | varh | Total VAr-hours Imported Q2 phase C |  |
| `TotVArhExpQ3` | float32 | varh | Total VAr-hours Exported Q3 | Total Reactive Power Exported Quadrant 3 |
| `TotVArhExpQ3phA` | float32 | varh | Total VAr-hours Exported Q3 phase A |  |
| `TotVArhExpQ3phB` | float32 | varh | Total VAr-hours Exported Q3 phase B |  |
| `TotVArhExpQ3phC` | float32 | varh | Total VAr-hours Exported Q3 phase C |  |
| `TotVArhExpQ4` | float32 | varh | Total VAr-hours Exported Q4 | Total Reactive Power Exported Quadrant 4 |
| `TotVArhExpQ4phA` | float32 | varh | Total VAr-hours Exported Q4 Imported phase A |  |
| `TotVArhExpQ4phB` | float32 | varh | Total VAr-hours Exported Q4 Imported phase B |  |
| `TotVArhExpQ4phC` | float32 | varh | Total VAr-hours Exported Q4 Imported phase C |  |
| `Evt` | bitfield32 |  | Events | Meter Event Flags |

Value names:

- `Evt` (bitfield32): bit 2 `M_EVENT_Power_Failure`, bit 3 `M_EVENT_Under_Voltage`, bit 4 `M_EVENT_Low_PF`, bit 5 `M_EVENT_Over_Current`, bit 6 `M_EVENT_Over_Voltage`, bit 7 `M_EVENT_Missing_Sensor`, bit 8 `M_EVENT_Reserved1`, bit 9 `M_EVENT_Reserved2`, bit 10 `M_EVENT_Reserved3`, bit 11 `M_EVENT_Reserved4`, bit 12 `M_EVENT_Reserved5`, bit 13 `M_EVENT_Reserved6`, bit 14 `M_EVENT_Reserved7`, bit 15 `M_EVENT_Reserved8`, bit 16 `M_EVENT_OEM01`, bit 17 `M_EVENT_OEM02`, bit 18 `M_EVENT_OEM03`, bit 19 `M_EVENT_OEM04`, bit 20 `M_EVENT_OEM05`, bit 21 `M_EVENT_OEM06`, bit 22 `M_EVENT_OEM07`, bit 23 `M_EVENT_OEM08`, bit 24 `M_EVENT_OEM09`, bit 25 `M_EVENT_OEM10`, bit 26 `M_EVENT_OEM11`, bit 27 `M_EVENT_OEM12`, bit 28 `M_EVENT_OEM13`, bit 29 `M_EVENT_OEM14`, bit 30 `M_EVENT_OEM15`

## Model 212: split single phase (ABN) meter

Name in records: `ac_meter_abn_float`.

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `A` | float32 | A | Amps | Total AC Current |
| `AphA` | float32 | A | Amps PhaseA | Phase A Current |
| `AphB` | float32 | A | Amps PhaseB | Phase B Current |
| `AphC` | float32 | A | Amps PhaseC | Phase C Current |
| `PhV` | float32 | V | Voltage LN | Line to Neutral AC Voltage (average of active phases) |
| `PhVphA` | float32 | V | Phase Voltage AN | Phase Voltage AN |
| `PhVphB` | float32 | V | Phase Voltage BN | Phase Voltage BN |
| `PhVphC` | float32 | V | Phase Voltage CN | Phase Voltage CN |
| `PPV` | float32 | V | Voltage LL | Line to Line AC Voltage (average of active phases) |
| `PPVphAB` | float32 | V | Phase Voltage AB | Phase Voltage AB |
| `PPVphBC` | float32 | V | Phase Voltage BC | Phase Voltage BC |
| `PPVphCA` | float32 | V | Phase Voltage CA | Phase Voltage CA |
| `Hz` | float32 | Hz | Hz | Frequency |
| `W` | float32 | W | Watts | Total Real Power |
| `WphA` | float32 | W | Watts phase A |  |
| `WphB` | float32 | W | Watts phase B |  |
| `WphC` | float32 | W | Watts phase C |  |
| `VA` | float32 | VA | VA | AC Apparent Power |
| `VAphA` | float32 | VA | VA phase A |  |
| `VAphB` | float32 | VA | VA phase B |  |
| `VAphC` | float32 | VA | VA phase C |  |
| `VAR` | float32 | var | VAR | Reactive Power |
| `VARphA` | float32 | var | VAR phase A |  |
| `VARphB` | float32 | var | VAR phase B |  |
| `VARphC` | float32 | var | VAR phase C |  |
| `PF` | float32 | PF | PF | Power Factor |
| `PFphA` | float32 | PF | PF phase A |  |
| `PFphB` | float32 | PF | PF phase B |  |
| `PFphC` | float32 | PF | PF phase C |  |
| `TotWhExp` | float32 | Wh | Total Watt-hours Exported | Total Real Energy Exported |
| `TotWhExpPhA` | float32 | Wh | Total Watt-hours Exported phase A |  |
| `TotWhExpPhB` | float32 | Wh | Total Watt-hours Exported phase B |  |
| `TotWhExpPhC` | float32 | Wh | Total Watt-hours Exported phase C |  |
| `TotWhImp` | float32 | Wh | Total Watt-hours Imported | Total Real Energy Imported |
| `TotWhImpPhA` | float32 | Wh | Total Watt-hours Imported phase A |  |
| `TotWhImpPhB` | float32 | Wh | Total Watt-hours Imported phase B |  |
| `TotWhImpPhC` | float32 | Wh | Total Watt-hours Imported phase C |  |
| `TotVAhExp` | float32 | VAh | Total VA-hours Exported | Total Apparent Energy Exported |
| `TotVAhExpPhA` | float32 | VAh | Total VA-hours Exported phase A |  |
| `TotVAhExpPhB` | float32 | VAh | Total VA-hours Exported phase B |  |
| `TotVAhExpPhC` | float32 | VAh | Total VA-hours Exported phase C |  |
| `TotVAhImp` | float32 | VAh | Total VA-hours Imported | Total Apparent Energy Imported |
| `TotVAhImpPhA` | float32 | VAh | Total VA-hours Imported phase A |  |
| `TotVAhImpPhB` | float32 | VAh | Total VA-hours Imported phase B |  |
| `TotVAhImpPhC` | float32 | VAh | Total VA-hours Imported phase C |  |
| `TotVArhImpQ1` | float32 | varh | Total VAR-hours Imported Q1 | Total Reactive Energy Imported Quadrant 1 |
| `TotVArhImpQ1phA` | float32 | varh | Total VAr-hours Imported Q1 phase A |  |
| `TotVArhImpQ1phB` | float32 | varh | Total VAr-hours Imported Q1 phase B |  |
| `TotVArhImpQ1phC` | float32 | varh | Total VAr-hours Imported Q1 phase C |  |
| `TotVArhImpQ2` | float32 | varh | Total VAr-hours Imported Q2 | Total Reactive Power Imported Quadrant 2 |
| `TotVArhImpQ2phA` | float32 | varh | Total VAr-hours Imported Q2 phase A |  |
| `TotVArhImpQ2phB` | float32 | varh | Total VAr-hours Imported Q2 phase B |  |
| `TotVArhImpQ2phC` | float32 | varh | Total VAr-hours Imported Q2 phase C |  |
| `TotVArhExpQ3` | float32 | varh | Total VAr-hours Exported Q3 | Total Reactive Power Exported Quadrant 3 |
| `TotVArhExpQ3phA` | float32 | varh | Total VAr-hours Exported Q3 phase A |  |
| `TotVArhExpQ3phB` | float32 | varh | Total VAr-hours Exported Q3 phase B |  |
| `TotVArhExpQ3phC` | float32 | varh | Total VAr-hours Exported Q3 phase C |  |
| `TotVArhExpQ4` | float32 | varh | Total VAr-hours Exported Q4 | Total Reactive Power Exported Quadrant 4 |
| `TotVArhExpQ4phA` | float32 | varh | Total VAr-hours Exported Q4 Imported phase A |  |
| `TotVArhExpQ4phB` | float32 | varh | Total VAr-hours Exported Q4 Imported phase B |  |
| `TotVArhExpQ4phC` | float32 | varh | Total VAr-hours Exported Q4 Imported phase C |  |
| `Evt` | bitfield32 |  | Events | Meter Event Flags |

Value names:

- `Evt` (bitfield32): bit 2 `M_EVENT_Power_Failure`, bit 3 `M_EVENT_Under_Voltage`, bit 4 `M_EVENT_Low_PF`, bit 5 `M_EVENT_Over_Current`, bit 6 `M_EVENT_Over_Voltage`, bit 7 `M_EVENT_Missing_Sensor`, bit 8 `M_EVENT_Reserved1`, bit 9 `M_EVENT_Reserved2`, bit 10 `M_EVENT_Reserved3`, bit 11 `M_EVENT_Reserved4`, bit 12 `M_EVENT_Reserved5`, bit 13 `M_EVENT_Reserved6`, bit 14 `M_EVENT_Reserved7`, bit 15 `M_EVENT_Reserved8`, bit 16 `M_EVENT_OEM01`, bit 17 `M_EVENT_OEM02`, bit 18 `M_EVENT_OEM03`, bit 19 `M_EVENT_OEM04`, bit 20 `M_EVENT_OEM05`, bit 21 `M_EVENT_OEM06`, bit 22 `M_EVENT_OEM07`, bit 23 `M_EVENT_OEM08`, bit 24 `M_EVENT_OEM09`, bit 25 `M_EVENT_OEM10`, bit 26 `M_EVENT_OEM11`, bit 27 `M_EVENT_OEM12`, bit 28 `M_EVENT_OEM13`, bit 29 `M_EVENT_OEM14`, bit 30 `M_EVENT_OEM15`

## Model 213: wye-connect three phase (abcn) meter

Name in records: `ac_meter_abcn_float`.

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `A` | float32 | A | Amps | Total AC Current |
| `AphA` | float32 | A | Amps PhaseA | Phase A Current |
| `AphB` | float32 | A | Amps PhaseB | Phase B Current |
| `AphC` | float32 | A | Amps PhaseC | Phase C Current |
| `PhV` | float32 | V | Voltage LN | Line to Neutral AC Voltage (average of active phases) |
| `PhVphA` | float32 | V | Phase Voltage AN | Phase Voltage AN |
| `PhVphB` | float32 | V | Phase Voltage BN | Phase Voltage BN |
| `PhVphC` | float32 | V | Phase Voltage CN | Phase Voltage CN |
| `PPV` | float32 | V | Voltage LL | Line to Line AC Voltage (average of active phases) |
| `PPVphAB` | float32 | V | Phase Voltage AB | Phase Voltage AB |
| `PPVphBC` | float32 | V | Phase Voltage BC | Phase Voltage BC |
| `PPVphCA` | float32 | V | Phase Voltage CA | Phase Voltage CA |
| `Hz` | float32 | Hz | Hz | Frequency |
| `W` | float32 | W | Watts | Total Real Power |
| `WphA` | float32 | W | Watts phase A |  |
| `WphB` | float32 | W | Watts phase B |  |
| `WphC` | float32 | W | Watts phase C |  |
| `VA` | float32 | VA | VA | AC Apparent Power |
| `VAphA` | float32 | VA | VA phase A |  |
| `VAphB` | float32 | VA | VA phase B |  |
| `VAphC` | float32 | VA | VA phase C |  |
| `VAR` | float32 | var | VAR | Reactive Power |
| `VARphA` | float32 | var | VAR phase A |  |
| `VARphB` | float32 | var | VAR phase B |  |
| `VARphC` | float32 | var | VAR phase C |  |
| `PF` | float32 | PF | PF | Power Factor |
| `PFphA` | float32 | PF | PF phase A |  |
| `PFphB` | float32 | PF | PF phase B |  |
| `PFphC` | float32 | PF | PF phase C |  |
| `TotWhExp` | float32 | Wh | Total Watt-hours Exported | Total Real Energy Exported |
| `TotWhExpPhA` | float32 | Wh | Total Watt-hours Exported phase A |  |
| `TotWhExpPhB` | float32 | Wh | Total Watt-hours Exported phase B |  |
| `TotWhExpPhC` | float32 | Wh | Total Watt-hours Exported phase C |  |
| `TotWhImp` | float32 | Wh | Total Watt-hours Imported | Total Real Energy Imported |
| `TotWhImpPhA` | float32 | Wh | Total Watt-hours Imported phase A |  |
| `TotWhImpPhB` | float32 | Wh | Total Watt-hours Imported phase B |  |
| `TotWhImpPhC` | float32 | Wh | Total Watt-hours Imported phase C |  |
| `TotVAhExp` | float32 | VAh | Total VA-hours Exported | Total Apparent Energy Exported |
| `TotVAhExpPhA` | float32 | VAh | Total VA-hours Exported phase A |  |
| `TotVAhExpPhB` | float32 | VAh | Total VA-hours Exported phase B |  |
| `TotVAhExpPhC` | float32 | VAh | Total VA-hours Exported phase C |  |
| `TotVAhImp` | float32 | VAh | Total VA-hours Imported | Total Apparent Energy Imported |
| `TotVAhImpPhA` | float32 | VAh | Total VA-hours Imported phase A |  |
| `TotVAhImpPhB` | float32 | VAh | Total VA-hours Imported phase B |  |
| `TotVAhImpPhC` | float32 | VAh | Total VA-hours Imported phase C |  |
| `TotVArhImpQ1` | float32 | varh | Total VAR-hours Imported Q1 | Total Reactive Energy Imported Quadrant 1 |
| `TotVArhImpQ1phA` | float32 | varh | Total VAr-hours Imported Q1 phase A |  |
| `TotVArhImpQ1phB` | float32 | varh | Total VAr-hours Imported Q1 phase B |  |
| `TotVArhImpQ1phC` | float32 | varh | Total VAr-hours Imported Q1 phase C |  |
| `TotVArhImpQ2` | float32 | varh | Total VAr-hours Imported Q2 | Total Reactive Power Imported Quadrant 2 |
| `TotVArhImpQ2phA` | float32 | varh | Total VAr-hours Imported Q2 phase A |  |
| `TotVArhImpQ2phB` | float32 | varh | Total VAr-hours Imported Q2 phase B |  |
| `TotVArhImpQ2phC` | float32 | varh | Total VAr-hours Imported Q2 phase C |  |
| `TotVArhExpQ3` | float32 | varh | Total VAr-hours Exported Q3 | Total Reactive Power Exported Quadrant 3 |
| `TotVArhExpQ3phA` | float32 | varh | Total VAr-hours Exported Q3 phase A |  |
| `TotVArhExpQ3phB` | float32 | varh | Total VAr-hours Exported Q3 phase B |  |
| `TotVArhExpQ3phC` | float32 | varh | Total VAr-hours Exported Q3 phase C |  |
| `TotVArhExpQ4` | float32 | varh | Total VAr-hours Exported Q4 | Total Reactive Power Exported Quadrant 4 |
| `TotVArhExpQ4phA` | float32 | varh | Total VAr-hours Exported Q4 Imported phase A |  |
| `TotVArhExpQ4phB` | float32 | varh | Total VAr-hours Exported Q4 Imported phase B |  |
| `TotVArhExpQ4phC` | float32 | varh | Total VAr-hours Exported Q4 Imported phase C |  |
| `Evt` | bitfield32 |  | Events | Meter Event Flags |

Value names:

- `Evt` (bitfield32): bit 2 `M_EVENT_Power_Failure`, bit 3 `M_EVENT_Under_Voltage`, bit 4 `M_EVENT_Low_PF`, bit 5 `M_EVENT_Over_Current`, bit 6 `M_EVENT_Over_Voltage`, bit 7 `M_EVENT_Missing_Sensor`, bit 8 `M_EVENT_Reserved1`, bit 9 `M_EVENT_Reserved2`, bit 10 `M_EVENT_Reserved3`, bit 11 `M_EVENT_Reserved4`, bit 12 `M_EVENT_Reserved5`, bit 13 `M_EVENT_Reserved6`, bit 14 `M_EVENT_Reserved7`, bit 15 `M_EVENT_Reserved8`, bit 16 `M_EVENT_OEM01`, bit 17 `M_EVENT_OEM02`, bit 18 `M_EVENT_OEM03`, bit 19 `M_EVENT_OEM04`, bit 20 `M_EVENT_OEM05`, bit 21 `M_EVENT_OEM06`, bit 22 `M_EVENT_OEM07`, bit 23 `M_EVENT_OEM08`, bit 24 `M_EVENT_OEM09`, bit 25 `M_EVENT_OEM10`, bit 26 `M_EVENT_OEM11`, bit 27 `M_EVENT_OEM12`, bit 28 `M_EVENT_OEM13`, bit 29 `M_EVENT_OEM14`, bit 30 `M_EVENT_OEM15`

## Model 214: delta-connect three phase (abc) meter

Name in records: `ac_meter_abc_float`.

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `A` | float32 | A | Amps | Total AC Current |
| `AphA` | float32 | A | Amps PhaseA | Phase A Current |
| `AphB` | float32 | A | Amps PhaseB | Phase B Current |
| `AphC` | float32 | A | Amps PhaseC | Phase C Current |
| `PhV` | float32 | V | Voltage LN | Line to Neutral AC Voltage (average of active phases) |
| `PhVphA` | float32 | V | Phase Voltage AN | Phase Voltage AN |
| `PhVphB` | float32 | V | Phase Voltage BN | Phase Voltage BN |
| `PhVphC` | float32 | V | Phase Voltage CN | Phase Voltage CN |
| `PPV` | float32 | V | Voltage LL | Line to Line AC Voltage (average of active phases) |
| `PPVphAB` | float32 | V | Phase Voltage AB | Phase Voltage AB |
| `PPVphBC` | float32 | V | Phase Voltage BC | Phase Voltage BC |
| `PPVphCA` | float32 | V | Phase Voltage CA | Phase Voltage CA |
| `Hz` | float32 | Hz | Hz | Frequency |
| `W` | float32 | W | Watts | Total Real Power |
| `WphA` | float32 | W | Watts phase A |  |
| `WphB` | float32 | W | Watts phase B |  |
| `WphC` | float32 | W | Watts phase C |  |
| `VA` | float32 | VA | VA | AC Apparent Power |
| `VAphA` | float32 | VA | VA phase A |  |
| `VAphB` | float32 | VA | VA phase B |  |
| `VAphC` | float32 | VA | VA phase C |  |
| `VAR` | float32 | var | VAR | Reactive Power |
| `VARphA` | float32 | var | VAR phase A |  |
| `VARphB` | float32 | var | VAR phase B |  |
| `VARphC` | float32 | var | VAR phase C |  |
| `PF` | float32 | PF | PF | Power Factor |
| `PFphA` | float32 | PF | PF phase A |  |
| `PFphB` | float32 | PF | PF phase B |  |
| `PFphC` | float32 | PF | PF phase C |  |
| `TotWhExp` | float32 | Wh | Total Watt-hours Exported | Total Real Energy Exported |
| `TotWhExpPhA` | float32 | Wh | Total Watt-hours Exported phase A |  |
| `TotWhExpPhB` | float32 | Wh | Total Watt-hours Exported phase B |  |
| `TotWhExpPhC` | float32 | Wh | Total Watt-hours Exported phase C |  |
| `TotWhImp` | float32 | Wh | Total Watt-hours Imported | Total Real Energy Imported |
| `TotWhImpPhA` | float32 | Wh | Total Watt-hours Imported phase A |  |
| `TotWhImpPhB` | float32 | Wh | Total Watt-hours Imported phase B |  |
| `TotWhImpPhC` | float32 | Wh | Total Watt-hours Imported phase C |  |
| `TotVAhExp` | float32 | VAh | Total VA-hours Exported | Total Apparent Energy Exported |
| `TotVAhExpPhA` | float32 | VAh | Total VA-hours Exported phase A |  |
| `TotVAhExpPhB` | float32 | VAh | Total VA-hours Exported phase B |  |
| `TotVAhExpPhC` | float32 | VAh | Total VA-hours Exported phase C |  |
| `TotVAhImp` | float32 | VAh | Total VA-hours Imported | Total Apparent Energy Imported |
| `TotVAhImpPhA` | float32 | VAh | Total VA-hours Imported phase A |  |
| `TotVAhImpPhB` | float32 | VAh | Total VA-hours Imported phase B |  |
| `TotVAhImpPhC` | float32 | VAh | Total VA-hours Imported phase C |  |
| `TotVArhImpQ1` | float32 | varh | Total VAR-hours Imported Q1 | Total Reactive Energy Imported Quadrant 1 |
| `TotVArhImpQ1phA` | float32 | varh | Total VAr-hours Imported Q1 phase A |  |
| `TotVArhImpQ1phB` | float32 | varh | Total VAr-hours Imported Q1 phase B |  |
| `TotVArhImpQ1phC` | float32 | varh | Total VAr-hours Imported Q1 phase C |  |
| `TotVArhImpQ2` | float32 | varh | Total VAr-hours Imported Q2 | Total Reactive Power Imported Quadrant 2 |
| `TotVArhImpQ2phA` | float32 | varh | Total VAr-hours Imported Q2 phase A |  |
| `TotVArhImpQ2phB` | float32 | varh | Total VAr-hours Imported Q2 phase B |  |
| `TotVArhImpQ2phC` | float32 | varh | Total VAr-hours Imported Q2 phase C |  |
| `TotVArhExpQ3` | float32 | varh | Total VAr-hours Exported Q3 | Total Reactive Power Exported Quadrant 3 |
| `TotVArhExpQ3phA` | float32 | varh | Total VAr-hours Exported Q3 phase A |  |
| `TotVArhExpQ3phB` | float32 | varh | Total VAr-hours Exported Q3 phase B |  |
| `TotVArhExpQ3phC` | float32 | varh | Total VAr-hours Exported Q3 phase C |  |
| `TotVArhExpQ4` | float32 | varh | Total VAr-hours Exported Q4 | Total Reactive Power Exported Quadrant 4 |
| `TotVArhExpQ4phA` | float32 | varh | Total VAr-hours Exported Q4 Imported phase A |  |
| `TotVArhExpQ4phB` | float32 | varh | Total VAr-hours Exported Q4 Imported phase B |  |
| `TotVArhExpQ4phC` | float32 | varh | Total VAr-hours Exported Q4 Imported phase C |  |
| `Evt` | bitfield32 |  | Events | Meter Event Flags |

Value names:

- `Evt` (bitfield32): bit 2 `M_EVENT_Power_Failure`, bit 3 `M_EVENT_Under_Voltage`, bit 4 `M_EVENT_Low_PF`, bit 5 `M_EVENT_Over_Current`, bit 6 `M_EVENT_Over_Voltage`, bit 7 `M_EVENT_Missing_Sensor`, bit 8 `M_EVENT_Reserved1`, bit 9 `M_EVENT_Reserved2`, bit 10 `M_EVENT_Reserved3`, bit 11 `M_EVENT_Reserved4`, bit 12 `M_EVENT_Reserved5`, bit 13 `M_EVENT_Reserved6`, bit 14 `M_EVENT_Reserved7`, bit 15 `M_EVENT_Reserved8`, bit 16 `M_EVENT_OEM01`, bit 17 `M_EVENT_OEM02`, bit 18 `M_EVENT_OEM03`, bit 19 `M_EVENT_OEM04`, bit 20 `M_EVENT_OEM05`, bit 21 `M_EVENT_OEM06`, bit 22 `M_EVENT_OEM07`, bit 23 `M_EVENT_OEM08`, bit 24 `M_EVENT_OEM09`, bit 25 `M_EVENT_OEM10`, bit 26 `M_EVENT_OEM11`, bit 27 `M_EVENT_OEM12`, bit 28 `M_EVENT_OEM13`, bit 29 `M_EVENT_OEM14`, bit 30 `M_EVENT_OEM15`

## Model 220: Secure AC Meter Selected Readings

Name in records: `ac_meter_secure`.

Include this model for secure metering

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `A` | int16 | A | Amps | Total AC Current |
| `PhV` | int16 | V | Voltage | Average phase or line voltage |
| `Hz` | int16 | Hz | Hz | Frequency |
| `W` | int16 | W | Watts | Total Real Power |
| `VA` | int16 | VA | VA | AC Apparent Power |
| `VAR` | int16 | var | VAR | Reactive Power |
| `PF` | int16 | Pct | PF | Power Factor |
| `TotWhExp` | acc32 | Wh | Total Watt-hours Exported | Total Real Energy Exported |
| `TotWhImp` | acc32 | Wh | Total Watt-hours Imported | Total Real Energy Imported |
| `TotVAhExp` | acc32 | VAh | Total VA-hours Exported | Total Apparent Energy Exported |
| `TotVAhImp` | acc32 | VAh | Total VA-hours Imported | Total Apparent Energy Imported |
| `TotVArhImpQ1` | acc32 | varh | Total VAR-hours Imported Q1 | Total Reactive Energy Imported Quadrant 1 |
| `TotVArhImpQ2` | acc32 | varh | Total VAr-hours Imported Q2 | Total Reactive Power Imported Quadrant 2 |
| `TotVArhExpQ3` | acc32 | varh | Total VAr-hours Exported Q3 | Total Reactive Power Exported Quadrant 3 |
| `TotVArhExpQ4` | acc32 | varh | Total VAr-hours Exported Q4 | Total Reactive Power Exported Quadrant 4 |
| `Evt` | bitfield32 |  | Events | Meter Event Flags |
| `Ts` | uint32 |  | Timestamp | Timestamp value is the number of seconds since January 1, 2000 |
| `Ms` | uint16 |  | Milliseconds | Millisecond counter 0-999 |
| `Seq` | uint16 |  | Sequence | Sequence number of request |
| `Alg` | enum16 |  | Algorithm | Algorithm used to compute the digital signature |
| `N` | uint16 |  | N | Number of registers comprising the digital signature. |
| `repeating[*].DS` | uint16 |  |  |  |

Value names:

- `Evt` (bitfield32): bit 2 `Power_Failure`, bit 3 `Under_Voltage`, bit 4 `Low_PF`, bit 5 `Over_Current`, bit 6 `Over_Voltage`, bit 7 `Missing_Sensor`, bit 16 `OEM01`, bit 17 `OEM02`, bit 18 `OEM03`, bit 19 `OEM04`, bit 20 `OEM05`, bit 21 `OEM06`, bit 22 `OEM07`, bit 23 `OEM08`, bit 24 `OEM09`, bit 25 `OEM10`, bit 26 `OEM11`, bit 27 `OEM12`, bit 28 `OEM13`, bit 29 `OEM14`, bit 30 `OEM15`
- `Alg` (enum16): 0 `NONE`, 1 `AES-GMAC-64`, 2 `ECC-256`
