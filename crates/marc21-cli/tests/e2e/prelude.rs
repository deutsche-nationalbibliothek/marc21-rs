use std::env::current_dir;
use std::path::PathBuf;
use std::sync::LazyLock;

use assert_cmd::Command;
pub(crate) use assert_fs::TempDir;
pub(crate) use assert_fs::prelude::*;

pub(crate) type TestResult = anyhow::Result<()>;

#[inline(always)]
pub(crate) fn marc21_cmd() -> Command {
    Command::new(assert_cmd::cargo::cargo_bin!("marc21"))
}

pub(crate) fn data_dir() -> &'static PathBuf {
    static DATA_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
        current_dir()
            .unwrap()
            .join("../../tests/data")
            .canonicalize()
            .unwrap()
            .to_path_buf()
    });

    &DATA_DIR
}
