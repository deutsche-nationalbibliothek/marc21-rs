use std::fs::File;
use std::io::{Read, read_to_string};

use flate2::read::GzDecoder;

use crate::prelude::*;

#[test]
fn count_stdout() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert =
        cmd.arg("count").arg(data_dir().join("ada.mrc")).assert();

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
        .args(["count", "-s"])
        .arg(data_dir().join("DUMP.mrc.gz"))
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq("7\n"))
        .stderr(predicates::str::is_empty());

    Ok(())
}

#[test]
fn count_multiple_files() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["count", "-s"])
        .arg(data_dir().join("DUMP.mrc.gz"))
        .arg(data_dir().join("ada.mrc"))
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq("8\n"))
        .stderr(predicates::str::is_empty());

    Ok(())
}

#[test]
fn count_output() -> TestResult {
    let temp_dir = TempDir::new()?;
    let output = temp_dir.child("count.txt");

    let mut cmd = marc21_cmd();
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
fn count_output_gzip() -> TestResult {
    let temp_dir = TempDir::new()?;
    let output = temp_dir.child("count.txt.gz");

    let mut cmd = marc21_cmd();
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

#[test]
fn count_where() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["count", "-s"])
        .arg(data_dir().join("DUMP.mrc.gz"))
        .args(["--where", "075{ b == 'p' && 2 == 'gndgen' }"])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq("4\n"))
        .stderr(predicates::str::is_empty());

    Ok(())
}

#[test]
fn count_skip_invalid() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert = cmd
        .arg("count")
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
        .args(["count", "-s"])
        .arg(data_dir().join("invalid.mrc"))
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq("0\n"))
        .stderr(predicates::str::is_empty());

    Ok(())
}

#[test]
fn count_limit() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["count", "--limit", "2"])
        .arg(data_dir().join("DUMP.mrc.gz"))
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
fn count_filter_normalization_nfc() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["count", "-s"])
        .args(["--filter-normalization", "nfc"])
        .arg(data_dir().join("minna.mrc"))
        .args(["--where", "100/1#.t == 'Minna von Barnhelm'"])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq("1\n"))
        .stderr(predicates::str::is_empty());

    Ok(())
}

#[test]
fn count_filter_normalization_nfkc() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["count", "-s"])
        .args(["--filter-normalization", "nfkc"])
        .arg(data_dir().join("minna.mrc"))
        .args(["--where", "100/1#.t == 'Minna von Barnhelm'"])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq("1\n"))
        .stderr(predicates::str::is_empty());

    Ok(())
}

#[test]
fn count_filter_normalization_nfd() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["count", "-s"])
        .args(["--filter-normalization", "nfd"])
        .arg(data_dir().join("minna.mrc"))
        .args(["--where", "678.b == 'Epoche: Aufklärung'"])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq("1\n"))
        .stderr(predicates::str::is_empty());

    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["count", "-s"])
        .arg(data_dir().join("minna.mrc"))
        .args(["--where", "678.b == 'Epoche: Aufklärung'"])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq("0\n"))
        .stderr(predicates::str::is_empty());

    Ok(())
}

#[test]
fn count_filter_normalization_nfkd() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["count", "-s"])
        .args(["--filter-normalization", "nfkd"])
        .arg(data_dir().join("minna.mrc"))
        .args(["--where", "678.b == 'Epoche: Aufklärung'"])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq("1\n"))
        .stderr(predicates::str::is_empty());

    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["count", "-s"])
        .arg(data_dir().join("minna.mrc"))
        .args(["--where", "678.b == 'Epoche: Aufklärung'"])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq("0\n"))
        .stderr(predicates::str::is_empty());

    Ok(())
}
