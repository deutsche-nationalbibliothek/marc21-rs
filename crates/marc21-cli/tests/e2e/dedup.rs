use std::fs;

use crate::prelude::*;

#[test]
fn dedup_stdout() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["dedup", "-s"])
        .arg(data_dir().join("ada.mrc"))
        .arg(data_dir().join("ada.mrc"))
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq(fs::read(
            data_dir().join("ada.mrc"),
        )?))
        .stderr(predicates::str::is_empty());

    Ok(())
}

#[test]
fn dedup_write_output() -> TestResult {
    let temp_dir = TempDir::new()?;
    let output = temp_dir.child("out.mrc");

    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["dedup", "-s"])
        .arg(data_dir().join("DUMP.mrc.gz"))
        .arg(data_dir().join("DUMP.mrc.gz"))
        .args(["-o", output.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["count", "-s"])
        .arg(output.to_str().unwrap())
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq("7\n"))
        .stderr(predicates::str::is_empty());

    temp_dir.close()?;
    Ok(())
}

#[test]
fn dedup_where() -> TestResult {
    let temp_dir = TempDir::new()?;
    let output = temp_dir.child("out.mrc");

    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["dedup", "-s"])
        .arg(data_dir().join("DUMP.mrc.gz"))
        .arg(data_dir().join("DUMP.mrc.gz"))
        .args(["--where", "075{ b == 'p' && 2 == 'gndgen' }"])
        .args(["-o", output.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["count", "-s"])
        .arg(output.to_str().unwrap())
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq("4\n"))
        .stderr(predicates::str::is_empty());

    temp_dir.close()?;
    Ok(())
}

#[test]
fn dedup_skip_invalid() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert = cmd
        .arg("dedup")
        .arg(data_dir().join("invalid.mrc"))
        .assert();

    assert
        .failure()
        .code(1)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::starts_with(
            "error: could not parse record (line 1",
        ));

    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["dedup", "-s"])
        .arg(data_dir().join("invalid.mrc"))
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    Ok(())
}

#[test]
fn dedup_limit() -> TestResult {
    let temp_dir = TempDir::new()?;
    let output = temp_dir.child("out.mrc");

    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["dedup", "-s", "--limit", "3"])
        .arg(data_dir().join("DUMP.mrc.gz"))
        .arg(data_dir().join("ada.mrc"))
        .args(["-o", output.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let mut cmd = marc21_cmd();
    let assert =
        cmd.arg("count").arg(output.to_str().unwrap()).assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq("3\n"))
        .stderr(predicates::str::is_empty());

    temp_dir.close()?;
    Ok(())
}
