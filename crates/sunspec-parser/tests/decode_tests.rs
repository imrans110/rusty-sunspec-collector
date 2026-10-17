use std::collections::HashMap;

use sunspec_parser::{
    catalog, decode_model, decode_model_labeled, model_definition, model_info, model_name,
    supported_models, PointType,
};
use types::PointValue;

/// A model's registers (header included), zeroed, with `ID` and `L` filled in.
fn model_registers(model_id: u16, body_len: u16) -> Vec<u16> {
    let mut registers = vec![0u16; usize::from(body_len) + 2];
    registers[0] = model_id;
    registers[1] = body_len;
    registers
}

fn set(registers: &mut [u16], model_id: u16, point: &str, values: &[u16]) {
    let model = model_definition(model_id).expect("bundled model");
    let (offset, _) = model.point_offset(point).expect("point exists");
    let offset = usize::from(offset);
    registers[offset..offset + values.len()].copy_from_slice(values);
}

fn text(value: &str, registers: usize) -> Vec<u16> {
    let mut bytes = value.as_bytes().to_vec();
    bytes.resize(registers * 2, 0);
    bytes
        .chunks(2)
        .map(|pair| u16::from_be_bytes([pair[0], pair[1]]))
        .collect()
}

fn points(model_id: u16, registers: &[u16]) -> HashMap<String, PointValue> {
    decode_model(model_id, registers)
        .expect("model has a definition")
        .into_iter()
        .collect()
}

/// Model IDs of the `models/model_<id>.json` files shipped with the crate.
fn model_files() -> Vec<u16> {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("models");
    let mut ids: Vec<u16> = std::fs::read_dir(dir)
        .expect("read models directory")
        .filter_map(|entry| {
            let name = entry.expect("directory entry").file_name();
            let name = name.to_str()?;
            name.strip_prefix("model_")?
                .strip_suffix(".json")?
                .parse()
                .ok()
        })
        .collect();
    ids.sort_unstable();
    ids
}

#[test]
fn every_model_file_is_bundled() {
    let files = model_files();
    assert_eq!(supported_models(), files);
    assert!(files.len() >= 112, "only {} models", files.len());
    for model in [1, 103, 160, 211, 213, 302, 502, 714, 802, 64101] {
        assert!(files.contains(&model), "model {model} missing");
    }
    assert_eq!(model_name(103), "inverter_three_phase");
    assert_eq!(model_name(701), "DERMeasureAC");
    assert_eq!(model_name(9_999), "model_9999");
    assert!(decode_model(9_999, &[9_999, 0]).is_none());
}

/// Every bundled model decodes a zeroed register map: each top-level point appears
/// once, names are unique (records carry them as map keys), and short or long maps
/// never panic.
#[test]
fn every_bundled_model_decodes_its_fixed_points() {
    for model_id in supported_models() {
        let model = model_definition(model_id).unwrap();
        let fixed = model.group.fixed_size();
        let mut registers = vec![0u16; fixed];
        registers[0] = model_id;
        registers[1] = (fixed - 2) as u16;

        let names: Vec<String> = decode_model(model_id, &registers)
            .unwrap()
            .into_iter()
            .map(|(name, _)| name)
            .collect();
        let unique: std::collections::BTreeSet<&String> = names.iter().collect();
        assert_eq!(unique.len(), names.len(), "model {model_id} repeats a name");
        for point in &model.group.points {
            let omitted = matches!(point.kind, PointType::Pad | PointType::Sunssf)
                || point.name == "ID"
                || point.name == "L";
            assert_eq!(
                names.contains(&point.name),
                !omitted,
                "model {model_id} point {}",
                point.name
            );
        }

        // Repeat-to-fill groups take the extra registers; nothing else may break.
        registers.resize(fixed + 200, 0);
        let filled = decode_model(model_id, &registers).unwrap();
        let unique: std::collections::BTreeSet<&String> =
            filled.iter().map(|(name, _)| name).collect();
        assert_eq!(
            unique.len(),
            filled.len(),
            "model {model_id} with fill groups"
        );
        decode_model(model_id, &registers[..fixed / 2]).unwrap();
    }
}

fn float(value: f32) -> [u16; 2] {
    let bits = value.to_bits();
    [(bits >> 16) as u16, bits as u16]
}

#[test]
fn float_meters_decode_without_scale_factors() {
    for model_id in [211, 212, 213, 214] {
        let fixed = model_definition(model_id).unwrap().group.fixed_size();
        let mut registers = model_registers(model_id, fixed as u16 - 2);
        set(&mut registers, model_id, "W", &float(-1234.5));
        set(&mut registers, model_id, "Hz", &float(50.0));
        set(&mut registers, model_id, "A", &float(f32::NAN));

        let points = points(model_id, &registers);

        assert_eq!(points["W"], PointValue::Float(-1234.5), "model {model_id}");
        assert_eq!(points["Hz"], PointValue::Float(50.0), "model {model_id}");
        assert_eq!(points["A"], PointValue::Null, "model {model_id}: NaN");
    }
}

#[test]
fn three_phase_inverter_values_are_scaled() {
    let mut registers = model_registers(103, 50);
    set(&mut registers, 103, "A", &[1234]);
    set(&mut registers, 103, "A_SF", &[(-2i16) as u16]);
    set(&mut registers, 103, "PhVphA", &[2301]);
    set(&mut registers, 103, "V_SF", &[(-1i16) as u16]);
    set(&mut registers, 103, "W", &[4321]);
    set(&mut registers, 103, "W_SF", &[(-1i16) as u16]);
    set(&mut registers, 103, "Hz", &[5000]);
    set(&mut registers, 103, "Hz_SF", &[(-2i16) as u16]);
    set(&mut registers, 103, "WH", &[0x0001, 0x0005]);
    set(&mut registers, 103, "WH_SF", &[0]);
    set(&mut registers, 103, "St", &[4]);

    let points = points(103, &registers);

    assert_eq!(points["A"], PointValue::Float(12.34));
    assert_eq!(points["PhVphA"], PointValue::Float(230.1));
    assert_eq!(points["W"], PointValue::Float(432.1));
    assert_eq!(points["Hz"], PointValue::Float(50.0));
    assert_eq!(points["WH"], PointValue::Float(65_541.0));
    assert_eq!(points["St"], PointValue::Int(4), "enums stay integers");
}

#[test]
fn not_implemented_values_are_null() {
    let mut registers = model_registers(103, 50);
    set(&mut registers, 103, "W", &[0x8000]);
    set(&mut registers, 103, "W_SF", &[0]);
    set(&mut registers, 103, "VA", &[100]);
    set(&mut registers, 103, "VA_SF", &[0x8000]);
    set(&mut registers, 103, "St", &[0xFFFF]);
    set(&mut registers, 103, "Evt1", &[0xFFFF, 0xFFFF]);

    let points = points(103, &registers);

    assert_eq!(points["W"], PointValue::Null, "int16 sentinel");
    assert_eq!(
        points["VA"],
        PointValue::Null,
        "scale factor not implemented"
    );
    assert_eq!(points["St"], PointValue::Null, "enum16 sentinel");
    assert_eq!(points["Evt1"], PointValue::Null, "bitfield32 sentinel");
    assert_eq!(
        points["WH"],
        PointValue::Null,
        "acc32 of 0 is not implemented"
    );
}

#[test]
fn header_scale_factors_and_padding_are_omitted() {
    let names: Vec<String> = decode_model(103, &model_registers(103, 50))
        .unwrap()
        .into_iter()
        .map(|(name, _)| name)
        .collect();

    assert!(names.contains(&"W".to_string()));
    for omitted in ["ID", "L", "W_SF", "A_SF"] {
        assert!(
            !names.iter().any(|name| name == omitted),
            "{omitted} should be omitted"
        );
    }
}

#[test]
fn common_model_strings_are_decoded() {
    let mut registers = model_registers(1, 66);
    set(&mut registers, 1, "Mn", &text("Rusty Sun", 16));
    set(&mut registers, 1, "Md", &text("RS-10K", 16));
    set(&mut registers, 1, "DA", &[1]);

    let points = points(1, &registers);

    assert_eq!(points["Mn"], PointValue::Text("Rusty Sun".to_string()));
    assert_eq!(points["Md"], PointValue::Text("RS-10K".to_string()));
    assert_eq!(points["SN"], PointValue::Null, "empty string");
    assert_eq!(points["DA"], PointValue::Int(1));
    assert!(!points.contains_key("Pad"));
}

#[test]
fn mppt_repeating_modules_use_top_level_scale_factors() {
    // 8 fixed body registers + 2 modules of 20.
    let mut registers = model_registers(160, 48);
    set(&mut registers, 160, "DCA_SF", &[(-1i16) as u16]);
    set(&mut registers, 160, "N", &[2]);
    // Module 0 starts right after the 10 fixed registers: ID, IDStr (8), then DCA.
    registers[10] = 1;
    registers[11..19].copy_from_slice(&text("MPPT-1", 8));
    registers[10 + 9] = 55;
    registers[30 + 9] = 66;

    let points = points(160, &registers);

    assert_eq!(points["N"], PointValue::Int(2));
    assert_eq!(
        points["module[0].IDStr"],
        PointValue::Text("MPPT-1".to_string())
    );
    assert_eq!(points["module[0].DCA"], PointValue::Float(5.5));
    assert_eq!(points["module[1].DCA"], PointValue::Float(6.6));
    assert!(!points.contains_key("module[2].DCA"));
}

#[test]
fn float_models_decode_and_nan_is_null() {
    let mut registers = model_registers(113, 60);
    let watts = 1234.5f32.to_bits();
    set(
        &mut registers,
        113,
        "W",
        &[(watts >> 16) as u16, watts as u16],
    );
    set(&mut registers, 113, "Hz", &[0x7FC0, 0x0000]);

    let points = points(113, &registers);

    assert_eq!(points["W"], PointValue::Float(1234.5));
    assert_eq!(points["Hz"], PointValue::Null);
}

#[test]
fn truncated_registers_decode_what_is_present() {
    let registers = &model_registers(103, 50)[..10];

    let points = points(103, registers);

    assert!(points.contains_key("A"));
    assert!(!points.contains_key("W"));
}

/// Registers in the top-level (non-repeating) part of a model, header included.
fn top_level_size(model_id: u16) -> usize {
    model_definition(model_id)
        .unwrap()
        .group
        .points
        .iter()
        .map(|point| usize::from(point.size))
        .sum()
}

#[test]
fn ieee_1547_measurements_are_scaled() {
    let mut registers = model_registers(701, top_level_size(701) as u16 - 2);
    set(&mut registers, 701, "St", &[1]);
    set(&mut registers, 701, "W", &[5432]);
    set(&mut registers, 701, "W_SF", &[(-1i16) as u16]);
    set(&mut registers, 701, "Hz", &[0, 60_000]);
    set(&mut registers, 701, "Hz_SF", &[(-3i16) as u16]);
    set(&mut registers, 701, "TotWhInj", &[0, 0, 0x075B, 0xCD15]);
    set(&mut registers, 701, "TotWh_SF", &[0]);

    let points = points(701, &registers);

    assert_eq!(points["St"], PointValue::Int(1));
    assert_eq!(points["W"], PointValue::Float(543.2));
    assert_eq!(
        points["Hz"],
        PointValue::Float(60.0),
        "uint32 with scale factor"
    );
    assert_eq!(
        points["TotWhInj"],
        PointValue::Float(123_456_789.0),
        "uint64 energy"
    );
}

#[test]
fn storage_capacity_state_of_charge() {
    let mut registers = model_registers(713, top_level_size(713) as u16 - 2);
    set(&mut registers, 713, "SoC", &[875]);
    set(&mut registers, 713, "Pct_SF", &[(-1i16) as u16]);

    let points = points(713, &registers);

    assert_eq!(points["SoC"], PointValue::Float(87.5));
}

#[test]
fn volt_var_curves_are_sized_by_top_level_counts() {
    // Model 705: NCrv curves, each with 10 fixed registers and NPt points of (V, Var).
    let (curves, curve_points, crv_fixed, pt_size) = (2usize, 3usize, 10usize, 2usize);
    let top = top_level_size(705);
    let curve_size = crv_fixed + curve_points * pt_size;
    let mut registers = model_registers(705, (top - 2 + curves * curve_size) as u16);
    set(&mut registers, 705, "NCrv", &[curves as u16]);
    set(&mut registers, 705, "NPt", &[curve_points as u16]);
    set(&mut registers, 705, "V_SF", &[(-1i16) as u16]);
    set(&mut registers, 705, "DeptRef_SF", &[0]);
    let point = |curve: usize, index: usize| top + curve * curve_size + crv_fixed + index * pt_size;
    registers[top] = 3; // Crv[0].ActPt
    registers[point(1, 2)] = 1050; // Crv[1].Pt[2].V
    registers[point(1, 2) + 1] = (-300i16) as u16; // Crv[1].Pt[2].Var

    let points = points(705, &registers);

    assert_eq!(points["Crv[0].ActPt"], PointValue::Int(3));
    assert_eq!(
        points["Crv[1].Pt[2].V"],
        PointValue::Float(105.0),
        "top-level V_SF"
    );
    assert_eq!(points["Crv[1].Pt[2].Var"], PointValue::Float(-300.0));
    assert!(points.contains_key("Crv[0].Pt[0].V"));
    assert!(!points.contains_key("Crv[2].ActPt"));
    assert!(!points.contains_key("Crv[1].Pt[3].V"));
}

#[test]
fn trip_curves_nest_fixed_groups_inside_repeating_ones() {
    // Model 707: Crv[i] = ReadOnly, then MustTrip, MayTrip and MomCess, each an ActPt
    // followed by NPt points of (V, Tms as uint32).
    let (curve_points, pt_size) = (2usize, 3usize);
    let trip_group = 1 + curve_points * pt_size;
    let top = top_level_size(707);
    let mut registers = model_registers(707, (top - 2 + 1 + 3 * trip_group) as u16);
    set(&mut registers, 707, "NCrvSet", &[1]);
    set(&mut registers, 707, "NPt", &[curve_points as u16]);
    set(&mut registers, 707, "V_SF", &[(-1i16) as u16]);
    set(&mut registers, 707, "Tms_SF", &[(-2i16) as u16]);
    let may_trip_pt1 = top + 1 + trip_group + 1 + pt_size;
    registers[may_trip_pt1] = 880;
    registers[may_trip_pt1 + 1..may_trip_pt1 + 3].copy_from_slice(&[0, 150]);

    let points = points(707, &registers);

    assert_eq!(points["Crv[0].MayTrip.Pt[1].V"], PointValue::Float(88.0));
    assert_eq!(points["Crv[0].MayTrip.Pt[1].Tms"], PointValue::Float(1.5));
    assert!(points.contains_key("Crv[0].MomCess.Pt[1].V"));
    assert!(!points.contains_key("Crv[0].MomCess.Pt[2].V"));
}

#[test]
fn unimplemented_counts_produce_no_repetitions() {
    let top = top_level_size(705);
    let mut registers = model_registers(705, (top - 2) as u16);
    set(&mut registers, 705, "NCrv", &[0xFFFF]);
    set(&mut registers, 705, "NPt", &[0xFFFF]);

    let points = points(705, &registers);

    assert!(points.keys().all(|name| !name.starts_with("Crv[")));
}

fn labels(model_id: u16, registers: &[u16]) -> HashMap<String, String> {
    decode_model_labeled(model_id, registers)
        .expect("model has a definition")
        .labels
        .into_iter()
        .collect()
}

#[test]
fn enum_and_bitfield_values_get_their_names() {
    let mut registers = model_registers(103, 50);
    set(&mut registers, 103, "St", &[4]);
    set(&mut registers, 103, "Evt1", &[0, 0b101]);
    set(&mut registers, 103, "Evt2", &[0, 0]);

    let labels = labels(103, &registers);

    assert_eq!(labels["St"], "MPPT");
    assert_eq!(labels["Evt1"], "GROUND_FAULT,AC_DISCONNECT", "bits 0 and 2");
    assert_eq!(labels["Evt2"], "", "no bit set");
    assert!(
        !labels.contains_key("W"),
        "only enums and bitfields are labelled"
    );
    let decoded = decode_model_labeled(103, &registers).unwrap();
    assert_eq!(decoded.points, decode_model(103, &registers).unwrap());
}

#[test]
fn unknown_and_unimplemented_values_get_no_name_and_unnamed_bits_get_numbers() {
    let mut registers = model_registers(103, 50);
    set(&mut registers, 103, "St", &[99]);
    set(&mut registers, 103, "StVnd", &[0xFFFF]);
    set(&mut registers, 103, "Evt1", &[0x0010, 0x0001]);

    let labels = labels(103, &registers);

    assert!(!labels.contains_key("St"), "99 is not a defined state");
    assert!(!labels.contains_key("StVnd"), "not implemented");
    assert_eq!(labels["Evt1"], "GROUND_FAULT,bit20");
}

#[test]
fn repeating_groups_are_labelled_per_instance() {
    let mut registers = model_registers(160, 48);
    set(&mut registers, 160, "N", &[2]);
    // Modules start after 10 fixed registers and are 20 long; DCSt (an enum) is 17
    // registers into a module (after ID, IDStr, DCA, DCV, DCW, DCWH, Tms and Tmp).
    registers[10 + 20 + 17] = 4;

    let labels = labels(160, &registers);

    assert_eq!(
        labels.get("module[1].DCSt").map(String::as_str),
        Some("MPPT")
    );
}

/// `module[3].DCA` -> `module[*].DCA`, as the catalog names repeating groups.
fn wildcard(name: &str) -> String {
    let mut out = String::new();
    let mut in_index = false;
    for ch in name.chars() {
        match ch {
            '[' => {
                in_index = true;
                out.push_str("[*]");
            }
            ']' => in_index = false,
            _ if in_index => {}
            _ => out.push(ch),
        }
    }
    out
}

#[test]
fn the_catalog_describes_every_point_records_carry() {
    let catalog = catalog();
    assert_eq!(catalog.len(), supported_models().len());
    for info in &catalog {
        let model = model_definition(info.id).unwrap();
        let fixed = model.group.fixed_size();
        let mut registers = vec![0u16; fixed + 200];
        registers[0] = info.id;
        registers[1] = (fixed + 198) as u16;
        let names: Vec<&str> = info
            .points
            .iter()
            .map(|point| point.name.as_str())
            .collect();
        for (name, _) in decode_model(info.id, &registers).unwrap() {
            assert!(
                names.contains(&wildcard(&name).as_str()),
                "model {}: {name} is not in the catalog",
                info.id
            );
        }
    }
}

#[test]
fn the_catalog_carries_units_labels_and_symbols() {
    let inverter = model_info(103).unwrap();
    assert_eq!(inverter.name, "inverter_three_phase");
    assert_eq!(inverter.label.as_deref(), Some("Inverter (Three Phase)"));
    let amps = inverter
        .points
        .iter()
        .find(|point| point.name == "A")
        .unwrap();
    assert_eq!(amps.units.as_deref(), Some("A"));
    assert_eq!(amps.label.as_deref(), Some("Amps"));
    let state = inverter
        .points
        .iter()
        .find(|point| point.name == "St")
        .unwrap();
    assert!(state
        .symbols
        .iter()
        .any(|symbol| symbol.name == "MPPT" && symbol.value == 4));
    assert!(inverter.points.iter().all(|point| point.name != "W_SF"));

    let mppt = model_info(160).unwrap();
    assert!(mppt
        .points
        .iter()
        .any(|point| point.name == "module[*].DCA"));
    assert!(model_info(9_999).is_none());
}
