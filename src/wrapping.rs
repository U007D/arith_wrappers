use arith_traits::Wrap;
use std::{
    cmp::Ordering,
    fmt::{Display, Formatter, Result as FmtResult},
    hash::{Hash, Hasher},
    ops::{Add, Deref},
};

#[derive(Debug)]
pub struct Wrapping<T>(pub T);

// Suppress false-positive recursion warning (`self.wrapping_*()` is not recursive)
#[allow(unconditional_recursion)]
impl<T: Wrap> Wrap for Wrapping<T> {
    type Output = Self;

    #[inline]
    fn wrapping_abs(self) -> Self::Output {
        self.wrapping_abs()
    }

    #[inline]
    fn wrapping_add(self, rhs: Self) -> Self::Output {
        self.wrapping_add(rhs)
    }

    #[inline]
    fn wrapping_div(self, rhs: Self) -> Self::Output {
        self.wrapping_div(rhs)
    }

    #[inline]
    fn wrapping_div_euclid(self, rhs: Self) -> Self::Output {
        self.wrapping_div_euclid(rhs)
    }

    #[inline]
    fn wrapping_mul(self, rhs: Self) -> Self::Output {
        self.wrapping_mul(rhs)
    }

    #[inline]
    fn wrapping_neg(self) -> Self::Output {
        self.wrapping_neg()
    }

    #[inline]
    fn wrapping_pow(self, rhs: u32) -> Self::Output {
        self.wrapping_pow(rhs)
    }

    #[inline]
    fn wrapping_rem(self, rhs: Self) -> Self::Output {
        self.wrapping_rem(rhs)
    }

    #[inline]
    fn wrapping_rem_euclid(self, rhs: Self) -> Self::Output {
        self.wrapping_rem_euclid(rhs)
    }

    #[inline]
    fn wrapping_shl(self, rhs: u32) -> Self::Output {
        self.wrapping_shl(rhs)
    }

    #[inline]
    fn wrapping_shr(self, rhs: u32) -> Self::Output {
        self.wrapping_shr(rhs)
    }

    #[inline]
    fn wrapping_sub(self, rhs: Self) -> Self::Output {
        self.wrapping_sub(rhs)
    }
}

impl<T: Clone> Clone for Wrapping<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: Deref> Deref for Wrapping<T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Display> Display for Wrapping<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.0.fmt(f)
    }
}

impl<T: Hash> Hash for Wrapping<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T: Ord> Ord for Wrapping<T> {
    #[inline]
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.0.cmp(&rhs.0)
    }
}

impl<T: PartialOrd> PartialOrd for Wrapping<T> {
    #[inline]
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&rhs.0)
    }
}

impl<T: Eq> Eq for Wrapping<T> {}

impl<T: PartialEq> PartialEq for Wrapping<T> {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        self.0.eq(&rhs.0)
    }
}

// impl<T: Add<Output = T>> Deref for Wrapping<T> {
//     type Target = T;
//
//     #[inline]
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
//

impl<T: Wrap<R, Output = T>, R> Add<R> for Wrapping<T> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: R) -> Self::Output {
        Self(self.0.wrapping_add(rhs))
    }
}
