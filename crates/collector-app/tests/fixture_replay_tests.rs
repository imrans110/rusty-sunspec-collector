use std::path::{Path, PathBuf};

use collector_app::CollectorConfig;
use sunspec_sim::{read_dump, SimConfig, UnitSpec};
use types::{DeviceDump, DeviceIdentity};

fn fixture_files() -> Vec<PathBuf> {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/devices");
    let mut files: Vec<PathBuf> = std::fs::read_dir(&dir)
        .expect("read fixtures/devices")
        .map(|entry| entry.expect("directory entry").path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    files.sort();
    files
}

/// Serves `dump` with the simulator and probes it again.
async fn replay(dump: &DeviceDump) -> DeviceDump {
    let unit = UnitSpec::from_dump(dump).expect("fixture chain is consistent");
    let sim = sunspec_sim::start(SimConfig::single(unit), "127.0.0.1:0".parse().unwrap())
        .await
        .expect("start simulator");
    sim.load_dump(dump.unit_id, dump);

    let mut config = CollectorConfig::default();
    config.modbus.port = sim.port();
    config.discovery.port = sim.port();
    config.modbus.retry_count = 0;
    // Real devices may use a base other than the standard ones.
    config.base_address = dump.base_address;
    let identity = DeviceIdentity {
        ip: "127.0.0.1".to_string(),
        unit_id: dump.unit_id,
    };
    collector_app::probe::probe(&config, identity, true)
        .await
        .expect("probe the replayed device")
}

#[tokio::test]
async fn every_device_fixture_replays_to_the_same_dump() {
    let files = fixture_files();
    assert!(!files.is_empty(), "no fixtures in fixtures/devices");
    for path in files {
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        let expected = read_dump(&path).unwrap_or_else(|err| panic!("{err}"));
        let actual = replay(&expected).await;

        assert_eq!(actual.base_address, expected.base_address, "{name}: base");
        let chain = |dump: &DeviceDump| -> Vec<(u16, u16, u16)> {
            dump.models
                .iter()
                .map(|model| (model.id, model.start, model.length))
                .collect()
        };
        assert_eq!(chain(&actual), chain(&expected), "{name}: model chain");
        for (want, got) in expected.models.iter().zip(&actual.models) {
            if want.error.is_some() {
                // The probe could not read it, so the fixture has no registers to replay.
                continue;
            }
            assert_eq!(
                got.registers, want.registers,
                "{name}: model {} registers",
                want.id
            );
            assert_eq!(
                got.points, want.points,
                "{name}: model {} decodes differently; if the decoder changed on purpose, \
                 regenerate the fixture's points (see fixtures/devices/README.md)",
                want.id
            );
        }
    }
}

#[tokio::test]
async fn a_fixture_with_a_broken_chain_is_rejected() {
    let path = fixture_files().into_iter().next().unwrap();
    let mut dump = read_dump(&path).unwrap();
    dump.models[1].start += 1;
    let err = UnitSpec::from_dump(&dump).expect_err("gap in the chain");
    assert!(err.contains("starts at"), "{err}");
}
