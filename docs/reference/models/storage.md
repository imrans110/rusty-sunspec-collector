# Storage

<!-- Generated from the bundled SunSpec definitions by
     UPDATE_MODEL_DOCS=1 cargo test -p sunspec-parser --test models_docs
     Do not edit by hand. -->

Point names are the ones records use; `[*]` stands for a repeating group's index. See [Supported SunSpec models](README.md) for every group.

| Model | Name in records | Points |
|-------|-----------------|--------|
| [801](#model-801-energy-storage-base-model-deprecated) | `storage` | 1 |
| [802](#model-802-battery-base-model) | `battery` | 44 |
| [803](#model-803-lithium-ion-battery-bank-model) | `lithium_ion_bank` | 44 |
| [804](#model-804-lithium-ion-string-model) | `lithium_ion_string` | 41 |
| [805](#model-805-lithium-ion-module-model) | `lithium-ion-module` | 23 |
| [806](#model-806-flow-battery-model) | `flow_battery` | 2 |
| [807](#model-807-flow-battery-string-model) | `flow_battery_string` | 44 |
| [808](#model-808-flow-battery-module-model) | `flow_battery_module` | 2 |
| [809](#model-809-flow-battery-stack-model) | `flow_battery_stack` | 2 |

## Model 801: Energy Storage Base Model (DEPRECATED)

Name in records: `storage`.

This model has been deprecated.

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `DEPRECATED` | enum16 |  | Deprecated Model | This model has been deprecated. |

## Model 802: Battery Base Model

Name in records: `battery`.

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `AHRtg` | uint16 | Ah | Nameplate Charge Capacity | Nameplate charge capacity in amp-hours. |
| `WHRtg` | uint16 | Wh | Nameplate Energy Capacity | Nameplate energy capacity in DC watt-hours. |
| `WChaRteMax` | uint16 | W | Nameplate Max Charge Rate | Maximum rate of energy transfer into the storage device in DC watts. |
| `WDisChaRteMax` | uint16 | W | Nameplate Max Discharge Rate | Maximum rate of energy transfer out of the storage device in DC watts. |
| `DisChaRte` | uint16 | %WHRtg | Self Discharge Rate | Self discharge rate. Percentage of capacity (WHRtg) discharged per day. |
| `SoCMax` | uint16 | %WHRtg | Nameplate Max SoC | Manufacturer maximum state of charge, expressed as a percentage. |
| `SoCMin` | uint16 | %WHRtg | Nameplate Min SoC | Manufacturer minimum state of charge, expressed as a percentage. |
| `SocRsvMax` | uint16 | %WHRtg | Max Reserve Percent | Setpoint for maximum reserve for storage as a percentage of the nominal maximum storage. |
| `SoCRsvMin` | uint16 | %WHRtg | Min Reserve Percent | Setpoint for minimum reserve for storage as a percentage of the nominal maximum storage. |
| `SoC` | uint16 | %WHRtg | State of Charge | State of charge, expressed as a percentage. |
| `DoD` | uint16 | % | Depth of Discharge | Depth of discharge, expressed as a percentage. |
| `SoH` | uint16 | % | State of Health | Percentage of battery life remaining. |
| `NCyc` | uint32 |  | Cycle Count | Number of cycles executed in the battery. |
| `ChaSt` | enum16 |  | Charge Status | Charge status of storage device. Enumeration. |
| `LocRemCtl` | enum16 |  | Control Mode | Battery control mode. Enumeration. |
| `Hb` | uint16 |  | Battery Heartbeat | Value is incremented every second with periodic resets to zero. |
| `CtrlHb` | uint16 |  | Controller Heartbeat | Value is incremented every second with periodic resets to zero. |
| `AlmRst` | uint16 |  | Alarm Reset | Used to reset any latched alarms. 1 = Reset. |
| `Typ` | enum16 |  | Battery Type | Type of battery. Enumeration. |
| `State` | enum16 |  | State of the Battery Bank | State of the battery bank. Enumeration. |
| `StateVnd` | enum16 |  | Vendor Battery Bank State | Vendor specific battery bank state. Enumeration. |
| `WarrDt` | uint32 |  | Warranty Date | Date the device warranty expires. |
| `Evt1` | bitfield32 |  | Battery Event 1 Bitfield | Alarms and warnings. |
| `Evt2` | bitfield32 |  | Battery Event 2 Bitfield | Alarms and warnings. |
| `EvtVnd1` | bitfield32 |  | Vendor Event Bitfield 1 | Vendor defined events. |
| `EvtVnd2` | bitfield32 |  | Vendor Event Bitfield 2 | Vendor defined events. |
| `V` | uint16 | V | External Battery Voltage | DC Bus Voltage. |
| `VMax` | uint16 | V | Max Battery Voltage | Instantaneous maximum battery voltage. |
| `VMin` | uint16 | V | Min Battery Voltage | Instantaneous minimum battery voltage. |
| `CellVMax` | uint16 | V | Max Cell Voltage | Maximum voltage for all cells in the bank. |
| `CellVMaxStr` | uint16 |  | Max Cell Voltage String | String containing the cell with maximum voltage. |
| `CellVMaxMod` | uint16 |  | Max Cell Voltage Module | Module containing the cell with maximum voltage. |
| `CellVMin` | uint16 | V | Min Cell Voltage | Minimum voltage for all cells in the bank. |
| `CellVMinStr` | uint16 |  | Min Cell Voltage String | String containing the cell with minimum voltage. |
| `CellVMinMod` | uint16 |  | Min Cell Voltage Module | Module containing the cell with minimum voltage. |
| `CellVAvg` | uint16 | V | Average Cell Voltage | Average cell voltage for all cells in the bank. |
| `A` | int16 | A | Total DC Current | Total DC current flowing to/from the battery bank. |
| `AChaMax` | uint16 | A | Max Charge Current | Instantaneous maximum DC charge current. |
| `ADisChaMax` | uint16 | A | Max Discharge Current | Instantaneous maximum DC discharge current. |
| `W` | int16 | W | Total Power | Total power flowing to/from the battery bank. |
| `ReqInvState` | enum16 |  | Inverter State Request | Request from battery to start or stop the inverter. Enumeration. |
| `ReqW` | int16 | W | Battery Power Request | AC Power requested by battery. |
| `SetOp` | enum16 |  | Set Operation | Instruct the battery bank to perform an operation such as connecting. Enumeration. |
| `SetInvState` | enum16 |  | Set Inverter State | Set the current state of the inverter. |

Value names:

- `ChaSt` (enum16): 1 `OFF`, 2 `EMPTY`, 3 `DISCHARGING`, 4 `CHARGING`, 5 `FULL`, 6 `HOLDING`, 7 `TESTING`
- `LocRemCtl` (enum16): 0 `REMOTE`, 1 `LOCAL`
- `Typ` (enum16): 0 `NOT APPLICABLE_UNKNOWN`, 1 `LEAD_ACID`, 2 `NICKEL_METAL_HYDRATE`, 3 `NICKEL_CADMIUM`, 4 `LITHIUM_ION`, 5 `CARBON_ZINC`, 6 `ZINC_CHLORIDE`, 7 `ALKALINE`, 8 `RECHARGEABLE_ALKALINE`, 9 `SODIUM_SULFUR`, 10 `FLOW`, 99 `OTHER`
- `State` (enum16): 1 `DISCONNECTED`, 2 `INITIALIZING`, 3 `CONNECTED`, 4 `STANDBY`, 5 `SOC PROTECTION`, 6 `SUSPENDING`, 99 `FAULT`
- `Evt1` (bitfield32): bit 0 `COMMUNICATION_ERROR`, bit 1 `OVER_TEMP_ALARM`, bit 2 `OVER_TEMP_WARNING`, bit 3 `UNDER_TEMP_ALARM`, bit 4 `UNDER_TEMP_WARNING`, bit 5 `OVER_CHARGE_CURRENT_ALARM`, bit 6 `OVER_CHARGE_CURRENT_WARNING`, bit 7 `OVER_DISCHARGE_CURRENT_ALARM`, bit 8 `OVER_DISCHARGE_CURRENT_WARNING`, bit 9 `OVER_VOLT_ALARM`, bit 10 `OVER_VOLT_WARNING`, bit 11 `UNDER_VOLT_ALARM`, bit 12 `UNDER_VOLT_WARNING`, bit 13 `UNDER_SOC_MIN_ALARM`, bit 14 `UNDER_SOC_MIN_WARNING`, bit 15 `OVER_SOC_MAX_ALARM`, bit 16 `OVER_SOC_MAX_WARNING`, bit 17 `VOLTAGE_IMBALANCE_WARNING`, bit 18 `TEMPERATURE_IMBALANCE_ALARM`, bit 19 `TEMPERATURE_IMBALANCE_WARNING`, bit 20 `CONTACTOR_ERROR`, bit 21 `FAN_ERROR`, bit 22 `GROUND_FAULT`, bit 23 `OPEN_DOOR_ERROR`, bit 24 `CURRENT_IMBALANCE_WARNING`, bit 25 `OTHER_ALARM`, bit 26 `OTHER_WARNING`, bit 27 `RESERVED_1`, bit 28 `CONFIGURATION_ALARM`, bit 29 `CONFIGURATION_WARNING`
- `ReqInvState` (enum16): 0 `NO REQUEST`, 1 `START`, 2 `STOP`
- `SetOp` (enum16): 1 `CONNECT`, 2 `DISCONNECT`
- `SetInvState` (enum16): 1 `INVERTER_STOPPED`, 2 `INVERTER_STANDBY`, 3 `INVERTER_STARTED`

## Model 803: Lithium-Ion Battery Bank Model

Name in records: `lithium_ion_bank`.

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `NStr` | uint16 |  | String Count | Number of strings in the bank. |
| `NStrCon` | uint16 |  | Connected String Count | Number of strings with contactor closed. |
| `ModTmpMax` | int16 | C | Max Module Temperature | Maximum temperature for all modules in the bank. |
| `ModTmpMaxStr` | uint16 |  | Max Module Temperature String | String containing the module with maximum temperature. |
| `ModTmpMaxMod` | uint16 |  | Max Module Temperature Module | Module with maximum temperature. |
| `ModTmpMin` | int16 | C | Min Module Temperature | Minimum temperature for all modules in the bank. |
| `ModTmpMinStr` | uint16 |  | Min Module Temperature String | String containing the module with minimum temperature. |
| `ModTmpMinMod` | uint16 |  | Min Module Temperature Module | Module with minimum temperature. |
| `ModTmpAvg` | int16 | C | Average Module Temperature | Average temperature for all modules in the bank. |
| `StrVMax` | uint16 | V | Max String Voltage | Maximum string voltage for all strings in the bank. |
| `StrVMaxStr` | uint16 |  | Max String Voltage String | String with maximum voltage. |
| `StrVMin` | uint16 | V | Min String Voltage | Minimum string voltage for all strings in the bank. |
| `StrVMinStr` | uint16 |  | Min String Voltage String | String with minimum voltage. |
| `StrVAvg` | uint16 | V | Average String Voltage | Average string voltage for all strings in the bank. |
| `StrAMax` | int16 | A | Max String Current | Maximum current of any string in the bank. |
| `StrAMaxStr` | uint16 |  | Max String Current String | String with the maximum current. |
| `StrAMin` | int16 | A | Min String Current | Minimum current of any string in the bank. |
| `StrAMinStr` | uint16 |  | Min String Current String | String with the minimum current. |
| `StrAAvg` | int16 | A | Average String Current | Average string current for all strings in the bank. |
| `NCellBal` | uint16 |  | Battery Cell Balancing Count | Total number of cells that are currently being balanced. |
| `string[*].StrNMod` | uint16 |  | Module Count | Count of modules in the string. |
| `string[*].StrSt` | bitfield32 |  | String Status | Current status of the string. |
| `string[*].StrConFail` | enum16 |  | Connection Failure Reason |  |
| `string[*].StrSoC` | uint16 | % | String State of Charge | Battery string state of charge, expressed as a percentage. |
| `string[*].StrSoH` | uint16 | % | String State of Health | Battery string state of health, expressed as a percentage. |
| `string[*].StrA` | int16 | A | String Current | String current measurement. |
| `string[*].StrCellVMax` | uint16 | V | Max Cell Voltage | Maximum voltage for all cells in the string. |
| `string[*].StrCellVMaxMod` | uint16 |  | Max Cell Voltage Module | Module containing the maximum cell voltage. |
| `string[*].StrCellVMin` | uint16 | V | Min Cell Voltage | Minimum voltage for all cells in the string. |
| `string[*].StrCellVMinMod` | uint16 |  | Min Cell Voltage Module | Module containing the minimum cell voltage. |
| `string[*].StrCellVAvg` | uint16 | V | Average Cell Voltage | Average voltage for all cells in the string. |
| `string[*].StrModTmpMax` | int16 | C | Max Module Temperature | Maximum temperature for all modules in the bank. |
| `string[*].StrModTmpMaxMod` | uint16 |  | Max Module Temperature Module | Module with the maximum temperature. |
| `string[*].StrModTmpMin` | int16 | C | Min Module Temperature | Minimum temperature for all modules in the bank. |
| `string[*].StrModTmpMinMod` | uint16 |  | Min Module Temperature Module | Module with the minimum temperature. |
| `string[*].StrModTmpAvg` | int16 | C | Average Module Temperature | Average temperature for all modules in the bank. |
| `string[*].StrDisRsn` | enum16 |  | Disabled Reason | Reason why the string is currently disabled. |
| `string[*].StrConSt` | bitfield32 |  | Contactor Status | Status of the contactor(s) for the string. |
| `string[*].StrEvt1` | bitfield32 |  | String Event 1 | Alarms, warnings and status values. |
| `string[*].StrEvt2` | bitfield32 |  | String Event 2 | Alarms, warnings and status values. |
| `string[*].StrEvtVnd1` | bitfield32 |  | Vendor String Event Bitfield 1 | Vendor defined events. |
| `string[*].StrEvtVnd2` | bitfield32 |  | Vendor String Event Bitfield 2 | Vendor defined events. |
| `string[*].StrSetEna` | enum16 |  | Enable/Disable String | Enables and disables the string. |
| `string[*].StrSetCon` | enum16 |  | Connect/Disconnect String | Connects and disconnects the string. |

Value names:

- `string[*].StrSt` (bitfield32): bit 0 `STRING_ENABLED`, bit 1 `CONTACTOR_STATUS`
- `string[*].StrConFail` (enum16): 0 `NO_FAILURE`, 1 `BUTTON_PUSHED`, 2 `STR_GROUND_FAULT`, 3 `OUTSIDE_VOLTAGE_RANGE`, 4 `STRING_NOT_ENABLED`, 5 `FUSE_OPEN`, 6 `CONTACTOR_FAILURE`, 7 `PRECHARGE_FAILURE`, 8 `STRING_FAULT`
- `string[*].StrDisRsn` (enum16): 0 `NONE`, 1 `FAULT`, 2 `MAINTENANCE`, 3 `EXTERNAL`, 4 `OTHER`
- `string[*].StrConSt` (bitfield32): bit 0 `CONTACTOR_0`, bit 1 `CONTACTOR_1`, bit 2 `CONTACTOR_2`, bit 3 `CONTACTOR_3`, bit 4 `CONTACTOR_4`, bit 5 `CONTACTOR_5`, bit 6 `CONTACTOR_6`, bit 7 `CONTACTOR_7`, bit 8 `CONTACTOR_8`, bit 9 `CONTACTOR_9`, bit 10 `CONTACTOR_10`, bit 11 `CONTACTOR_11`, bit 12 `CONTACTOR_12`, bit 13 `CONTACTOR_13`, bit 14 `CONTACTOR_14`, bit 15 `CONTACTOR_15`, bit 16 `CONTACTOR_16`, bit 17 `CONTACTOR_17`, bit 18 `CONTACTOR_18`, bit 19 `CONTACTOR_19`, bit 20 `CONTACTOR_20`, bit 21 `CONTACTOR_21`, bit 22 `CONTACTOR_22`, bit 23 `CONTACTOR_23`, bit 24 `CONTACTOR_24`, bit 25 `CONTACTOR_25`, bit 26 `CONTACTOR_26`, bit 27 `CONTACTOR_27`, bit 28 `CONTACTOR_28`, bit 29 `CONTACTOR_29`, bit 30 `CONTACTOR_30`
- `string[*].StrEvt1` (bitfield32): bit 0 `COMMUNICATION_ERROR`, bit 1 `OVER_TEMP_ALARM`, bit 2 `OVER_TEMP_WARNING`, bit 3 `UNDER_TEMP_ALARM`, bit 4 `UNDER_TEMP_WARNING`, bit 5 `OVER_CHARGE_CURRENT_ALARM`, bit 6 `OVER_CHARGE_CURRENT_WARNING`, bit 7 `OVER_DISCHARGE_CURRENT_ALARM`, bit 8 `OVER_DISCHARGE_CURRENT_WARNING`, bit 9 `OVER_VOLT_ALARM`, bit 10 `OVER_VOLT_WARNING`, bit 11 `UNDER_VOLT_ALARM`, bit 12 `UNDER_VOLT_WARNING`, bit 13 `UNDER_SOC_MIN_ALARM`, bit 14 `UNDER_SOC_MIN_WARNING`, bit 15 `OVER_SOC_MAX_ALARM`, bit 16 `OVER_SOC_MAX_WARNING`, bit 17 `VOLTAGE_IMBALANCE_WARNING`, bit 18 `TEMPERATURE_IMBALANCE_ALARM`, bit 19 `TEMPERATURE_IMBALANCE_WARNING`, bit 20 `CONTACTOR_ERROR`, bit 21 `FAN_ERROR`, bit 22 `GROUND_FAULT`, bit 23 `OPEN_DOOR_ERROR`, bit 24 `RESERVED_1`, bit 25 `OTHER_ALARM`, bit 26 `OTHER_WARNING`, bit 27 `RESERVED_2`, bit 28 `CONFIGURATION_ALARM`, bit 29 `CONFIGURATION_WARNING`
- `string[*].StrSetEna` (enum16): 1 `ENABLE_STRING`, 2 `DISABLE_STRING`
- `string[*].StrSetCon` (enum16): 1 `CONNECT_STRING`, 2 `DISCONNECT_STRING`

## Model 804: Lithium-Ion String Model

Name in records: `lithium_ion_string`.

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `Idx` | uint16 |  | String Index | Index of the string within the bank. |
| `NMod` | uint16 |  | Module Count | Count of modules in the string. |
| `St` | bitfield32 |  | String Status | Current status of the string. |
| `ConFail` | enum16 |  | Connection Failure Reason |  |
| `NCellBal` | uint16 |  | String Cell Balancing Count | Number of cells currently being balanced in the string. |
| `SoC` | uint16 | % | String State of Charge | Battery string state of charge, expressed as a percentage. |
| `DoD` | uint16 | % | String Depth of Discharge | Depth of discharge for the string, expressed as a percentage. |
| `NCyc` | uint32 |  | String Cycle Count | Number of discharge cycles executed upon the string. |
| `SoH` | uint16 | % | String State of Health | Battery string state of health, expressed as a percentage. |
| `A` | int16 | A | String Current | String current measurement. |
| `V` | uint16 | V | String Voltage | String voltage measurement. |
| `CellVMax` | uint16 | V | Max Cell Voltage | Maximum voltage for all cells in the string. |
| `CellVMaxMod` | uint16 |  | Max Cell Voltage Module | Module containing the cell with maximum cell voltage. |
| `CellVMin` | uint16 | V | Min Cell Voltage | Minimum voltage for all cells in the string. |
| `CellVMinMod` | uint16 |  | Min Cell Voltage Module | Module containing the cell with minimum cell voltage. |
| `CellVAvg` | uint16 | V | Average Cell Voltage | Average voltage for all cells in the string. |
| `ModTmpMax` | int16 | C | Max Module Temperature | Maximum temperature for all modules in the string. |
| `ModTmpMaxMod` | uint16 |  | Max Module Temperature Module | Module with the maximum temperature. |
| `ModTmpMin` | int16 | C | Min Module Temperature | Minimum temperature for all modules in the string. |
| `ModTmpMinMod` | uint16 |  | Min Module Temperature Module | Module with the minimum temperature. |
| `ModTmpAvg` | int16 | C | Average Module Temperature | Average temperature for all modules in the string. |
| `ConSt` | bitfield32 |  | Contactor Status | Status of the contactor(s) for the string. |
| `Evt1` | bitfield32 |  | String Event 1 | Alarms, warnings and status values. |
| `Evt2` | bitfield32 |  | String Event 2 | Alarms, warnings and status values. |
| `EvtVnd1` | bitfield32 |  | Vendor Event Bitfield 1 | Vendor defined events. |
| `EvtVnd2` | bitfield32 |  | Vendor Event Bitfield 2 | Vendor defined events. |
| `SetEna` | enum16 |  | Enable/Disable String | Enables and disables the string. Should reset to 0 upon completion. |
| `SetCon` | enum16 |  | Connect/Disconnect String | Connects and disconnects the string. |
| `lithium_ion_string_module[*].ModNCell` | uint16 |  | Module Cell Count | Count of all cells in the module. |
| `lithium_ion_string_module[*].ModSoC` | uint16 | % | Module SoC | Module state of charge, expressed as a percentage. |
| `lithium_ion_string_module[*].ModSoH` | uint16 | % | Module SoH | Module state of health, expressed as a percentage. |
| `lithium_ion_string_module[*].ModCellVMax` | uint16 | V | Max Cell Voltage | Maximum voltage for all cells in the module. |
| `lithium_ion_string_module[*].ModCellVMaxCell` | uint16 |  | Max Cell Voltage Cell | Cell with maximum voltage. |
| `lithium_ion_string_module[*].ModCellVMin` | uint16 | V | Min Cell Voltage | Minimum voltage for all cells in the module. |
| `lithium_ion_string_module[*].ModCellVMinCell` | uint16 |  | Min Cell Voltage Cell | Cell with minimum voltage. |
| `lithium_ion_string_module[*].ModCellVAvg` | uint16 | V | Average Cell Voltage | Average voltage for all cells in the module. |
| `lithium_ion_string_module[*].ModCellTmpMax` | int16 | C | Max Cell Temperature | Maximum temperature for all cells in the module. |
| `lithium_ion_string_module[*].ModCellTmpMaxCell` | uint16 |  | Max Cell Temperature Cell | Cell with maximum temperature. |
| `lithium_ion_string_module[*].ModCellTmpMin` | int16 | C | Min Cell Temperature | Minimum temperature for all cells in the module. |
| `lithium_ion_string_module[*].ModCellTmpMinCell` | uint16 |  | Min Cell Temperature Cell | Cell with minimum temperature. |
| `lithium_ion_string_module[*].ModCellTmpAvg` | int16 | C | Average Cell Temperature | Average temperature for all cells in the module. |

Value names:

- `St` (bitfield32): bit 0 `STRING_ENABLED`, bit 1 `CONTACTOR_STATUS`
- `ConFail` (enum16): 0 `NO_FAILURE`, 1 `BUTTON_PUSHED`, 2 `STR_GROUND_FAULT`, 3 `OUTSIDE_VOLTAGE_RANGE`, 4 `STRING_NOT_ENABLED`, 5 `FUSE_OPEN`, 6 `CONTACTOR_FAILURE`, 7 `PRECHARGE_FAILURE`, 8 `STRING_FAULT`
- `ConSt` (bitfield32): bit 0 `CONTACTOR_0`, bit 1 `CONTACTOR_1`, bit 2 `CONTACTOR_2`, bit 3 `CONTACTOR_3`, bit 4 `CONTACTOR_4`, bit 5 `CONTACTOR_5`, bit 6 `CONTACTOR_6`, bit 7 `CONTACTOR_7`, bit 8 `CONTACTOR_8`, bit 9 `CONTACTOR_9`, bit 10 `CONTACTOR_10`, bit 11 `CONTACTOR_11`, bit 12 `CONTACTOR_12`, bit 13 `CONTACTOR_13`, bit 14 `CONTACTOR_14`, bit 15 `CONTACTOR_15`, bit 16 `CONTACTOR_16`, bit 17 `CONTACTOR_17`, bit 18 `CONTACTOR_18`, bit 19 `CONTACTOR_19`, bit 20 `CONTACTOR_20`, bit 21 `CONTACTOR_21`, bit 22 `CONTACTOR_22`, bit 23 `CONTACTOR_23`, bit 24 `CONTACTOR_24`, bit 25 `CONTACTOR_25`, bit 26 `CONTACTOR_26`, bit 27 `CONTACTOR_27`, bit 28 `CONTACTOR_28`, bit 29 `CONTACTOR_29`, bit 30 `CONTACTOR_30`
- `Evt1` (bitfield32): bit 0 `COMMUNICATION_ERROR`, bit 1 `OVER_TEMP_ALARM`, bit 2 `OVER_TEMP_WARNING`, bit 3 `UNDER_TEMP_ALARM`, bit 4 `UNDER_TEMP_WARNING`, bit 5 `OVER_CHARGE_CURRENT_ALARM`, bit 6 `OVER_CHARGE_CURRENT_WARNING`, bit 7 `OVER_DISCHARGE_CURRENT_ALARM`, bit 8 `OVER_DISCHARGE_CURRENT_WARNING`, bit 9 `OVER_VOLT_ALARM`, bit 10 `OVER_VOLT_WARNING`, bit 11 `UNDER_VOLT_ALARM`, bit 12 `UNDER_VOLT_WARNING`, bit 13 `UNDER_SOC_MIN_ALARM`, bit 14 `UNDER_SOC_MIN_WARNING`, bit 15 `OVER_SOC_MAX_ALARM`, bit 16 `OVER_SOC_MAX_WARNING`, bit 17 `VOLTAGE_IMBALANCE_WARNING`, bit 18 `TEMPERATURE_IMBALANCE_ALARM`, bit 19 `TEMPERATURE_IMBALANCE_WARNING`, bit 20 `CONTACTOR_ERROR`, bit 21 `FAN_ERROR`, bit 22 `GROUND_FAULT`, bit 23 `OPEN_DOOR_ERROR`, bit 24 `RESERVED_1`, bit 25 `OTHER_ALARM`, bit 26 `OTHER_WARNING`, bit 27 `RESERVED_2`, bit 28 `CONFIGURATION_ALARM`, bit 29 `CONFIGURATION_WARNING`
- `SetCon` (enum16): 1 `CONNECT_STRING`, 2 `DISCONNECT_STRING`

## Model 805: Lithium-Ion Module Model

Name in records: `lithium-ion-module`.

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `StrIdx` | uint16 |  | String Index | Index of the string containing the module. |
| `ModIdx` | uint16 |  | Module Index | Index of the module within the string. |
| `NCell` | uint16 |  | Module Cell Count | Count of all cells in the module. |
| `SoC` | uint16 | % | Module SoC | Module state of charge, expressed as a percentage. |
| `DoD` | uint16 | % | Depth of Discharge | Depth of discharge for the module. |
| `SoH` | uint16 | % | Module SoH | Module state of health, expressed as a percentage. |
| `NCyc` | uint32 |  | Cycle Count | Count of cycles executed. |
| `V` | uint16 | V | Module Voltage | Voltage of the module. |
| `CellVMax` | uint16 | V | Max Cell Voltage | Maximum voltage for all cells in the module. |
| `CellVMaxCell` | uint16 |  | Max Cell Voltage Cell | Cell with the maximum voltage. |
| `CellVMin` | uint16 | V | Min Cell Voltage | Minimum voltage for all cells in the module. |
| `CellVMinCell` | uint16 |  | Min Cell Voltage Cell | Cell with the minimum voltage. |
| `CellVAvg` | uint16 | V | Average Cell Voltage | Average voltage for all cells in the module. |
| `CellTmpMax` | int16 | C | Max Cell Temperature | Maximum temperature for all cells in the module. |
| `CellTmpMaxCell` | uint16 |  | Max Cell Temperature Cell | Cell with the maximum cell temperature. |
| `CellTmpMin` | int16 | C | Min Cell Temperature | Minimum temperature for all cells in the module. |
| `CellTmpMinCell` | uint16 |  | Min Cell Temperature Cell | Cell with the minimum cell temperature. |
| `CellTmpAvg` | int16 | C | Average Cell Temperature | Average temperature for all cells in the module. |
| `NCellBal` | uint16 |  | Balanced Cell Count | Number of cells currently being balanced in the module. |
| `SN` | string |  | Serial Number | Serial number for the module. |
| `lithium-ion-module-cell[*].CellV` | uint16 | V | Cell Voltage | Cell terminal voltage. |
| `lithium-ion-module-cell[*].CellTmp` | int16 | C | Cell Temperature | Cell temperature. |
| `lithium-ion-module-cell[*].CellSt` | bitfield32 |  | Cell Status | Status of the cell. |

Value names:

- `lithium-ion-module-cell[*].CellSt` (bitfield32): bit 0 `CELL_IS_BALANCING`

## Model 806: Flow Battery Model

Name in records: `flow_battery`.

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `BatTBD` | uint16 |  | Battery Points To Be Determined |  |
| `battery_string[*].BatStTBD` | uint16 |  | Battery String Points To Be Determined |  |

## Model 807: Flow Battery String Model

Name in records: `flow_battery_string`.

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `Idx` | uint16 |  | String Index | Index of the string within the bank. |
| `NMod` | uint16 |  | Module Count | Number of modules in this string. |
| `NModCon` | uint16 |  | Connected Module Count | Number of electrically connected modules in this string. |
| `ModVMax` | uint16 | V | Max Module Voltage | Maximum voltage for all modules in the string. |
| `ModVMaxMod` | uint16 |  | Max Module Voltage Module | Module with the maximum voltage. |
| `ModVMin` | uint16 | V | Min Module Voltage | Minimum voltage for all modules in the string. |
| `ModVMinMod` | uint16 |  | Min Module Voltage Module | Module with the minimum voltage. |
| `ModVAvg` | uint16 | V | Average Module Voltage | Average voltage for all modules in the string. |
| `CellVMax` | uint16 | V | Max Cell Voltage | Maximum voltage for all cells in the string. |
| `CellVMaxMod` | uint16 |  | Max Cell Voltage Module | Module containing the cell with the maximum voltage. |
| `CellVMaxStk` | uint16 |  | Max Cell Voltage Stack | Stack containing the cell with the maximum voltage. |
| `CellVMin` | uint16 | V | Min Cell Voltage | Minimum voltage for all cells in the string. |
| `CellVMinMod` | uint16 |  | Min Cell Voltage Module | Module containing the cell with the minimum voltage. |
| `CellVMinStk` | uint16 |  | Min Cell Voltage Stack | Stack containing the cell with the minimum voltage. |
| `CellVAvg` | uint16 | V | Average Cell Voltage | Average voltage for all cells in the string. |
| `TmpMax` | int16 | C | Max Temperature | Maximum electrolyte temperature for all modules in the string. |
| `TmpMaxMod` | uint16 |  | Max Temperature Module | Module with the maximum temperature. |
| `TmpMin` | int16 | C | Min Temperature | Minimum electrolyte temperature for all modules in the string. |
| `TmpMinMod` | uint16 |  | Min Temperature Module | Module with the minimum temperature. |
| `TmpAvg` | int16 | C | Average Temperature | Average electrolyte temperature for all modules in the string. |
| `Evt1` | bitfield32 |  | String Event 1 | Alarms, warnings and status values. |
| `Evt2` | bitfield32 |  | String Event 2 | Alarms, warnings and status values. |
| `EvtVnd1` | bitfield32 |  | Vendor Event Bitfield 1 | Vendor defined events. |
| `EvtVnd2` | bitfield32 |  | Vendor Event Bitfield 2 | Vendor defined events. |
| `module[*].ModIdx` | uint16 |  | Module Index | Index of the module within the string. |
| `module[*].ModNStk` | uint16 |  | Stack Count | Number of stacks in this module. |
| `module[*].ModSt` | bitfield32 |  | Module Status | Current status of the module. |
| `module[*].ModSoC` | uint16 | % | Module State of Charge | State of charge for this module. |
| `module[*].ModOCV` | uint16 | V | Open Circuit Voltage | Open circuit voltage for this module. |
| `module[*].ModV` | uint16 | V | External Voltage | External voltage fo this module. |
| `module[*].ModCellVMax` | uint16 | V | Maximum Cell Voltage | Maximum voltage for all cells in this module. |
| `module[*].ModCellVMaxCell` | uint16 |  | Max Cell Voltage Cell | Cell with the maximum cell voltage. |
| `module[*].ModCellVMin` | uint16 | V | Minimum Cell Voltage | Minimum voltage for all cells in this module. |
| `module[*].ModCellVMinCell` | uint16 |  | Min Cell Voltage Cell | Cell with the minimum cell voltage. |
| `module[*].ModCellVAvg` | uint16 | V | Average Cell Voltage | Average voltage for all cells in this module. |
| `module[*].ModAnoTmp` | uint16 | C | Anolyte Temperature |  |
| `module[*].ModCatTmp` | uint16 | C | Catholyte Temperature |  |
| `module[*].ModConSt` | bitfield32 |  | Contactor Status |  |
| `module[*].ModEvt1` | bitfield32 |  | Module Event 1 | Alarms, warnings and status values. |
| `module[*].ModEvt2` | bitfield32 |  | Module Event 2 | Alarms, warnings and status values. |
| `module[*].ModConFail` | enum16 |  | Connection Failure Reason |  |
| `module[*].ModSetEna` | enum16 |  | Enable/Disable Module | Enables and disables the module. |
| `module[*].ModSetCon` | enum16 |  | Connect/Disconnect Module | Connects and disconnects the module. |
| `module[*].ModDisRsn` | enum16 |  | Disabled Reason | Reason why the module is currently disabled. |

Value names:

- `Evt1` (bitfield32): bit 0 `COMMUNICATION_ERROR`, bit 1 `OVER_TEMP_ALARM`, bit 2 `OVER_TEMP_WARNING`, bit 3 `UNDER_TEMP_ALARM`, bit 4 `UNDER_TEMP_WARNING`, bit 5 `OVER_CHARGE_CURRENT_ALARM`, bit 6 `OVER_CHARGE_CURRENT_WARNING`, bit 7 `OVER_DISCHARGE_CURRENT_ALARM`, bit 8 `OVER_DISCHARGE_CURRENT_WARNING`, bit 9 `OVER_VOLT_ALARM`, bit 10 `OVER_VOLT_WARNING`, bit 11 `UNDER_VOLT_ALARM`, bit 12 `UNDER_VOLT_WARNING`, bit 13 `UNDER_SOC_MIN_ALARM`, bit 14 `UNDER_SOC_MIN_WARNING`, bit 15 `OVER_SOC_MAX_ALARM`, bit 16 `OVER_SOC_MAX_WARNING`, bit 17 `VOLTAGE_IMBALANCE_WARNING`, bit 18 `RESERVED_1`, bit 19 `RESERVED_2`, bit 20 `CONTACTOR_ERROR`, bit 21 `FAN_ERROR`, bit 22 `GROUND_FAULT`, bit 23 `OPEN_DOOR_ERROR`, bit 24 `RESERVED_3`, bit 25 `OTHER_ALARM`, bit 26 `OTHER_WARNING`, bit 27 `FIRE_ALARM`, bit 28 `CONFIGURATION_ALARM`, bit 29 `CONFIGURATION_WARNING`
- `Evt2` (bitfield32): bit 0 `LEAK_ALARM`, bit 1 `PUMP_ALARM`, bit 2 `HIGH_PRESSURE_ALARM`, bit 3 `HIGH_PRESSURE_WARNING`, bit 4 `LOW_FLOW_ALARM`, bit 5 `LOW_FLOW_WARNING`
- `module[*].ModSt` (bitfield32): bit 0 `MODULE_ENABLED`, bit 1 `CONTACTOR_STATUS`
- `module[*].ModConSt` (bitfield32): bit 0 `CONTACTOR_0`, bit 1 `CONTACTOR_1`, bit 2 `CONTACTOR_2`, bit 3 `CONTACTOR_3`, bit 4 `CONTACTOR_4`, bit 5 `CONTACTOR_5`, bit 6 `CONTACTOR_6`, bit 7 `CONTACTOR_7`, bit 8 `CONTACTOR_8`, bit 9 `CONTACTOR_9`, bit 10 `CONTACTOR_10`, bit 11 `CONTACTOR_11`, bit 12 `CONTACTOR_12`, bit 13 `CONTACTOR_13`, bit 14 `CONTACTOR_14`, bit 15 `CONTACTOR_15`, bit 16 `CONTACTOR_16`, bit 17 `CONTACTOR_17`, bit 18 `CONTACTOR_18`, bit 19 `CONTACTOR_19`, bit 20 `CONTACTOR_20`, bit 21 `CONTACTOR_21`, bit 22 `CONTACTOR_22`, bit 23 `CONTACTOR_23`, bit 24 `CONTACTOR_24`, bit 25 `CONTACTOR_25`, bit 26 `CONTACTOR_26`, bit 27 `CONTACTOR_27`, bit 28 `CONTACTOR_28`, bit 29 `CONTACTOR_29`, bit 30 `CONTACTOR_30`
- `module[*].ModEvt1` (bitfield32): bit 0 `COMMUNICATION_ERROR`, bit 1 `OVER_TEMP_ALARM`, bit 2 `OVER_TEMP_WARNING`, bit 3 `UNDER_TEMP_ALARM`, bit 4 `UNDER_TEMP_WARNING`, bit 5 `OVER_CHARGE_CURRENT_ALARM`, bit 6 `OVER_CHARGE_CURRENT_WARNING`, bit 7 `OVER_DISCHARGE_CURRENT_ALARM`, bit 8 `OVER_DISCHARGE_CURRENT_WARNING`, bit 9 `OVER_VOLT_ALARM`, bit 10 `OVER_VOLT_WARNING`, bit 11 `UNDER_VOLT_ALARM`, bit 12 `UNDER_VOLT_WARNING`, bit 13 `UNDER_SOC_MIN_ALARM`, bit 14 `UNDER_SOC_MIN_WARNING`, bit 15 `OVER_SOC_MAX_ALARM`, bit 16 `OVER_SOC_MAX_WARNING`, bit 17 `VOLTAGE_IMBALANCE_WARNING`, bit 18 `RESERVED_1`, bit 19 `RESERVED_2`, bit 20 `CONTACTOR_ERROR`, bit 21 `FAN_ERROR`, bit 22 `GROUND_FAULT`, bit 23 `OPEN_DOOR_ERROR`, bit 24 `RESERVED_3`, bit 25 `RESERVED_4`, bit 26 `RESERVED_5`, bit 27 `FIRE_ALARM`, bit 28 `MODULE_CONFIGURATION_ALARM`, bit 29 `MODULE_CONFIGURATION_WARNING`
- `module[*].ModEvt2` (bitfield32): bit 0 `LEAK_ALARM`, bit 1 `PUMP_ALARM`, bit 2 `HIGH_PRESSURE_ALARM`, bit 3 `HIGH_PRESSURE_WARNING`, bit 4 `LOW_FLOW_ALARM`, bit 5 `LOW_FLOW_WARNING`
- `module[*].ModConFail` (enum16): 0 `NO_FAILURE`, 1 `BUTTON_PUSHED`, 2 `MODULE_GROUND_FAULT`, 3 `OUTSIDE_VOLTAGE_RANGE`, 4 `MODULE_NOT_ENABLED`, 5 `FUSE_OPEN`, 6 `CONTACTOR_FAILURE`, 7 `PRECHARGE_FAILURE`, 8 `MODULE_FAULT`
- `module[*].ModSetEna` (enum16): 1 `ENABLE_MODULE`, 2 `DISABLE_MODULE`
- `module[*].ModSetCon` (enum16): 1 `CONNECT_MODULE`, 2 `DISCONNECT_MODULE`
- `module[*].ModDisRsn` (enum16): 0 `NONE`, 1 `FAULT`, 2 `MAINTENANCE`, 3 `EXTERNAL`, 4 `OTHER`

## Model 808: Flow Battery Module Model

Name in records: `flow_battery_module`.

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `ModuleTBD` | uint16 |  | Module Points To Be Determined |  |
| `stack[*].StackTBD` | uint16 |  | Stack Points To Be Determined |  |

## Model 809: Flow Battery Stack Model

Name in records: `flow_battery_stack`.

| Point | Type | Units | Label | Description |
|-------|------|-------|-------|-------------|
| `StackTBD` | uint16 |  | Stack Points To Be Determined |  |
| `cell[*].CellTBD` | uint16 |  | Cell Points To Be Determined |  |
