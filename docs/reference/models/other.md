# Test and vendor models

<!-- Generated from the bundled SunSpec definitions by
     UPDATE_MODEL_DOCS=1 cargo test -p sunspec-parser --test models_docs
     Do not edit by hand. -->

Point names are the ones records use; `[*]` stands for a repeating group's index. See [Supported SunSpec models](README.md) for every group.

| Model | Name in records | Points |
|-------|-----------------|--------|
| [63001](#model-63001-sunspec-test-model-1) | `model_63001` | 59 |
| [63002](#model-63002-sunspec-test-model-2) | `model_63002` | 2 |
| [64001](#model-64001-veris-status-and-configuration) | `model_64001` | 35 |
| [64020](#model-64020-mersen-greenstring) | `model_64020` | 30 |
| [64101](#model-64101-eltek-inverter-extension) | `model_64101` | 7 |
| [64111](#model-64111-basic-charge-controller) | `model_64111` | 18 |
| [64112](#model-64112-outback-fm-charge-controller) | `model_64112` | 58 |
| [64410](#model-64410-dc-simulator-control-interface) | `DCSimInterface` | 32 |
| [64411](#model-64411-ac-simulator-control-interface) | `ACSimInterface` | 51 |
| [64412](#model-64412-der-cyber-exploitation) | `DERExploitation` | 43 |
| [64413](#model-64413-pv-curves) | `PVSimCurves` | 5 |
| [64414](#model-64414-der-simulation-controls) | `DERSimControls` | 9 |
| [64415](#model-64415-csip-client-control) | `CSIPControl` | 5 |

## Model 63001: SunSpec Test Model 1

Name in records: `model_63001`.

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `int16_1` | int16 |  |  |  |
| `int16_2` | int16 |  |  |  |
| `int16_3` | int16 |  |  |  |
| `int16_4` | int16 |  |  |  |
| `int16_5` | int16 |  |  |  |
| `int16_u` | int16 |  |  |  |
| `uint16_1` | uint16 |  |  |  |
| `uint16_2` | uint16 |  |  |  |
| `uint16_3` | uint16 |  |  |  |
| `uint16_4` | uint16 |  |  |  |
| `uint16_5` | uint16 |  |  |  |
| `uint16_u` | uint16 |  |  |  |
| `acc16` | acc16 |  |  |  |
| `acc16_u` | acc16 |  |  |  |
| `enum16` | enum16 |  |  |  |
| `enum16_u` | enum16 |  |  |  |
| `bitfield16` | bitfield16 |  |  |  |
| `bitfield16_u` | bitfield16 |  |  |  |
| `int32_1` | int32 |  |  |  |
| `int32_2` | int32 |  |  |  |
| `int32_3` | int32 |  |  |  |
| `int32_4` | int32 |  |  |  |
| `int32_5` | int32 |  |  |  |
| `int32_u` | int32 |  |  |  |
| `uint32_1` | uint32 |  |  |  |
| `uint32_2` | uint32 |  |  |  |
| `uint32_3` | uint32 |  |  |  |
| `uint32_4` | uint32 |  |  |  |
| `uint32_5` | uint32 |  |  |  |
| `uint32_u` | uint32 |  |  |  |
| `acc32` | acc32 |  |  |  |
| `acc32_u` | acc32 |  |  |  |
| `enum32` | enum32 |  |  |  |
| `enum32_u` | enum32 |  |  |  |
| `bitfield32` | bitfield32 |  |  |  |
| `bitfield32_u` | bitfield32 |  |  |  |
| `ipaddr` | ipaddr |  |  |  |
| `ipaddr_u` | ipaddr |  |  |  |
| `int64` | int64 |  |  |  |
| `int64_u` | int64 |  |  |  |
| `acc64` | acc64 |  |  |  |
| `acc64_u` | acc64 |  |  |  |
| `ipv6addr` | ipv6addr |  |  |  |
| `ipv6addr_u` | ipv6addr |  |  |  |
| `float32` | float32 |  |  |  |
| `float32_u` | float32 |  |  |  |
| `string` | string |  |  |  |
| `string_u` | string |  |  |  |
| `repeating[*].int16_11` | int16 |  |  |  |
| `repeating[*].int16_12` | int16 |  |  |  |
| `repeating[*].int16_u` | int16 |  |  |  |
| `repeating[*].uint16_11` | uint16 |  |  |  |
| `repeating[*].uint16_12` | uint16 |  |  |  |
| `repeating[*].uint16_13` | uint16 |  |  |  |
| `repeating[*].uint16_u` | uint16 |  |  |  |
| `repeating[*].int32` | int32 |  |  |  |
| `repeating[*].int32_u` | int32 |  |  |  |
| `repeating[*].uint32` | uint32 |  |  |  |
| `repeating[*].uint32_u` | uint32 |  |  |  |

## Model 63002: SunSpec Test Model 2

Name in records: `model_63002`.

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `repeating[*].int16_1` | int16 |  |  |  |
| `repeating[*].int16_2` | int16 |  |  |  |

## Model 64001: Veris Status and Configuration

Name in records: `model_64001`.

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `Cmd` | enum16 |  | Command Code |  |
| `HWRev` | uint16 |  | Hardware Revision |  |
| `RSFWRev` | uint16 |  | RS FW Revision |  |
| `OSFWRev` | uint16 |  | OS FW Revision |  |
| `ProdRev` | string |  | Product Revision |  |
| `Boots` | uint16 |  | Boot Count |  |
| `Switch` | bitfield16 |  | DIP Switches |  |
| `Sensors` | uint16 |  | Num Detected Sensors |  |
| `Talking` | uint16 |  | Num Communicating Sensors |  |
| `Status` | bitfield16 |  | System Status |  |
| `Config` | bitfield16 |  | System Configuration |  |
| `LEDblink` | uint16 | Pct | LED Blink Threshold |  |
| `LEDon` | uint16 | Pct | LED On Threshold |  |
| `Reserved` | uint16 |  |  |  |
| `Loc` | string |  | Location String |  |
| `S1ID` | enum16 |  | Sensor 1 Unit ID |  |
| `S1Addr` | uint16 |  | Sensor 1 Address |  |
| `S1OSVer` | uint16 |  | Sensor 1 OS Version |  |
| `S1Ver` | string |  | Sensor 1 Product Version |  |
| `S1Serial` | string |  | Sensor 1 Serial Num |  |
| `S2ID` | enum16 |  | Sensor 2 Unit ID |  |
| `S2Addr` | uint16 |  | Sensor 2 Address |  |
| `S2OSVer` | uint16 |  | Sensor 2 OS Version |  |
| `S2Ver` | string |  | Sensor 2 Product Version |  |
| `S2Serial` | string |  | Sensor 2 Serial Num |  |
| `S3ID` | enum16 |  | Sensor 3 Unit ID |  |
| `S3Addr` | uint16 |  | Sensor 3 Address |  |
| `S3OSVer` | uint16 |  | Sensor 3 OS Version |  |
| `S3Ver` | string |  | Sensor 3 Product Version |  |
| `S3Serial` | string |  | Sensor 3 Serial Num |  |
| `S4ID` | enum16 |  | Sensor 4 Unit ID |  |
| `S4Addr` | uint16 |  | Sensor 4 Address |  |
| `S4OSVer` | uint16 |  | Sensor 4 OS Version |  |
| `S4Ver` | string |  | Sensor 4 Product Version |  |
| `S4Serial` | string |  | Sensor 4 Serial Num |  |

## Model 64020: Mersen GreenString

Name in records: `model_64020`.

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `Aux0Tmp` | int16 | C | Aux 0 temperature |  |
| `Aux1Tmp` | int16 | C | Aux 1 temperature |  |
| `Aux2Tmp` | int16 | C | Aux 2 temperature |  |
| `Aux3Tmp` | int16 | C | Aux 3 temperature |  |
| `Aux4Tmp` | int16 | C | Aux 4 temperature |  |
| `ProbeTmp` | int16 | C | Probe Temperature |  |
| `MainTmp` | int16 | C | Main Temperature |  |
| `Sensor1Voltage` | int16 | V | Sensor1 Voltage | scale of 0-10V |
| `Sensor2Voltage` | int16 | V | Sensor2 Voltage | scale of 0-10V |
| `Sensor3Voltage` | int16 | V | Sensor3 Voltage | scale of 0-10V |
| `Sensor4Voltage` | int16 | V | Sensor4 Voltage | scale of 0-10V |
| `Sensor5Voltage` | int16 | V | Sensor5 Voltage | scale of 0-10V |
| `Sensor6Voltage` | int16 | V | Sensor6 Voltage | scale of 0-10V |
| `Sensor7Voltage` | int16 | V | Sensor7 Voltage | scale of 0-10V |
| `Sensor1Current` | int16 | A | Sensor1 Current | scale of 4-20mA |
| `Sensor2Current` | int16 | A | Sensor2 Current | in 4-20mA or 4-20mA |
| `Sensor3Current` | int16 | A | Sensor3 Current | in 4-20mA or 4-20mA |
| `Sensor4Current` | int16 | A | Sensor4 Current | in 4-20mA or 4-20mA |
| `Sensor5Current` | int16 | A | Sensor5 Current | in 4-20mA or 4-20mA |
| `Sensor6Current` | int16 | A | Sensor6 Current | in 4-20mA or 4-20mA |
| `Sensor7Current` | int16 | A | Sensor7 Current | in 4-20mA or 4-20mA |
| `Sensor8` | uint16 | Hz | Sensor8 frequency | frequency in Hz |
| `Relay1` | uint16 |  | Relay 1 state |  |
| `Relay2` | uint16 |  | Relay 2 state |  |
| `Relay3` | uint16 |  | Relay 3 state |  |
| `ResetAccumulators` | uint16 |  | Reset the accumulators | always 0 in reading, used the code 0xC0DA during the writing for resetting them |
| `Reset` | uint16 |  | Reset the system | always 0 in reading, used the code 0xC0DA during the writing for resetting the system |
| `repeating[*].SerialNumber` | string |  | Serial number | strings of 16 characters |
| `repeating[*].Firmware` | string |  | Firmware version | string of 11 characters |
| `repeating[*].Hardware` | uint16 |  | Hardware version |  |

## Model 64101: Eltek Inverter Extension

Name in records: `model_64101`.

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `Eltek_Country_Code` | uint16 |  |  |  |
| `Eltek_Feeding_Phase` | uint16 |  |  |  |
| `Eltek_APD_Method` | uint16 |  |  |  |
| `Eltek_APD_Power_Ref` | uint16 |  |  |  |
| `Eltek_RPS_Method` | uint16 |  |  |  |
| `Eltek_RPS_Q_Ref` | uint16 |  |  |  |
| `Eltek_RPS_CosPhi_Ref` | int16 |  |  |  |

## Model 64111: Basic Charge Controller

Name in records: `model_64111`.

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `Port` | uint16 |  | Port Number |  |
| `BattV` | uint16 | V | Battery Voltage |  |
| `ArrayV` | uint16 | V | Array Voltage |  |
| `OutputA` | uint16 | A | Output Current |  |
| `InputA` | uint16 | A | Array Current |  |
| `ChargerSt` | enum16 |  | Operating State |  |
| `OutputW` | uint16 | W | Output Wattage |  |
| `TodayMinBatV` | uint16 | V | Today's Minimum Battery Voltage |  |
| `TodayMaxBatV` | uint16 | V | Today's Maximum Battery Voltage |  |
| `VOCV` | uint16 | V | VOC |  |
| `TodayMaxVOC` | uint16 | V | Today's Maximum VOC |  |
| `TodaykWhOutput` | uint16 | kWh | Today's kWh |  |
| `TodayAHOutput` | uint16 | AH | Today's AH |  |
| `LifeTimeKWHOut` | uint16 | kWh | Lifetime kWh |  |
| `LifeTimeAHOut` | uint16 | kAH | Lifetime kAH |  |
| `LifeTimeMaxOut` | uint16 | W | Lifetime Maximum Output Wattage |  |
| `LifeTimeMaxBatt` | uint16 | V | Lifetime Maximum Battery Voltage |  |
| `LifeTimeMaxVOC` | uint16 | V | Lifetime Maximum VOC Voltage |  |

Value names:

- `ChargerSt` (enum16): 0 `Off`, 1 `Float`, 2 `Bulk`, 3 `Absorb`, 4 `EQ`

## Model 64112: OutBack FM Charge Controller

Name in records: `model_64112`.

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `Port` | uint16 |  | Port Number |  |
| `CC_Config_fault` | bitfield16 |  | Faults |  |
| `CC_Config_absorb_V` | uint16 | V | Absorb |  |
| `CC_Config_absorb_Hr` | uint16 | Tmh | Absorb Time |  |
| `CC_Config_absorb_End_A` | uint16 | A | Absorb End |  |
| `CC_Config_rebulk_V` | uint16 | V | Rebulk |  |
| `CC_Config_float_V` | uint16 | V | Float |  |
| `CC_Config_max_Chg_A` | uint16 | A | Maximum Charge |  |
| `CC_Config_equalize_V` | uint16 | V | Equalize |  |
| `CC_Config_equalize_Hr` | uint16 | Tmh | Equalize Time |  |
| `CC_Config_auto_equalize` | uint16 | Tmd | Auto Equalize Interval |  |
| `CC_Config_MPPT_mode` | enum16 |  | MPPT mode |  |
| `CC_Config_sweep_width` | enum16 |  | Sweep Width |  |
| `CC_Config_sweep_max` | enum16 |  | Sweep Maximum |  |
| `CC_Config_U_Pick_Duty_cyc` | uint16 | Pct | U-Pick PWM Duty Cycle |  |
| `CC_Config_grid_tie` | enum16 |  | Grid Tie Mode |  |
| `CC_Config_temp_comp` | enum16 |  | Temp Comp Mode |  |
| `CC_Config_temp_comp_llimt` | uint16 | V | Temp Comp Lower Limit |  |
| `CC_Config_temp_comp_hlimt` | uint16 | V | Temp Comp Upper Limit |  |
| `CC_Config_auto_restart` | enum16 |  | Auto Restart Mode |  |
| `CC_Config_wakeup_VOC` | uint16 | V | Wakeup VOC Change |  |
| `CC_Config_snooze_mode_A` | uint16 | A | Snooze Mode |  |
| `CC_Config_wakeup_interval` | uint16 | Tms | Wakeup Interval |  |
| `CC_Config_AUX_mode` | enum16 |  | AUX Output Mode |  |
| `CC_Config_AUX_control` | enum16 |  | AUX Output Control |  |
| `CC_Config_AUX_state` | enum16 |  | AUX Output State |  |
| `CC_Config_AUX_polarity` | enum16 |  | AUX Output Polarity |  |
| `CC_Config_AUX_L_Batt_disc` | uint16 | V | AUX Low Battery Disconnect |  |
| `CC_Config_AUX_L_Batt_rcon` | uint16 | V | AUX Low Battery Reconnect |  |
| `CC_Config_AUX_L_Batt_dly` | uint16 | Tms | AUX Low Battery Disconnect Delay |  |
| `CC_Config_AUX_Vent_fan_V` | uint16 | V | AUX Vent Fan |  |
| `CC_Config_AUX_PV_triggerV` | uint16 | V | AUX PV Trigger |  |
| `CC_Config_AUX_PV_trg_h_tm` | uint16 | Tms | AUX PV Trigger Hold Time |  |
| `CC_Config_AUX_Nlite_ThrsV` | uint16 | V | AUX Night Light Threshold |  |
| `CC_Config_AUX_Nlite_On_tm` | uint16 | Tmh | AUX Night Light On Time |  |
| `CC_Config_AUX_Nlite_On_hist` | uint16 | Tms | AUX Night Light On Hysteresis |  |
| `CC_Config_AUX_Nlite_Off_hist` | uint16 | Tms | AUX Night Light Off Hysteresis |  |
| `CC_Config_AUX_Error_batt_V` | uint16 | V | AUX Error Output Low Battery |  |
| `CC_Config_AUX_Divert_h_time` | uint16 | Tms | AUX Divert Hold Time |  |
| `CC_Config_AUX_Divert_dly_time` | uint16 | Tms | AUX Divert Delay Time |  |
| `CC_Config_AUX_Divert_Rel_V` | uint16 | V | AUX Divert Relative |  |
| `CC_Config_AUX_Divert_Hyst_V` | uint16 | V | AUX Divert Hysteresis |  |
| `CC_Config_MajorFWRev` | uint16 |  | FM CC Major Firmware Number |  |
| `CC_Config_MidFWRev` | uint16 |  | FM CC Mid Firmware Number |  |
| `CC_Config_MinorFWRev` | uint16 |  | FM CC Minor Firmware Number |  |
| `CC_Config_DataLog_Day_offset` | uint16 | Tmd | Set Data Log Day Offset |  |
| `CC_Config_DataLog_Cur_Day_off` | uint16 | Tmd | Current Data Log Day Offset |  |
| `CC_Config_DataLog_Daily_AH` | uint16 | Ah | Data Log Daily (Ah) |  |
| `CC_Config_DataLog_Daily_KWH` | uint16 | kWh | Data Log Daily (kWh) |  |
| `CC_Config_DataLog_Max_Out_A` | uint16 | A | Data Log Daily Maximum Output (A) |  |
| `CC_Config_DataLog_Max_Out_W` | uint16 | W | Data Log Daily Maximum Output (W) |  |
| `CC_Config_DataLog_Absorb_T` | uint16 | Tms | Data Log Daily Absorb Time |  |
| `CC_Config_DataLog_Float_T` | uint16 | Tms | Data Log Daily Float Time |  |
| `CC_Config_DataLog_Min_Batt_V` | uint16 | V | Data Log Daily Minimum Battery |  |
| `CC_Config_DataLog_Max_Batt_V` | uint16 | V | Data Log Daily Maximum Battery |  |
| `CC_Config_DataLog_Max_Input_V` | uint16 | V | Data Log Daily Maximum Input |  |
| `CC_Config_DataLog_Clear` | uint16 |  | Data Log Clear |  |
| `CC_Config_DataLog_Clr_Comp` | uint16 |  | Data Log Clear Complement |  |

Value names:

- `CC_Config_MPPT_mode` (enum16): 0 `Auto`, 1 `U_Pick`, 2 `Wind`
- `CC_Config_sweep_width` (enum16): 0 `Half`, 1 `Full`
- `CC_Config_sweep_max` (enum16): 0 `Eighty_Percent`, 1 `Eighty_Five_Percent`, 2 `Ninty_Percent`, 3 `Ninty_Nine_Percent`
- `CC_Config_grid_tie` (enum16): 0 `Disabled`, 1 `Enabled`
- `CC_Config_temp_comp` (enum16): 0 `Wide`, 1 `Limited`
- `CC_Config_auto_restart` (enum16): 0 `Off`, 1 `Every_90_Minutes`, 2 `Every_90_Minutes_if_Absorb_or_Float`
- `CC_Config_AUX_mode` (enum16): 0 `Float`, 1 `Diversion_Relay`, 2 `Diversion_Solid_St`, 3 `Low_Batt_Disconnect`, 4 `Remote`, 5 `Vent_Fan`, 6 `PV_Trigger`, 7 `Error_Output`, 8 `Night_Light`
- `CC_Config_AUX_control` (enum16): 0 `Off`, 1 `Auto`, 2 `On`
- `CC_Config_AUX_state` (enum16): 0 `Disabled`, 1 `Enabled`
- `CC_Config_AUX_polarity` (enum16): 0 `Low`, 1 `High`

## Model 64410: DC Simulator Control Interface

Name in records: `DCSimInterface`.

A generic DC simulator/power supply control interface for DER electrical testing.

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `VMaxLim` | uint16 | V | Maximum Voltage | Upper Voltage Protection Limit |
| `PMaxLim` | uint16 | W | Maximum Power | Upper Power Protection Limit |
| `IMaxLim` | uint16 | A | Maximum Current | Upper Current Protection Limit |
| `Mode` | enum16 |  | CV or CC Mode | Constant Voltage (CV) or Constant Current (CC) Mode |
| `Ena` | enum16 |  | Power On/Off | Power On/Off |
| `Reset` | enum16 |  | Reset Device | Reset Device |
| `VSet` | uint16 | V | Voltage Setpoint | Voltage Setpoint |
| `PSet` | uint16 | W | Power Setpoint | Power Setpoint |
| `ISet` | uint16 | A | Current Setpoint | Current Setpoint |
| `EN50530` | enum16 |  | EN50530 Mode | EN50530 Mode - Enable or disable EN50530 profile mode |
| `Vmpp` | uint16 | V | EN50530 MPP Voltage | EN50530 MPP Voltage |
| `Pmpp` | uint16 | W | EN50530 MPP Power | EN50530 MPP Power |
| `GSet` | uint16 | W/m^2 | Irradiance Setpoint | Irradiance Setpoint |
| `VSlewRate` | uint16 | V/s | Voltage Slew Rate | Voltage Slew Rate |
| `PSlewRate` | uint16 | W/s | Power Slew Rate | Power Slew Rate |
| `ISlewRate` | uint16 | A/s | Current Slew Rate | Current Slew Rate |
| `EnaProf` | enum16 |  | Enable Profile | Start/Stop the Profile |
| `AdptProfReq` | uint16 |  | Profile Adoption Request | Index of profile points to adopt. First curve index is 1. |
| `AdptProfRslt` | enum16 |  | Adopt Profile Result | Result of last adopt profile operation. |
| `V` | int32 |  | Measured Voltage | Measured Voltage |
| `P` | int32 |  | Measured Power | Measured Power |
| `I` | int32 |  | Measured Current | Measured Current |
| `Errors` | string |  | Errors | Error States |
| `NPt` | uint16 |  | Number Of Points | Number of profile points supported. |
| `NProf` | uint16 |  | Stored Profile Count | Number of stored profiles supported. |
| `Prof[*].ActPt` | uint16 |  | Active Points | Number of active points. |
| `Prof[*].DeptRef` | bitfield32 |  | Dependent References | Profile references. |
| `Prof[*].Pt[*].Tms` | uint16 | Sec | Profile Time | Profile time. |
| `Prof[*].Pt[*].V` | uint16 | V | Voltage Point | Profile voltage point in Volts. |
| `Prof[*].Pt[*].P` | uint16 | W | Power Point | Profile power point in Watts. |
| `Prof[*].Pt[*].I` | uint16 | A | Current Point | Profile current point in Amps. |
| `Prof[*].Pt[*].G` | uint16 | Pct | Irradiance Point | Profile irradiance point as percentage. |

Value names:

- `Mode` (enum16): 0 `CV`, 1 `CC`
- `Ena` (enum16): 1 `ON`, 0 `OFF`
- `Reset` (enum16): 1 `RESET`, 0 `DO_NOT_RESET`
- `EN50530` (enum16): 1 `EN50530`, 0 `DO_NOT_EN50530`
- `EnaProf` (enum16): 1 `START`, 0 `STOP`
- `AdptProfRslt` (enum16): 0 `IN_PROGRESS`, 1 `COMPLETED`, 2 `FAILED`
- `Prof[*].DeptRef` (bitfield32): bit 0 `VOLTAGE`, bit 1 `POWER`, bit 2 `CURRENT`, bit 3 `IRRADIANCE`

## Model 64411: AC Simulator Control Interface

Name in records: `ACSimInterface`.

A generic AC simulator/power supply control interface for DER electrical testing.

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `Phases` | uint16 |  | Active Phases | Set the number of active phases for the power supply |
| `PhaseAngle` | uint16 |  | Phase Angle | Phase angle (deg) between phases. 0 for signle phase, 120 for two phase, 120 for three phase. |
| `VNom` | uint16 | V | Nominal Voltage | Nominal L-N Voltage |
| `VMax` | uint16 | V | Maximum Voltage | Maximum Voltage Protection Level |
| `IMax` | uint16 | A | Maximum Current | Maximum Current Protection Level |
| `Freq` | uint16 | Hz | Frequency | Frequency Setpoint |
| `Output` | enum16 |  | Output State | AC Output State |
| `Relay` | enum16 |  | Relay State | AC Relay State |
| `Regen` | enum16 |  | Regeneration State | Regeneration State |
| `VSet` | uint16 | V | Voltage Setpoint | Voltage Setpoint (all phases) |
| `VSetA` | uint16 | V | Voltage Setpoint Phase A | Voltage Setpoint Phase A |
| `VSetB` | uint16 | V | Voltage Setpoint Phase B | Voltage Setpoint Phase B |
| `VSetC` | uint16 | V | Voltage Setpoint Phase C | Voltage Setpoint Phase C |
| `FreqSlew` | uint16 | Hz/s | Frequency Slew Rate | Frequency Slew Rate |
| `VSlew` | uint16 | V/s | Voltage Slew Rate | Voltage Slew Rate |
| `VA` | int32 | V | Measured Voltage Phase A | Measured Voltage Phase A |
| `VB` | int32 | V | Measured Voltage Phase B | Measured Voltage Phase B |
| `VC` | int32 | V | Measured Voltage Phase C | Measured Voltage Phase C |
| `Hz` | int32 | Hz | Measured Frequency | Measured Frequency |
| `IA` | int32 | A | Measured Current Phase A | Measured Current Phase A |
| `IB` | int32 | A | Measured Current Phase B | Measured Current Phase B |
| `IC` | int32 | A | Measured Current Phase C | Measured Current Phase C |
| `VHarA` | string | V | Voltage Harmonics Phase A | Voltage Harmonics Pct, Phase A (comma seperated string for harmonics 1-50) |
| `VHarB` | string | V | Voltage Harmonics Phase B | Voltage Harmonics Pct, Phase B (comma seperated string for harmonics 1-50) |
| `VHarC` | string | V | Voltage Harmonics Phase C | Voltage Harmonics Pct, Phase C (comma seperated string for harmonics 1-50) |
| `IHarA` | string | A | Current Harmonics Phase A | Current Harmonics Pct, Phase A (comma seperated string for harmonics 1-50) |
| `IHarB` | string | A | Current Harmonics Phase B | Current Harmonics Pct, Phase B (comma seperated string for harmonics 1-50) |
| `IHarC` | string | A | Current Harmonics Phase C | Current Harmonics Pct, Phase C (comma seperated string for harmonics 1-50) |
| `IIntHarA` | string | A | Current Interharmonics Phase A | Current Interharmonics Pct, Phase A (comma seperated string for interharmonics 1-50) |
| `IIntHarB` | string | A | Current Interharmonics Phase B | Current Interharmonics Pct, Phase B (comma seperated string for interharmonics 1-50) |
| `IIntHarC` | string | A | Current Interharmonics Phase C | Current Interharmonics Pct, Phase C (comma seperated string for interharmonics 1-50) |
| `VThdA` | uint16 | % | Voltage THD Phase A | Voltage THD Phase A |
| `VThdB` | uint16 | % | Voltage THD Phase B | Voltage THD Phase B |
| `VThdC` | uint16 | % | Voltage THD Phase C | Voltage THD Phase C |
| `IThdA` | uint16 | % | Current THD Phase A | Current THD Phase A |
| `IThdB` | uint16 | % | Current THD Phase B | Current THD Phase B |
| `IThdC` | uint16 | % | Current THD Phase C | Current THD Phase C |
| `EnaProf` | enum16 |  | Enable Profile | Start/Stop the AC Profile |
| `ProfRslt` | enum16 |  | Profile Result | Result of last profile operation. |
| `NProf` | uint16 |  | Stored Profile Count | Number of stored profiles supported. |
| `NPt` | uint16 |  | Max Profile Point Count | Max profile points in the profiles. |
| `Prof[*].Name` | string |  | Profile Name | Profile name. |
| `Prof[*].ActPt` | uint16 |  | Active Points | Number of active points. |
| `Prof[*].Pt[*].Tms` | uint16 | Sec | Profile Time | Profile time. |
| `Prof[*].Pt[*].VA` | uint16 | V | Voltage Point | Profile voltage phase A point in Volts. |
| `Prof[*].Pt[*].VB` | uint16 | V | Voltage Point Phase B | Profile voltage phase B point in Volts. |
| `Prof[*].Pt[*].VC` | uint16 | V | Voltage Point Phase C | Profile voltage phase C point in Volts. |
| `Prof[*].Pt[*].Hz` | uint16 | Hz | Frequency Point | Profile frequency point in Hz. |
| `Prof[*].Pt[*].PhaseAngleA` | uint16 | Degrees | Phase Angle A | Profile phase A angle in degrees. |
| `Prof[*].Pt[*].PhaseAngleB` | uint16 | Degrees | Phase Angle B | Profile phase B angle in degrees. |
| `Prof[*].Pt[*].PhaseAngleC` | uint16 | Degrees | Phase Angle C | Profile phase C angle in degrees. |

Value names:

- `Output` (enum16): 0 `OFF`, 1 `ON`
- `Relay` (enum16): 0 `OPEN`, 1 `CLOSED`
- `Regen` (enum16): 0 `OFF`, 1 `ON`
- `EnaProf` (enum16): 0 `STOP`, 1 `START`, 2 `TRIGGER`
- `ProfRslt` (enum16): 0 `IN_PROGRESS`, 1 `COMPLETED`, 2 `FAILED`

## Model 64412: DER Cyber Exploitation

Name in records: `DERExploitation`.

Operations that represent a hacked DER device

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `DAManipulation` | enum16 |  | DA Manipulation | Modify the device ID of the DER |
| `FalsifyDeviceIdentity` | enum16 |  | Falsify Device Identity | Change the DER manufacturer and model |
| `MeasPAlwaysNameplate` | enum16 |  | Meas P Always Nameplate | Set the DER meas to always be at nameplate power |
| `MeasQAlwaysMinimum` | enum16 |  | Meas Q Always Minimum | Set the DER to always be at minimum reactive power |
| `MeasQAlwaysMaximum` | enum16 |  | Meas Q Always Maximum | Set the DER to always be at maximum reactive power |
| `MeasQAlwaysZero` | enum16 |  | Meas Q Always Zero | Set the DER to always be at zero reactive power |
| `MeasZeroP` | enum16 |  | Meas Zero P | Set the DER to always be at zero P, Q, and S |
| `MeasInvertQ` | enum16 |  | Meas Invert Q | Set the DER to reverse the Q measurement data |
| `MeasLowV` | enum16 |  | Meas Low V | Set the DER to always measure low voltage |
| `MeasHighV` | enum16 |  | Meas High V | Set the DER to always measure high voltage |
| `MeasLowL1V` | enum16 |  | Meas Low L1 V | Set the DER to always measure low line 1 voltage |
| `MeasHighL1V` | enum16 |  | Meas High L1 V | Set the DER to always measure high line 1 voltage |
| `MeasLowF` | enum16 |  | Meas Low F | Set the DER to always measure low frequency |
| `MeasHighF` | enum16 |  | Meas High F | Set the DER to always measure high frequency |
| `MeasLowAmps` | enum16 |  | Meas Low Amps | Set the DER to always measure low current |
| `MeasHighAmps` | enum16 |  | Meas High Amps | Set the DER to always measure high current |
| `MeasHighS` | enum16 |  | Meas High S | Set the DER to always measure high apparent power |
| `MeasLowS` | enum16 |  | Meas Low S | Set the DER to always measure low apparent power |
| `MeasHighQ` | enum16 |  | Meas High Q | Set the DER to always measure high reactive power |
| `MeasLowQ` | enum16 |  | Meas Low Q | Set the DER to always measure low reactive power |
| `MeasLowPF` | enum16 |  | Meas Low PF | Set the DER to always measure low power factor |
| `MeasLowReversedPF` | enum16 |  | Meas Low Reversed PF | Set the DER to always measure low reversed power factor |
| `NameplateHighP` | enum16 |  | Nameplate High P | Set the DER nameplate power to be high |
| `NameplateLowP` | enum16 |  | Nameplate Low P | Set the DER nameplate power to be low |
| `NameplateHighS` | enum16 |  | Nameplate High S | Set the DER nameplate apparent power to be high |
| `NameplateLowS` | enum16 |  | Nameplate Low S | Set the DER nameplate apparent power to be low |
| `NameplateHighQ` | enum16 |  | Nameplate High Q | Set the DER nameplate reactive power to be high |
| `NameplateLowQ` | enum16 |  | Nameplate Low Q | Set the DER nameplate reactive power to be low |
| `NameplateHighNomV` | enum16 |  | Nameplate High Nom V | Set the DER nameplate voltage to be high |
| `NameplateLowNomV` | enum16 |  | Nameplate Low Nom V | Set the DER nameplate voltage to be low |
| `NameplateLowAmps` | enum16 |  | Nameplate Low Amps | Set the DER nameplate current to be low |
| `NameplateLowVarmaxinj` | enum16 |  | Nameplate Low Varmaxinj | Set the DER nameplate VarMaxInj to be low |
| `NameplateLowVarmaxabs` | enum16 |  | Nameplate Low Varmaxabs | Set the DER nameplate VarMaxAbs to be low |
| `NameplateLowPF` | enum16 |  | Nameplate Low PF | Set the DER nameplate power factor to be low |
| `SettingsHighNomV` | enum16 |  | Settings High Nom V | Set the DER settings voltage to be high |
| `SettingsLowAmps` | enum16 |  | Settings Low Amps | Set the DER settings current to be low |
| `SettingsHighP` | enum16 |  | Settings High P | Set the DER settings power to be high |
| `SettingsLowP` | enum16 |  | Settings Low P | Set the DER settings power to be low |
| `SettingsHighVAMax` | enum16 |  | Settings High VAMax | Set the DER settings VAMax to be high |
| `SettingsHighVarmaxinj` | enum16 |  | Settings High Varmaxinj | Set the DER settings VarMaxInj to be high |
| `SettingsHighVarmaxabs` | enum16 |  | Settings High Varmaxabs | Set the DER settings VarMaxAbs to be high |
| `ChangeCommonModelID` | enum16 |  | Change Common Model ID | Change the common model ID |
| `ChangeCommonModelLength` | enum16 |  | Change Common Model Length | Change the common model length |

Value names:

- `DAManipulation` (enum16): 0 `OFF`, 1 `ON`
- `FalsifyDeviceIdentity` (enum16): 0 `OFF`, 1 `ON`
- `MeasPAlwaysNameplate` (enum16): 0 `OFF`, 1 `ON`
- `MeasQAlwaysMinimum` (enum16): 0 `OFF`, 1 `ON`
- `MeasQAlwaysMaximum` (enum16): 0 `OFF`, 1 `ON`
- `MeasQAlwaysZero` (enum16): 0 `OFF`, 1 `ON`
- `MeasZeroP` (enum16): 0 `OFF`, 1 `ON`
- `MeasInvertQ` (enum16): 0 `OFF`, 1 `ON`
- `MeasLowV` (enum16): 0 `OFF`, 1 `ON`
- `MeasHighV` (enum16): 0 `OFF`, 1 `ON`
- `MeasLowL1V` (enum16): 0 `OFF`, 1 `ON`
- `MeasHighL1V` (enum16): 0 `OFF`, 1 `ON`
- `MeasLowF` (enum16): 0 `OFF`, 1 `ON`
- `MeasHighF` (enum16): 0 `OFF`, 1 `ON`
- `MeasLowAmps` (enum16): 0 `OFF`, 1 `ON`
- `MeasHighAmps` (enum16): 0 `OFF`, 1 `ON`
- `MeasHighS` (enum16): 0 `OFF`, 1 `ON`
- `MeasLowS` (enum16): 0 `OFF`, 1 `ON`
- `MeasHighQ` (enum16): 0 `OFF`, 1 `ON`
- `MeasLowQ` (enum16): 0 `OFF`, 1 `ON`
- `MeasLowPF` (enum16): 0 `OFF`, 1 `ON`
- `MeasLowReversedPF` (enum16): 0 `OFF`, 1 `ON`
- `NameplateHighP` (enum16): 0 `OFF`, 1 `ON`
- `NameplateLowP` (enum16): 0 `OFF`, 1 `ON`
- `NameplateHighS` (enum16): 0 `OFF`, 1 `ON`
- `NameplateLowS` (enum16): 0 `OFF`, 1 `ON`
- `NameplateHighQ` (enum16): 0 `OFF`, 1 `ON`
- `NameplateLowQ` (enum16): 0 `OFF`, 1 `ON`
- `NameplateHighNomV` (enum16): 0 `OFF`, 1 `ON`
- `NameplateLowNomV` (enum16): 0 `OFF`, 1 `ON`
- `NameplateLowAmps` (enum16): 0 `OFF`, 1 `ON`
- `NameplateLowVarmaxinj` (enum16): 0 `OFF`, 1 `ON`
- `NameplateLowVarmaxabs` (enum16): 0 `OFF`, 1 `ON`
- `NameplateLowPF` (enum16): 0 `OFF`, 1 `ON`
- `SettingsHighNomV` (enum16): 0 `OFF`, 1 `ON`
- `SettingsLowAmps` (enum16): 0 `OFF`, 1 `ON`
- `SettingsHighP` (enum16): 0 `OFF`, 1 `ON`
- `SettingsLowP` (enum16): 0 `OFF`, 1 `ON`
- `SettingsHighVAMax` (enum16): 0 `OFF`, 1 `ON`
- `SettingsHighVarmaxinj` (enum16): 0 `OFF`, 1 `ON`
- `SettingsHighVarmaxabs` (enum16): 0 `OFF`, 1 `ON`
- `ChangeCommonModelID` (enum16): 0 `OFF`, 1 `ON`
- `ChangeCommonModelLength` (enum16): 0 `OFF`, 1 `ON`

## Model 64413: PV Curves

Name in records: `PVSimCurves`.

Current-Voltage and Power-Voltage Profiles for PV Simulation.

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `IVLen` | count |  | IV length | Number of points in the IV curve. |
| `Irr` | uint16 | W/m^2 | POA Irradiance | Plane of Array Irradiance |
| `IV[*].P` | float32 | W | Power | Power |
| `IV[*].I` | float32 | A | Current | Current |
| `IV[*].V` | float32 | V | Voltage | Voltage |

## Model 64414: DER Simulation Controls

Name in records: `DERSimControls`.

Configuration parameters for the DER device simulator.

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `Time` | string |  | Time offset | Time offset for simulation formatted 'HH:MM:SS' |
| `Temperature` | float32 |  |  | Ambient temperature (degrees Celsius) |
| `GridModelSource` | string |  |  | The data source for the grid model. 'csv' or 'const' |
| `IrradianceModelSource` | string |  |  | The data source for the irradiance model. 'csv' or 'const' |
| `Irradiance` | float32 |  |  | The irradiance on the DER device (W/m^2) for the 'const' irradiance model |
| `GridVoltageA` | float32 |  |  | Phase A RMS Voltage (pu) for the 'const' grid model |
| `GridVoltageB` | float32 |  |  | Phase B RMS Voltage (pu) for the 'const' grid model |
| `GridVoltageC` | float32 |  |  | Phase C RMS Voltage (pu) for the 'const' grid model |
| `GridFrequency` | float32 |  |  | Grid frequency (Hz) for the 'const' grid model |

## Model 64415: CSIP Client Control

Name in records: `CSIPControl`.

CSIP Client Control for Alarms and Error tests

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `LogEventEna` | enum16 |  | LogEvent Mode Enable | Enable or disable the LogEvent mode |
| `HTTPMsg` | enum16 |  | HTTP Message Mode Enable | Enable or disable the HTTP Message mode |
| `COMM004Cert` | enum16 |  | COMM-004 Certificate | Select COMM-004 certificate type |
| `SubscribedResource` | string |  | Subscribed Resource URL | The URL of the resource to subscribe to |
| `SubscriptionEna` | enum16 |  | Subscribtion Enable | Enable or disable the Subscription mode |

Value names:

- `LogEventEna` (enum16): 0 `DISABLED`, 1 `ENABLED`
- `HTTPMsg` (enum16): 0 `DISABLED`, 1 `ENABLED`
- `COMM004Cert` (enum16): 0 `Default Certificate`, 1 `COMM_004A`, 2 `COMM_004B`, 3 `COMM_004C`, 4 `COMM_004D`, 5 `COMM_004E`, 6 `COMM_004F`, 7 `COMM_004G`
- `SubscriptionEna` (enum16): 0 `DISABLED`, 1 `ENABLED`
