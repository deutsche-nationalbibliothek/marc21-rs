#[cfg(feature = "build")]
mod build_completion;
#[cfg(feature = "build")]
mod build_man;
mod concat;
mod count;
mod dedup;
mod describe;
mod filter;
mod frequency;
mod hash;
mod invalid;
mod partition;
mod print;
mod sample;
mod select;
mod split;

pub(crate) mod prelude {
    use std::env::current_dir;
    use std::path::PathBuf;
    use std::sync::LazyLock;

    use assert_cmd::Command;
    pub(crate) use assert_fs::TempDir;
    pub(crate) use assert_fs::prelude::*;
    pub(crate) use predicates::prelude::PredicateBooleanExt;

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
}
