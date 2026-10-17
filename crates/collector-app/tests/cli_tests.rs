use std::process::Command;

fn run(args: &[&str]) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_collector-app"))
        .args(args)
        .env_clear()
        .output()
        .expect("run collector-app");
    assert!(output.status.success(), "{args:?} failed: {output:?}");
    String::from_utf8(output.stdout).expect("utf-8 output")
}

// Release packaging and the install script rely on `--version` printing exactly this.
#[test]
fn version_prints_the_package_version() {
    let expected = format!("collector-app {}\n", env!("CARGO_PKG_VERSION"));
    assert_eq!(run(&["--version"]), expected);
    assert_eq!(run(&["-V"]), expected);
}

#[test]
fn help_lists_the_flags() {
    let help = run(&["--help"]);
    for flag in ["--config", "--print-schema", "--version"] {
        assert!(help.contains(flag), "{flag} missing from:\n{help}");
    }
}

#[test]
fn print_schema_prints_the_avro_schema() {
    let schema: serde_json::Value =
        serde_json::from_str(&run(&["--print-schema"])).expect("schema is JSON");
    assert_eq!(schema["type"], "record");
}
