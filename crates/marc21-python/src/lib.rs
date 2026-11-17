use pyo3::prelude::*;

pub mod error;
pub mod io;
pub mod pid;

#[derive(Debug)]
#[pyclass(name = "Match")]
pub struct PyMatch {
    #[pyo3(get)]
    value: String,
    #[pyo3(get)]
    start: usize,
    #[pyo3(get)]
    end: usize,
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

impl From<regex::Match<'_>> for PyMatch {
    fn from(m: regex::Match) -> Self {
        Self {
            value: m.as_str().to_string(),
            start: m.start(),
            end: m.end(),
        }
    }
}
