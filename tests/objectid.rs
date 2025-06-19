use assert_cmd::cargo_bin_cmd;
use predicates::prelude::*;

#[test]
fn test_objectid_generation() {
    cargo_bin_cmd!()
        .arg("oid")
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^[0-9a-f]{24}\n$").unwrap());
}

#[test]
fn test_objectid_alias() {
    cargo_bin_cmd!()
        .arg("objectid")
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^[0-9a-f]{24}\n$").unwrap());
}

#[test]
fn test_objectid_with_timestamp() {
    cargo_bin_cmd!()
        .args(["oid", "--timestamp", "1609459200"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with("5fee6600"));
}

#[test]
fn test_multiple_objectids() {
    cargo_bin_cmd!()
        .args(["-n", "4", "oid"])
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"(?m)^([0-9a-f]{24}\n){4}$").unwrap());
}
