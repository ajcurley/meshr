pub mod geometry;
pub mod mesh;
pub mod spatial;

// Python bindings
use pyo3::prelude::*;

mod bindings;

#[pymodule]
fn meshr(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<bindings::python::geometry::Vector3>()?;

    Ok(())
}
