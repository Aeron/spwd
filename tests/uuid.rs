use assert_cmd::cargo_bin_cmd;
use predicates::prelude::*;

#[test]
fn test_uuid_v1() {
    cargo_bin_cmd!("idgen")
        .args(["uuid", "-v", "1"])
        .assert()
        .success()
        .stdout(
            predicate::str::is_match(
                r"^[0-9a-f]{8}-[0-9a-f]{4}-1[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}\n$",
            )
            .unwrap(),
        );
}

#[test]
fn test_uuid_v3_with_namespace_and_name() {
    cargo_bin_cmd!()
        .args(["uuid", "-v", "3", "--namespace", "dns", "--name", "test"])
        .assert()
        .success()
        .stdout(
            predicate::str::is_match(
                r"^[0-9a-f]{8}-[0-9a-f]{4}-3[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}\n$",
            )
            .unwrap(),
        );
}

#[test]
fn test_uuid_v3_missing_namespace() {
    cargo_bin_cmd!()
        .args(["uuid", "-v", "3", "--name", "test"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("namespace"));
}

#[test]
fn test_uuid_v3_missing_name() {
    cargo_bin_cmd!()
        .args(["uuid", "-v", "3", "--namespace", "DNS"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("name"));
}

#[test]
fn test_uuid_v4_default() {
    cargo_bin_cmd!().arg("uuid").assert().success().stdout(
        predicate::str::is_match(
            r"^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}\n$",
        )
        .unwrap(),
    );
}

#[test]
fn test_uuid_v5_with_namespace_and_name() {
    cargo_bin_cmd!()
        .args([
            "uuid",
            "-v",
            "5",
            "--namespace",
            "url",
            "--name",
            "example.com",
        ])
        .assert()
        .success()
        .stdout(
            predicate::str::is_match(
                r"^[0-9a-f]{8}-[0-9a-f]{4}-5[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}\n$",
            )
            .unwrap(),
        );
}

#[test]
fn test_uuid_v7() {
    cargo_bin_cmd!()
        .args(["uuid", "-v", "7"])
        .assert()
        .success()
        .stdout(
            predicate::str::is_match(
                r"^[0-9a-f]{8}-[0-9a-f]{4}-7[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}\n$",
            )
            .unwrap(),
        );
}

#[test]
fn test_uuid_v8_with_data() {
    cargo_bin_cmd!()
        .args(["uuid", "-v", "8", "--data", "0123456789abcdef"])
        .assert()
        .success()
        .stdout(
            predicate::str::is_match(
                r"^[0-9a-f]{8}-[0-9a-f]{4}-8[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}\n$",
            )
            .unwrap(),
        );
}

#[test]
fn test_uuid_v8_missing_data() {
    cargo_bin_cmd!()
        .args(["uuid", "-v", "8"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("data"));
}

#[test]
fn test_multiple_uuids() {
    cargo_bin_cmd!()
        .args(["-n", "3", "uuid"])
        .assert()
        .success()
        .stdout(
            predicate::str::is_match(
                r"(?m)^([0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}\n){3}$",
            )
            .unwrap(),
        );
}

#[test]
fn test_uuid_v4_with_timestamp_rejected() {
    cargo_bin_cmd!()
        .args(["uuid", "-v", "4", "--timestamp", "1234567890000000000"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("timestamp"));
}

#[test]
fn test_uuid_v5_with_timestamp_rejected() {
    cargo_bin_cmd!()
        .args([
            "uuid",
            "-v",
            "5",
            "--namespace",
            "dns",
            "--name",
            "test",
            "--timestamp",
            "1234567890000000000",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("timestamp"));
}

#[test]
fn test_uuid_v8_with_timestamp_rejected() {
    cargo_bin_cmd!()
        .args([
            "uuid",
            "-v",
            "8",
            "--data",
            "0123456789abcdef",
            "--timestamp",
            "1234567890000000000",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("timestamp"));
}

#[test]
fn test_uuid_v8_with_invalid_data_too_long() {
    cargo_bin_cmd!()
        .args([
            "uuid",
            "-v",
            "8",
            "--data",
            "0123456789abcdef0123456789abcdef01",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("data"));
}

#[test]
fn test_uuid_v8_with_invalid_data_non_hex() {
    cargo_bin_cmd!()
        .args(["uuid", "-v", "8", "--data", "ghijklmnopqrstuv"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("hex"));
}

#[test]
fn test_uuid_v3_with_invalid_namespace() {
    cargo_bin_cmd!()
        .args([
            "uuid",
            "-v",
            "3",
            "--namespace",
            "invalid",
            "--name",
            "test",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("namespace"));
}

#[test]
fn test_uuid_v1_with_timestamp() {
    cargo_bin_cmd!()
        .args(["uuid", "-v", "1", "--timestamp", "1234567890000000000"])
        .assert()
        .success()
        .stdout(
            predicate::str::is_match(
                r"^[0-9a-f]{8}-[0-9a-f]{4}-1[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}\n$",
            )
            .unwrap(),
        );
}

#[test]
fn test_uuid_v6_with_timestamp() {
    cargo_bin_cmd!()
        .args(["uuid", "-v", "6", "--timestamp", "1234567890000000000"])
        .assert()
        .success()
        .stdout(
            predicate::str::is_match(
                r"^[0-9a-f]{8}-[0-9a-f]{4}-6[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}\n$",
            )
            .unwrap(),
        );
}

#[test]
fn test_uuid_v7_with_timestamp() {
    cargo_bin_cmd!()
        .args(["uuid", "-v", "7", "--timestamp", "1234567890000000000"])
        .assert()
        .success()
        .stdout(
            predicate::str::is_match(
                r"^[0-9a-f]{8}-[0-9a-f]{4}-7[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}\n$",
            )
            .unwrap(),
        );
}
