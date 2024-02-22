use std::cmp::Ordering;
use std::fmt::{self, Debug, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use ecow::{eco_format, EcoString};

use crate::foundations::{cast, ty, Fold, Resolve, TypstStyleChain, TypstValueRepr};
use crate::layout::{Ratio, TypstAbsLength, TypstEmLength, TypstLength};
use crate::util::TypstNumeric;

/// A length in relation to some known length.
///
/// This type is a combination of a [length]($length) with a [ratio]($ratio). It
/// results from addition and subtraction of a length and a ratio. Wherever a
/// relative length is expected, you can also use a bare length or ratio.
///
/// # Example
/// ```example
/// #rect(width: 100% - 50pt)
///
/// #(100% - 50pt).length \
/// #(100% - 50pt).ratio
/// ```
///
/// A relative length has the following fields:
/// - `length`: Its length component.
/// - `ratio`: Its ratio component.
#[ty(cast, name = "relative", title = "Relative Length")]
#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Rel<T: TypstNumeric = TypstLength> {
    /// The relative part.
    pub rel: Ratio,
    /// The absolute part.
    pub abs: T,
}

impl<T: TypstNumeric> Rel<T> {
    /// The zero relative.
    pub fn zero() -> Self {
        Self {
            rel: Ratio::zero(),
            abs: T::zero(),
        }
    }

    /// A relative with a ratio of `100%` and no absolute part.
    pub fn one() -> Self {
        Self {
            rel: Ratio::one(),
            abs: T::zero(),
        }
    }

    /// Create a new relative from its parts.
    pub fn new(rel: Ratio, abs: T) -> Self {
        Self { rel, abs }
    }

    /// Whether both parts are zero.
    pub fn is_zero(self) -> bool {
        self.rel.is_zero() && self.abs == T::zero()
    }

    /// Whether the relative part is one and the absolute part is zero.
    pub fn is_one(self) -> bool {
        self.rel.is_one() && self.abs == T::zero()
    }

    /// Evaluate this relative to the given `whole`.
    pub fn relative_to(self, whole: T) -> T {
        self.rel.of(whole) + self.abs
    }

    /// Map the absolute part with `f`.
    pub fn map<F, U>(self, f: F) -> Rel<U>
    where
        F: FnOnce(T) -> U,
        U: TypstNumeric,
    {
        Rel {
            rel: self.rel,
            abs: f(self.abs),
        }
    }
}

impl Rel<TypstLength> {
    /// Try to divide two relative lengths.
    pub fn try_div(self, other: Self) -> Option<f64> {
        if self.rel.is_zero() && other.rel.is_zero() {
            self.abs.try_div(other.abs)
        } else if self.abs.is_zero() && other.abs.is_zero() {
            Some(self.rel / other.rel)
        } else {
            None
        }
    }
}

impl<T: TypstNumeric + Debug> Debug for Rel<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match (self.rel.is_zero(), self.abs.is_zero()) {
            (false, false) => write!(f, "{:?} + {:?}", self.rel, self.abs),
            (false, true) => self.rel.fmt(f),
            (true, _) => self.abs.fmt(f),
        }
    }
}

impl<T: TypstNumeric + TypstValueRepr> TypstValueRepr for Rel<T> {
    fn repr(&self) -> EcoString {
        match (self.rel.is_zero(), self.abs.is_zero()) {
            (false, false) => eco_format!("{} + {}", self.rel.repr(), self.abs.repr()),
            (false, true) => self.rel.repr(),
            (true, _) => self.abs.repr(),
        }
    }
}

impl From<TypstAbsLength> for Rel<TypstLength> {
    fn from(abs: TypstAbsLength) -> Self {
        Rel::from(TypstLength::from(abs))
    }
}

impl From<TypstEmLength> for Rel<TypstLength> {
    fn from(em: TypstEmLength) -> Self {
        Rel::from(TypstLength::from(em))
    }
}

impl<T: TypstNumeric> From<T> for Rel<T> {
    fn from(abs: T) -> Self {
        Self {
            rel: Ratio::zero(),
            abs,
        }
    }
}

impl<T: TypstNumeric> From<Ratio> for Rel<T> {
    fn from(rel: Ratio) -> Self {
        Self {
            rel,
            abs: T::zero(),
        }
    }
}

impl<T: TypstNumeric + PartialOrd> PartialOrd for Rel<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.rel.is_zero() && other.rel.is_zero() {
            self.abs.partial_cmp(&other.abs)
        } else if self.abs.is_zero() && other.abs.is_zero() {
            self.rel.partial_cmp(&other.rel)
        } else {
            None
        }
    }
}

impl<T: TypstNumeric> Neg for Rel<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            rel: -self.rel,
            abs: -self.abs,
        }
    }
}

impl<T: TypstNumeric> Add for Rel<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            rel: self.rel + other.rel,
            abs: self.abs + other.abs,
        }
    }
}

impl<T: TypstNumeric> Sub for Rel<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self + -other
    }
}

impl<T: TypstNumeric> Mul<f64> for Rel<T> {
    type Output = Self;

    fn mul(self, other: f64) -> Self::Output {
        Self {
            rel: self.rel * other,
            abs: self.abs * other,
        }
    }
}

impl<T: TypstNumeric> Mul<Rel<T>> for f64 {
    type Output = Rel<T>;

    fn mul(self, other: Rel<T>) -> Self::Output {
        other * self
    }
}

impl<T: TypstNumeric> Div<f64> for Rel<T> {
    type Output = Self;

    fn div(self, other: f64) -> Self::Output {
        Self {
            rel: self.rel / other,
            abs: self.abs / other,
        }
    }
}

impl<T: TypstNumeric + AddAssign> AddAssign for Rel<T> {
    fn add_assign(&mut self, other: Self) {
        self.rel += other.rel;
        self.abs += other.abs;
    }
}

impl<T: TypstNumeric + SubAssign> SubAssign for Rel<T> {
    fn sub_assign(&mut self, other: Self) {
        self.rel -= other.rel;
        self.abs -= other.abs;
    }
}

impl<T: TypstNumeric + MulAssign<f64>> MulAssign<f64> for Rel<T> {
    fn mul_assign(&mut self, other: f64) {
        self.rel *= other;
        self.abs *= other;
    }
}

impl<T: TypstNumeric + DivAssign<f64>> DivAssign<f64> for Rel<T> {
    fn div_assign(&mut self, other: f64) {
        self.rel /= other;
        self.abs /= other;
    }
}

impl<T: TypstNumeric> Add<T> for Ratio {
    type Output = Rel<T>;

    fn add(self, other: T) -> Self::Output {
        Rel::from(self) + Rel::from(other)
    }
}

impl<T: TypstNumeric> Add<T> for Rel<T> {
    type Output = Self;

    fn add(self, other: T) -> Self::Output {
        self + Rel::from(other)
    }
}

impl<T: TypstNumeric> Add<Ratio> for Rel<T> {
    type Output = Self;

    fn add(self, other: Ratio) -> Self::Output {
        self + Rel::from(other)
    }
}

impl<T> Resolve for Rel<T>
where
    T: Resolve + TypstNumeric,
    <T as Resolve>::Output: TypstNumeric,
{
    type Output = Rel<<T as Resolve>::Output>;

    fn resolve(self, styles: TypstStyleChain) -> Self::Output {
        self.map(|abs| abs.resolve(styles))
    }
}

impl<T> Fold for Rel<T>
where
    T: TypstNumeric + Fold,
{
    fn fold(self, outer: Self) -> Self {
        Self {
            rel: self.rel,
            abs: self.abs.fold(outer.abs),
        }
    }
}

cast! {
    Rel<TypstAbsLength>,
    self => self.map(TypstLength::from).into_value(),
}
