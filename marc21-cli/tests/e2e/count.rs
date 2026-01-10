use std::fs::File;
use std::io::{Read, read_to_string};

use assert_fs::TempDir;
use assert_fs::prelude::PathChild;
use flate2::read::GzDecoder;

use crate::prelude::*;

#[test]
fn count_default() -> TestResult {
    let mut cmd = marc_cmd();
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
fn count_read_gzip() -> TestResult {
    let mut cmd = marc_cmd();
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
fn count_read_multiple_files() -> TestResult {
    let mut cmd = marc_cmd();
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

#[test]
fn count_write_output_plaintext() -> TestResult {
    let temp_dir = TempDir::new()?;
    let output = temp_dir.child("count.txt");

    let mut cmd = marc_cmd();
    let assert = cmd
        .args(["count"])
        .arg(data_dir().join("ada.mrc"))
        .args(["-o", output.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let mut actual = read_to_string(File::open(output.path())?)?;
    if cfg!(windows) {
        actual = actual.replace('\r', "");
    }

    assert_eq!(actual, "1\n");
    temp_dir.close()?;
    Ok(())
}

#[test]
fn count_write_output_gzip() -> TestResult {
    let temp_dir = TempDir::new()?;
    let output = temp_dir.child("count.txt.gz");

    let mut cmd = marc_cmd();
    let assert = cmd
        .args(["count"])
        .arg(data_dir().join("ada.mrc"))
        .args(["-o", output.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let mut gz = GzDecoder::new(File::open(output.path())?);
    let mut actual = String::new();
    gz.read_to_string(&mut actual)?;
    assert_eq!(actual, "1\n");

    temp_dir.close()?;
    Ok(())
}
