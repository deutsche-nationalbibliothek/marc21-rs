use std::str::FromStr;

use isbn::{Isbn, Isbn10, Isbn13};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::pybacked::PyBackedStr;

#[derive(Debug, PartialEq)]
pub enum Version {
    Isbn13,
    Isbn10,
}

impl<'a, 'py> FromPyObject<'a, 'py> for Version {
    type Error = PyErr;

    fn extract(
        o: Borrowed<'a, 'py, PyAny>,
    ) -> Result<Self, Self::Error> {
        let version = match &*o.extract::<PyBackedStr>()? {
            "ISBN-13" => Self::Isbn13,
            "ISBN-10" => Self::Isbn10,
            v => {
                return Err(PyValueError::new_err(format!(
                    "`version` must be one of {{'ISBN-13', 'ISBN-10'}}, got {v}"
                )));
            }
        };

        Ok(version)
    }
}

/// Checks whether the value is a valid ISBN.
///
/// This function checks whether the specified value is a formally valid
/// ISBN. It does not check whether the ISBN has actually been assigned
/// or is associated with a resource.
///
/// .. versionadded:: 0.3.0
///
/// Parameters
/// ----------
/// value : String
///     The value to be checked.
///
/// version : {'ISBN-13', 'ISBN-10'}
///     Restriction to a specific ISBN version to be checked. If no
///     version is selected, both ISBNs in the ISBN-13 format and
///     ISBN-10 format are accepted.
///
/// Returns
/// -------
/// bool
///     Returns ``True`` if the given value is valid, otherwise
///     ``False``.
///
/// Raises
/// ------
/// ValueError
///     If the version is invalid.
///
/// Examples
/// --------
/// >>> from marc21_learn.pid import isbn_validate
/// >>>
/// >>> isbn_validate("978-3-10-031815-2")
/// True
/// >>> isbn_validate("3-10-031815-3")
/// True
///
/// The following ISBN is invalid because the check digit is incorrect.
///
/// >>> isbn_validate("9783100318153")
/// False
#[pyfunction]
#[pyo3(signature = (value, *, version = None))]
#[pyo3(text_signature = "(value, *, version = None)")]
pub fn isbn_validate(value: &str, version: Option<Version>) -> bool {
    match version {
        Some(Version::Isbn13) => Isbn13::from_str(value).is_ok(),
        Some(Version::Isbn10) => Isbn10::from_str(value).is_ok(),
        None => Isbn::from_str(value).is_ok(),
    }
}

#[cfg(test)]
mod tests {
    use Version::*;

    use super::*;

    #[test]
    fn test_isbn_validate() {
        assert!(isbn_validate("978-0-571-08989-5", None));
        assert!(isbn_validate("978-0-571-08989-5", Some(Isbn13)));
        assert!(!isbn_validate("978-0-571-08989-5", Some(Isbn10)));

        assert!(isbn_validate("978 0 571 08989 5", None));
        assert!(isbn_validate("978 0 571 08989 5", Some(Isbn13)));
        assert!(!isbn_validate("978 0 571 08989 5", Some(Isbn10)));

        assert!(isbn_validate("9780571089895", None));
        assert!(isbn_validate("9780571089895", Some(Isbn13)));
        assert!(!isbn_validate("9780571089895", Some(Isbn10)));

        assert!(!isbn_validate("9780571089896", None));
        assert!(!isbn_validate("9780571089896", Some(Isbn13)));
        assert!(!isbn_validate("9780571089896", Some(Isbn10)));

        assert!(isbn_validate("3-10-031820-X", None));
        assert!(!isbn_validate("3-10-031820-X", Some(Isbn13)));
        assert!(isbn_validate("3-10-031820-X", Some(Isbn10)));

        assert!(isbn_validate("3 10 031820 X", None));
        assert!(!isbn_validate("3 10 031820 X", Some(Isbn13)));
        assert!(isbn_validate("3 10 031820 X", Some(Isbn10)));

        assert!(isbn_validate("310031820X", None));
        assert!(!isbn_validate("310031820X", Some(Isbn13)));
        assert!(isbn_validate("310031820X", Some(Isbn10)));

        assert!(!isbn_validate("3100318209", None));
        assert!(!isbn_validate("3100318209", Some(Isbn13)));
        assert!(!isbn_validate("3100318209", Some(Isbn10)));
    }
}
