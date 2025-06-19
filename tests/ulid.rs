use assert_cmd::cargo_bin_cmd;
use predicates::prelude::*;

#[test]
fn test_ulid_generation() {
    cargo_bin_cmd!()
        .arg("ulid")
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^[0-9A-Z]{26}\n$").unwrap());
}

#[test]
fn test_ulid_with_timestamp() {
    cargo_bin_cmd!()
        .args(["ulid", "--timestamp", "1609459200000"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with("01ETXK"));
}

#[test]
fn test_multiple_ulids() {
    cargo_bin_cmd!()
        .args(["-n", "5", "ulid"])
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"(?m)^([0-9A-Z]{26}\n){5}$").unwrap());
}
