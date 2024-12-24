use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

pub fn extract_vector_b(py: Python, len: usize, vector_b: PyObject) -> PyResult<Vec<f64>> {
    let vector_b = if let Ok(b) = vector_b.extract::<f64>(py) {
        vec![b; len]
    } else {
        vector_b.extract::<Vec<f64>>(py)?
    };
    if vector_b.len() != len {
        return Err(PyValueError::new_err("Vector lengths do not match"));
    }
    return Ok(vector_b);
}

pub fn vector_apply<F>(vector_a: &Vec<f64>, vector_b: &Vec<f64>, f: &mut F) -> PyResult<Vec<f64>>
where
    F: FnMut(f64, f64) -> f64,
{
    let result = vector_a
        .iter()
        .zip(vector_b.iter())
        .map(|(&a, &b)| f(a, b))
        .collect();
    return Ok(result);
}

#[pyfunction]
pub fn vector_add(py: Python, vector_a: Vec<f64>, vector_b: PyObject) -> PyResult<Vec<f64>> {
    let vector_b = extract_vector_b(py, vector_a.len(), vector_b)?;
    return vector_apply(&vector_a, &vector_b, &mut |a, b| a + b);
}

#[pyfunction]
pub fn vector_subtract(py: Python, vector_a: Vec<f64>, vector_b: PyObject) -> PyResult<Vec<f64>> {
    let vector_b = extract_vector_b(py, vector_a.len(), vector_b)?;
    return vector_apply(&vector_a, &vector_b, &mut |a, b| a - b);
}

#[pyfunction]
pub fn vector_multiply(py: Python, vector_a: Vec<f64>, vector_b: PyObject) -> PyResult<Vec<f64>> {
    let vector_b = extract_vector_b(py, vector_a.len(), vector_b)?;
    return vector_apply(&vector_a, &vector_b, &mut |a, b| a * b);
}

#[pyfunction]
pub fn vector_divide(py: Python, vector_a: Vec<f64>, vector_b: PyObject) -> PyResult<Vec<f64>> {
    let vector_b = extract_vector_b(py, vector_a.len(), vector_b)?;
    return vector_apply(&vector_a, &vector_b, &mut |a, b| a / b);
}
