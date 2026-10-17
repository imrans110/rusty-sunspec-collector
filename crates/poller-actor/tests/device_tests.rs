use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;
use std::time::Duration;

use modbus_client::{ClientConfig, ModbusClient};
use poller_actor::{
    base_candidates, discover_models, run_device, ActorConfig, DeviceConfig, DeviceHealth,
    ModelSchedule, PollerError, Sample,
};
use sunspec_sim::{parse_models, SimConfig, SimHandle, UnitSpec};
use tokio::sync::{mpsc, watch};
use tokio::time::timeout;
use types::DeviceIdentity;

/// Common, 3-phase inverter, nameplate, settings, status, controls and MPPT: 290
/// registers, longer than the old fixed 200-register discovery window.
const LONG_CHAIN: &str = "1:66,103:50,120:26,121:30,122:44,123:24,160:48";

async fn start_sim(unit: UnitSpec) -> SimHandle {
    sunspec_sim::start(SimConfig::single(unit), "127.0.0.1:0".parse().unwrap())
        .await
        .expect("start simulator")
}

async fn client(sim: &SimHandle) -> ModbusClient {
    ModbusClient::connect(ClientConfig {
        host: "127.0.0.1".to_string(),
        port: sim.port(),
        ..Default::default()
    })
    .await
    .expect("connect")
}

fn device(sim: &SimHandle, unit_id: u8, actor: ActorConfig) -> DeviceConfig {
    DeviceConfig {
        identity: DeviceIdentity {
            ip: "127.0.0.1".to_string(),
            unit_id,
        },
        modbus: ClientConfig {
            port: sim.port(),
            retry_count: 0,
            ..Default::default()
        },
        actor,
        base_address: 40_000,
        shared_client: None,
    }
}

fn fast_actor() -> ActorConfig {
    ActorConfig {
        poll_interval: Duration::from_millis(100),
        request_timeout: Duration::from_millis(300),
        respawn_delay: Duration::from_millis(100),
        ..Default::default()
    }
}

#[tokio::test]
async fn discovers_every_model_in_a_long_chain() {
    let sim = start_sim(UnitSpec::with_models(1, parse_models(LONG_CHAIN).unwrap())).await;

    let (base, models) = discover_models(&client(&sim).await, 1, &base_candidates(40_000))
        .await
        .expect("discover");

    assert_eq!(base, 40_000);
    let ids: Vec<u16> = models.iter().map(|model| model.id).collect();
    assert_eq!(ids, vec![1, 103, 120, 121, 122, 123, 160]);
    let mppt = models.last().unwrap();
    assert_eq!(mppt.start, sim.model_start(1, 160).unwrap());
    assert_eq!(mppt.length, 50, "header plus 48 registers");
}

#[tokio::test]
async fn finds_the_map_at_another_standard_base() {
    let mut unit = UnitSpec::inverter(1);
    unit.base = 0;
    let sim = start_sim(unit).await;

    let (base, models) = discover_models(&client(&sim).await, 1, &base_candidates(40_000))
        .await
        .expect("discover");

    assert_eq!(base, 0);
    assert_eq!(models.len(), 2);
}

#[tokio::test]
async fn reports_a_device_without_a_sunspec_map() {
    let mut unit = UnitSpec::inverter(1);
    unit.base = 1_000;
    let sim = start_sim(unit).await;

    let err = discover_models(&client(&sim).await, 1, &base_candidates(40_000))
        .await
        .expect_err("no map at a standard base");

    assert!(matches!(err, PollerError::NoSunSpecMap(_)), "got {err:?}");
}

#[tokio::test]
async fn device_offline_at_startup_is_polled_once_it_appears() {
    let sim = start_sim(UnitSpec::inverter(1)).await;
    sim.go_offline();
    let (tx, mut rx) = mpsc::channel::<Sample>(64);
    let (shutdown_tx, shutdown_rx) = watch::channel(false);
    let health = Arc::new(DeviceHealth::default());
    let task = tokio::spawn(run_device(
        device(&sim, 1, fast_actor()),
        tx,
        shutdown_rx,
        health.clone(),
    ));

    tokio::time::sleep(Duration::from_millis(800)).await;
    assert!(rx.try_recv().is_err(), "nothing to poll while offline");
    assert!(health.restarts() >= 1, "failed sessions are retried");

    sim.go_online();
    let sample = timeout(Duration::from_secs(5), rx.recv())
        .await
        .expect("polled after coming online")
        .expect("sample");
    assert_eq!(sample.device.unit_id, 1);
    assert!(health.is_up());

    shutdown_tx.send(true).unwrap();
    timeout(Duration::from_secs(5), task)
        .await
        .expect("stops on shutdown")
        .unwrap();
}

#[tokio::test]
async fn stalled_session_is_restarted() {
    let sim = start_sim(UnitSpec::inverter(1)).await;
    let actor = ActorConfig {
        // Reads would hang far longer than the stall timeout without it.
        request_timeout: Duration::from_secs(30),
        stall_timeout: Some(Duration::from_millis(500)),
        ..fast_actor()
    };
    let (tx, mut rx) = mpsc::channel::<Sample>(1024);
    let (shutdown_tx, shutdown_rx) = watch::channel(false);
    let health = Arc::new(DeviceHealth::default());
    let task = tokio::spawn(run_device(
        device(&sim, 1, actor),
        tx,
        shutdown_rx,
        health.clone(),
    ));
    timeout(Duration::from_secs(5), rx.recv())
        .await
        .expect("first sample")
        .expect("sample");

    sim.set_stalled(true);
    tokio::time::sleep(Duration::from_millis(1_500)).await;
    assert!(
        health.restarts() >= 1,
        "stall detected and session restarted"
    );
    assert!(!health.is_up());

    sim.set_stalled(false);
    while rx.try_recv().is_ok() {}
    timeout(Duration::from_secs(5), rx.recv())
        .await
        .expect("polling resumes after the stall")
        .expect("sample");

    shutdown_tx.send(true).unwrap();
    timeout(Duration::from_secs(5), task)
        .await
        .expect("stops on shutdown")
        .unwrap();
}

/// Runs one device for `duration`, then stops it and returns what it published.
async fn run_for(
    sim: &SimHandle,
    actor: ActorConfig,
    duration: Duration,
) -> (Vec<Sample>, Arc<DeviceHealth>) {
    let (tx, mut rx) = mpsc::channel::<Sample>(1024);
    let (shutdown_tx, shutdown_rx) = watch::channel(false);
    let health = Arc::new(DeviceHealth::default());
    let task = tokio::spawn(run_device(
        device(sim, 1, actor),
        tx,
        shutdown_rx,
        health.clone(),
    ));
    tokio::time::sleep(duration).await;
    shutdown_tx.send(true).unwrap();
    timeout(Duration::from_secs(5), task)
        .await
        .expect("stops on shutdown")
        .unwrap();
    let mut samples = Vec::new();
    while let Ok(sample) = rx.try_recv() {
        samples.push(sample);
    }
    (samples, health)
}

fn samples_of(samples: &[Sample], model_id: u16) -> usize {
    samples
        .iter()
        .filter(|sample| sample.model_id == model_id)
        .count()
}

/// Poll reads of a model, not counting the discovery read of its header.
fn poll_reads(sim: &SimHandle, model_id: u16) -> u64 {
    sim.reads_at(1, sim.model_start(1, model_id).expect("model in map"))
        .saturating_sub(1)
}

#[tokio::test]
async fn slow_models_are_read_once_per_interval() {
    let sim = start_sim(UnitSpec::with_models(1, parse_models("1,103,120").unwrap())).await;
    let actor = ActorConfig {
        models: ModelSchedule {
            intervals: BTreeMap::from([(1, Duration::from_secs(1)), (120, Duration::from_secs(1))]),
            ..Default::default()
        },
        ..fast_actor()
    };

    // 100 ms cycles for 2.5 s: model 103 about 25 times, the slow ones at 0, 1 and 2 s.
    let (samples, _) = run_for(&sim, actor, Duration::from_millis(2_500)).await;

    for slow in [1, 120] {
        let reads = poll_reads(&sim, slow);
        assert!((2..=4).contains(&reads), "model {slow} read {reads} times");
        assert_eq!(samples_of(&samples, slow) as u64, reads);
    }
    let fast = poll_reads(&sim, 103);
    assert!(fast >= 15, "model 103 read only {fast} times");
    assert_eq!(samples_of(&samples, 103) as u64, fast);
}

#[tokio::test]
async fn excluded_and_unlisted_models_are_never_read() {
    let sim = start_sim(UnitSpec::with_models(
        1,
        parse_models("1,103,120,160:48").unwrap(),
    ))
    .await;
    let actor = ActorConfig {
        models: ModelSchedule {
            include: Some(BTreeSet::from([103, 160])),
            exclude: BTreeSet::from([160]),
            ..Default::default()
        },
        ..fast_actor()
    };

    let (samples, _) = run_for(&sim, actor, Duration::from_millis(800)).await;

    for skipped in [1, 120, 160] {
        assert_eq!(poll_reads(&sim, skipped), 0, "model {skipped} was polled");
    }
    assert!(poll_reads(&sim, 103) >= 3);
    assert!(samples.iter().all(|sample| sample.model_id == 103));
}

#[tokio::test]
async fn idle_cycles_between_slow_reads_are_not_stalls() {
    let sim = start_sim(UnitSpec::inverter(1)).await;
    let actor = ActorConfig {
        stall_timeout: Some(Duration::from_millis(400)),
        models: ModelSchedule {
            include: Some(BTreeSet::from([1])),
            intervals: BTreeMap::from([(1, Duration::from_secs(60))]),
            ..Default::default()
        },
        ..fast_actor()
    };

    let (samples, health) = run_for(&sim, actor, Duration::from_millis(1_500)).await;

    assert_eq!(health.restarts(), 0, "nothing was due, so nothing stalled");
    assert_eq!(poll_reads(&sim, 1), 1);
    assert_eq!(samples.len(), 1);
}

#[tokio::test]
async fn a_device_with_nothing_selected_is_retried_without_polling() {
    let sim = start_sim(UnitSpec::inverter(1)).await;
    let actor = ActorConfig {
        models: ModelSchedule {
            include: Some(BTreeSet::from([701])),
            ..Default::default()
        },
        ..fast_actor()
    };

    let (samples, health) = run_for(&sim, actor, Duration::from_millis(600)).await;

    assert!(samples.is_empty());
    assert!(
        health.restarts() >= 1,
        "rediscovered in case the models change"
    );
    // Each session reads every model header once while discovering, and nothing more.
    for model_id in [1, 103] {
        let reads = sim.reads_at(1, sim.model_start(1, model_id).unwrap());
        assert!(
            reads <= health.restarts() + 1,
            "model {model_id} read {reads} times"
        );
    }
}

#[tokio::test]
async fn the_snapshot_holds_discovery_and_the_latest_reading_of_each_polled_model() {
    let sim = start_sim(UnitSpec::with_models(1, parse_models("1,103,120").unwrap())).await;
    sim.set_point(1, 103, "W", &[4321]);
    let actor = ActorConfig {
        models: ModelSchedule {
            exclude: BTreeSet::from([120]),
            ..Default::default()
        },
        ..fast_actor()
    };

    let (_, health) = run_for(&sim, actor, Duration::from_millis(500)).await;

    let snapshot = health.snapshot();
    assert_eq!(snapshot.base_address, Some(40_000));
    assert_eq!(snapshot.models, vec![1, 103, 120]);
    assert_eq!(snapshot.polled_models, vec![1, 103]);
    assert_eq!(
        snapshot.readings.keys().copied().collect::<Vec<_>>(),
        vec![1, 103],
        "one reading per polled model"
    );
    let inverter = &snapshot.readings[&103];
    assert!(inverter
        .points
        .iter()
        .any(|(name, value)| name == "W" && *value == types::PointValue::Float(4321.0)));
}
