use std::fs::File;
use std::io::{Read, read_to_string};

use flate2::read::GzDecoder;

use crate::prelude::*;

const EXPECTED: &'static str = r#"$9 r:DE-101, r:DE-101, r:DE-101, r:DE-101, r:DE-101, r:DE-101, r:DE-101
$a DE-101, DE-101, DE-101, DE-101, DE-101, DE-101, DE-101
$b ger, ger, ger, ger, ger, ger, ger
$c DE-101, DE-101, DE-101, DE-101, DE-101, DE-101, DE-101
$d 0547, 0018, 0032, 0292, 1764, 1140, 1764
$e rda, rda, rda, rda, rda, rda, rda
"#;

#[test]
fn glimpse_stdout() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["glimpse", "-s", "040{ _ | e == 'rda' }"])
        .arg(data_dir().join("DUMP.mrc.gz"))
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq(EXPECTED))
        .stderr(predicates::str::is_empty());

    Ok(())
}

#[test]
fn glimpse_output() -> TestResult {
    let temp_dir = TempDir::new()?;
    let output = temp_dir.child("glimpse.txt");

    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["glimpse", "-s", "040{ _ | e == 'rda' }"])
        .arg(data_dir().join("DUMP.mrc.gz"))
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

    assert_eq!(actual, EXPECTED);

    temp_dir.close()?;
    Ok(())
}

#[test]
fn glimpse_gzip() -> TestResult {
    let temp_dir = TempDir::new()?;
    let output = temp_dir.child("glimpse.txt.gz");

    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["glimpse", "-s", "040{ _ | e == 'rda' }"])
        .arg(data_dir().join("DUMP.mrc.gz"))
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
    assert_eq!(actual, EXPECTED);

    temp_dir.close()?;
    Ok(())
}

#[test]
fn glimpse_max_values() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["glimpse", "-n", "3", "-s", "040{ [ad] | e == 'rda' }"])
        .arg(data_dir().join("DUMP.mrc.gz"))
        .assert();

    let expected = r#"$a DE-101, DE-101, DE-101
$d 0547, 0018, 0032
"#;

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq(expected))
        .stderr(predicates::str::is_empty());

    Ok(())
}

#[test]
fn glimpse_skip_invalid() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["glimpse", "040.a"])
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
        .args(["glimpse", "-s", "040.a"])
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
fn glimpse_limit() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["glimpse", "-s", "-n", "10"])
        .args(["-l", "3"])
        .arg("040{ [ad] | e == 'rda' }")
        .arg(data_dir().join("DUMP.mrc.gz"))
        .assert();

    let expected = r#"$a DE-101, DE-101, DE-101
$d 0547, 0018, 0032
"#;

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq(expected))
        .stderr(predicates::str::is_empty());

    Ok(())
}

#[test]
fn glimpse_where() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["glimpse", "-s", "040{ [ad] | e == 'rda' }"])
        .arg(data_dir().join("DUMP.mrc.gz"))
        .args(["--where", "040.d >= '1000'"])
        .assert();

    let expected = r#"$a DE-101, DE-101, DE-101
$d 1764, 1140, 1764
"#;

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq(expected))
        .stderr(predicates::str::is_empty());

    Ok(())
}
