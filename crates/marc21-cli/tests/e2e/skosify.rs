use std::fs::{File, read_to_string};
use std::io::Read;

use flate2::read::GzDecoder;
use sophia::api::source::TripleSource;
use sophia::inmem::graph::LightGraph;
use sophia::turtle::parser::turtle;

use crate::prelude::*;

const CONFIG: &str = r#"
scope = 'ldr.type == "z" && 042.a == "gnd1" && 079.q == "s"'
uri = { path = '024/7#{ 0 | 2 == "gnd" }' }

[group.concepts]
scope = '075{ b == "p" && 2 == "gndgen" }'
labels = [
    { kind = 'preferred', path = '100/1#.a' },
    { kind = 'alternative', path = '400/1#.a' },
]
"#;

fn isomorphic_graphs(g1: &str, g2: &str) -> bool {
    let g1: LightGraph =
        turtle::parse_str(g1).collect_triples().unwrap();
    let g2: LightGraph =
        turtle::parse_str(g2).collect_triples().unwrap();

    sophia::isomorphism::isomorphic_graphs(&g1, &g2).unwrap()
}

#[test]
fn skosify_stdout() -> TestResult {
    let temp_dir = TempDir::new()?;
    let config = temp_dir.child("config.toml");
    config.write_str(CONFIG)?;

    let mut cmd = marc21_cmd();
    let assert = cmd
        .arg("skosify")
        .args(["--config", config.to_str().unwrap()])
        .arg(data_dir().join("ada.mrc.gz"))
        .assert();

    let stdout = assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty().not())
        .stderr(predicates::str::is_empty())
        .get_output()
        .stdout
        .clone();

    let expected = read_to_string(data_dir().join("ada.ttl"))?;
    let actual = String::from_utf8(stdout)?;

    assert!(isomorphic_graphs(&expected, &actual));

    temp_dir.close()?;
    Ok(())
}

#[test]
fn skosify_output() -> TestResult {
    let temp_dir = TempDir::new()?;
    let output = temp_dir.child("ada.ttl");
    let config = temp_dir.child("config.toml");
    config.write_str(CONFIG)?;

    let mut cmd = marc21_cmd();
    let assert = cmd
        .arg("skosify")
        .args(["-c", config.to_str().unwrap()])
        .arg(data_dir().join("ada.mrc.gz"))
        .args(["-o", output.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let expected = read_to_string(data_dir().join("ada.ttl"))?;
    let actual = read_to_string(output)?;
    assert!(isomorphic_graphs(&expected, &actual));

    temp_dir.close()?;
    Ok(())
}

#[test]
fn skosify_gzip() -> TestResult {
    let temp_dir = TempDir::new()?;
    let output = temp_dir.child("ada.ttl.gz");

    let config = temp_dir.child("config.toml");
    config.write_str(CONFIG)?;

    let mut cmd = marc21_cmd();
    let assert = cmd
        .arg("skosify")
        .args(["-c", config.to_str().unwrap()])
        .arg(data_dir().join("ada.mrc.gz"))
        .args(["-o", output.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let mut gz = GzDecoder::new(File::open(output.path())?);
    let mut data = Vec::new();
    gz.read_to_end(&mut data)?;

    let expected = read_to_string(data_dir().join("ada.ttl"))?;
    let actual = String::from_utf8(data)?;

    assert!(isomorphic_graphs(&expected, &actual));

    temp_dir.close()?;
    Ok(())
}

#[test]
fn skosify_pretty() -> TestResult {
    let temp_dir = TempDir::new()?;
    let output = temp_dir.child("ada.ttl");

    let config = temp_dir.child("config.toml");
    config.write_str("pretty = true\n")?;
    config.write_str(CONFIG)?;

    let mut cmd = marc21_cmd();
    let assert = cmd
        .arg("skosify")
        .args(["-c", config.to_str().unwrap()])
        .arg(data_dir().join("ada.mrc.gz"))
        .args(["-o", output.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let expected = read_to_string(data_dir().join("ada.ttl"))?;
    let actual = read_to_string(output)?;

    assert!(isomorphic_graphs(&expected, &actual));

    temp_dir.close()?;
    Ok(())
}

#[test]
fn skosify_skip_invalid() -> TestResult {
    let temp_dir = TempDir::new()?;
    let config = temp_dir.child("config.toml");
    config.write_str(CONFIG)?;

    let mut cmd = marc21_cmd();
    let assert = cmd
        .arg("skosify")
        .args(["--config", config.to_str().unwrap()])
        .arg(data_dir().join("DUMP.mrc.gz"))
        .assert();

    assert
        .failure()
        .code(1)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::starts_with(
            "error: could not parse record (line 8",
        ));

    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["skosify", "-s"])
        .args(["--config", config.to_str().unwrap()])
        .arg(data_dir().join("DUMP.mrc.gz"))
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty().not())
        .stderr(predicates::str::is_empty());

    temp_dir.close()?;
    Ok(())
}

#[test]
fn skosify_where() -> TestResult {
    let temp_dir = TempDir::new()?;
    let output = temp_dir.child("ada.ttl");
    let config = temp_dir.child("config.toml");
    config.write_str(CONFIG)?;

    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["skosify", "-s"])
        .args(["-c", config.to_str().unwrap()])
        .arg(data_dir().join("DUMP.mrc.gz"))
        .arg(data_dir().join("ada.mrc.gz"))
        .args(["--where", "001 == '119232022'"])
        .args(["-o", output.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    let expected = read_to_string(data_dir().join("ada.ttl"))?;
    let actual = read_to_string(output)?;
    assert!(isomorphic_graphs(&expected, &actual));

    temp_dir.close()?;
    Ok(())
}
