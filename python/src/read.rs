use std::path::PathBuf;
use std::sync::Mutex;

use marc21::matcher::{MatchOptions, RecordMatcher};
use marc21::prelude::*;
use pyo3::prelude::*;

use crate::error::Error;

#[pyclass]
pub(crate) struct LazyReader {
    sources: Mutex<Box<dyn Iterator<Item = PathBuf> + Send>>,
    rows: Mutex<Box<dyn Iterator<Item = Vec<String>> + Send>>,
    query: Query,
    options: MatchOptions,
    matcher: Option<RecordMatcher>,
    arity: usize,
}

#[pymethods]
impl LazyReader {
    #[new]
    fn py_new(
        sources: Vec<PathBuf>,
        query: String,
        predicate: Option<String>,
    ) -> Result<Self, Error> {
        let query = Query::new(&query)?;
        let options = MatchOptions::default();

        let matcher = if let Some(matcher) = predicate {
            Some(RecordMatcher::new(matcher)?)
        } else {
            None
        };

        Ok(Self {
            sources: Mutex::new(Box::new(sources.into_iter())),
            rows: Mutex::new(Box::new(vec![].into_iter())),
            arity: query.arity(),
            options,
            matcher,
            query,
        })
    }

    fn arity(slf: PyRef<'_, Self>) -> usize {
        slf.arity
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<Vec<String>> {
        let iter = slf.rows.get_mut().unwrap();

        match iter.next() {
            Some(row) => Some(row),
            None => match slf.sources.get_mut().unwrap().next() {
                Some(path) => {
                    let mut rows: Vec<Vec<String>> = vec![];
                    let mut rdr = MarcReadOptions::default()
                        .try_into_reader_from_path(path)
                        .unwrap(); // FIXME

                    while let Some(result) = rdr.next_byte_record() {
                        let Ok(record) = result else {
                            continue;
                        };

                        if let Some(ref matcher) = slf.matcher
                            && !matcher.is_match(&record, &slf.options)
                        {
                            continue;
                        }

                        let record =
                            StringRecord::try_from(record).unwrap();

                        let values: Vec<Vec<String>> = record
                            .query(&slf.query, &slf.options)
                            .iter()
                            .map(|values| {
                                values
                                    .iter()
                                    .map(|value| {
                                        value
                                            .to_str_unchecked()
                                            .to_string()
                                    })
                                    .collect()
                            })
                            .collect();

                        rows.extend(values);
                    }

                    slf.rows = Mutex::new(Box::new(rows.into_iter()));
                    slf.rows.lock().unwrap().next()
                }
                None => None,
            },
        }
    }
}
