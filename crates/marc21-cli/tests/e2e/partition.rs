use std::collections::HashMap;

use assert_fs::TempDir;

use crate::prelude::*;

#[test]
fn partition_query_arity0() -> TestResult {
    let outdir = TempDir::new()?;
    let mut cmd = marc21_cmd();

    let assert = cmd
        .args(["partition", "-s"])
        .arg("065{ _ | 2 == 'sswd' }")
        .arg(data_dir().join("DUMP.mrc.gz"))
        .args(["-o", outdir.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    assert!(outdir.read_dir()?.next().is_none());
    Ok(())
}

#[test]
fn partition_by_065a() -> TestResult {
    let outdir = TempDir::new()?;
    let mut cmd = marc21_cmd();

    let assert = cmd
        .args(["partition", "-s"])
        .arg("065{ a | 2 == 'sswd' }")
        .arg(data_dir().join("DUMP.mrc.gz"))
        .args(["-o", outdir.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    assert_eq!(outdir.read_dir()?.count(), 8);

    let mut counts = HashMap::new();
    counts.insert("12.2p.mrc", "7\n".to_string());
    counts.insert("16.5p.mrc", "1\n".to_string());
    counts.insert("15.1p.mrc", "1\n".to_string());
    counts.insert("13.4p.mrc", "1\n".to_string());
    counts.insert("7.14p.mrc", "1\n".to_string());
    counts.insert("18p.mrc", "1\n".to_string());
    counts.insert("4.7p.mrc", "1\n".to_string());
    counts.insert("16.1p.mrc", "1\n".to_string());

    for (query, count) in counts.iter() {
        let mut cmd = marc21_cmd();
        let assert = cmd.arg("count").arg(outdir.join(query)).assert();
        assert
            .success()
            .code(0)
            .stdout(predicates::ord::eq(count.as_str()))
            .stderr(predicates::str::is_empty());
    }

    Ok(())
}

#[test]
fn partition_by_065a_gzip() -> TestResult {
    let outdir = TempDir::new()?;
    let mut cmd = marc21_cmd();

    let assert = cmd
        .args(["partition", "-s"])
        .args(["--template", "{}.mrc.gz"])
        .arg("065{ a | 2 == 'sswd' }")
        .arg(data_dir().join("DUMP.mrc.gz"))
        .args(["-o", outdir.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    assert_eq!(outdir.read_dir()?.count(), 8);

    let mut counts = HashMap::new();
    counts.insert("12.2p.mrc.gz", "7\n".to_string());
    counts.insert("16.5p.mrc.gz", "1\n".to_string());
    counts.insert("15.1p.mrc.gz", "1\n".to_string());
    counts.insert("13.4p.mrc.gz", "1\n".to_string());
    counts.insert("7.14p.mrc.gz", "1\n".to_string());
    counts.insert("18p.mrc.gz", "1\n".to_string());
    counts.insert("4.7p.mrc.gz", "1\n".to_string());
    counts.insert("16.1p.mrc.gz", "1\n".to_string());

    for (query, count) in counts.iter() {
        let mut cmd = marc21_cmd();
        let assert = cmd.arg("count").arg(outdir.join(query)).assert();
        assert
            .success()
            .code(0)
            .stdout(predicates::ord::eq(count.as_str()))
            .stderr(predicates::str::is_empty());
    }

    Ok(())
}

#[test]
fn partition_by_065a_template() -> TestResult {
    let outdir = TempDir::new()?;
    let mut cmd = marc21_cmd();

    let assert = cmd
        .args(["partition", "-s"])
        .args(["--template", "065a-{}.mrc"])
        .arg("065{ a | 2 == 'sswd' }")
        .arg(data_dir().join("DUMP.mrc.gz"))
        .args(["-o", outdir.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    assert_eq!(outdir.read_dir()?.count(), 8);

    let mut counts = HashMap::new();
    counts.insert("065a-12.2p.mrc", "7\n".to_string());
    counts.insert("065a-16.5p.mrc", "1\n".to_string());
    counts.insert("065a-15.1p.mrc", "1\n".to_string());
    counts.insert("065a-13.4p.mrc", "1\n".to_string());
    counts.insert("065a-7.14p.mrc", "1\n".to_string());
    counts.insert("065a-18p.mrc", "1\n".to_string());
    counts.insert("065a-4.7p.mrc", "1\n".to_string());
    counts.insert("065a-16.1p.mrc", "1\n".to_string());

    for (query, count) in counts.iter() {
        let mut cmd = marc21_cmd();
        let assert = cmd.arg("count").arg(outdir.join(query)).assert();
        assert
            .success()
            .code(0)
            .stdout(predicates::ord::eq(count.as_str()))
            .stderr(predicates::str::is_empty());
    }

    Ok(())
}

#[test]
fn partition_skip_invalid() -> TestResult {
    let outdir = TempDir::new()?;

    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["partition"])
        .arg("065{ a | 2 == 'sswd' }")
        .arg(data_dir().join("invalid.mrc"))
        .args(["-o", outdir.to_str().unwrap()])
        .assert();

    assert
        .failure()
        .code(1)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::starts_with(
            "error: could not parse record (line 1",
        ));

    assert!(outdir.read_dir()?.next().is_none());

    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["partition", "-s"])
        .arg("065{ a | 2 == 'sswd' }")
        .arg(data_dir().join("invalid.mrc"))
        .args(["-o", outdir.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    assert!(outdir.read_dir()?.next().is_none());

    Ok(())
}

#[test]
fn partition_by_065a_where() -> TestResult {
    let outdir = TempDir::new()?;
    let mut cmd = marc21_cmd();

    let assert = cmd
        .args(["partition", "-s"])
        .arg("065{ a | 2 == 'sswd' }")
        .arg(data_dir().join("DUMP.mrc.gz"))
        .args(["--where", "001 == '118572121'"])
        .args(["-o", outdir.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    assert_eq!(outdir.read_dir()?.count(), 2);

    let mut counts = HashMap::new();
    counts.insert("4.7p.mrc", "1\n".to_string());
    counts.insert("12.2p.mrc", "1\n".to_string());

    for (query, count) in counts.iter() {
        let mut cmd = marc21_cmd();
        let assert = cmd.arg("count").arg(outdir.join(query)).assert();
        assert
            .success()
            .code(0)
            .stdout(predicates::ord::eq(count.as_str()))
            .stderr(predicates::str::is_empty());
    }

    Ok(())
}

#[test]
fn partition_limit() -> TestResult {
    let outdir = TempDir::new()?;
    let mut cmd = marc21_cmd();

    let assert = cmd
        .args(["partition", "--limit", "1"])
        .arg("065{ a | 2 == 'sswd' }")
        .arg(data_dir().join("DUMP.mrc.gz"))
        .args(["-o", outdir.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    assert_eq!(outdir.read_dir()?.count(), 6);

    let mut counts = HashMap::new();
    counts.insert("12.2p.mrc", "1\n".to_string());
    counts.insert("16.5p.mrc", "1\n".to_string());
    counts.insert("15.1p.mrc", "1\n".to_string());
    counts.insert("13.4p.mrc", "1\n".to_string());
    counts.insert("7.14p.mrc", "1\n".to_string());
    counts.insert("18p.mrc", "1\n".to_string());

    for (query, count) in counts.iter() {
        let mut cmd = marc21_cmd();
        let assert = cmd.arg("count").arg(outdir.join(query)).assert();
        assert
            .success()
            .code(0)
            .stdout(predicates::ord::eq(count.as_str()))
            .stderr(predicates::str::is_empty());
    }

    Ok(())
}
