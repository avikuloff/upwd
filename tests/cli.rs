use assert_cmd::Command;
use predicates::prelude::predicate;
use std::io::{BufRead, BufReader};

fn cmd() -> Command {
    Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap()
}

#[test]
fn default_password_length_is_12() {
    let mut cmd = cmd();
    let predicate_fn = predicate::function(|x: &str| x.trim().len() == 12);

    cmd.assert().success().stdout(predicate_fn);
}

#[test]
fn password_length_eq_arg_length() {
    let mut cmd = cmd();
    cmd.args(&["-L", "6"]);

    let predicate_fn = predicate::function(|x: &str| x.trim().len() == 6);

    cmd.assert().success().stdout(predicate_fn);
}

#[test]
fn created_10_passwords() {
    let mut cmd = cmd();
    cmd.args(&["-c", "10"]);

    let assert = cmd.assert().success();

    let output = assert.get_output().stdout.as_slice().clone();
    let reader = BufReader::new(output);

    assert_eq!(reader.lines().count(), 10);
}
