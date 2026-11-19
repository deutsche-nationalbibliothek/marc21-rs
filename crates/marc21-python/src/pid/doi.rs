use std::str::FromStr;

use isbn::{Isbn, Isbn13};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

/// Creates a DOI (ISBN-A) from an ISBN.
///
/// The syntax of a the DOI (ISBN-A) corresponding to an ISBN is
/// specified in `DOI® System and the ISBN System`_.
///
/// .. _DOI® System and the ISBN System: https://www.doi.org/the-identifier/resources/factsheets/doi-system-and-the-isbn-system
///
/// .. versionadded:: 0.3.0
///
/// .. warning::
///     This functionality is considered unstable. It may be changed at
///     any point without it being considered a breaking change.
///
/// Parameters
/// ----------
/// value : str
///     The ISBN to be converted.
///
/// strict : bool, default=False
///     If set, a ``ValueError`` is raised if an invalid ISBN is passed,
///     otherwise ``None`` is returned.
///
/// Returns
/// -------
/// str
///     The DOI (ISBN-A) of the given value.
///
/// Examples
/// --------
/// >>> from marc21_learn.pid import doi_from_isbn
/// >>>
/// >>> doi_from_isbn("978-3-10-031815-2")
/// '10.978.310/0318152'
#[pyfunction]
#[pyo3(signature = (value, *, strict = false))]
pub fn doi_from_isbn(
    value: &str,
    strict: bool,
) -> PyResult<Option<String>> {
    let isbn = match Isbn::from_str(value) {
        Ok(Isbn::_10(value)) => Isbn13::from(value),
        Ok(Isbn::_13(value)) => value,
        Err(_) if !strict => return Ok(None),
        Err(_) => {
            return Err(PyValueError::new_err(format!(
                "The value '{value}' is not a valid ISBN."
            )));
        }
    };

    let isbn_str = isbn.hyphenate().unwrap().to_string();
    let elements = isbn_str.split("-").collect::<Vec<_>>();
    debug_assert_eq!(elements.len(), 5);

    let doi = format!(
        "10.{}.{}{}/{}{}",
        elements[0], elements[1], elements[2], elements[3], elements[4],
    );

    Ok(Some(doi))
}
