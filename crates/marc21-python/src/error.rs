use std::error::Error;
use std::fmt::Display;
use std::sync::PoisonError;

use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;

pub(crate) fn value_err<E>(e: E) -> PyErr
where
    E: Display + Error,
{
    PyValueError::new_err(e.to_string())
}

pub(crate) fn poison_err<T>(_: PoisonError<T>) -> PyErr {
    PyRuntimeError::new_err(
        "a previous access panicked while holding this lock",
    )
}
