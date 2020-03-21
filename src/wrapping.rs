use arith_traits::Wrap;
use std::ops::Add;

pub struct Wrapping<T>(T);

// Suppress false positive recursion warning
#[allow(unconditional_recursion)]
impl<T: Wrap> Wrap for Wrapping<T> {
    type Output = Self;

    fn wrapping_abs(self) -> Self::Output {
        self.wrapping_abs()
    }

    fn wrapping_add(self, rhs: Self) -> Self::Output {
        self.wrapping_add(rhs)
    }

    fn wrapping_div(self, rhs: Self) -> Self::Output {
        self.wrapping_div(rhs)
    }

    fn wrapping_div_euclid(self, rhs: Self) -> Self::Output {
        self.wrapping_div_euclid(rhs)
    }

    fn wrapping_mul(self, rhs: Self) -> Self::Output {
        self.wrapping_mul(rhs)
    }

    fn wrapping_neg(self) -> Self::Output {
        self.wrapping_neg()
    }

    fn wrapping_pow(self, rhs: u32) -> Self::Output {
        self.wrapping_pow(rhs)
    }

    fn wrapping_rem(self, rhs: Self) -> Self::Output {
        self.wrapping_rem(rhs)
    }

    fn wrapping_rem_euclid(self, rhs: Self) -> Self::Output {
        self.wrapping_rem_euclid(rhs)
    }

    fn wrapping_shl(self, rhs: u32) -> Self::Output {
        self.wrapping_shl(rhs)
    }

    fn wrapping_shr(self, rhs: u32) -> Self::Output {
        self.wrapping_shr(rhs)
    }

    fn wrapping_sub(self, rhs: Self) -> Self::Output {
        self.wrapping_sub(rhs)
    }
}

impl<T: Add<Output = T>> Add for Wrapping<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
