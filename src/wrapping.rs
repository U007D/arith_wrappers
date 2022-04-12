#![allow(clippy::inline_always)]

use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    hash::Hash,
    ops::{Add, Deref},
};

use arith_traits::{IMinMax, IWrappingNonGenericOps, IWrappingOps};

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub struct WrappingWrapper<T>(pub T);

impl<T> IWrappingOps<T> for WrappingWrapper<T>
where
    T: IWrappingOps<Output = T> + PartialOrd,
    Self: IMinMax,
{
    #[inline(always)]
    fn wrapping_add(self, rhs: T) -> <Self as IWrappingNonGenericOps>::Output {
        Self(self.0.wrapping_add(rhs))
    }

    #[inline(always)]
    fn wrapping_div(self, rhs: T) -> <Self as IWrappingNonGenericOps>::Output {
        Self(self.0.wrapping_div(rhs))
    }

    #[inline(always)]
    fn wrapping_div_euclid(self, rhs: T) -> <Self as IWrappingNonGenericOps>::Output {
        Self(self.0.wrapping_div_euclid(rhs))
    }

    #[inline(always)]
    fn wrapping_mul(self, rhs: T) -> <Self as IWrappingNonGenericOps>::Output {
        Self(self.0.wrapping_mul(rhs))
    }

    #[inline(always)]
    fn wrapping_rem(self, rhs: T) -> <Self as IWrappingNonGenericOps>::Output {
        Self(self.0.wrapping_rem(rhs))
    }

    #[inline(always)]
    fn wrapping_rem_euclid(self, rhs: T) -> <Self as IWrappingNonGenericOps>::Output {
        Self(self.0.wrapping_rem_euclid(rhs))
    }

    #[inline(always)]
    fn wrapping_sub(self, rhs: T) -> <Self as IWrappingNonGenericOps>::Output {
        Self(self.0.wrapping_sub(rhs))
    }
}

impl<T> IWrappingNonGenericOps for WrappingWrapper<T>
where
    T: IWrappingOps<Output = T> + PartialOrd,
    Self: IMinMax,
{
    type Output = Self;

    #[inline(always)]
    fn wrapping_abs(self) -> Self::Output {
        Self(self.0.wrapping_abs())
    }

    #[inline(always)]
    fn wrapping_neg(self) -> Self::Output {
        Self(self.0.wrapping_neg())
    }

    #[inline(always)]
    fn wrapping_pow(self, rhs: u32) -> Self::Output {
        Self(self.0.wrapping_pow(rhs))
    }

    #[inline(always)]
    fn wrapping_shl(self, rhs: u32) -> Self::Output {
        Self(self.0.wrapping_shl(rhs))
    }

    #[inline(always)]
    fn wrapping_shr(self, rhs: u32) -> Self::Output {
        Self(self.0.wrapping_shr(rhs))
    }
}

impl<T> Deref for WrappingWrapper<T> {
    type Target = T;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Display for WrappingWrapper<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.0.fmt(f)
    }
}

impl<T, TRhs> Add<TRhs> for WrappingWrapper<T>
where
    TRhs: Into<T>,
    T: IWrappingOps<Output = T>,
{
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: TRhs) -> Self::Output {
        Self(self.0.wrapping_add(rhs.into()))
    }
}

macro_rules! min_max_impl {
    ($($t:ty)*) => ($(
        impl IMinMax for WrappingWrapper<$t> {
            const MAX: Self = Self(<$t>::MAX);
            const MIN: Self = Self(<$t>::MIN);
        }

        impl From<WrappingWrapper<Self>> for $t {
            #[inline(always)]
            fn from(wrapper: WrappingWrapper<Self>) -> Self {
                wrapper.0
            }
        }
    )*)
}

min_max_impl! { i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }
