use std::fs::{self, File};
use std::io::Read;

use flate2::read::GzDecoder;

use crate::prelude::*;

#[test]
fn grep_stdout() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["grep", "-s", "^Epoche:\\s+Auf.*"])
        .arg(data_dir().join("DUMP.mrc.gz"))
        .assert();

    let expected = fs::read(data_dir().join("minna.mrc"))?;

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq(expected))
        .stderr(predicates::str::is_empty());

    Ok(())
}

#[test]
fn grep_write_output() -> TestResult {
    let temp_dir = TempDir::new()?;
    let output = temp_dir.child("out.mrc");

    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["grep", "-s", "^Epoche:\\s+Auf.*"])
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

    temp_dir.close().unwrap();
    Ok(())
}

#[test]
fn grep_write_output_gzip() -> TestResult {
    let temp_dir = TempDir::new()?;
    let output = temp_dir.child("out.mrc.gz");

    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["grep", "-s", "^Epoche:\\s+Auf.*"])
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

    temp_dir.close().unwrap();
    Ok(())
}

#[test]
fn grep_multiple_patterns() -> TestResult {
    let temp_dir = TempDir::new()?;
    let output = temp_dir.child("out.mrc");

    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["grep", "-s", "abcdefghijkl"])
        .args(["--or", "^(foo|bar|baz).*"])
        .args(["--or", "^Epoche:\\sAuf"])
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

    temp_dir.close().unwrap();
    Ok(())
}

#[test]
fn grep_where() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["grep", "-s", "^Epoche:\\s+Auf.*"])
        .arg(data_dir().join("DUMP.mrc.gz"))
        .args(["--where", "001 == '040992918'"])
        .assert();

    let expected = fs::read(data_dir().join("minna.mrc"))?;

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq(expected))
        .stderr(predicates::str::is_empty());

    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["grep", "-s", "^Epoche:\\s+Auf.*"])
        .arg(data_dir().join("DUMP.mrc.gz"))
        .args(["--where", "001 != '040992918'"])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    Ok(())
}

#[test]
fn grep_invert_match() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["grep", "-s", "-v", "^Epoche:\\s+Auf"])
        .arg(data_dir().join("minna.mrc"))
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["grep", "-s", "-v", "^Epoche:\\s+Sturm"])
        .arg(data_dir().join("minna.mrc"))
        .assert();

    let expected = fs::read(data_dir().join("minna.mrc"))?;

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq(expected))
        .stderr(predicates::str::is_empty());

    Ok(())
}

#[test]
fn grep_skip_invalid() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["grep", "Lessing"])
        .arg(data_dir().join("DUMP.mrc.gz"))
        .assert();

    assert
        .failure()
        .code(1)
        .stdout(predicates::str::is_empty().not())
        .stderr(predicates::str::starts_with(
            "error: could not parse record (line 8",
        ));

    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["grep", "-s", "Lessing"])
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
fn grep_ignore_case() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["grep", "-s", "^epoche:\\s+auf.*"])
        .arg(data_dir().join("DUMP.mrc.gz"))
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["grep", "-s", "-i", "^epoche:\\s+auf.*"])
        .arg(data_dir().join("DUMP.mrc.gz"))
        .assert();

    let expected = fs::read(data_dir().join("minna.mrc"))?;

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq(expected))
        .stderr(predicates::str::is_empty());

    Ok(())
}
