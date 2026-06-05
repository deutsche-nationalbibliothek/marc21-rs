use std::fs::File;
use std::io::{Read, read_to_string};

use flate2::read::GzDecoder;

use crate::prelude::*;

#[test]
fn describe_stdout_csv() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["describe", "-s"])
        .arg(data_dir().join("ada.mrc"))
        .assert();

    let expected = r#"field,ind1,ind2,0,2,4,9,S,a,b,c,d,e,i,q,u,w,z
024,7, ,1,1,0,0,0,1,0,0,0,0,0,0,0,0,0
035, , ,0,0,0,3,0,2,0,0,0,0,0,0,0,0,4
040, , ,0,0,0,1,0,1,1,1,1,0,0,0,0,0,0
042, , ,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0
043, , ,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0
065, , ,0,2,0,0,0,2,0,0,0,0,0,0,0,0,0
075, , ,0,2,0,0,0,0,2,0,0,0,0,0,0,0,0
079, , ,0,0,0,0,0,1,0,0,0,0,0,3,3,0,0
100,1, ,0,0,0,0,0,1,0,0,1,0,0,0,0,0,0
375, , ,0,1,0,0,0,1,0,0,0,0,0,0,0,0,0
400,1, ,0,0,4,0,0,13,0,2,13,2,2,0,0,2,0
500,1, ,9,0,6,3,0,3,0,1,3,3,3,0,0,3,0
548, , ,0,0,4,0,0,2,0,0,0,0,2,0,0,2,0
550, , ,6,0,4,0,0,2,0,0,0,0,2,0,0,2,0
551, , ,6,0,4,0,0,2,0,0,0,0,2,0,0,2,0
667, , ,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0
670, , ,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0
913, , ,2,0,0,0,2,2,0,0,0,0,2,0,0,0,0
"#;

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq(expected))
        .stderr(predicates::str::is_empty());

    Ok(())
}

#[test]
fn describe_write_csv() -> TestResult {
    let temp_dir = TempDir::new()?;
    let output = temp_dir.child("out.csv");

    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["describe", "-s"])
        .arg(data_dir().join("ada.mrc"))
        .args(["-o", output.to_str().unwrap()])
        .assert();

    let expected = r#"field,ind1,ind2,0,2,4,9,S,a,b,c,d,e,i,q,u,w,z
024,7, ,1,1,0,0,0,1,0,0,0,0,0,0,0,0,0
035, , ,0,0,0,3,0,2,0,0,0,0,0,0,0,0,4
040, , ,0,0,0,1,0,1,1,1,1,0,0,0,0,0,0
042, , ,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0
043, , ,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0
065, , ,0,2,0,0,0,2,0,0,0,0,0,0,0,0,0
075, , ,0,2,0,0,0,0,2,0,0,0,0,0,0,0,0
079, , ,0,0,0,0,0,1,0,0,0,0,0,3,3,0,0
100,1, ,0,0,0,0,0,1,0,0,1,0,0,0,0,0,0
375, , ,0,1,0,0,0,1,0,0,0,0,0,0,0,0,0
400,1, ,0,0,4,0,0,13,0,2,13,2,2,0,0,2,0
500,1, ,9,0,6,3,0,3,0,1,3,3,3,0,0,3,0
548, , ,0,0,4,0,0,2,0,0,0,0,2,0,0,2,0
550, , ,6,0,4,0,0,2,0,0,0,0,2,0,0,2,0
551, , ,6,0,4,0,0,2,0,0,0,0,2,0,0,2,0
667, , ,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0
670, , ,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0
913, , ,2,0,0,0,2,2,0,0,0,0,2,0,0,0,0
"#;

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let mut actual = read_to_string(File::open(output.path())?)?;
    if cfg!(windows) {
        actual = actual.replace('\r', "");
    }

    assert_eq!(actual, expected);

    temp_dir.close()?;
    Ok(())
}

#[test]
fn describe_write_csv_gz() -> TestResult {
    let temp_dir = TempDir::new()?;
    let output = temp_dir.child("out.csv.gz");

    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["describe", "-s"])
        .arg(data_dir().join("ada.mrc"))
        .args(["-o", output.to_str().unwrap()])
        .assert();

    let expected = r#"field,ind1,ind2,0,2,4,9,S,a,b,c,d,e,i,q,u,w,z
024,7, ,1,1,0,0,0,1,0,0,0,0,0,0,0,0,0
035, , ,0,0,0,3,0,2,0,0,0,0,0,0,0,0,4
040, , ,0,0,0,1,0,1,1,1,1,0,0,0,0,0,0
042, , ,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0
043, , ,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0
065, , ,0,2,0,0,0,2,0,0,0,0,0,0,0,0,0
075, , ,0,2,0,0,0,0,2,0,0,0,0,0,0,0,0
079, , ,0,0,0,0,0,1,0,0,0,0,0,3,3,0,0
100,1, ,0,0,0,0,0,1,0,0,1,0,0,0,0,0,0
375, , ,0,1,0,0,0,1,0,0,0,0,0,0,0,0,0
400,1, ,0,0,4,0,0,13,0,2,13,2,2,0,0,2,0
500,1, ,9,0,6,3,0,3,0,1,3,3,3,0,0,3,0
548, , ,0,0,4,0,0,2,0,0,0,0,2,0,0,2,0
550, , ,6,0,4,0,0,2,0,0,0,0,2,0,0,2,0
551, , ,6,0,4,0,0,2,0,0,0,0,2,0,0,2,0
667, , ,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0
670, , ,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0
913, , ,2,0,0,0,2,2,0,0,0,0,2,0,0,0,0
"#;

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

    assert_eq!(actual, expected);

    temp_dir.close()?;
    Ok(())
}

#[test]
fn describe_stdout_tsv() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["describe", "-s", "--tsv"])
        .arg(data_dir().join("ada.mrc"))
        .assert();

    let expected = r#"field,ind1,ind2,0,2,4,9,S,a,b,c,d,e,i,q,u,w,z
024,7, ,1,1,0,0,0,1,0,0,0,0,0,0,0,0,0
035, , ,0,0,0,3,0,2,0,0,0,0,0,0,0,0,4
040, , ,0,0,0,1,0,1,1,1,1,0,0,0,0,0,0
042, , ,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0
043, , ,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0
065, , ,0,2,0,0,0,2,0,0,0,0,0,0,0,0,0
075, , ,0,2,0,0,0,0,2,0,0,0,0,0,0,0,0
079, , ,0,0,0,0,0,1,0,0,0,0,0,3,3,0,0
100,1, ,0,0,0,0,0,1,0,0,1,0,0,0,0,0,0
375, , ,0,1,0,0,0,1,0,0,0,0,0,0,0,0,0
400,1, ,0,0,4,0,0,13,0,2,13,2,2,0,0,2,0
500,1, ,9,0,6,3,0,3,0,1,3,3,3,0,0,3,0
548, , ,0,0,4,0,0,2,0,0,0,0,2,0,0,2,0
550, , ,6,0,4,0,0,2,0,0,0,0,2,0,0,2,0
551, , ,6,0,4,0,0,2,0,0,0,0,2,0,0,2,0
667, , ,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0
670, , ,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0
913, , ,2,0,0,0,2,2,0,0,0,0,2,0,0,0,0
"#;

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq(expected.replace(",", "\t")))
        .stderr(predicates::str::is_empty());

    Ok(())
}

#[test]
fn describe_write_tsv() -> TestResult {
    let temp_dir = TempDir::new()?;
    let output = temp_dir.child("out.tsv");

    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["describe", "-s"])
        .arg(data_dir().join("ada.mrc"))
        .args(["-o", output.to_str().unwrap()])
        .assert();

    let expected = r#"field,ind1,ind2,0,2,4,9,S,a,b,c,d,e,i,q,u,w,z
024,7, ,1,1,0,0,0,1,0,0,0,0,0,0,0,0,0
035, , ,0,0,0,3,0,2,0,0,0,0,0,0,0,0,4
040, , ,0,0,0,1,0,1,1,1,1,0,0,0,0,0,0
042, , ,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0
043, , ,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0
065, , ,0,2,0,0,0,2,0,0,0,0,0,0,0,0,0
075, , ,0,2,0,0,0,0,2,0,0,0,0,0,0,0,0
079, , ,0,0,0,0,0,1,0,0,0,0,0,3,3,0,0
100,1, ,0,0,0,0,0,1,0,0,1,0,0,0,0,0,0
375, , ,0,1,0,0,0,1,0,0,0,0,0,0,0,0,0
400,1, ,0,0,4,0,0,13,0,2,13,2,2,0,0,2,0
500,1, ,9,0,6,3,0,3,0,1,3,3,3,0,0,3,0
548, , ,0,0,4,0,0,2,0,0,0,0,2,0,0,2,0
550, , ,6,0,4,0,0,2,0,0,0,0,2,0,0,2,0
551, , ,6,0,4,0,0,2,0,0,0,0,2,0,0,2,0
667, , ,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0
670, , ,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0
913, , ,2,0,0,0,2,2,0,0,0,0,2,0,0,0,0
"#;

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let mut actual = read_to_string(File::open(output.path())?)?;
    if cfg!(windows) {
        actual = actual.replace('\r', "");
    }

    assert_eq!(actual, expected.replace(",", "\t"));

    temp_dir.close()?;
    Ok(())
}

#[test]
fn describe_write_tsv_gz() -> TestResult {
    let temp_dir = TempDir::new()?;
    let output = temp_dir.child("out.tsv.gz");

    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["describe", "-s"])
        .arg(data_dir().join("ada.mrc"))
        .args(["-o", output.to_str().unwrap()])
        .assert();

    let expected = r#"field,ind1,ind2,0,2,4,9,S,a,b,c,d,e,i,q,u,w,z
024,7, ,1,1,0,0,0,1,0,0,0,0,0,0,0,0,0
035, , ,0,0,0,3,0,2,0,0,0,0,0,0,0,0,4
040, , ,0,0,0,1,0,1,1,1,1,0,0,0,0,0,0
042, , ,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0
043, , ,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0
065, , ,0,2,0,0,0,2,0,0,0,0,0,0,0,0,0
075, , ,0,2,0,0,0,0,2,0,0,0,0,0,0,0,0
079, , ,0,0,0,0,0,1,0,0,0,0,0,3,3,0,0
100,1, ,0,0,0,0,0,1,0,0,1,0,0,0,0,0,0
375, , ,0,1,0,0,0,1,0,0,0,0,0,0,0,0,0
400,1, ,0,0,4,0,0,13,0,2,13,2,2,0,0,2,0
500,1, ,9,0,6,3,0,3,0,1,3,3,3,0,0,3,0
548, , ,0,0,4,0,0,2,0,0,0,0,2,0,0,2,0
550, , ,6,0,4,0,0,2,0,0,0,0,2,0,0,2,0
551, , ,6,0,4,0,0,2,0,0,0,0,2,0,0,2,0
667, , ,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0
670, , ,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0
913, , ,2,0,0,0,2,2,0,0,0,0,2,0,0,0,0
"#;

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

    assert_eq!(actual, expected.replace(",", "\t"));

    temp_dir.close()?;
    Ok(())
}

#[test]
fn describe_skip_invalid() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert = cmd
        .arg("describe")
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
        .args(["describe", "-s"])
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
fn describe_where() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["describe", "-s"])
        .arg(data_dir().join("DUMP.mrc.gz"))
        .arg(data_dir().join("ada.mrc"))
        .args(["--where", "001 == '119232022'"])
        .assert();

    let expected = r#"field,ind1,ind2,0,2,4,9,S,a,b,c,d,e,i,q,u,w,z
024,7, ,1,1,0,0,0,1,0,0,0,0,0,0,0,0,0
035, , ,0,0,0,3,0,2,0,0,0,0,0,0,0,0,4
040, , ,0,0,0,1,0,1,1,1,1,0,0,0,0,0,0
042, , ,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0
043, , ,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0
065, , ,0,2,0,0,0,2,0,0,0,0,0,0,0,0,0
075, , ,0,2,0,0,0,0,2,0,0,0,0,0,0,0,0
079, , ,0,0,0,0,0,1,0,0,0,0,0,3,3,0,0
100,1, ,0,0,0,0,0,1,0,0,1,0,0,0,0,0,0
375, , ,0,1,0,0,0,1,0,0,0,0,0,0,0,0,0
400,1, ,0,0,4,0,0,13,0,2,13,2,2,0,0,2,0
500,1, ,9,0,6,3,0,3,0,1,3,3,3,0,0,3,0
548, , ,0,0,4,0,0,2,0,0,0,0,2,0,0,2,0
550, , ,6,0,4,0,0,2,0,0,0,0,2,0,0,2,0
551, , ,6,0,4,0,0,2,0,0,0,0,2,0,0,2,0
667, , ,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0
670, , ,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0
913, , ,2,0,0,0,2,2,0,0,0,0,2,0,0,0,0
"#;

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq(expected))
        .stderr(predicates::str::is_empty());

    Ok(())
}
