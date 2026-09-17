use pyo3::intern;
use pyo3::prelude::*;

pub(crate) fn register_module(
    py: Python<'_>,
    parent: &Bound<'_, PyModule>,
) -> PyResult<()> {
    let name = "marc21_learn.preprocessing";

    let m = PyModule::new(parent.py(), "preprocessing")?;
    parent.add_submodule(&m)?;

    m.setattr("__name__", name)?;

    py.import(intern!(py, "sys"))?
        .getattr(intern!(py, "modules"))?
        .set_item(name, &m)?;

    Ok(())
}
