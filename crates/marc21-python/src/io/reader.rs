use std::path::PathBuf;
use std::sync::Mutex;

use marc21::io::{ByteRecordsIter, MarcReadOptions, ReadMarcError};
use marc21::matcher::RecordMatcher;
use marc21::{Query, QueryOptions, StringRecord};
use pyo3::exceptions::PyUnicodeError;
use pyo3::prelude::*;

use crate::error::{poison_err, value_err};

#[pyclass(name = "MarcReader")]
pub struct PyMarcReader {
    rows: Mutex<Box<dyn Iterator<Item = Vec<String>> + Send>>,
    sources: Mutex<Box<dyn Iterator<Item = PathBuf> + Send>>,
    query: Query,
    options: QueryOptions,
    skip_invalid: bool,
    predicate: Option<RecordMatcher>,
}

#[pymethods]
impl PyMarcReader {
    #[new]
    fn py_new(
        sources: Vec<PathBuf>,
        query: String,
        predicate: Option<String>,
        skip_invalid: Option<bool>,
    ) -> PyResult<Self> {
        let query = Query::new(&query).map_err(value_err)?;
        let predicate = predicate
            .map(RecordMatcher::new)
            .transpose()
            .map_err(value_err)?;

        Ok(Self {
            rows: Mutex::new(Box::new(vec![].into_iter())),
            sources: Mutex::new(Box::new(sources.into_iter())),
            options: Default::default(),
            skip_invalid: skip_invalid.unwrap_or_default(),
            query,
            predicate,
        })
    }

    fn width(slf: PyRef<'_, Self>) -> usize {
        slf.query.width()
    }

    fn names(slf: PyRef<'_, Self>) -> Vec<String> {
        slf.query.names()
    }

    fn dtypes(slf: PyRef<'_, Self>) -> Vec<String> {
        slf.query.dtypes().iter().map(ToString::to_string).collect()
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    pub fn __next__(
        mut slf: PyRefMut<'_, Self>,
    ) -> Option<PyResult<Vec<String>>> {
        // If there are still rows from the current file, those are
        // processed first before a new file is read in.
        let iter = slf.rows.get_mut().unwrap();
        if let Some(row) = iter.next() {
            return Some(Ok(row));
        }

        loop {
            let path = slf.sources.get_mut().unwrap().next()?;
            let mut rows: Vec<Vec<String>> = vec![];

            let mut rdr = match MarcReadOptions::default()
                .try_into_reader_from_path(&path)
            {
                // If an I/O error occurs while creating a reader, the
                // iterator is terminated early with a corresponding
                // error.
                Err(e) => return Some(Err(e.into())),
                Ok(rdr) => rdr,
            };

            while let Some(result) = rdr.next_byte_record() {
                let record = match result {
                    Err(ReadMarcError::IO(e)) => {
                        return Some(Err(e.into()));
                    }
                    Err(ReadMarcError::Parse(e))
                        if !slf.skip_invalid =>
                    {
                        return Some(Err(value_err(e)));
                    }
                    Err(_) => continue,
                    Ok(record) => record,
                };

                if let Some(ref matcher) = slf.predicate
                    && !matcher
                        .is_match(&record, slf.options.match_options())
                {
                    continue;
                }

                let record = match StringRecord::try_from(record) {
                    // If the record does not contain valid UTF-8
                    // values, a corresponding UnicodeDecodeError is
                    // returned.
                    Err(e) => {
                        return Some(Err(PyUnicodeError::new_err(
                            e.to_string(),
                        )));
                    }
                    Ok(record) => record,
                };

                let values: Vec<Vec<String>> = record
                    .query(&slf.query, &slf.options)
                    .iter()
                    .map(|row| {
                        row.iter()
                            .map(|value| {
                                value.to_str_unchecked().to_string()
                            })
                            .collect()
                    })
                    .collect();

                rows.extend(values);
            }

            slf.rows = Mutex::new(Box::new(rows.into_iter()));

            match slf.rows.lock() {
                Err(e) => return Some(Err(poison_err(e))),
                Ok(mut rows) => {
                    if let Some(row) = rows.next() {
                        return Some(Ok(row));
                    }
                }
            }
        }
    }
}
