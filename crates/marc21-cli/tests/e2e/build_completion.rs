use assert_fs::TempDir;
use assert_fs::prelude::*;
use predicates::prelude::*;

use crate::prelude::*;

#[test]
fn build_completion_bash() -> TestResult {
    let temp_dir = TempDir::new().unwrap();
    let out = temp_dir.child("marc21.bash");

    let mut cmd = marc21_cmd();
    let assert = cmd
        .arg("build-completion")
        .arg("bash")
        .args(["-o", out.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    assert!(predicates::path::exists().eval(out.path()));

    temp_dir.close().unwrap();
    Ok(())
}

#[test]
fn build_completion_zsh() -> TestResult {
    let temp_dir = TempDir::new().unwrap();
    let out = temp_dir.child("marc21.zsh");

    let mut cmd = marc21_cmd();
    let assert = cmd
        .arg("build-completion")
        .arg("zsh")
        .args(["-o", out.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    assert!(predicates::path::exists().eval(out.path()));

    temp_dir.close().unwrap();
    Ok(())
}

#[test]
fn build_completion_elvish() -> TestResult {
    let temp_dir = TempDir::new().unwrap();
    let out = temp_dir.child("elvish.sh");

    let mut cmd = marc21_cmd();
    let assert = cmd
        .arg("build-completion")
        .arg("elvish")
        .args(["-o", out.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    assert!(predicates::path::exists().eval(out.path()));

    temp_dir.close().unwrap();
    Ok(())
}

#[test]
fn build_completion_fish() -> TestResult {
    let temp_dir = TempDir::new().unwrap();
    let out = temp_dir.child("completion.fish");

    let mut cmd = marc21_cmd();
    let assert = cmd
        .arg("build-completion")
        .arg("fish")
        .args(["-o", out.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    assert!(predicates::path::exists().eval(out.path()));

    temp_dir.close().unwrap();
    Ok(())
}

#[test]
fn build_completion_powershell() -> TestResult {
    let temp_dir = TempDir::new().unwrap();
    let out = temp_dir.child("completion.ps1");

    let mut cmd = marc21_cmd();
    let assert = cmd
        .arg("build-completion")
        .arg("powershell")
        .args(["-o", out.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    assert!(predicates::path::exists().eval(out.path()));

    temp_dir.close().unwrap();
    Ok(())
}
