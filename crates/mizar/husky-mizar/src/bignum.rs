use num_bigint::BigInt;
use num_traits::sign::Signed;
use std::borrow::Cow;
use std::cmp::Ordering;
use std::fmt::{Debug, Display};

#[derive(PartialEq, Eq, Clone, Debug)]
enum Integer {
  Small(i32),
  Large(Box<BigInt>),
}
impl From<i32> for Integer {
  fn from(v: i32) -> Self { Self::Small(v) }
}
impl From<BigInt> for Integer {
  fn from(v: BigInt) -> Self { Self::large(v) }
}

impl Display for Integer {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Integer::Small(n) => Display::fmt(n, f),
      Integer::Large(n) => Display::fmt(n, f),
    }
  }
}

impl Integer {
  const ZERO: Self = Self::Small(0);
  const ONE: Self = Self::Small(1);

  fn large(i: BigInt) -> Self {
    match i.try_into() {
      Ok(n) => Self::Small(n),
      Err(n) => Self::Large(Box::new(n.into_original())),
    }
  }

  fn as_large(&self) -> Cow<'_, BigInt> {
    match self {
      &Integer::Small(n) => Cow::Owned(n.into()),
      Integer::Large(n) => Cow::Borrowed(n),
    }
  }

  fn gcd(mut self, mut rhs: Self) -> Self {
    if self >= rhs {
      std::mem::swap(&mut self, &mut rhs)
    }
    loop {
      if self == Self::ZERO {
        return rhs
      }
      (self, rhs) = (rhs % self.clone(), self)
    }
  }

  fn abs(self) -> Self {
    if let Integer::Small(a) = self {
      if let Some(out) = a.checked_abs() {
        return out.into()
      }
    }
    self.as_large().abs().into()
  }
}

impl std::ops::Add for Integer {
  type Output = Self;
  fn add(self, rhs: Self) -> Self {
    if let (&Integer::Small(a), &Integer::Small(b)) = (&self, &rhs) {
      if let Some(out) = a.checked_add(b) {
        return out.into()
      }
    }
    self.as_large().into_owned().add(&*rhs.as_large()).into()
  }
}

impl std::ops::Neg for Integer {
  type Output = Self;
  fn neg(self) -> Self {
    if let Integer::Small(a) = self {
      if let Some(out) = a.checked_neg() {
        return out.into()
      }
    }
    self.as_large().into_owned().neg().into()
  }
}

impl std::ops::Sub for Integer {
  type Output = Self;
  fn sub(self, rhs: Self) -> Self { self + (-rhs) }
}

impl std::ops::Mul for Integer {
  type Output = Self;
  fn mul(self, rhs: Self) -> Self {
    if let (&Integer::Small(a), &Integer::Small(b)) = (&self, &rhs) {
      if let Some(out) = a.checked_mul(b) {
        return out.into()
      }
    }
    self.as_large().into_owned().mul(&*rhs.as_large()).into()
  }
}

// panics on division by zero
impl std::ops::Div for Integer {
  type Output = Self;
  fn div(self, rhs: Self) -> Self::Output {
    if let (&Integer::Small(a), &Integer::Small(b)) = (&self, &rhs) {
      if let Some(out) = a.checked_div(b) {
        return out.into()
      }
    }
    self.as_large().into_owned().div(&*rhs.as_large()).into()
  }
}

// panics on division by zero
impl std::ops::Rem for Integer {
  type Output = Self;
  fn rem(self, rhs: Self) -> Self {
    if let (&Integer::Small(a), &Integer::Small(b)) = (&self, &rhs) {
      if let Some(out) = a.checked_rem(b) {
        return out.into()
      }
    }
    self.as_large().into_owned().rem(&*rhs.as_large()).into()
  }
}

impl Ord for Integer {
  fn cmp(&self, other: &Self) -> Ordering {
    if let (Integer::Small(a), Integer::Small(b)) = (self, other) {
      a.cmp(b)
    } else {
      self.as_large().cmp(&other.as_large())
    }
  }
}

impl PartialOrd for Integer {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

#[derive(PartialEq, Eq, Clone)]
pub struct Rational {
  num: Integer,
  den: Integer, // Invariant: positive
}

impl Default for Rational {
  fn default() -> Self { Self::ZERO }
}

impl Display for Rational {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if self.den == Integer::ONE {
      write!(f, "{}", self.num)
    } else {
      write!(f, "{}/{}", self.num, self.den)
    }
  }
}

impl Rational {
  pub const fn int(num: i32) -> Self { Self { num: Integer::Small(num), den: Integer::ONE } }
  pub const ZERO: Self = Self::int(0);
  pub const ONE: Self = Self::int(1);
  pub const NEG_ONE: Self = Self::int(-1);

  fn lex(&self, other: &Self) -> Ordering {
    self.num.cmp(&other.num).then_with(|| self.den.cmp(&other.den))
  }
}

impl std::ops::Add for Rational {
  type Output = Self;
  fn add(self, rhs: Self) -> Self::Output {
    let g = self.den.clone().gcd(rhs.den.clone());
    if g == Integer::ONE {
      Self { num: self.num * rhs.den.clone() + rhs.num * self.den.clone(), den: self.den * rhs.den }
    } else {
      let den = (self.den.clone() / g.clone()) * rhs.den.clone();
      let num = self.num * (rhs.den / g.clone()) + rhs.num * (self.den / g.clone());
      let g1 = (num.clone().abs() % g.clone()).gcd(g);
      if g1 == Integer::ONE {
        Self { num, den }
      } else {
        Self { num: num / g1.clone(), den: den / g1 }
      }
    }
  }
}

impl std::ops::Neg for Rational {
  type Output = Self;
  fn neg(self) -> Self::Output { Self { num: -self.num, den: self.den } }
}

impl std::ops::Sub for Rational {
  type Output = Self;
  fn sub(self, rhs: Self) -> Self::Output { self + (-rhs) }
}

impl std::ops::Mul for Rational {
  type Output = Self;
  fn mul(self, rhs: Self) -> Self::Output {
    let g1 = self.num.clone().abs().gcd(rhs.den.clone());
    let g2 = rhs.num.clone().abs().gcd(self.den.clone());
    Self {
      num: (self.num / g1.clone()) * (rhs.num / g2.clone()),
      den: (self.den / g2) * (rhs.den / g1),
    }
  }
}

impl Rational {
  pub fn inv(self) -> Self {
    match self.num.cmp(&Integer::ZERO) {
      Ordering::Less => Self { num: -self.den, den: -self.num },
      Ordering::Equal => panic!("division by zero"),
      Ordering::Greater => Self { num: self.den, den: self.num },
    }
  }
}

impl std::ops::Div for Rational {
  type Output = Self;
  #[allow(clippy::suspicious_arithmetic_impl)]
  fn div(self, rhs: Self) -> Self::Output { self * rhs.inv() }
}

impl Ord for Rational {
  fn cmp(&self, other: &Self) -> Ordering {
    #[allow(clippy::comparison_chain)]
    if self < other {
      Ordering::Less
    } else if other < self {
      Ordering::Greater
    } else {
      Ordering::Equal
    }
  }
}
impl PartialOrd for Rational {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
  fn lt(&self, other: &Self) -> bool {
    if self.num < Integer::ZERO && other.num >= Integer::ZERO {
      true
    } else if self.num == Integer::ZERO {
      Integer::ZERO < other.num
    } else if Integer::ZERO < self.num && other.num <= Integer::ZERO {
      false
    } else {
      self.num.clone() * other.den.clone() < other.num.clone() * self.den.clone()
    }
  }
  fn le(&self, other: &Self) -> bool { !other.lt(self) }
  fn gt(&self, other: &Self) -> bool { other < self }
  fn ge(&self, other: &Self) -> bool { !self.lt(other) }
}

#[derive(PartialEq, Eq, Clone, Default)]
pub struct Complex {
  pub re: Rational,
  pub im: Rational,
}
impl Ord for Complex {
  fn cmp(&self, other: &Self) -> Ordering {
    self.re.lex(&other.re).then_with(|| self.im.lex(&other.im))
  }
}
impl PartialOrd for Complex {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl Display for Complex {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if self.im == Rational::ZERO {
      self.re.fmt(f)
    } else if self.re == Rational::ZERO {
      write!(f, "{}i", self.im)
    } else {
      write!(f, "{}+{}i", self.re, self.im)
    }
  }
}

impl Debug for Complex {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{self}") }
}

impl Complex {
  pub const fn real(re: Rational) -> Self { Self { re, im: Rational::ZERO } }
  // TODO: this should be infallible
  pub fn int(num: u64) -> Self { Self::real(Rational::int(num.try_into().unwrap())) }
  pub const ZERO: Self = Self::real(Rational::ZERO);
  pub const ONE: Self = Self::real(Rational::ONE);
  pub const NEG_ONE: Self = Self::real(Rational::NEG_ONE);
  pub const I: Self = Self { re: Rational::ZERO, im: Rational::ONE };

  pub fn pow(mut self, n: u64) -> Self {
    match n {
      0 => Complex::ONE,
      1 => self,
      _ => {
        let a = self.clone();
        for i in (0..n.ilog2()).rev() {
          self *= self.clone();
          if (n >> i) & 1 != 0 {
            self *= a.clone();
          }
        }
        self
      }
    }
  }
}
impl From<Rational> for Complex {
  fn from(value: Rational) -> Self { Self::real(value) }
}
impl From<u32> for Complex {
  fn from(value: u32) -> Self { Self::int(value.into()) }
}

impl std::ops::Add for Complex {
  type Output = Self;
  fn add(self, rhs: Self) -> Self::Output { Self { re: self.re + rhs.re, im: self.im + rhs.im } }
}
impl std::ops::AddAssign for Complex {
  fn add_assign(&mut self, rhs: Self) { *self = std::mem::take(self) + rhs }
}

impl std::ops::Neg for Complex {
  type Output = Self;
  fn neg(self) -> Self::Output { Self { re: -self.re, im: -self.im } }
}

impl std::ops::Sub for Complex {
  type Output = Self;
  fn sub(self, rhs: Self) -> Self::Output { Self { re: self.re - rhs.re, im: self.im - rhs.im } }
}

impl std::ops::Mul for Complex {
  type Output = Self;
  fn mul(self, rhs: Self) -> Self::Output {
    Self {
      re: self.re.clone() * rhs.re.clone() - self.im.clone() * rhs.im.clone(),
      im: self.re * rhs.im + rhs.re * self.im,
    }
  }
}
impl std::ops::MulAssign for Complex {
  fn mul_assign(&mut self, rhs: Self) { *self = std::mem::take(self) * rhs }
}

impl Complex {
  pub fn inv(self) -> Self {
    let d = (self.re.clone() * self.re.clone() + self.im.clone() * self.im.clone()).inv();
    Self { re: self.re * d.clone(), im: -self.im * d }
  }
}

impl std::ops::Div for Complex {
  type Output = Self;
  #[allow(clippy::suspicious_arithmetic_impl)]
  fn div(self, rhs: Self) -> Self::Output { self * rhs.inv() }
}
