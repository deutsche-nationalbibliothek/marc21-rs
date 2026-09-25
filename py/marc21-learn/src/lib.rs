use marc21_python::io::PyMarcReader;
use pyo3::intern;
use pyo3::prelude::*;

macro_rules! set_name {
    ($python:expr, $module:expr, $name:expr) => {
        $module.setattr("__name__", $name)?;
        $python
            .import(intern!($python, "sys"))?
            .getattr(intern!($python, "modules"))?
            .set_item($name, &$module)?;
    };
}

#[pyfunction]
const fn __version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[pymodule]
#[rustfmt::skip]
fn  _runtime(py: Python<'_>, parent: &Bound<'_, PyModule>) -> PyResult<()> {
    // Top-Level Module
    parent.add_wrapped(wrap_pyfunction!(__version))?;

    // Input/Output
    let m = PyModule::new(parent.py(), "io")?;
    m.add_class::<PyMarcReader>()?;
    parent.add_submodule(&m)?;

    set_name!(py, m, "marc21_learn._runtime.io");
    
    Ok(())
}
