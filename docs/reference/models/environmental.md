# Environmental sensors

<!-- Generated from the bundled SunSpec definitions by
     UPDATE_MODEL_DOCS=1 cargo test -p sunspec-parser --test models_docs
     Do not edit by hand. -->

Point names are the ones records use; `[*]` stands for a repeating group's index. See [Supported SunSpec models](README.md) for every group.

| Model | Name in records | Points |
|-------|-----------------|--------|
| [302](#model-302-irradiance-model) | `irradiance` | 5 |
| [303](#model-303-back-of-module-temperature-model) | `bom_temp` | 1 |
| [304](#model-304-inclinometer-model) | `inclinometer` | 3 |
| [305](#model-305-gps) | `location` | 6 |
| [306](#model-306-reference-point-model) | `ref_point` | 4 |
| [307](#model-307-base-met) | `base_met` | 11 |
| [308](#model-308-mini-met-model) | `mini_met` | 4 |

## Model 302: Irradiance Model

Name in records: `irradiance`.

Include to support various irradiance measurements

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `repeating[*].GHI` | uint16 | W/m2 | GHI | Global Horizontal Irradiance |
| `repeating[*].POAI` | uint16 | W/m2 | POAI | Plane-of-Array Irradiance |
| `repeating[*].DFI` | uint16 | W/m2 | DFI | Diffuse Irradiance |
| `repeating[*].DNI` | uint16 | W/m2 | DNI | Direct Normal Irradiance |
| `repeating[*].OTI` | uint16 | W/m2 | OTI | Other Irradiance |

## Model 303: Back of Module Temperature Model

Name in records: `bom_temp`.

Include to support variable number of back of module temperature measurements

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `temp[*].TmpBOM` | int16 | C | Temp | Back of module temperature measurement |

## Model 304: Inclinometer Model

Name in records: `inclinometer`.

Include to support orientation measurements

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `incl[*].Inclx` | int32 | Degrees | X | X-Axis inclination |
| `incl[*].Incly` | int32 | Degrees | Y | Y-Axis inclination |
| `incl[*].Inclz` | int32 | Degrees | Z | Z-Axis inclination |

## Model 305: GPS

Name in records: `location`.

Include to support location measurements

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `Tm` | string | hhmmss.sssZ | Tm | UTC 24 hour time stamp to millisecond hhmmss.sssZ format |
| `Date` | string | YYYYMMDD | Date | UTC Date string YYYYMMDD format |
| `Loc` | string | text | Location | Location string (40 chars max) |
| `Lat` | int32 | Degrees | Lat | Latitude with seven degrees of precision |
| `Long` | int32 | Degrees | Long | Longitude with seven degrees of precision |
| `Alt` | int32 | meters | Altitude | Altitude measurement in meters |

## Model 306: Reference Point Model

Name in records: `ref_point`.

Include to support a standard reference point

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `GHI` | uint16 | W/m2 | GHI | Global Horizontal Irradiance |
| `A` | uint16 | W/m2 | Amps | Current measurement at reference point |
| `V` | uint16 | W/m2 | Voltage | Voltage measurement at reference point |
| `Tmp` | uint16 | W/m2 | Temperature | Temperature measurement at reference point |

## Model 307: Base Met

Name in records: `base_met`.

Base Meteorological Model

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `TmpAmb` | int16 | C | Ambient Temperature |  |
| `RH` | int16 | Pct | Relative Humidity |  |
| `Pres` | int16 | HPa | Barometric Pressure |  |
| `WndSpd` | int16 | mps | Wind Speed |  |
| `WndDir` | int16 | deg | Wind Direction |  |
| `Rain` | int16 | mm | Rainfall |  |
| `Snw` | int16 | mm | Snow Depth |  |
| `PPT` | int16 |  | Precipitation Type | Precipitation Type (WMO 4680 SYNOP code reference) |
| `ElecFld` | int16 | Vm | Electric Field |  |
| `SurWet` | int16 | kO | Surface Wetness |  |
| `SoilWet` | int16 | Pct | Soil Wetness |  |

## Model 308: Mini Met Model

Name in records: `mini_met`.

Include to support a few basic measurements

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `GHI` | uint16 | W/m2 | GHI | Global Horizontal Irradiance |
| `TmpBOM` | int16 | C | Temp | Back of module temperature measurement |
| `TmpAmb` | int16 | C | Ambient Temperature |  |
| `WndSpd` | uint16 | m/s | Wind Speed |  |
