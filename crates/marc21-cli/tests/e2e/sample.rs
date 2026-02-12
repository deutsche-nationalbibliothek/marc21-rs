/// TODO: Add further tests once a meaningful dump is available.
use std::fs;

use crate::prelude::*;

#[test]
fn sample_write_output_stdout() -> TestResult {
    let mut cmd = marc21_cmd();
    let assert = cmd
        .args(["sample", "-s", "1"])
        .arg(data_dir().join("ada.mrc.gz"))
        .arg(data_dir().join("invalid.mrc"))
        .arg(data_dir().join("ada.mrc"))
        .assert();

    assert
        .success()
        .code(0)
        .stdout(predicates::ord::eq(fs::read(
            data_dir().join("ada.mrc"),
        )?))
        .stderr(predicates::str::is_empty());

    Ok(())
}
