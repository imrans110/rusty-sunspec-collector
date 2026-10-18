# Trackers

<!-- Generated from the bundled SunSpec definitions by
     UPDATE_MODEL_DOCS=1 cargo test -p sunspec-parser --test models_docs
     Do not edit by hand. -->

Point names are the ones records use; `[*]` stands for a repeating group's index. See [Supported SunSpec models](README.md) for every group.

| Model | Name in records | Points |
|-------|-----------------|--------|
| [601](#model-601-tracker-controller-draft-2) | `tracker_controller` | 19 |

## Model 601: Tracker Controller DRAFT 2

Name in records: `tracker_controller`.

Monitors and controls multiple trackers

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `Nam` | string |  | Controller | Descriptive name for this control unit |
| `Typ` | enum16 |  | Type | Type of tracker |
| `DtLoc` | string | YYYYMMDD | Date | Local date in YYYYMMDD format |
| `TmLoc` | string | hhmmss | Time | 24 hour local time stamp to second |
| `Day` | uint16 |  | Day | Number of the day in the year (1-366) |
| `GlblElCtl` | int32 | Degrees | Manual Elevation | Global manual override target position of elevation in degrees from horizontal. Unimplemented for single axis azimuth tracker type |
| `GlblAzCtl` | int32 | Degrees | Manual Azimuth | Global manual override target position of azimuth in degrees from true north towards east. Unimplemented for single axis azimuth tracker type |
| `GlblCtl` | enum16 |  | Global Mode | Global Control register operates on all trackers. Normal operation is automatic. Operator can override the position by setting the ElCtl, AzCtl and enabling Manual operation. Entering calibration mode will revert to automatic operation after calibration is complete. |
| `GlblAlm` | bitfield16 |  | Global Alarm | Global tracker alarm conditions |
| `N` | uint16 |  | Trackers | Number of trackers being controlled. Size of repeating block. |
| `tracker[*].Id` | string |  | Tracker | Descriptive name for this tracker unit |
| `tracker[*].ElTrgt` | int32 | Degrees | Target Elevation | Auto target elevation in degrees from horizontal. Unimplemented for single axis azimuth tracker type |
| `tracker[*].AzTrgt` | int32 | Degrees | Target Azimuth | Auto target azimuth in degrees from true north towards east. Unimplemented for single axis horizontal tracker type |
| `tracker[*].ElPos` | int32 | Degrees | Elevation | Actual elevation position in degrees from horizontal. Unimplemented for single axis azimuth tracker type |
| `tracker[*].AzPos` | int32 | Degrees | Azimuth | Actual azimuth position in degrees from true north towards east. Unimplemented for single axis horizontal tracker type |
| `tracker[*].ElCtl` | int32 | Degrees | Manual Elevation | Manual override target position of elevation in degrees from horizontal. Unimplemented for single axis azimuth tracker type |
| `tracker[*].AzCtl` | int32 | Degrees | Manual Azimuth | Manual override target position of azimuth in degrees from true north towards east. Unimplemented for single axis azimuth tracker type |
| `tracker[*].Ctl` | enum16 |  | Mode | Control register. Normal operation is automatic. Operator can override the position by setting the ElCtl, AzCtl and enabling Manual operation. Entering calibration mode will revert to automatic operation after calibration is complete. |
| `tracker[*].Alm` | bitfield16 |  | Alarm | Tracker alarm conditions |

Value names:

- `Typ` (enum16): 0 `Unknown`, 1 `Fixed`, 2 `Horizontal`, 3 `Tilted`, 4 `Azimuth`, 5 `Dual`, 99 `Other`
- `GlblCtl` (enum16): 0 `Automatic`, 1 `Manual`, 2 `Calibrate`
- `GlblAlm` (bitfield16): bit 0 `SetPoint`, bit 1 `ObsEl`, bit 2 `ObsAz`
- `tracker[*].Ctl` (enum16): 0 `Automatic`, 1 `Manual`, 2 `Calibrate`
- `tracker[*].Alm` (bitfield16): bit 0 `SetPoint`, bit 1 `ObsEl`, bit 2 `ObsAz`
