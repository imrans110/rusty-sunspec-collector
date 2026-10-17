//! Decodes arbitrary registers as a bundled SunSpec model. Whatever a device returns,
//! decoding must not panic, and the points and labels must be consistent.

#![no_main]

use std::collections::HashSet;
use std::sync::OnceLock;

use libfuzzer_sys::fuzz_target;
use sunspec_parser::{decode_model, decode_model_labeled, model_definition, supported_models};
use types::PointValue;

fn bundled_models() -> &'static [u16] {
    static MODELS: OnceLock<Vec<u16>> = OnceLock::new();
    MODELS.get_or_init(supported_models)
}

fuzz_target!(|data: &[u8]| {
    // The first two bytes pick the model: a bundled model ID as is, anything else
    // maps onto one. The rest are the registers, from the model header on.
    let Some((selector, rest)) = data.split_first_chunk::<2>() else {
        return;
    };
    let selector = u16::from_be_bytes(*selector);
    let model_id = if model_definition(selector).is_some() {
        selector
    } else {
        let models = bundled_models();
        models[usize::from(selector) % models.len()]
    };
    let registers: Vec<u16> = rest
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| u16::from_be_bytes(*pair))
        .collect();

    let decoded = decode_model_labeled(model_id, &registers).expect("a bundled model decodes");
    assert_eq!(
        decode_model(model_id, &registers).as_ref(),
        Some(&decoded.points)
    );

    let names: HashSet<&str> = decoded
        .points
        .iter()
        .map(|(name, _)| name.as_str())
        .collect();
    assert_eq!(names.len(), decoded.points.len(), "point names are unique");
    for (name, _) in &decoded.labels {
        assert!(names.contains(name.as_str()), "label {name} has a point");
    }
    for (name, value) in &decoded.points {
        if let PointValue::Float(number) = value {
            assert!(number.is_finite(), "{name} = {number}");
        }
    }
});
