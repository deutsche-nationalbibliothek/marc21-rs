use std::str::FromStr;
use std::sync::OnceLock;

use isbn::{Isbn, Isbn10, Isbn13};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::pybacked::PyBackedStr;
use regex::Regex;

use crate::PyMatch;

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
/// value : str
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

/// Converts a ISBN into a unique, uniform format.
///
/// The provided ISBN is first validated and, if necessary, converted to
/// an ISBN-13. The individual components are separated by hyphens.
///
/// .. versionadded:: 0.3.0
///
/// Parameters
/// ----------
/// value : str
///     The value to be checked.
///
/// hyphenate : bool, default=True
///     Whether hyphens should be used to separate the components or
///     not.
///
/// strict : bool, default=False
///     If set, a ``ValueError`` is raised if an invalid ISBN is passed,
///     otherwise ``None`` is returned.
///
/// Returns
/// -------
/// str
///     The canonical form of the given ISBN.
///
/// Raises
/// ------
/// ValueError
///     If the given ISBN is invalid.
///
/// Examples
/// --------
/// >>> from marc21_learn.pid import isbn_canonicalize
/// >>>
/// >>> isbn_canonicalize("9783100318152")
/// '978-3-10-031815-2'
/// >>> isbn_canonicalize("3100318153")
/// '978-3-10-031815-2'
/// >>> isbn_canonicalize("9783100318152", hyphenate=False)
/// '9783100318152'
#[pyfunction]
#[pyo3(signature = (value, *, hyphenate = true, strict = false))]
pub fn isbn_canonicalize(
    value: &str,
    hyphenate: bool,
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

    let result = if hyphenate {
        isbn.hyphenate().unwrap().to_string()
    } else {
        isbn.to_string()
    };

    Ok(Some(result))
}

fn isbn_re() -> &'static Regex {
    static ISBN_RE: OnceLock<Regex> = OnceLock::new();

    ISBN_RE.get_or_init(|| {
        Regex::new(
            r"(?:(?:ISBN(?:-1[03])?(?::\s)?\s*)?((?:97[89][-\ ]?)?\d{1,5}[-\ ]?(?:\d+[-\ ]?){2}(?:\d|X)))",
        )
        .unwrap()
    })
}

/// Find the first ISBN in the given haystack.
///
/// This function searches for the first ISBN in the given haystack. By
/// default, all ISBNs are validated (check digit verification); if
/// invalid ISBNs are to be included as well, the ``validate`` flag must
/// be set to ``False``.
///
/// The underlying regular expression is based on the approach described
/// in the book *Regular Expressions Cookbook* by Jan Goyvaerts and
/// Steven Levithan [1]_. It finds ISBNs in both the ISBN-10 and ISBN-13
/// formats. The individual components of an ISBN can be separated
/// either by a space (``SPACE U+0020``) or a hyphen (``HYPHEN-MINUS
/// U+002D``).
///
/// .. [1] Jan Goyvaerts and Steven Levithan. O'Reilly, Beijing u.a.,
///      2nd ed., ISBN 978-1-4493-1943-4, 2012, p. 292.
///
/// .. versionadded:: 0.3.0
///
/// .. warning::
///     This functionality is considered unstable. It may be changed at
///     any point without it being considered a breaking change.
///
/// Parameters
/// ----------
/// haystack : str
///     The string to be searched.
///
/// validate : bool, default=True
///     If set, the value found is verified to ensure that it is a
///     formally valid ISBN.
///
/// Returns
/// -------
/// Match | None
///     The first match, if any.
///
/// See Also
/// --------
/// isbn_findall : Find all non-overlapping ISBNs in the given haystack.
///
/// Examples
/// --------
/// >>> from marc21_learn.pid import isbn_find
/// >>>
/// >>> isbn_find("ISBN 978-3-10-031815-2")
/// Match(value: '978-3-10-031815-2', start: 5, end: 22)
/// >>> isbn_find("ISBN-13: 9783100318152")
/// Match(value: '9783100318152', start: 9, end: 22)
///
/// The result can be converted to a string by using the ``__str__``
/// interface:
///
/// >>> if (isbn := isbn_find("978-0-571-08989-5")):
/// ...     print(isbn)
/// ...
/// 978-0-571-08989-5
#[pyfunction]
#[pyo3(signature = (haystack, *, validate = true))]
pub fn isbn_find(
    haystack: String,
    validate: bool,
) -> PyResult<Option<PyMatch>> {
    if let Some(caps) = isbn_re().captures(&haystack) {
        debug_assert_eq!(caps.len(), 2);

        let group1 = caps.get(1).unwrap();
        let r#match = PyMatch {
            value: group1.as_str().into(),
            start: group1.start(),
            end: group1.end(),
        };

        if validate {
            if let Ok(_isbn) = Isbn::from_str(&r#match.value) {
                return Ok(Some(r#match));
            }
        } else {
            return Ok(Some(r#match));
        }
    }

    Ok(None)
}

/// Find all non-overlapping ISBNs in the given haystack.
///
/// This function searches for all non-averlapping ISBNs in the given
/// haystack. By default, all ISBNs are validated (check digit
/// verification); if invalid ISBNs are to be included as well, the
/// ``validate`` flag must be set to ``False``.
///
/// The underlying regular expression is based on the approach described
/// in the book *Regular Expressions Cookbook* by Jan Goyvaerts and
/// Steven Levithan [1]_. It finds ISBNs in both the ISBN-10 and ISBN-13
/// formats. The individual components of an ISBN can be separated
/// either by a space (``SPACE U+0020``) or a hyphen (``HYPHEN-MINUS
/// U+002D``).
///
/// .. [1] Jan Goyvaerts and Steven Levithan. O'Reilly, Beijing u.a.,
///      2nd ed., ISBN 978-1-4493-1943-4, 2012, p. 292.
///
/// .. versionadded:: 0.3.0
///
/// .. warning::
///     This functionality is considered unstable. It may be changed at
///     any point without it being considered a breaking change.
///
/// Parameters
/// ----------
/// haystack : str
///     The string to be searched.
///
/// validate : bool, default=True
///     If set, the value found is verified to ensure that it is a
///     formally valid ISBN.
///
/// Returns
/// -------
/// list[Match]
///     The list of all matches.
///
/// See Also
/// --------
/// isbn_find : Find the first ISBN in the given haystack.
///
/// Examples
/// --------
/// >>> from marc21_learn.pid import isbn_findall
/// >>>
/// >>> haystack = "ISBN-13: 978-1-098-17130-8 ISBN-10: 1098171306"
/// >>> result = isbn_findall(haystack)
/// >>> assert len(result) == 2
/// >>> result[0]
/// Match(value: '978-1-098-17130-8', start: 9, end: 26)
/// >>> result[1]
/// Match(value: '1098171306', start: 36, end: 46)
#[pyfunction]
#[pyo3(signature = (haystack, *, validate = true))]
pub fn isbn_findall(
    haystack: String,
    validate: bool,
) -> PyResult<Vec<PyMatch>> {
    let mut result = vec![];

    for caps in isbn_re().captures_iter(&haystack) {
        debug_assert_eq!(caps.len(), 2);

        let group1 = caps.get(1).unwrap();
        let r#match = PyMatch {
            value: group1.as_str().into(),
            start: group1.start(),
            end: group1.end(),
        };

        if validate {
            if let Ok(_isbn) = Isbn::from_str(&r#match.value) {
                result.push(r#match);
            }
        } else {
            result.push(r#match);
        }
    }

    Ok(result)
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
