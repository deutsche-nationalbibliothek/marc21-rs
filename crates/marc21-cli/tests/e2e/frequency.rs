use std::fs::File;
use std::io::{Read, read_to_string};

use flate2::read::GzDecoder;

use crate::prelude::*;

#[test]
fn frequency_stdout_csv() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["frequency", "-s", "065{ a | 2 == 'sswd' }"])
        .arg(data_dir().join("DUMP.mrc.gz"))
        .assert();

    let expected = r#"12.2p,7
13.4p,1
15.1p,1
16.1p,1
16.5p,1
18p,1
4.7p,1
7.14p,1
"#;

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq(expected))
        .stderr(predicates::str::is_empty());

    Ok(())
}

#[test]
fn frequency_stdout_tsv() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["frequency", "-s", "--tsv"])
        .arg("065{ a | 2 == 'sswd' }")
        .arg(data_dir().join("ada.mrc"))
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq("28p\t1\n9.5p\t1\n"))
        .stderr(predicates::str::is_empty());

    Ok(())
}

#[test]
fn frequency_header() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["frequency", "-s"])
        .args(["--header", "gndsys,count"])
        .arg("065{ a | 2 == 'sswd' }")
        .arg(data_dir().join("DUMP.mrc.gz"))
        .assert();

    let expected = r#"gndsys,count
12.2p,7
13.4p,1
15.1p,1
16.1p,1
16.5p,1
18p,1
4.7p,1
7.14p,1
"#;

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq(expected))
        .stderr(predicates::str::is_empty());

    Ok(())
}

#[test]
fn frequency_write_csv() -> TestResult {
    let temp_dir = TempDir::new()?;
    let output = temp_dir.child("out.csv");

    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["freq", "-s", "065{ a | 2 == 'sswd' }"])
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

    let expected = r#"12.2p,7
13.4p,1
15.1p,1
16.1p,1
16.5p,1
18p,1
4.7p,1
7.14p,1
"#;

    assert_eq!(actual, expected);
    temp_dir.close()?;

    Ok(())
}

#[test]
fn frequency_write_csv_gz() -> TestResult {
    let temp_dir = TempDir::new()?;
    let output = temp_dir.child("out.csv.gz");

    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["freq", "-s", "065{ a | 2 == 'sswd' }"])
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

    if cfg!(windows) {
        actual = actual.replace('\r', "");
    }

    let expected = r#"12.2p,7
13.4p,1
15.1p,1
16.1p,1
16.5p,1
18p,1
4.7p,1
7.14p,1
"#;

    assert_eq!(actual, expected);
    temp_dir.close()?;

    Ok(())
}

#[test]
fn frequency_write_tsv() -> TestResult {
    let temp_dir = TempDir::new()?;
    let output = temp_dir.child("out.tsv");

    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["freq", "-s", "065{ a | 2 == 'sswd' }"])
        .arg(data_dir().join("ada.mrc.gz"))
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

    assert_eq!(actual, "28p\t1\n9.5p\t1\n");
    temp_dir.close()?;

    Ok(())
}

#[test]
fn frequency_write_tsv_gz() -> TestResult {
    let temp_dir = TempDir::new()?;
    let output = temp_dir.child("out.tsv.gz");

    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["freq", "-s", "065{ a | 2 == 'sswd' }"])
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

    if cfg!(windows) {
        actual = actual.replace('\r', "");
    }

    assert_eq!(actual, "28p\t1\n9.5p\t1\n");
    temp_dir.close()?;

    Ok(())
}

#[test]
fn frequency_where() -> TestResult {
    let temp_dir = TempDir::new()?;
    let output = temp_dir.child("out.tsv");

    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["freq", "-s", "065{ a | 2 == 'sswd' }"])
        .arg(data_dir().join("DUMP.mrc.gz"))
        .args(["--where", "001 == '118572121'"])
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

    assert_eq!(actual, "12.2p\t1\n4.7p\t1\n");
    temp_dir.close()?;

    Ok(())
}

#[test]
fn frequency_skip_invalid() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["frequency", "065.a"])
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
        .args(["frequency", "-s", "065.a"])
        .arg(data_dir().join("invalid.mrc"))
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    Ok(())
}
