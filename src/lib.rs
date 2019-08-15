macro_rules! finite {
    ($name:tt, $ty:ty) => (
        finite!($name, $ty, stringify!($ty));
    );
    ($name:tt, $ty:ty, $tyname:expr) => {
        #[doc = "A finite `"]
        #[doc = $tyname]
        #[doc = "`. May not be infinite nor NaN."]
        #[derive(Debug, Clone, Copy)]
        pub struct $name($ty);

        impl $name {
            #[doc = "Create a new finite `"]
            #[doc = $tyname]
            #[doc = "`. Will return `None` if given value is infinite or NaN."]
            pub fn new(n: $ty) -> Option<Self> {
                if n.is_finite() {
                    Some(Self(n))
                } else {
                    None
                }
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }

        impl PartialEq<$ty> for $name {
            fn eq(&self, other: &$ty) -> bool {
                &self.0 == other
            }
        }

        impl Eq for $name {}

        impl PartialOrd for $name {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                self.0.partial_cmp(&other.0)
            }
        }

        impl PartialOrd<$ty> for $name {
            fn partial_cmp(&self, other: &$ty) -> Option<std::cmp::Ordering> {
                self.0.partial_cmp(other)
            }
        }

        impl Ord for $name {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.0.partial_cmp(&other.0).expect("must be finite")
            }
        }
    }
}

finite!(FiniteF32, f32);
finite!(FiniteF64, f64);

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

    #[test]
    fn cmp32() {
        let finite = FiniteF32::new(1f32).unwrap();
        assert_eq!(finite < 32f32, true);
        assert_eq!(finite == 1f32, true);
        assert_eq!(finite > -1f32, true);

        assert_eq!(finite > f32::NAN, false);
        assert_eq!(finite > f32::NEG_INFINITY, true);
        assert_eq!(finite < f32::INFINITY, true);
    }

    #[test]
    fn cmp64() {
        let finite = FiniteF64::new(1f64).unwrap();
        assert_eq!(finite < 64f64, true);
        assert_eq!(finite == 1f64, true);
        assert_eq!(finite > -1f64, true);

        assert_eq!(finite > f64::NAN, false);
        assert_eq!(finite > f64::NEG_INFINITY, true);
        assert_eq!(finite < f64::INFINITY, true);
    }
}
