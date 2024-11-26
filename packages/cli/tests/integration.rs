use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_get_packages_cli() {
    let mut cmd = Command::cargo_bin("kittynode").unwrap();

    cmd.arg("get-packages")
        .assert()
        .success()
        .stdout(predicate::str::contains("Package: Ethereum"));
}
