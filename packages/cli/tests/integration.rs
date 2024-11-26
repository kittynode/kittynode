use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn get_packages() {
    let mut cmd = Command::cargo_bin("kittynode").unwrap();
    cmd.arg("get-packages")
        .assert()
        .success()
        .stdout(predicate::str::contains("Package: Ethereum"));
}

#[test]
#[ignore]
fn install_and_delete_a_package() {
    let mut cmd = Command::cargo_bin("kittynode").unwrap();
    cmd.arg("install-package")
        .arg("Ethereum")
        .assert()
        .success();
    cmd = Command::cargo_bin("kittynode").unwrap();
    cmd.arg("delete-package").arg("Ethereum").assert().success();
}
