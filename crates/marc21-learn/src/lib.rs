use pyo3::prelude::*;

/// A Python module implemented in Rust. The name of this module must
/// match the `lib.name` setting in the `Cargo.toml`, else Python will
/// not be able to import the module.
#[pymodule]
mod _learn {
    use numpy::{IntoPyArray, PyArrayDyn, PyReadonlyArrayDyn};
    use pyo3::prelude::*;

    #[pyfunction]
    fn scale_array<'py>(
        py: Python<'py>,
        a: f64,
        x: PyReadonlyArrayDyn<'py, f64>,
    ) -> Bound<'py, PyArrayDyn<f64>> {
        let x = x.as_array();
        (a * &x).into_pyarray(py)
    }
}
