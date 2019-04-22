use pyo3::prelude::*;
use pyo3::typeob::PyTypeObject;

pub fn is_instance<T: PyTypeObject>(py: Python, datum: &PyObject) -> bool {
    py.is_instance::<T, PyObject>(datum).unwrap_or(false)
}
