use assert_cmd::cargo_bin_cmd;
use predicates::prelude::*;

#[test]
fn test_help_flag() {
    cargo_bin_cmd!()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Generate a new UUID"))
        .stdout(predicate::str::contains("Generate a new ULID"))
        .stdout(predicate::str::contains("Generate a new ObjectId"));
}

#[test]
fn test_version_flag() {
    cargo_bin_cmd!()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("idgen"));
}

#[test]
fn test_count_zero() {
    cargo_bin_cmd!()
        .args(["-n", "0", "uuid"])
        .assert()
        .success()
        .stdout(predicate::str::is_empty());
}

#[test]
fn test_count_one_explicit() {
    cargo_bin_cmd!()
        .args(["-n", "1", "uuid"])
        .assert()
        .success()
        .stdout(
            predicate::str::is_match(
                r"^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}\n$",
            )
            .unwrap(),
        );
}

#[test]
fn test_count_invalid_negative() {
    cargo_bin_cmd!()
        .args(["-n", "-1", "uuid"])
        .assert()
        .failure();
}

#[test]
fn test_count_invalid_non_numeric() {
    cargo_bin_cmd!()
        .args(["-n", "abc", "uuid"])
        .assert()
        .failure();
}

#[test]
fn test_invalid_timestamp_negative() {
    cargo_bin_cmd!()
        .args(["ulid", "--timestamp=-1000"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("timestamp").or(predicate::str::contains("invalid")));
}

#[test]
fn test_invalid_timestamp_non_numeric() {
    cargo_bin_cmd!()
        .args(["ulid", "--timestamp", "not_a_number"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("timestamp").or(predicate::str::contains("invalid")));
}

#[test]
fn test_invalid_command() {
    cargo_bin_cmd!().arg("invalid_command").assert().failure();
}
