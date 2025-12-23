use sunspec_parser::{
    parse_models_from_json, parse_models_from_registers, parse_models_from_registers_lenient,
    parse_models_from_xml, ModelCatalog,
};

#[test]
fn parse_json_fixture_models() {
    let data = include_str!("fixtures/models.json");
    let models = parse_models_from_json(data).expect("json parse");
    assert_eq!(models.len(), 2);
    assert_eq!(models[0].id, 1);
    assert_eq!(models[0].name, "common");
    assert_eq!(models[0].length, 68);
    assert_eq!(models[1].id, 103);
    assert_eq!(models[1].name, "three_phase_inverter");
    assert_eq!(models[1].length, 52);
}

#[test]
fn parse_xml_fixture_models() {
    let data = include_str!("fixtures/models.xml");
    let models = parse_models_from_xml(data).expect("xml parse");
    assert_eq!(models.len(), 2);
    assert_eq!(models[0].id, 1);
    assert_eq!(models[0].name, "common");
    assert_eq!(models[0].length, 68);
    assert_eq!(models[1].id, 103);
    assert_eq!(models[1].name, "three_phase_inverter");
    assert_eq!(models[1].length, 52);
}

#[test]
fn parse_register_map_strict_and_lenient() {
    let base = 40_000u16;
    let registers = vec![
        0x5375,
        0x6e53,
        1,
        2,
        0,
        0,
        103,
        4,
        0,
        0,
        0,
        0,
        0xFFFF,
        0,
    ];

    let models = parse_models_from_registers(base, &registers).expect("register parse");
    assert_eq!(models.len(), 2);
    assert_eq!(models[0].start, 40_002);
    assert_eq!(models[0].length, 4);
    assert_eq!(models[1].start, 40_006);
    assert_eq!(models[1].length, 6);

    let truncated = vec![
        0x5375,
        0x6e53,
        1,
        2,
        0,
        0,
        103,
        4,
        0,
        0,
    ];

    assert!(parse_models_from_registers(base, &truncated).is_err());
    let models =
        parse_models_from_registers_lenient(base, &truncated).expect("lenient parse");
    assert_eq!(models.len(), 1);
    assert_eq!(models[0].id, 1);
}

#[test]
fn model_catalog_caches_results() {
    let json_data = include_str!("fixtures/models.json");
    let xml_data = include_str!("fixtures/models.xml");

    let mut catalog = ModelCatalog::default();
    let _ = catalog.parse_json(json_data).expect("json parse");
    let _ = catalog.parse_json(json_data).expect("json cache");
    assert_eq!(catalog.json_cache_len(), 1);

    let _ = catalog.parse_xml(xml_data).expect("xml parse");
    let _ = catalog.parse_xml(xml_data).expect("xml cache");
    assert_eq!(catalog.xml_cache_len(), 1);
}
