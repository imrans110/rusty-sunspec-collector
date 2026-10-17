//! Loads an arbitrary config file as TOML and as JSON. Loading must fail with an error
//! or produce a configuration, never panic, and so must validating it and deriving
//! each static device's settings.

#![no_main]

use collector_app::{CollectorConfig, ConfigFormat};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Ok(content) = std::str::from_utf8(data) else {
        return;
    };
    // `*_file` settings read the named file (a secret); keep the fuzzer away from
    // reading arbitrary files such as /dev/zero. Escapes could spell such a key too.
    if content.contains("_file") || content.contains("\\u") || content.contains("\\U") {
        return;
    }
    for format in [ConfigFormat::Toml, ConfigFormat::Json] {
        let Ok(config) = CollectorConfig::from_file_contents(content, format) else {
            continue;
        };
        if config.validate().is_ok() {
            for device in &config.discovery.static_devices {
                config.device_config(device.clone());
            }
        }
    }
});
