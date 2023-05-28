use std::ops::RangeInclusive;

use fundsp::Float;

/// Map value from one set of bounds to another
pub fn map<F: Float>(value: F, i_start: F, i_stop: F, o_start: F, o_stop: F) -> F {
    o_start + ((o_stop - o_start) * ((value - i_start) / (i_stop - i_start)))
}

/// Map value from one set of bounds to another
pub fn map_range<F: Float>(value: F, i: &RangeInclusive<F>, o: &RangeInclusive<F>) -> F {
    map(value, *i.start(), *i.end(), *o.start(), *o.end())
}
