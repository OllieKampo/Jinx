use pyo3::prelude::*;
use pyo3::{wrap_pyfunction, wrap_pymodule};


mod planning;
mod moremath;


#[pymodule]
#[pyo3(name="vectors")]
fn vectors(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(moremath::vectors::vector_add, m)?)?;
    m.add_function(wrap_pyfunction!(moremath::vectors::vector_subtract, m)?)?;
    Ok(())
}


#[pymodule]
#[pyo3(name="moremath")]
fn moremath_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(vectors))?;
    Ok(())
}


#[pymodule]
#[pyo3(name="rrt")]
fn rrt(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(planning::rrt::rrt, m)?)?;
    Ok(())
}


#[pymodule]
#[pyo3(name="planning")]
fn planning_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(rrt))?;
    Ok(())
}


#[pymodule]
#[pyo3(name="rost")]
fn rost(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(moremath_py))?;
    m.add_wrapped(wrap_pymodule!(planning_py))?;
    Ok(())
}
