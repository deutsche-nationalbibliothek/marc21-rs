use std::fs::{self, File};
use std::io::Read;

use flate2::read::GzDecoder;

use crate::prelude::*;

#[test]
fn invalid_write_output_stdout() -> TestResult {
    let mut cmd = marc_cmd();
    let assert = cmd
        .arg("invalid")
        .arg(data_dir().join("ada.mrc"))
        .arg(data_dir().join("invalid.mrc"))
        .arg(data_dir().join("ada.mrc.gz"))
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq(fs::read(
            data_dir().join("invalid.mrc"),
        )?))
        .stderr(predicates::str::is_empty());

    let mut cmd = marc_cmd();
    let assert =
        cmd.arg("invalid").arg(data_dir().join("ada.mrc")).assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    Ok(())
}

#[test]
fn invalid_write_output_text() -> TestResult {
    let temp_dir = TempDir::new()?;
    let output = temp_dir.child("invalid.mrc");

    let mut cmd = marc_cmd();
    let assert = cmd
        .arg("invalid")
        .arg(data_dir().join("invalid.mrc"))
        .arg(data_dir().join("ada.mrc.gz"))
        .args(["-o", output.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    assert_eq!(
        fs::read(data_dir().join("invalid.mrc"))?,
        fs::read(output.path())?,
    );

    temp_dir.close()?;
    Ok(())
}

#[test]
fn invalid_write_output_gzip() -> TestResult {
    let temp_dir = TempDir::new()?;
    let output = temp_dir.child("invalid.mrc.gz");

    let mut cmd = marc_cmd();
    let assert = cmd
        .arg("invalid")
        .arg(data_dir().join("invalid.mrc"))
        .arg(data_dir().join("ada.mrc.gz"))
        .args(["-o", output.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let mut gz = GzDecoder::new(File::open(output.path())?);
    let mut actual = Vec::new();
    gz.read_to_end(&mut actual)?;

    assert_eq!(fs::read(data_dir().join("invalid.mrc"))?, actual);

    temp_dir.close()?;
    Ok(())
}
