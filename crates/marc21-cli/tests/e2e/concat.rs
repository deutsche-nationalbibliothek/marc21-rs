use std::fs::{self, File};
use std::io::Read;

use flate2::read::GzDecoder;

use crate::prelude::*;

#[test]
fn concat_stdin() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert = cmd
        .arg("concat")
        .write_stdin(fs::read(data_dir().join("ada.mrc"))?)
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq(fs::read(
            data_dir().join("ada.mrc"),
        )?))
        .stderr(predicates::str::is_empty());

    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["concat", "-"])
        .write_stdin(fs::read(data_dir().join("ada.mrc"))?)
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
fn concat_stdout() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["concat", "-s"])
        .arg(data_dir().join("invalid.mrc"))
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
fn concat_output() -> TestResult {
    let temp_dir = TempDir::new()?;
    let output = temp_dir.child("out.mrc");

    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["concat", "-s"])
        .arg(data_dir().join("ada.mrc.gz"))
        .arg(data_dir().join("invalid.mrc"))
        .arg(data_dir().join("ada.mrc"))
        .args(["-o", output.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let actual = fs::read(output.path())?;

    let ada = fs::read(data_dir().join("ada.mrc"))?;
    let mut expected = Vec::with_capacity(ada.len() * 2);
    expected.extend_from_slice(&ada);
    expected.extend_from_slice(&ada);

    assert_eq!(actual, expected);

    temp_dir.close()?;
    Ok(())
}

#[test]
fn concat_output_gzip() -> TestResult {
    let temp_dir = TempDir::new()?;
    let output = temp_dir.child("out.mrc.gz");

    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["concat", "-s"])
        .arg(data_dir().join("invalid.mrc"))
        .arg(data_dir().join("ada.mrc"))
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

    assert_eq!(fs::read(data_dir().join("ada.mrc"))?, actual);

    temp_dir.close()?;
    Ok(())
}

#[test]
fn concat_skip_invalid() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert = cmd
        .arg("concat")
        .arg(data_dir().join("invalid.mrc"))
        .assert();

    assert
        .failure()
        .code(1)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::starts_with(
            "error: could not parse record 0",
        ));

    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["concat", "-s"])
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
fn concat_where() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["concat", "-s"])
        .arg(data_dir().join("DUMP.mrc.gz"))
        .arg(data_dir().join("ada.mrc"))
        .args(["--where", "001 == '119232022'"])
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
