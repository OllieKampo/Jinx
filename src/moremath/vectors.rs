use pyo3::prelude::*;

fn vector_apply<F>(py: Python, vector_a: Vec<f64>, vector_b: PyObject, f: F) -> PyResult<Vec<f64>>
where
    F: Fn(f64, f64) -> f64,
{
    if vector_a.len() == 0 {
        return Ok(Vec::new());
    }
    let vector_b = if let Ok(b) = vector_b.extract::<f64>(py) {
        vec![b; vector_a.len()]
    } else {
        vector_b.extract::<Vec<f64>>(py)?
    };
    let result = vector_a
        .iter()
        .zip(vector_b.iter())
        .map(|(&a, &b)| f(a, b))
        .collect();
    return Ok(result);
}

#[pyfunction]
pub fn vector_add(py: Python, vector_a: Vec<f64>, vector_b: PyObject) -> PyResult<Vec<f64>> {
    return vector_apply(py, vector_a, vector_b, |a, b| a + b);
}

#[pyfunction]
pub fn vector_subtract(py: Python, vector_a: Vec<f64>, vector_b: PyObject) -> PyResult<Vec<f64>> {
    return vector_apply(py, vector_a, vector_b, |a, b| a - b);
}

#[pyfunction]
pub fn vector_multiply(py: Python, vector_a: Vec<f64>, vector_b: PyObject) -> PyResult<Vec<f64>> {
    return vector_apply(py, vector_a, vector_b, |a, b| a * b);
}

#[pyfunction]
pub fn vector_divide(py: Python, vector_a: Vec<f64>, vector_b: PyObject) -> PyResult<Vec<f64>> {
    return vector_apply(py, vector_a, vector_b, |a, b| a / b);
}

// #[pyfunction]
// pub fn cloud_add(py: Python, cloud_a: Vec<Vec<f64>>, cloud_b: Vec<Vec<f64>>) -> PyResult<Vec<Vec<f64>>> {
//     if cloud_a.len() == 0 {
//         return Ok(Vec::new());
//     }
//     let result = cloud_a
//         .iter()
//         .zip(cloud_b.iter())
//         .map(|(a, b)| vector_add(py, a.clone(), b.clone()).unwrap())
//         .collect();
//     return Ok(result);
// }
