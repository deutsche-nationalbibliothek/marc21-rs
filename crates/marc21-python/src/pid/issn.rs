use issn::Issn;
use pyo3::prelude::*;

/// Checks whether the value is a valid ISSN.
///
/// This function checks whether the specified value is a formally valid
/// `International Standard Serial Number`_ (ISSN). It does not check
/// whether the ISSN has actually been assigned or is associated with a
/// resource.
///
/// .. versionadded:: 0.4.0
///
/// .. _International Standard Serial Number: https://en.wikipedia.org/wiki/ISSN
///
/// .. warning::
///     This functionality is considered unstable. It may be changed at
///     any point without it being considered a breaking change.
///
/// Parameters
/// ----------
/// value : str
///     The value to be checked.
///
/// Returns
/// -------
/// bool
///     Returns ``True`` if the given value is valid, otherwise
///     ``False``.
///
/// Examples
/// --------
/// >>> from marc21_learn.pid import issn_validate
/// >>>
/// >>> assert issn_validate("2191-4664") == True
/// >>> assert issn_validate("0720-6763") == True
///
/// The following ISSN is invalid because the checksum is incorrect.
///
/// >>> assert issn_validate("2191-466X") == False
#[pyfunction]
#[pyo3(signature = (value, *))]
pub fn issn_validate(value: &str) -> bool {
    Issn::new(value).is_ok()
}
