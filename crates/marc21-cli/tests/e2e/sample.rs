use std::fs;

use predicates::prelude::PredicateBooleanExt;

use crate::prelude::*;

#[test]
fn sample_stdout() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["sample", "-s", "--seed", "23", "1"])
        .arg(data_dir().join("DUMP.mrc.gz"))
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq(fs::read(
            data_dir().join("minna.mrc"),
        )?))
        .stderr(predicates::str::is_empty());

    Ok(())
}

#[test]
fn sample_output() -> TestResult {
    let temp_dir = TempDir::new()?;
    let output = temp_dir.child("out.mrc");

    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["sample", "-s", "--seed", "23", "1"])
        .arg(data_dir().join("DUMP.mrc.gz"))
        .args(["-o", output.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    assert_eq!(
        fs::read(data_dir().join("minna.mrc"))?,
        fs::read(output)?,
    );

    Ok(())
}

#[test]
fn sample_where() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["sample", "-s", "--seed", "23", "1"])
        .arg(data_dir().join("DUMP.mrc.gz"))
        .args(["--where", "001 == '040992918' || 001 == '118572121'"])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq(fs::read(
            data_dir().join("minna.mrc"),
        )?))
        .stderr(predicates::str::is_empty());

    Ok(())
}

#[test]
fn sample_skip_invalid() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["sample", "3"])
        .arg(data_dir().join("DUMP.mrc.gz"))
        .assert();

    assert
        .failure()
        .code(1)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::starts_with(
            "error: could not parse record 7",
        ));

    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["sample", "-s", "3"])
        .arg(data_dir().join("DUMP.mrc.gz"))
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty().not())
        .stderr(predicates::str::is_empty());

    Ok(())
}
