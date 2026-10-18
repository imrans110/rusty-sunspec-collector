# Supported SunSpec models

<!-- Generated from the bundled SunSpec definitions by
     UPDATE_MODEL_DOCS=1 cargo test -p sunspec-parser --test models_docs
     Do not edit by hand. -->

The collector bundles 112 official SunSpec model definitions (3217 points) and decodes every one of them. Each page below lists a group of models with their points: the name records use, the type, units, label and description from the definitions, and the names of enum values and bitfield bits. In a point name, `[*]` stands for a repeating group's index (`module[*].DCA` covers `module[0].DCA`, `module[1].DCA` and so on).

The same data is available from a running collector at `/models` and from `collector-app --print-models`.

## [Common and network](common.md)

| Model | Name in records | Label | Points |
|-------|-----------------|-------|--------|
| [1](common.md#model-1-common) | `common` | Common | 6 |
| [2](common.md#model-2-basic-aggregator) | `aggregator` | Basic Aggregator | 10 |
| [3](common.md#model-3-secure-dataset-read-request) | `model_3` | Secure Dataset Read Request | 58 |
| [4](common.md#model-4-secure-dataset-read-response) | `model_4` | Secure Dataset Read Response | 60 |
| [5](common.md#model-5-secure-write-request) | `model_5` | Secure Write Request | 88 |
| [6](common.md#model-6-secure-write-sequential-request) | `model_6` | Secure Write Sequential Request | 89 |
| [7](common.md#model-7-secure-write-response-model-draft-1) | `model_7` | Secure Write Response Model (DRAFT 1) | 9 |
| [8](common.md#model-8-get-device-security-certificate) | `model_8` | Get Device Security Certificate | 3 |
| [9](common.md#model-9-set-operator-security-certificate) | `model_9` | Set Operator Security Certificate | 92 |
| [10](common.md#model-10-communication-interface-header) | `model_10` | Communication Interface Header | 3 |
| [11](common.md#model-11-ethernet-link-layer) | `model_11` | Ethernet Link Layer | 7 |
| [12](common.md#model-12-ipv4) | `model_12` | IPv4 | 15 |
| [13](common.md#model-13-ipv6) | `model_13` | IPv6 | 15 |
| [14](common.md#model-14-proxy-server) | `model_14` | Proxy Server | 8 |
| [15](common.md#model-15-interface-counters-model) | `model_15` | Interface Counters Model | 12 |
| [16](common.md#model-16-simple-ip-network) | `model_16` | Simple IP Network | 10 |
| [17](common.md#model-17-serial-interface) | `model_17` | Serial Interface | 8 |
| [18](common.md#model-18-cellular-link) | `model_18` | Cellular Link | 5 |
| [19](common.md#model-19-ppp-link) | `model_19` | PPP Link | 9 |

## [Inverters and inverter controls](inverters.md)

| Model | Name in records | Label | Points |
|-------|-----------------|-------|--------|
| [101](inverters.md#model-101-inverter-single-phase) | `inverter_single_phase` | Inverter (Single Phase) | 31 |
| [102](inverters.md#model-102-inverter-split-phase) | `inverter_split_phase` | Inverter (Split-Phase) | 31 |
| [103](inverters.md#model-103-inverter-three-phase) | `inverter_three_phase` | Inverter (Three Phase) | 31 |
| [111](inverters.md#model-111-inverter-single-phase-float) | `inverter_single_phase_float` | Inverter (Single Phase) FLOAT | 31 |
| [112](inverters.md#model-112-inverter-split-phase-float) | `inverter_split_phase_float` | Inverter (Split Phase) FLOAT | 31 |
| [113](inverters.md#model-113-inverter-three-phase-float) | `inverter_three_phase_float` | Inverter (Three Phase) FLOAT | 31 |
| [120](inverters.md#model-120-nameplate) | `nameplate` | Nameplate | 16 |
| [121](inverters.md#model-121-basic-settings) | `settings` | Basic Settings | 20 |
| [122](inverters.md#model-122-measurements_status) | `status` | Measurements\_Status | 17 |
| [123](inverters.md#model-123-immediate-controls) | `controls` | Immediate Controls | 21 |
| [124](inverters.md#model-124-storage) | `storage_basic` | Storage | 16 |
| [125](inverters.md#model-125-pricing) | `pricing` | Pricing | 6 |
| [126](inverters.md#model-126-static-volt-var) | `volt_var` | Static Volt-VAR | 54 |
| [127](inverters.md#model-127-freq-watt-param) | `freq_watt_param` | Freq-Watt Param | 6 |
| [128](inverters.md#model-128-dynamic-reactive-current) | `reactive_current` | Dynamic Reactive Current | 11 |
| [129](inverters.md#model-129-lvrtd) | `lvrt` | LVRTD | 50 |
| [130](inverters.md#model-130-hvrtd) | `hvrt` | HVRTD | 50 |
| [131](inverters.md#model-131-watt-pf) | `watt_pf` | Watt-PF | 53 |
| [132](inverters.md#model-132-volt-watt) | `volt_watt` | Volt-Watt | 54 |
| [133](inverters.md#model-133-basic-scheduling) | `schedule` | Basic Scheduling | 34 |
| [134](inverters.md#model-134-freq-watt-crv) | `freq_watt` | Freq-Watt Crv | 58 |
| [135](inverters.md#model-135-lfrt) | `lfrt` | LFRT | 50 |
| [136](inverters.md#model-136-hfrt) | `hfrt` | HFRT | 50 |
| [137](inverters.md#model-137-lvrtc) | `lvrtc` | LVRTC | 50 |
| [138](inverters.md#model-138-hvrtc) | `hvrtc` | HVRTC | 50 |
| [139](inverters.md#model-139-lvrtx) | `lvrtx` | LVRTX | 51 |
| [140](inverters.md#model-140-hvrtx) | `hvrtx` | HVRTX | 51 |
| [141](inverters.md#model-141-lfrtc) | `lfrtc` | LFRTC | 50 |
| [142](inverters.md#model-142-hfrtc) | `hfrtc` | HFRTC | 50 |
| [143](inverters.md#model-143-lfrtx) | `lfrtx` | LFRTX | 51 |
| [144](inverters.md#model-144-hfrtx) | `hfrtx` | HFRTX | 51 |
| [145](inverters.md#model-145-extended-settings) | `ext_settings` | Extended Settings | 7 |
| [160](inverters.md#model-160-multiple-mppt-inverter-extension-model) | `mppt` | Multiple MPPT Inverter Extension Model | 13 |

## [Meters](meters.md)

| Model | Name in records | Label | Points |
|-------|-----------------|-------|--------|
| [201](meters.md#model-201-meter-single-phase-single-phase-an-or-ab-meter) | `ac_meter_an_or_ab` | Meter (Single Phase) single phase (AN or AB) meter | 62 |
| [202](meters.md#model-202-split-single-phase-abn-meter) | `ac_meter_abn` | split single phase (ABN) meter | 62 |
| [203](meters.md#model-203-wye-connect-three-phase-abcn-meter) | `ac_meter_abcn` | wye-connect three phase (abcn) meter | 62 |
| [204](meters.md#model-204-delta-connect-three-phase-abc-meter) | `ac_meter_abc` | delta-connect three phase (abc) meter | 62 |
| [211](meters.md#model-211-single-phase-an-or-ab-meter) | `ac_meter_an_or_ab_float` | single phase (AN or AB) meter | 62 |
| [212](meters.md#model-212-split-single-phase-abn-meter) | `ac_meter_abn_float` | split single phase (ABN) meter | 62 |
| [213](meters.md#model-213-wye-connect-three-phase-abcn-meter) | `ac_meter_abcn_float` | wye-connect three phase (abcn) meter | 62 |
| [214](meters.md#model-214-delta-connect-three-phase-abc-meter) | `ac_meter_abc_float` | delta-connect three phase (abc) meter | 62 |
| [220](meters.md#model-220-secure-ac-meter-selected-readings) | `ac_meter_secure` | Secure AC Meter Selected Readings | 22 |

## [Environmental sensors](environmental.md)

| Model | Name in records | Label | Points |
|-------|-----------------|-------|--------|
| [302](environmental.md#model-302-irradiance-model) | `irradiance` | Irradiance Model | 5 |
| [303](environmental.md#model-303-back-of-module-temperature-model) | `bom_temp` | Back of Module Temperature Model | 1 |
| [304](environmental.md#model-304-inclinometer-model) | `inclinometer` | Inclinometer Model | 3 |
| [305](environmental.md#model-305-gps) | `location` | GPS | 6 |
| [306](environmental.md#model-306-reference-point-model) | `ref_point` | Reference Point Model | 4 |
| [307](environmental.md#model-307-base-met) | `base_met` | Base Met | 11 |
| [308](environmental.md#model-308-mini-met-model) | `mini_met` | Mini Met Model | 4 |

## [String combiners](string-combiners.md)

| Model | Name in records | Label | Points |
|-------|-----------------|-------|--------|
| [401](string-combiners.md#model-401-string-combiner-current) | `string_combiner_current` | String Combiner (Current) | 13 |
| [402](string-combiners.md#model-402-string-combiner-advanced) | `string_combiner_advanced` | String Combiner (Advanced) | 21 |
| [403](string-combiners.md#model-403-string-combiner-current) | `string_combiner_current_input` | String Combiner (Current) | 13 |
| [404](string-combiners.md#model-404-string-combiner-advanced) | `string_combiner_advanced_inputs` | String Combiner (Advanced) | 21 |

## [Solar modules](modules.md)

| Model | Name in records | Label | Points |
|-------|-----------------|-------|--------|
| [501](modules.md#model-501-solar-module) | `solar_module_float` | Solar Module | 17 |
| [502](modules.md#model-502-solar-module) | `solar_module` | Solar Module | 17 |

## [Trackers](trackers.md)

| Model | Name in records | Label | Points |
|-------|-----------------|-------|--------|
| [601](trackers.md#model-601-tracker-controller-draft-2) | `tracker_controller` | Tracker Controller DRAFT 2 | 19 |

## [DER (IEEE 1547-2018)](der.md)

| Model | Name in records | Label | Points |
|-------|-----------------|-------|--------|
| [701](der.md#model-701-der-ac-measurement) | `DERMeasureAC` | DER AC Measurement | 60 |
| [702](der.md#model-702-der-capacity) | `DERCapacity` | DER Capacity | 42 |
| [703](der.md#model-703-enter-service) | `DEREnterService` | Enter Service | 9 |
| [704](der.md#model-704-der-ac-controls) | `DERCtlAC` | DER AC Controls | 45 |
| [705](der.md#model-705-der-volt-var) | `DERVoltVar` | DER Volt-Var | 19 |
| [706](der.md#model-706-der-volt-watt) | `DERVoltWatt` | DER Volt-Watt | 14 |
| [707](der.md#model-707-der-trip-lv) | `DERTripLV` | DER Trip LV | 15 |
| [708](der.md#model-708-der-trip-hv) | `DERTripHV` | DER Trip HV | 15 |
| [709](der.md#model-709-der-trip-lf) | `DERTripLF` | DER Trip LF | 15 |
| [710](der.md#model-710-der-trip-hf) | `DERTripHF` | DER Trip HF | 15 |
| [711](der.md#model-711-der-frequency-droop) | `DERFreqDroop` | DER Frequency Droop | 14 |
| [712](der.md#model-712-der-watt-var) | `DERWattVar` | DER Watt-Var | 14 |
| [713](der.md#model-713-der-storage-capacity) | `DERStorageCapacity` | DER Storage Capacity | 5 |
| [714](der.md#model-714-der-dc-measurement) | `DERMeasureDC` | DER DC Measurement | 17 |
| [715](der.md#model-715-derctl) | `DERCtl` | DERCtl | 5 |

## [Storage](storage.md)

| Model | Name in records | Label | Points |
|-------|-----------------|-------|--------|
| [801](storage.md#model-801-energy-storage-base-model-deprecated) | `storage` | Energy Storage Base Model (DEPRECATED) | 1 |
| [802](storage.md#model-802-battery-base-model) | `battery` | Battery Base Model | 44 |
| [803](storage.md#model-803-lithium-ion-battery-bank-model) | `lithium_ion_bank` | Lithium-Ion Battery Bank Model | 44 |
| [804](storage.md#model-804-lithium-ion-string-model) | `lithium_ion_string` | Lithium-Ion String Model | 41 |
| [805](storage.md#model-805-lithium-ion-module-model) | `lithium-ion-module` | Lithium-Ion Module Model | 23 |
| [806](storage.md#model-806-flow-battery-model) | `flow_battery` | Flow Battery Model | 2 |
| [807](storage.md#model-807-flow-battery-string-model) | `flow_battery_string` | Flow Battery String Model | 44 |
| [808](storage.md#model-808-flow-battery-module-model) | `flow_battery_module` | Flow Battery Module Model | 2 |
| [809](storage.md#model-809-flow-battery-stack-model) | `flow_battery_stack` | Flow Battery Stack Model | 2 |

## [Test and vendor models](other.md)

| Model | Name in records | Label | Points |
|-------|-----------------|-------|--------|
| [63001](other.md#model-63001-sunspec-test-model-1) | `model_63001` | SunSpec Test Model 1 | 59 |
| [63002](other.md#model-63002-sunspec-test-model-2) | `model_63002` | SunSpec Test Model 2 | 2 |
| [64001](other.md#model-64001-veris-status-and-configuration) | `model_64001` | Veris Status and Configuration | 35 |
| [64020](other.md#model-64020-mersen-greenstring) | `model_64020` | Mersen GreenString | 30 |
| [64101](other.md#model-64101-eltek-inverter-extension) | `model_64101` | Eltek Inverter Extension | 7 |
| [64111](other.md#model-64111-basic-charge-controller) | `model_64111` | Basic Charge Controller | 18 |
| [64112](other.md#model-64112-outback-fm-charge-controller) | `model_64112` | OutBack FM Charge Controller | 58 |
| [64410](other.md#model-64410-dc-simulator-control-interface) | `DCSimInterface` | DC Simulator Control Interface | 32 |
| [64411](other.md#model-64411-ac-simulator-control-interface) | `ACSimInterface` | AC Simulator Control Interface | 51 |
| [64412](other.md#model-64412-der-cyber-exploitation) | `DERExploitation` | DER Cyber Exploitation | 43 |
| [64413](other.md#model-64413-pv-curves) | `PVSimCurves` | PV Curves | 5 |
| [64414](other.md#model-64414-der-simulation-controls) | `DERSimControls` | DER Simulation Controls | 9 |
| [64415](other.md#model-64415-csip-client-control) | `CSIPControl` | CSIP Client Control | 5 |
