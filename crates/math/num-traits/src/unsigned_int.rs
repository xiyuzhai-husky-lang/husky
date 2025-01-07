use std::{
    iter::{Product, Sum},
    ops::{Add, Div, Mul, Sub},
};

pub trait UnsignedInt:
    std::fmt::Debug
    + std::fmt::Display
    + Copy
    + PartialEq
    + PartialOrd
    + Ord
    + Add
    + Sub
    + Mul
    + Div
    + Sum
    + Product
{
    const ZERO: Self;
    const ONE: Self;

    fn as_usize(self) -> usize;

    fn from_usize(n: usize) -> Self;

    fn checked_add(self, other: Self) -> Option<Self>;

    fn checked_sub(self, other: Self) -> Option<Self>;

    fn checked_mul(self, other: Self) -> Option<Self>;

    fn checked_div(self, other: Self) -> Option<Self>;

    fn checked_pow(self, other: Self) -> Option<Self>;

    fn checked_factorial(self) -> Option<Self> {
        let mut result = Self::ONE;
        for i in 1..=self.as_usize() {
            result = result.checked_mul(Self::from_usize(i))?;
        }
        Some(result)
    }
}

macro_rules! impl_unsigned_int {
    ($t:ty) => {
        impl UnsignedInt for $t {
            const ZERO: Self = 0;
            const ONE: Self = 1;

            #[inline]
            fn as_usize(self) -> usize {
                self as usize
            }

            #[inline]
            fn from_usize(n: usize) -> Self {
                n as Self
            }

            #[inline]
            fn checked_add(self, other: Self) -> Option<Self> {
                self.checked_add(other)
            }

            #[inline]
            fn checked_sub(self, other: Self) -> Option<Self> {
                self.checked_sub(other)
            }

            #[inline]
            fn checked_mul(self, other: Self) -> Option<Self> {
                self.checked_mul(other)
            }

            #[inline]
            fn checked_div(self, other: Self) -> Option<Self> {
                self.checked_div(other)
            }

            #[inline]
            fn checked_pow(self, other: Self) -> Option<Self> {
                self.checked_pow(other.try_into().ok()?)
            }
        }
    };
}

impl_unsigned_int!(u8);
impl_unsigned_int!(u16);
impl_unsigned_int!(u32);
impl_unsigned_int!(u64);
impl_unsigned_int!(u128);
impl_unsigned_int!(usize);

#[test]
fn checked_factorial64_works() {
    fn test(n: u64, expected: u64) {
        assert_eq!(n.checked_factorial(), Some(expected));
    }
    test(0, 1);
    test(1, 1);
    test(2, 2);
    test(3, 6);
    test(4, 24);
}
