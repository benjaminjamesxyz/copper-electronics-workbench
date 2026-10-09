use std::{io::Write, process::{Command, Stdio}};
use serde_json::Value;
mod common;
use common::{calculate, assert_close};

#[test]
fn exact_inputs_and_finite_values_are_enforced() {
    assert!(calculate("dc.ohm_current", &[("voltage", 5.0)]).is_err());
    assert!(calculate("dc.ohm_current", &[("voltage", 5.0), ("resistance", 1000.0), ("extra", 1.0)]).is_err());
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(calculate("dc.ohm_current", &[("voltage", value), ("resistance", 1000.0)]).is_err());
    }
    assert_close(calculate("dc.ohm_current", &[("voltage", -5.0), ("resistance", 1000.0)]).unwrap()["current"], -0.005, 1e-12, 0.0);
}

#[test]
fn bad_requests_do_not_poison_the_next_process_response() {
    let mut process = Command::new(env!("CARGO_BIN_EXE_pcb-calculator"))
        .stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    let mut stdin = process.stdin.take().unwrap();
    for line in [
        "{malformed",
        r#"{"command":"not_a_command"}"#,
        r#"{"command":"calculate","id":"unknown.calculator","inputs":{}}"#,
        r#"{"command":"calculate","id":"dc.ohm_current","inputs":{"voltage":"5","resistance":1000}}"#,
        r#"{"command":"calculate","id":"dc.ohm_current","inputs":{"voltage":-5,"resistance":1000}}"#,
    ] { writeln!(stdin, "{line}").unwrap(); }
    drop(stdin);
    let result = process.wait_with_output().unwrap();
    assert!(result.status.success());
    assert!(result.stderr.is_empty());
    let responses: Vec<Value> = String::from_utf8(result.stdout).unwrap().lines().map(|line| serde_json::from_str(line).unwrap()).collect();
    assert_eq!(responses.len(), 5);
    for response in &responses[..4] { assert_eq!(response["ok"], false); }
    assert_eq!(responses[4]["ok"], true);
    assert_eq!(responses[4]["id"], "dc.ohm_current");
    assert_close(responses[4]["outputs"]["current"].as_f64().unwrap(), -0.005, 1e-12, 0.0);
}
