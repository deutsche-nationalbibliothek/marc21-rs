use pyo3::prelude::*;

mod preprocessing;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[pyfunction]
fn __version() -> &'static str {
    VERSION
}

#[pymodule]
fn _rust(py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(__version))?;

    preprocessing::register_module(py, m)?;

    Ok(())
}
