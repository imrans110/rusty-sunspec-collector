use std::collections::HashMap;

use sunspec_parser::{decode_model, model_definition, model_name, supported_models};
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

#[test]
fn every_bundled_model_definition_loads() {
    assert_eq!(
        supported_models(),
        vec![
            1, 101, 102, 103, 111, 112, 113, 120, 121, 122, 123, 124, 160, 201, 202, 203, 204, 802
        ]
    );
    assert_eq!(model_name(103), "inverter_three_phase");
    assert_eq!(model_name(9_999), "model_9999");
    assert!(decode_model(9_999, &[9_999, 0]).is_none());
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
