use fundsp::Float;

/// Map value from one set of bounds to another
pub fn map<F: Float>(value: F, i_start: F, i_stop: F, o_start: F, o_stop: F) -> F {
    o_start + ((o_stop - o_start) * ((value - i_start) / (i_stop - i_start)))
}
