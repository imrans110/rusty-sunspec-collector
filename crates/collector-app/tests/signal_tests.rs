#![cfg(unix)]

use std::io::Read;
use std::process::{Child, Command, ExitStatus, Stdio};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use sunspec_sim::{SimConfig, UnitSpec, Unmapped};

// systemd and Docker stop services with SIGTERM; it must take the graceful path.
#[tokio::test]
async fn sigterm_triggers_graceful_shutdown() {
    let mut sim_config = SimConfig::single(UnitSpec::inverter(1));
    sim_config.unmapped = Unmapped::Zeros;
    let sim = sunspec_sim::start(sim_config, "127.0.0.1:0".parse().unwrap())
        .await
        .expect("start simulator");

    let mut child = Command::new(env!("CARGO_BIN_EXE_collector-app"))
        .env_clear()
        .env("RUST_LOG", "info")
        .env("SUNSPEC_STATIC_DEVICES", "127.0.0.1:1")
        .env("SUNSPEC_PORT", sim.port().to_string())
        .env("SUNSPEC_METRICS_PORT", "0")
        .env("SUNSPEC_BUFFER_PATH", temp_db_path())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn collector");

    let deadline = Instant::now() + Duration::from_secs(15);
    while sim.requests_served() < 4 {
        assert!(
            Instant::now() < deadline,
            "collector never polled the simulator"
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    }

    let killed = Command::new("kill")
        .args(["-TERM", &child.id().to_string()])
        .status()
        .expect("send SIGTERM");
    assert!(killed.success());

    let status = wait_for_exit(&mut child, Duration::from_secs(15));
    let mut stdout = String::new();
    child
        .stdout
        .take()
        .expect("stdout")
        .read_to_string(&mut stdout)
        .expect("read stdout");

    assert!(status.success(), "exit status {status:?}\n{stdout}");
    assert!(
        stdout.contains("shutdown signal received"),
        "graceful shutdown not logged:\n{stdout}"
    );
}

fn wait_for_exit(child: &mut Child, timeout: Duration) -> ExitStatus {
    let deadline = Instant::now() + timeout;
    loop {
        if let Some(status) = child.try_wait().expect("poll child") {
            return status;
        }
        if Instant::now() > deadline {
            let _ = child.kill();
            panic!("collector did not exit within {timeout:?} of SIGTERM");
        }
        std::thread::sleep(Duration::from_millis(50));
    }
}

fn temp_db_path() -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    std::env::temp_dir()
        .join(format!("sigterm-{}-{nanos}.sqlite", std::process::id()))
        .display()
        .to_string()
}
