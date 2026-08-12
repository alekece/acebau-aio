use std::process::Command;

fn cli() -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_acebau-cli"));
    command
        .arg("--database-url")
        .arg("postgres://postgres:password@localhost/acebau");
    command
}

#[test]
fn backup_requires_exactly_one_mode() {
    let missing_mode = cli()
        .args(["database", "backup"])
        .output()
        .expect("acebau-cli should run");
    assert!(!missing_mode.status.success());

    let conflicting_modes = cli()
        .args([
            "database",
            "backup",
            "--dump",
            "backup.dump",
            "--restore",
            "backup.dump",
        ])
        .output()
        .expect("acebau-cli should run");
    assert!(!conflicting_modes.status.success());
}
