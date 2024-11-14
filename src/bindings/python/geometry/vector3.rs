use pyo3::prelude::*;

use crate::geometry::Vector3;

#[pyclass(name="Vector3")]
#[derive(Debug, Copy, Clone)]
pub struct PyVector3(Vector3);

#[pymethods]
impl PyVector3 {
    #[new]
    pub fn new(x: f64, y: f64, z: f64) -> PyVector3 {
        PyVector3(Vector3::new(x, y, z))
    }
}
