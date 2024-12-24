use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use crate::moremath::vectors::vector_apply;

pub fn cloud_apply<F>(cloud_a: &Vec<Vec<f64>>, cloud_b: &Vec<Vec<f64>>, f: &mut F) -> Result<Vec<Vec<f64>>, &'static str>
where
    F: FnMut(f64, f64) -> f64,
{
    if cloud_a.len() != cloud_b.len() {
        return Err("Cloud lengths do not match");
    }
    let result = cloud_a
        .iter()
        .zip(cloud_b.iter())
        .map(|(a, b)| vector_apply(&a, &b, f).unwrap())
        .collect();
    return Ok(result);
}

pub fn rust_result_to_py_result<T, E>(result: Result<T, E>) -> PyResult<T> {
    return match result {
        Ok(result) => Ok(result),
        Err(e) => Err(PyValueError::new_err("Error")),
    };
}

#[pyfunction]
pub fn cloud_add(_py: Python, cloud_a: Vec<Vec<f64>>, cloud_b: Vec<Vec<f64>>) -> PyResult<Vec<Vec<f64>>> {
    return rust_result_to_py_result(cloud_apply(&cloud_a, &cloud_b, &mut |a, b| a + b));
}

#[pyfunction]
pub fn cloud_subtract(_py: Python, cloud_a: Vec<Vec<f64>>, cloud_b: Vec<Vec<f64>>) -> PyResult<Vec<Vec<f64>>> {
    return rust_result_to_py_result(cloud_apply(&cloud_a, &cloud_b, &mut |a, b| a - b));
}

#[pyfunction]
pub fn cloud_multiply(_py: Python, cloud_a: Vec<Vec<f64>>, cloud_b: Vec<Vec<f64>>) -> PyResult<Vec<Vec<f64>>> {
    return rust_result_to_py_result(cloud_apply(&cloud_a, &cloud_b, &mut |a, b| a * b));
}

#[pyfunction]
pub fn cloud_divide(_py: Python, cloud_a: Vec<Vec<f64>>, cloud_b: Vec<Vec<f64>>) -> PyResult<Vec<Vec<f64>>> {
    return rust_result_to_py_result(cloud_apply(&cloud_a, &cloud_b, &mut |a, b| a / b));
}

#[pymodule]
#[pyo3(name="cloud")]
fn moremath_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(cloud_add, m)?)?;
    m.add_function(wrap_pyfunction!(cloud_subtract, m)?)?;
    m.add_function(wrap_pyfunction!(cloud_multiply, m)?)?;
    m.add_function(wrap_pyfunction!(cloud_divide, m)?)?;
    Ok(())
}
