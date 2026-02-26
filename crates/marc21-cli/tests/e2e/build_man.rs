use assert_fs::TempDir;
use assert_fs::prelude::*;

use crate::prelude::*;

#[test]
fn build_man_pages() -> TestResult {
    let temp_dir = TempDir::new().unwrap();
    let mut cmd = marc21_cmd();
    let assert = cmd
        .arg("build-man")
        .args(["-o", temp_dir.to_str().unwrap()])
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::is_empty());

    assert!(temp_dir.child("marc21.1").exists());
    assert!(temp_dir.child("marc21-concat.1").exists());
    assert!(temp_dir.child("marc21-count.1").exists());
    assert!(temp_dir.child("marc21-filter.1").exists());
    assert!(temp_dir.child("marc21-hash.1").exists());
    assert!(temp_dir.child("marc21-invalid.1").exists());
    assert!(temp_dir.child("marc21-print.1").exists());
    assert!(temp_dir.child("marc21-sample.1").exists());
    assert!(temp_dir.child("marc21-split.1").exists());

    temp_dir.close()?;
    Ok(())
}
