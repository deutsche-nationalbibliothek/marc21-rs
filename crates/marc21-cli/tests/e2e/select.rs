use std::fs::File;
use std::io::{Read, read_to_string};

use flate2::read::GzDecoder;

use crate::prelude::*;

#[test]
fn select_simple() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["select", "-s", "001,075{ b | 2 == 'gndgen' }"])
        .arg(data_dir().join("DUMP.mrc.gz"))
        .assert();

    let expected = r#"118540238,p
118572121,p
118607626,p
118632477,p
040992020,u
040992918,u
040993396,u
"#;

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq(expected))
        .stderr(predicates::str::is_empty());

    Ok(())
}

#[test]
fn select_cartesian_product1() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["select", "-s", "001,075.b"])
        .arg(data_dir().join("ada.mrc"))
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq("119232022,p\n119232022,piz\n"))
        .stderr(predicates::str::is_empty());

    Ok(())
}

#[test]
fn select_cartesian_product2() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["select", "-s", "001,079{ a, q }"])
        .arg(data_dir().join("ada.mrc"))
        .assert();

    let expected = r#"119232022,g,f
119232022,g,s
119232022,g,z
"#;

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq(expected))
        .stderr(predicates::str::is_empty());

    Ok(())
}

#[test]
fn select_write_csv() -> TestResult {
    let temp_dir = TempDir::new()?;
    let output = temp_dir.child("out.csv");

    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["select", "-s", "001,075{ b | 2 == 'gndgen' }"])
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

    let expected = r#"118540238,p
118572121,p
118607626,p
118632477,p
040992020,u
040992918,u
040993396,u
"#;

    assert_eq!(actual, expected);
    temp_dir.close()?;

    Ok(())
}

#[test]
fn select_write_csv_gz() -> TestResult {
    let temp_dir = TempDir::new()?;
    let output = temp_dir.child("out.csv.gz");

    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["select", "-s", "001,075{ b | 2 == 'gndgen' }"])
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

    let expected = r#"118540238,p
118572121,p
118607626,p
118632477,p
040992020,u
040992918,u
040993396,u
"#;

    assert_eq!(actual, expected);
    temp_dir.close()?;

    Ok(())
}

#[test]
fn select_write_tsv() -> TestResult {
    let temp_dir = TempDir::new()?;
    let output = temp_dir.child("out.tsv");

    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["select", "-s", "001,075{ b | 2 == 'gndgen' }"])
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

    assert_eq!(actual, "119232022\tp\n");

    temp_dir.close()?;
    Ok(())
}

#[test]
fn select_write_tsv_gz() -> TestResult {
    let temp_dir = TempDir::new()?;
    let output = temp_dir.child("out.tsv.gz");

    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["select", "-s", "001,075{ b | 2 == 'gndgen' }"])
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

    assert_eq!(actual, "119232022\tp\n");

    temp_dir.close()?;
    Ok(())
}

#[test]
fn select_where() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["select", "-s", "001,075{ b | 2 == 'gndgen' }"])
        .arg(data_dir().join("DUMP.mrc.gz"))
        .args(["--where", "001 in ['118540238', '040993396']"])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq("118540238,p\n040993396,u\n"))
        .stderr(predicates::str::is_empty());

    Ok(())
}

#[test]
fn select_skip_invalid() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["select", "001"])
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
        .args(["select", "-s", "001"])
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
fn select_limit() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["select", "-s", "--limit", "3"])
        .arg("001,075{ b | 2 == 'gndgen' }")
        .arg(data_dir().join("DUMP.mrc.gz"))
        .assert();

    let expected = r#"118540238,p
118572121,p
118607626,p
"#;

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq(expected))
        .stderr(predicates::str::is_empty());

    Ok(())
}
