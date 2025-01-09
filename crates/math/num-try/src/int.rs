pub mod error;

use self::error::*;

pub trait IntTry:
    std::fmt::Debug + std::fmt::Display + Copy + PartialEq + PartialOrd + Ord
{
    const ZERO: Self;
    const ONE: Self;

    fn into_usize(self) -> IntResult<usize>;

    fn from_usize(n: usize) -> IntResult<Self>;

    fn try_add(self, other: Self) -> IntResult<Self>;

    fn try_sub(self, other: Self) -> IntResult<Self>;

    fn try_mul(self, other: Self) -> IntResult<Self>;

    fn try_div(self, other: Self) -> IntResult<Self>;

    fn try_pow(self, other: Self) -> IntResult<Self>;

    fn try_factorial(self) -> IntResult<Self> {
        let mut result = Self::ONE;
        for i in 1..=self.into_usize()? {
            result = result.try_mul(Self::from_usize(i)?)?;
        }
        Ok(result)
    }

    fn try_sum(iter: impl IntoIterator<Item = Self>) -> IntResult<Self>;
}

macro_rules! impl_unsigned_int {
    ($t:ty) => {
        impl IntTry for $t {
            const ZERO: Self = 0;
            const ONE: Self = 1;

            #[inline]
            fn into_usize(self) -> IntResult<usize> {
                self.try_into().map_err(|_| IntError::IntoUsize)
            }

            #[inline]
            fn from_usize(n: usize) -> IntResult<Self> {
                n.try_into().map_err(|_| IntError::FromUsize)
            }

            #[inline]
            fn try_add(self, other: Self) -> IntResult<Self> {
                self.checked_add(other).ok_or(IntError::Overflow)
            }

            #[inline]
            fn try_sub(self, other: Self) -> IntResult<Self> {
                self.checked_sub(other).ok_or(IntError::Overflow)
            }

            #[inline]
            fn try_mul(self, other: Self) -> IntResult<Self> {
                self.checked_mul(other).ok_or(IntError::Overflow)
            }

            #[inline]
            fn try_div(self, other: Self) -> IntResult<Self> {
                self.checked_div(other).ok_or(IntError::Overflow)
            }

            #[inline]
            fn try_pow(self, other: Self) -> IntResult<Self> {
                self.checked_pow(other.try_into().map_err(|_| IntError::AsPow)?)
                    .ok_or(IntError::Overflow)
            }

            #[inline]
            fn try_sum(iter: impl IntoIterator<Item = Self>) -> IntResult<Self> {
                iter.into_iter()
                    .try_fold(Self::ZERO, |acc, x| -> IntResult<Self> {
                        acc.checked_add(x).ok_or(IntError::Overflow)
                    })
            }
        }
    };
}

impl_unsigned_int!(i8);
impl_unsigned_int!(i16);
impl_unsigned_int!(i32);
impl_unsigned_int!(i64);
impl_unsigned_int!(i128);
impl_unsigned_int!(isize);
impl_unsigned_int!(u8);
impl_unsigned_int!(u16);
impl_unsigned_int!(u32);
impl_unsigned_int!(u64);
impl_unsigned_int!(u128);
impl_unsigned_int!(usize);

#[test]
fn checked_factorial64_works() {
    fn test(n: u64, expected: u64) {
        assert_eq!(n.try_factorial(), Ok(expected));
    }
    test(0, 1);
    test(1, 1);
    test(2, 2);
    test(3, 6);
    test(4, 24);
}
