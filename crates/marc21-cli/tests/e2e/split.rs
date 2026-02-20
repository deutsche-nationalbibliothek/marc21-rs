use crate::prelude::*;

#[test]
fn split_simple() -> TestResult {
    let temp_dir = TempDir::new()?;
    let outdir = temp_dir.child("out");

    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["split", "-s", "3"])
        .arg(data_dir().join("DUMP.mrc.gz"))
        .args(["--outdir", outdir.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    // CHUNK 0
    let mut cmd = marc21_cmd();
    let assert = cmd
        .arg("count")
        .arg(outdir.join("0.mrc").to_str().unwrap())
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq("3\n"))
        .stderr(predicates::str::is_empty());

    // CHUNK 1
    let mut cmd = marc21_cmd();
    let assert = cmd
        .arg("count")
        .arg(outdir.join("1.mrc").to_str().unwrap())
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq("3\n"))
        .stderr(predicates::str::is_empty());

    // CHUNK 2
    let mut cmd = marc21_cmd();
    let assert = cmd
        .arg("count")
        .arg(outdir.join("2.mrc").to_str().unwrap())
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq("1\n"))
        .stderr(predicates::str::is_empty());

    Ok(())
}

#[test]
fn split_skip_invalid() -> TestResult {
    let temp_dir = TempDir::new()?;
    let outdir = temp_dir.child("out");

    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["split", "3"])
        .arg(data_dir().join("invalid.mrc"))
        .args(["-o", outdir.to_str().unwrap()])
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
        .args(["split", "-s", "3"])
        .arg(data_dir().join("invalid.mrc"))
        .args(["-o", outdir.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());
    Ok(())
}

#[test]
fn split_where() -> TestResult {
    let temp_dir = TempDir::new()?;
    let outdir = temp_dir.child("out");

    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["split", "-s", "2"])
        .arg(data_dir().join("DUMP.mrc.gz"))
        .args(["--outdir", outdir.to_str().unwrap()])
        .args(["--where", "075{ b == 'p' && 2 == 'gndgen' }"])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    // CHUNK 0
    let mut cmd = marc21_cmd();
    let assert = cmd
        .arg("count")
        .arg(outdir.join("0.mrc").to_str().unwrap())
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq("2\n"))
        .stderr(predicates::str::is_empty());

    // CHUNK 1
    let mut cmd = marc21_cmd();
    let assert = cmd
        .arg("count")
        .arg(outdir.join("1.mrc").to_str().unwrap())
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq("2\n"))
        .stderr(predicates::str::is_empty());

    Ok(())
}

#[test]
fn split_filename() -> TestResult {
    let temp_dir = TempDir::new()?;
    let outdir = temp_dir.child("out");

    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["split", "-s", "3"])
        .arg(data_dir().join("DUMP.mrc.gz"))
        .args(["--outdir", outdir.to_str().unwrap()])
        .args(["--filename", "SPLIT-{}.mrc.gz"])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    // CHUNK 0
    let mut cmd = marc21_cmd();
    let assert = cmd
        .arg("count")
        .arg(outdir.join("SPLIT-0.mrc.gz").to_str().unwrap())
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq("3\n"))
        .stderr(predicates::str::is_empty());

    // CHUNK 1
    let mut cmd = marc21_cmd();
    let assert = cmd
        .arg("count")
        .arg(outdir.join("SPLIT-1.mrc.gz").to_str().unwrap())
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq("3\n"))
        .stderr(predicates::str::is_empty());

    // CHUNK 2
    let mut cmd = marc21_cmd();
    let assert = cmd
        .arg("count")
        .arg(outdir.join("SPLIT-2.mrc.gz").to_str().unwrap())
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq("1\n"))
        .stderr(predicates::str::is_empty());

    Ok(())
}
