use pyo3::prelude::*;
use pyo3::wrap_pymodule;


pub mod common;
pub mod planning;
pub mod moremath;


#[pymodule]
#[pyo3(name="moremath")]
fn moremath_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(moremath::vector::mod_vector))?;
    m.add_wrapped(wrap_pymodule!(moremath::cloud::mod_cloud))?;
    m.add_wrapped(wrap_pymodule!(moremath::mathutils::mod_mathutils))?;
    Ok(())
}

#[pymodule]
#[pyo3(name="planning")]
fn planning_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(planning::rrt::mod_rrt))?;
    Ok(())
}


#[pymodule]
#[pyo3(name="rost")]
fn rost(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(moremath_py))?;
    m.add_wrapped(wrap_pymodule!(planning_py))?;
    Ok(())
}
