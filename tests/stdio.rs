use serde_json::Value;
use std::{
    io::Write,
    process::{Command, Stdio},
};

#[test]
fn binary_handles_multiple_lines_notifications_and_eof() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_influxdb-plugin"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut input = child.stdin.take().unwrap();
    writeln!(input, "{{").unwrap();
    writeln!(
        input,
        r#"{{"jsonrpc":"2.0","method":"cancel","params":{{"id":1}}}}"#
    )
    .unwrap();
    writeln!(input, r#"{{"jsonrpc":"2.0","method":"ping","id":2}}"#).unwrap();
    drop(input);
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    let responses: Vec<Value> = String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    assert_eq!(responses.len(), 2);
    assert_eq!(responses[0]["error"]["code"], -32700);
    assert_eq!(responses[1]["id"], 2);
    assert_eq!(responses[1]["error"]["code"], -32000);
}
