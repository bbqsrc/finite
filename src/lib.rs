use std::hash::{Hash, Hasher};

macro_rules! finite {
    (@op => $opname:ty, $opnamety:ty, $func:tt, $name:tt, $ty:ty) => {
        impl $opname for $name {
            type Output = Option<$ty>;

            fn $func(self, other: Self) -> Option<$ty> {
                let result = (self.0).$func(other.0);

                if result.is_finite() {
                    Some(result)
                } else {
                    None
                }
            }
        }

        impl $opnamety for $name {
            type Output = $ty;

            fn $func(self, other: $ty) -> $ty {
                (self.0).$func(other)
            }
        }
    };
    ($name:tt, $ty:ty) => (
        finite!(@finish => $name, $ty, stringify!($ty));
    );
    (@finish => $name:tt, $ty:ty, $tyname:expr) => {
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

            #[inline(always)]
            pub fn checked_add(self, other: $ty) -> Option<Self> {
                Self::new(std::ops::Add::add(self, other))
            }

            #[inline(always)]
            pub fn checked_sub(self, other: $ty) -> Option<Self> {
                Self::new(std::ops::Sub::sub(self, other))
            }

            #[inline(always)]
            pub fn checked_mul(self, other: $ty) -> Option<Self> {
                Self::new(std::ops::Mul::mul(self, other))
            }

            #[inline(always)]
            pub fn checked_div(self, other: $ty) -> Option<Self> {
                Self::new(std::ops::Div::div(self, other))
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

        finite!(@op => std::ops::Add, std::ops::Add<$ty>, add, $name, $ty);
        finite!(@op => std::ops::Sub, std::ops::Sub<$ty>, sub, $name, $ty);
        finite!(@op => std::ops::Div, std::ops::Div<$ty>, div, $name, $ty);
        finite!(@op => std::ops::Mul, std::ops::Mul<$ty>, mul, $name, $ty);

        impl std::convert::TryFrom<$ty> for $name {
            type Error = $crate::TryFromFloatError;

            fn try_from(value: $ty) -> Result<Self, Self::Error> {
                match Self::new(value) {
                    Some(v) => Ok(v),
                    None => Err($crate::TryFromFloatError(value.classify()))
                }
            }
        }
    }
}

pub struct TryFromFloatError(std::num::FpCategory);

finite!(FiniteF32, f32);
finite!(FiniteF64, f64);

impl Hash for FiniteF32 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(unsafe { std::mem::transmute::<f32, u32>(self.0) });
    }
}

impl Hash for FiniteF64 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(unsafe { std::mem::transmute::<f64, u64>(self.0) });
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

    #[test]
    fn add32() {
        let finite = FiniteF32::new(1f32).unwrap();
        assert_eq!(finite + 32f32, 33f32);
        assert_eq!(finite - 32f32, -31f32);
        assert_eq!(finite + f32::INFINITY, f32::INFINITY);
        assert_eq!(finite - f32::INFINITY, f32::NEG_INFINITY);
    }

    #[test]
    fn add64() {
        let finite = FiniteF64::new(1f64).unwrap();
        assert_eq!(finite + 32f64, 33f64);
        assert_eq!(finite - 32f64, -31f64);
        assert_eq!(finite + f64::INFINITY, f64::INFINITY);
        assert!(finite.checked_add(f64::INFINITY).is_none());
        assert_eq!(finite - f64::INFINITY, f64::NEG_INFINITY);
        assert!(finite.checked_sub(f64::INFINITY).is_none());
    }

    #[test]
    fn hash() {
        let mut map = std::collections::HashMap::new();
        let f = FiniteF32::new(32f32).unwrap();
        map.insert(f, "oh yes");
        assert_eq!(map[&f], "oh yes");
    }
}
