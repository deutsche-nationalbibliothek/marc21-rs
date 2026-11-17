use pyo3::prelude::*;

pub mod error;
pub mod io;
pub mod pid;

#[derive(Debug)]
#[pyclass(name = "Match")]
pub struct PyMatch {
    #[pyo3(get)]
    pub(crate) value: String,
    #[pyo3(get)]
    pub(crate) start: usize,
    #[pyo3(get)]
    pub(crate) end: usize,
}

#[pymethods]
impl PyMatch {
    fn __repr__(&self) -> String {
        format!(
            "Match(value: '{}', start: {}, end: {})",
            self.value, self.start, self.end
        )
    }

    fn __str__(&self) -> String {
        self.value.clone()
    }
}
