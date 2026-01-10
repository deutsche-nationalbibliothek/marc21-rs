use crate::prelude::*;

#[test]
fn count_default() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert =
        cmd.args(["count"]).arg(data_dir().join("ada.mrc")).assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq("1\n"))
        .stderr(predicates::str::is_empty());

    Ok(())
}

#[test]
fn count_gzip() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["count"])
        .arg(data_dir().join("ada.mrc.gz"))
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq("1\n"))
        .stderr(predicates::str::is_empty());

    Ok(())
}

#[test]
fn count_multiple_files() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["count"])
        .arg(data_dir().join("ada.mrc.gz"))
        .arg(data_dir().join("ada.mrc"))
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq("2\n"))
        .stderr(predicates::str::is_empty());

    Ok(())
}
