use pyo3::prelude::*;

mod error;
mod read;

#[pymodule]
mod _core {
    #[pymodule_export]
    use super::read::LazyReader;
}
