use pyo3::prelude::*;

#[pymodule]
mod _rust {
    #[pymodule_export]
    pub const __VERSION: &str = env!("CARGO_PKG_VERSION");
}
