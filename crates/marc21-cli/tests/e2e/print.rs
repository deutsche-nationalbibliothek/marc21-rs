use std::fs::{File, read_to_string};
use std::io::Read;

use assert_fs::TempDir;
use assert_fs::prelude::PathChild;
use flate2::read::GzDecoder;
use predicates::prelude::PredicateBooleanExt;

use crate::prelude::*;

#[test]
fn print_stdout() -> TestResult {
    let mut cmd = marc21_cmd();
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
fn print_output_text() -> TestResult {
    let mut cmd = marc21_cmd();
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
fn print_output_gzip() -> TestResult {
    let temp_dir = TempDir::new()?;
    let output = temp_dir.child("out.txt.gz");

    let mut cmd = marc21_cmd();
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

#[test]
fn print_where() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["print", "-s"])
        .arg(data_dir().join("DUMP.mrc.gz"))
        .args(["--where", "001 == '040992918'"])
        .assert();

    let mut output = read_to_string(data_dir().join("minna.txt"))?;
    if cfg!(windows) {
        output = output.replace('\r', "");
    }

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq(output))
        .stderr(predicates::str::is_empty());

    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["print", "-s"])
        .arg(data_dir().join("DUMP.mrc.gz"))
        .args(["--where", "001 == '04099291X'"])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    Ok(())
}

#[test]
fn print_skip_invalid() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["print", "-s"])
        .arg(data_dir().join("DUMP.mrc.gz"))
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty().not())
        .stderr(predicates::str::is_empty());

    let mut cmd = marc21_cmd();
    let assert = cmd
        .arg("print")
        .arg(data_dir().join("DUMP.mrc.gz"))
        .assert();

    assert
        .failure()
        .code(1)
        .stdout(predicates::str::is_empty().not())
        .stderr(predicates::str::starts_with(
            "error: could not parse record 7",
        ));

    Ok(())
}
