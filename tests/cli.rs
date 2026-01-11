use std::fs;
use assert_cmd::Command;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
    let mut cmd  = Command::cargo_bin("echor")?;
    cmd.assert().failure().stderr(predicates::str::contains("USAGE"));
    Ok(())
}


#[test]
fn sample_runs() -> TestResult {
    let mut cmd  = Command::cargo_bin("echor")?;
    cmd.arg("This program should runs successfully").assert().success();
    Ok(())
}

fn run(args: &[&str], file: &str) -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    let expected = fs::read_to_string(file)?;
    cmd.args(args).assert().success().stdout(expected);
    Ok(())
}



#[test]
fn hello1() -> TestResult {
   run(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn hello2() -> TestResult {
    run(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn hello1_no_newline() -> TestResult {
run(&["Hello there", "-n"], "tests/expected/hello1.n.txt")
}
#[test]
fn hello2_no_newline() -> TestResult {
run(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}
