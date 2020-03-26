use arith_traits::Wrap;
use std::ops::{Add, Deref};

#[derive(Debug)]
pub struct Wrapping<T>(pub T);

// Suppress false positive recursion warning
#[allow(unconditional_recursion)]
impl<T: Wrap> Wrap for Wrapping<T> {
    type Output = Self;

    #[inline]
    fn wrapping_abs(self) -> Self::Output {
        self.wrapping_abs()
    }

    #[inline]
    fn wrapping_add(self, rhs: impl Into<Self>) -> Self::Output {
        self.wrapping_add(rhs)
    }

    #[inline]
    fn wrapping_div(self, rhs: impl Into<Self>) -> Self::Output {
        self.wrapping_div(rhs)
    }

    #[inline]
    fn wrapping_div_euclid(self, rhs: impl Into<Self>) -> Self::Output {
        self.wrapping_div_euclid(rhs)
    }

    #[inline]
    fn wrapping_mul(self, rhs: impl Into<Self>) -> Self::Output {
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
    fn wrapping_rem(self, rhs: impl Into<Self>) -> Self::Output {
        self.wrapping_rem(rhs)
    }

    #[inline]
    fn wrapping_rem_euclid(self, rhs: impl Into<Self>) -> Self::Output {
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
    fn wrapping_sub(self, rhs: impl Into<Self>) -> Self::Output {
        self.wrapping_sub(rhs)
    }
}

impl<T: PartialEq> PartialEq for Wrapping<T> {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        self.0 == rhs.0
    }
}
// TODO: Add conditional impls for Eq, Partial/Ord, Hash, etc

impl<T: Add<Output = T>> Deref for Wrapping<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Wrap<R, Output = T>, R> Add<R> for Wrapping<T> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: R) -> Self::Output {
        Self(self.0.wrapping_add(rhs))
    }
}
