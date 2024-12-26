
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
    return one - (begin.log10() / max.log10());
}
