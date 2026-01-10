use std::fs::{File, read_to_string};
use std::io::Read;

use assert_fs::TempDir;
use assert_fs::prelude::PathChild;
use flate2::read::GzDecoder;

use crate::prelude::*;

#[test]
fn print_stdout() -> TestResult {
    let mut cmd = marc_cmd();
    let assert =
        cmd.arg("print").arg(data_dir().join("ada.mrc")).assert();

    let mut output = read_to_string(data_dir().join("ada.txt"))?;
    if cfg!(windows) {
        output = output.replace('\r', "");
    }

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq(output))
        .stderr(predicates::str::is_empty());

    Ok(())
}

#[test]
fn print_plaintext() -> TestResult {
    let mut cmd = marc_cmd();
    let temp_dir = TempDir::new().unwrap();
    let output = temp_dir.child("out.txt");

    let assert = cmd
        .arg("print")
        .arg(data_dir().join("ada.mrc"))
        .args(["-o", output.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let mut actual = read_to_string(output.path())?;
    if cfg!(windows) {
        actual = actual.replace('\r', "");
    }

    let mut expected = read_to_string(data_dir().join("ada.txt"))?;
    if cfg!(windows) {
        expected = expected.replace('\r', "");
    }

    assert_eq!(expected, actual);
    temp_dir.close()?;
    Ok(())
}

#[test]
fn print_gzip() -> TestResult {
    let temp_dir = TempDir::new()?;
    let output = temp_dir.child("out.txt.gz");

    let mut cmd = marc_cmd();
    let assert = cmd
        .arg("print")
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

    let mut expected = read_to_string(data_dir().join("ada.txt"))?;
    if cfg!(windows) {
        expected = expected.replace('\r', "");
    }

    assert_eq!(expected, actual);
    temp_dir.close()?;
    Ok(())
}
