use std::process::Command;

#[test]
fn version_prints_the_package_version() {
    let output = Command::new(env!("CARGO_BIN_EXE_sunspec-sim"))
        .arg("--version")
        .output()
        .expect("run sunspec-sim");
    assert!(output.status.success(), "{output:?}");
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        format!("sunspec-sim {}\n", env!("CARGO_PKG_VERSION"))
    );
}
