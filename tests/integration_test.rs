use std::process::Command;
use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn test_default_behavior() {
    Command::cargo_bin("cat_say") // Update the binary name here
        .expect("binary exists")
        .assert()
        .success()
        .stdout(predicate::str::contains("Expected output"));

    // Add more assertions based on your program's behavior
}

#[test]
fn test_with_options() {
    Command::cargo_bin("cat_say") // Update the binary name here
        .expect("binary exists")
        .args(&["--some-option", "value"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Expected output"));

    // Add more assertions based on your program's behavior with options
}
