extern crate meshr;

use pyo3::prelude::*;

mod geometry;

#[pymodule(name="meshr")]
fn pymeshr(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<geometry::PyVector3>()?;

    Ok(())
}
