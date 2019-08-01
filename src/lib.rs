/// A finite `f32`. May not be infinite nor NaN.
#[derive(Debug, Clone, Copy)]
pub struct FiniteF32(f32);

impl FiniteF32 {
    /// Create a new finite `f32`. Will return `None` if given value is infinite or NaN.
    pub fn new(n: f32) -> Option<FiniteF32> {
        if n.is_finite() {
            Some(FiniteF32(n))
        } else {
            None
        }
    }
}

/// A finite `f64`. May not be infinite nor NaN.
#[derive(Debug, Clone, Copy)]
pub struct FiniteF64(f64);

impl FiniteF64 {
    /// Create a new finite `f64`. Will return `None` if given value is infinite or NaN.
    pub fn new(n: f64) -> Option<FiniteF64> {
        if n.is_finite() {
            Some(FiniteF64(n))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{f32, f64};

    #[test]
    fn smoke() {
        assert!(FiniteF32::new(1f32).is_some());
        assert!(FiniteF64::new(42f64).is_some());
        assert!(FiniteF32::new(f32::NAN).is_none());
        assert!(FiniteF64::new(f64::NAN).is_none());
        assert!(FiniteF32::new(f32::INFINITY).is_none());
        assert!(FiniteF64::new(f64::INFINITY).is_none());
        assert!(FiniteF32::new(f32::NEG_INFINITY).is_none());
        assert!(FiniteF64::new(f64::NEG_INFINITY).is_none());
    }
}
