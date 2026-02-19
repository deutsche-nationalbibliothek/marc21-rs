use std::fs::{self, File};
use std::io::Read;

use flate2::read::GzDecoder;
use predicates::prelude::PredicateBooleanExt;

use crate::prelude::*;

#[test]
fn filter_stdout() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["filter", "-s"])
        .arg("001 == '040992918'")
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
fn filter_stdin() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["filter", "-s"])
        .arg("001 == '040992918'")
        .write_stdin(fs::read(data_dir().join("minna.mrc"))?)
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq(fs::read(
            data_dir().join("minna.mrc"),
        )?))
        .stderr(predicates::str::is_empty());

    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["filter", "-s"])
        .arg("001 == '040992918'")
        .arg("-")
        .write_stdin(fs::read(data_dir().join("minna.mrc"))?)
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
fn filter_output() -> TestResult {
    let temp_dir = TempDir::new()?;
    let output = temp_dir.child("out.mrc");

    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["filter", "-s"])
        .arg("100/1#.t == 'Minna von Barnhelm' && 075{ b == 'u' && 2 == 'gndgen' }")
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
        fs::read(output.path())?
    );

    Ok(())
}

#[test]
fn filter_gzip() -> TestResult {
    let temp_dir = TempDir::new()?;
    let output = temp_dir.child("out.mrc.gz");

    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["filter", "-s"])
        .arg("400/1#.t =^ 'Minna von Barnhelm'")
        .arg(data_dir().join("DUMP.mrc.gz"))
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

    assert_eq!(actual, fs::read(data_dir().join("minna.mrc"))?,);
    Ok(())
}

#[test]
fn filter_skip_invalid() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["filter"])
        .arg("001 == '040992918'")
        .arg(data_dir().join("DUMP.mrc.gz"))
        .assert();

    assert
        .failure()
        .code(1)
        .stdout(predicates::str::is_empty().not())
        .stderr(predicates::str::starts_with(
            "error: could not parse record 7",
        ));

    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["filter", "-s"])
        .arg("001 == '040992918'")
        .arg(data_dir().join("DUMP.mrc.gz"))
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty().not())
        .stderr(predicates::str::is_empty());

    Ok(())
}

#[test]
fn filter_strsim_threshold() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["filter", "-s"])
        .arg("100/1#.a =* 'Lovelace, Bda'")
        .arg(data_dir().join("ada.mrc.gz"))
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
        .args(["filter", "-s"])
        .args(["--strsim-threshold", "95"])
        .arg("100/1#.a =* 'Lovelace, Bda'")
        .arg(data_dir().join("ada.mrc.gz"))
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    Ok(())
}

#[test]
fn filter_invalid_filter() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["filter", "-s"])
        .arg("001 == 040992918'")
        .arg(data_dir().join("DUMP.mrc.gz"))
        .assert();

    assert
        .failure()
        .code(2)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::starts_with("error: invalid value"));

    Ok(())
}
