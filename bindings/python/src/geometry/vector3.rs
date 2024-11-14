use meshr::geometry::Vector3;
use pyo3::prelude::*;
use pyo3::exceptions::PyIndexError;

#[pyclass(name="Vector3")]
pub struct PyVector3(Vector3);

#[pymethods]
impl PyVector3 {
    /// Construct a PyVector3 from its components
    #[new]
    pub fn new(x: f64, y: f64, z: f64) -> PyVector3 {
        Vector3::new(x, y, z).into()
    }

    /// Compute the dot product u * v
    pub fn dot(&self, v: &PyVector3) -> f64 {
        Vector3::dot(&self.0, &v.0)
    }

    /// Compute the cross product self x v
    pub fn cross(&self, v: &PyVector3) -> PyVector3 {
        Vector3::cross(&self.0, &v.0).into()
    }

    /// Get the unit PyVector3
    pub fn unit(&self) -> PyVector3 {
        self.0.unit().into()
    }

    /// Get the magnitude
    pub fn mag(&self) -> f64 {
        self.0.mag()
    }

    /// Get the string representation
    pub fn __str__(&self) -> PyResult<String> {
        Ok(format!("({} {} {})", self.0[0], self.0[1], self.0[2]))
    }

    /// Get the instance representation
    pub fn __repr__(&self) -> PyResult<String> {
        self.__str__()
    }

    /// Get the component by index
    pub fn __getitem__(&self, index: i32) -> PyResult<f64> {
        if index > 2 || index < -3 {
            return Err(PyErr::new::<PyIndexError, _>("Index out of range"));
        }

        if index < 0 {
            return Ok(self.0[(index + 3) as usize])
        }

        Ok(self.0[index as usize])
    }
}

impl From<Vector3> for PyVector3 {
    fn from(v: Vector3) -> Self {
        Self(v)
    }
}
