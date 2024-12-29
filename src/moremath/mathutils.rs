use pyo3::prelude::*;

/// Exponentially decay a value between a given range (inclusive).
pub fn exp_decay_between<T>(value: &T, min: &T, max: &T) -> T
where
    T: num::Float + num::FromPrimitive
{
    if value < min {
        return *min;
    } else if value > max {
        return *max;
    }
    let one: T = T::from_f64(1.0).unwrap();
    let begin: T = *value - (*min - one);
    let end: T = *max - (*min - one);
    return one - (begin.log10() / end.log10());
}

/// Normalize a vector of numbers between a given range (inclusive).
pub fn normalize_between<T>(values: &Vec<T>, lower: &T, upper: &T) -> Vec<T>
where
    T: num::Float
{
    let min: T = *values.iter().min_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
    let max: T = *values.iter().max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
    let min_max_range: T = max - min;
    let lower_upper_range: T = *upper - *lower;
    return values.iter().map(|x| {
        let normalized: T = (*x - min) / min_max_range;
        return *lower + (normalized * lower_upper_range);
    }).collect();
}

/// Normalize a vector of numbers to sum to a given value.
pub fn normalize_sum<T>(values: Vec<T>, sum: T) -> Vec<T>
where
    T: num::Float + std::iter::Sum<T>
{
    let total: T = values.iter().copied().sum::<T>();
    return values.iter().map(|x| {
        return *x * (sum / total);
    }).collect();
}

/// Find the closest two integer factors of a given integer.
/// Such that `x_factor * y_factor == value` and `x_factor <= y_factor`.
pub fn closest_factors(value: usize) -> (usize, usize) {
    let mut x_factor: usize = 1;
    let mut y_factor: usize = value;
    let mut min_diff: usize = y_factor - x_factor;
    for x in 1..=value {
        if value % x == 0 {
            let y: usize = value / x;
            let diff: usize = y - x;
            if diff < min_diff {
                x_factor = x;
                y_factor = y;
                min_diff = diff;
            }
        }
    }
    return (x_factor, y_factor);
}

#[pyfunction]
pub fn py_exp_decay_between(value: f64, min: f64, max: f64) -> f64 {
    return exp_decay_between(&value, &min, &max);
}

#[pyfunction]
pub fn py_normalize_between(values: Vec<f64>, lower: f64, upper: f64) -> Vec<f64> {
    return normalize_between(&values, &lower, &upper);
}

#[pyfunction]
pub fn py_normalize_sum(values: Vec<f64>, sum: f64) -> Vec<f64> {
    return normalize_sum(values, sum);
}

#[pyfunction]
pub fn py_closest_factors(value: usize) -> (usize, usize) {
    return closest_factors(value);
}

#[pymodule]
#[pyo3(name="mathutils")]
pub fn mod_mathutils(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_exp_decay_between, m)?)?;
    m.add_function(wrap_pyfunction!(py_normalize_between, m)?)?;
    m.add_function(wrap_pyfunction!(py_normalize_sum, m)?)?;
    m.add_function(wrap_pyfunction!(py_closest_factors, m)?)?;
    Ok(())
}
