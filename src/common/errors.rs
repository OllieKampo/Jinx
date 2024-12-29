use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

pub fn rust_result_to_py_result<T, E>(result: Result<T, E>) -> PyResult<T> {
    return match result {
        Ok(result) => Ok(result),
        Err(e) => Err(PyValueError::new_err("Error")),
    };
}
