use std::fs::{self, File, read, read_to_string};
use std::io::Read;

use flate2::read::GzDecoder;

use crate::prelude::*;

#[test]
fn hash_stdin() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert = cmd
        .arg("hash")
        .write_stdin(fs::read(data_dir().join("ada.mrc"))?)
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq(
            "cn,hash\n119232022,59e1a2702f5e1dab410c07dc8c37961b4e873b2712c680b034ae452fada48c33\n"))
        .stderr(predicates::str::is_empty());

    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["hash", "-"])
        .write_stdin(fs::read(data_dir().join("ada.mrc"))?)
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq(
            "cn,hash\n119232022,59e1a2702f5e1dab410c07dc8c37961b4e873b2712c680b034ae452fada48c33\n"))
        .stderr(predicates::str::is_empty());

    Ok(())
}

#[test]
fn hash_stdout() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert =
        cmd.arg("hash").arg(data_dir().join("ada.mrc.gz")).assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq(
            "cn,hash\n119232022,59e1a2702f5e1dab410c07dc8c37961b4e873b2712c680b034ae452fada48c33\n"))
        .stderr(predicates::str::is_empty());

    Ok(())
}

#[test]
fn hash_output_csv() -> TestResult {
    let mut cmd = marc21_cmd();
    let temp_dir = TempDir::new()?;
    let output = temp_dir.child("out.csv");

    let assert = cmd
        .args(["hash", "-s"])
        .arg(data_dir().join("DUMP.mrc.gz"))
        .args(["-o", output.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let expected = read_to_string(data_dir().join("hashes.csv"))?;
    let mut actual = read_to_string(output)?;
    if cfg!(windows) {
        actual = actual.replace('\r', "");
    }

    assert_eq!(expected, actual);
    temp_dir.close()?;
    Ok(())
}

#[test]
fn hash_output_tsv() -> TestResult {
    let temp_dir = TempDir::new()?;
    let output = temp_dir.child("out.txt");

    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["hash", "-s", "--tsv"])
        .arg(data_dir().join("DUMP.mrc.gz"))
        .args(["-o", output.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let expected = read_to_string(data_dir().join("hashes.tsv"))?;
    let mut actual = read_to_string(output)?;
    if cfg!(windows) {
        actual = actual.replace('\r', "");
    }

    assert_eq!(expected, actual);

    let output = temp_dir.child("out.tsv");

    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["hash", "-s"])
        .arg(data_dir().join("DUMP.mrc.gz"))
        .args(["-o", output.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let expected = read_to_string(data_dir().join("hashes.tsv"))?;
    let mut actual = read_to_string(output)?;
    if cfg!(windows) {
        actual = actual.replace('\r', "");
    }

    assert_eq!(expected, actual);

    let output = temp_dir.child("out.tsv.gz");

    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["hash", "-s"])
        .arg(data_dir().join("DUMP.mrc.gz"))
        .args(["-o", output.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let expected = read(data_dir().join("hashes.tsv"))?;

    let mut gz = GzDecoder::new(File::open(output.path())?);
    let mut actual = Vec::new();
    gz.read_to_end(&mut actual)?;

    assert_eq!(expected, actual);
    temp_dir.close()?;
    Ok(())
}

#[test]
fn hash_output_tsv_gz() -> TestResult {
    let temp_dir = TempDir::new()?;
    let output = temp_dir.child("out.tsv.gz");

    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["hash", "-s"])
        .arg(data_dir().join("DUMP.mrc.gz"))
        .args(["-o", output.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let expected = read(data_dir().join("hashes.tsv"))?;

    let mut gz = GzDecoder::new(File::open(output.path())?);
    let mut actual = Vec::new();
    gz.read_to_end(&mut actual)?;

    assert_eq!(expected, actual);
    temp_dir.close()?;
    Ok(())
}

#[test]
fn hash_where() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["hash", "-s"])
        .arg(data_dir().join("DUMP.mrc.gz"))
        .args(["--where", "001 == '040992918'"])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq(
            "cn,hash\n040992918,520e1f9abc3bea7b719cc87a790f24c672a0b503a2a395450695d6bfd46b0cc2\n"))
        .stderr(predicates::str::is_empty());

    Ok(())
}

#[test]
fn hash_skip_invalid() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert =
        cmd.arg("hash").arg(data_dir().join("DUMP.mrc.gz")).assert();

    assert
        .failure()
        .code(1)
        .stdout(predicates::str::is_empty().not())
        .stderr(predicates::str::starts_with(
            "error: could not parse record 7",
        ));

    Ok(())
}
