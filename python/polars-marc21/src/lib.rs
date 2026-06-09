use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use marc21::io::{ByteRecordsIter, MarcReadOptions};
use marc21::{Query, StringRecord};
use pyo3::prelude::*;

#[pyclass]
struct Reader {
    iter: Mutex<Box<dyn Iterator<Item = Vec<String>> + Send>>,
    arity: usize,
}

#[pymethods]
impl Reader {
    #[new]
    fn py_new(path: PathBuf, query: String) -> PyResult<Self> {
        let mut reader = MarcReadOptions::default()
            .try_into_reader_from_path(path)
            .unwrap();

        let mut records: Vec<Vec<String>> = Vec::new();
        let query = Query::new(&query).unwrap();
        let options = Default::default();

        while let Some(result) = reader.next_byte_record() {
            let record =
                StringRecord::try_from(result.unwrap()).unwrap();
            let values: Vec<Vec<String>> = record
                .query(&query, &options)
                .iter()
                .map(|values| {
                    values
                        .iter()
                        .map(|value| value.to_str_lossy().to_string())
                        .collect()
                })
                .collect();
            records.extend(values);
        }

        Ok(Self {
            iter: Mutex::new(Box::new(records.into_iter())),
            arity: query.arity(),
        })
    }

    fn arity(slf: PyRef<'_, Self>) -> usize {
        slf.arity
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(slf: PyRefMut<'_, Self>) -> Option<Vec<String>> {
        slf.iter.lock().unwrap().next()
    }
}

#[pyfunction]
fn scan_marc21_impl(path: PathBuf, query: String) -> PyResult<Reader> {
    Reader::py_new(path, query)
}

#[pymodule]
mod _core {

    #[pymodule_export]
    use super::Reader;
    #[pymodule_export]
    use super::scan_marc21_impl;
}
