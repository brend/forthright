use std::{
    io::Write,
    process::{Command, Output, Stdio},
};

fn run_args(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_forthright"))
        .args(args)
        .output()
        .expect("forthright should run")
}

fn run_stdin(input: &str) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_forthright"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("forthright should run");

    child
        .stdin
        .take()
        .expect("stdin should be piped")
        .write_all(input.as_bytes())
        .expect("input should be written");

    child
        .wait_with_output()
        .expect("output should be collected")
}

fn assert_success(output: Output, expected_stdout: &str) {
    assert!(output.status.success(), "status: {}", output.status);
    assert_eq!(String::from_utf8(output.stdout).unwrap(), expected_stdout);
    assert_eq!(String::from_utf8(output.stderr).unwrap(), "");
}

fn assert_failure(output: Output, expected_stderr: &str) {
    assert!(!output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "");
    assert_eq!(String::from_utf8(output.stderr).unwrap(), expected_stderr);
}

#[test]
fn evaluates_program_from_arguments() {
    assert_success(run_args(&["3", "4", "+", "."]), "7\n");
}

#[test]
fn evaluates_program_from_stdin() {
    assert_success(run_stdin("5 dup * ."), "25\n");
}

#[test]
fn evaluates_arithmetic_with_correct_operand_order() {
    assert_success(
        run_args(&[
            "7", "2", "+", ".", "7", "2", "-", ".", "7", "2", "*", ".", "7", "2", "/", ".",
        ]),
        "9\n5\n14\n3\n",
    );
}

#[test]
fn evaluates_stack_operations_and_multiple_prints() {
    assert_success(
        run_args(&["1", "2", "swap", ".", "dup", ".", "drop"]),
        "1\n2\n",
    );
}

#[test]
fn reports_an_invalid_token() {
    assert_failure(
        run_args(&["1", "nope", "+", "."]),
        "error: could not parse token: \"nope\"\n",
    );
}

#[test]
fn reports_stack_underflow_for_each_stack_operation() {
    for args in [
        &["+"][..],
        &["-"][..],
        &["*"][..],
        &["/"][..],
        &["dup"][..],
        &["drop"][..],
        &["swap"][..],
        &["."][..],
    ] {
        assert_failure(run_args(args), "error: stack underflow\n");
    }
}

#[test]
fn reports_division_by_zero() {
    assert_failure(run_args(&["1", "0", "/"]), "error: division by zero\n");
}
