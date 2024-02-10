use crate::bignum::{Complex, Rational};
use crate::checker::{OrUnsat, Unsat};
use crate::mk_id;
use crate::types::*;
use itertools::{EitherOrBoth, Itertools};
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone)]
pub struct Monomial<I> {
  /// invariant: not zero inside polynomial
  coeff: Complex,
  /// invariant: map does not contain zero powers
  powers: BTreeMap<I, u32>,
}

impl<I: Idx> std::fmt::Debug for Monomial<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut strs = vec![];
    if self.coeff == Complex::NEG_ONE {
      write!(f, "-")?
    } else if self.coeff != Complex::ONE {
      if self.coeff.im != Rational::ZERO {
        strs.push(format!("({})", self.coeff))
      } else {
        strs.push(format!("{}", self.coeff))
      }
    }
    for (&et, &k) in &self.powers {
      if k == 1 {
        strs.push(format!("x{et:?}"))
      } else {
        strs.push(format!("x{et:?}^{k}"))
      }
    }
    if strs.is_empty() {
      write!(f, "1")
    } else {
      write!(f, "{}", strs.iter().format("*"))
    }
  }
}

// This ignores the coefficients
impl<I: Idx> Ord for Monomial<I> {
  fn cmp(&self, other: &Self) -> Ordering {
    self.degree().cmp(&other.degree()).then_with(|| {
      self.powers.len().cmp(&other.powers.len()).then_with(|| self.powers.cmp(&other.powers))
    })
  }
}
impl<I: Idx> PartialOrd for Monomial<I> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}
impl<I: Idx> PartialEq for Monomial<I> {
  fn eq(&self, other: &Self) -> bool { self.cmp(other) == Ordering::Equal }
}
impl<I: Idx> Eq for Monomial<I> {}

impl<I: Idx> Monomial<I> {
  pub const fn cnst(coeff: Complex) -> Self { Self { coeff, powers: BTreeMap::new() } }
  pub fn coeff_atom(coeff: Complex, var: I) -> Self {
    Self { coeff, powers: std::iter::once((var, 1)).collect() }
  }
  pub fn atom(var: I) -> Self { Self::coeff_atom(Complex::ONE, var) }
  pub fn degree(&self) -> u32 { self.powers.iter().map(|p| p.1).sum() }

  fn mul(&self, other: &Self) -> Self {
    let coeff = self.coeff.clone() * other.coeff.clone();
    let mut powers = BTreeMap::new();
    for item in self.powers.iter().merge_join_by(&other.powers, |a, b| a.0.cmp(b.0)) {
      match item {
        EitherOrBoth::Left((&et, &n)) | EitherOrBoth::Right((&et, &n)) => powers.insert(et, n),
        EitherOrBoth::Both((&et, &n1), (_, &n2)) => powers.insert(et, n1.checked_add(n2).unwrap()),
      };
    }
    Monomial { coeff, powers }
  }

  fn lex_powers(a: &BTreeMap<I, u32>, b: &BTreeMap<I, u32>) -> Ordering {
    a.iter().map(|p| (p.0, !*p.1)).cmp(b.iter().map(|p| (p.0, !*p.1)))
  }

  fn lex(&self, other: &Self) -> Ordering { Self::lex_powers(&self.powers, &other.powers) }

  pub fn contains(&self, v: I) -> bool { self.powers.contains_key(&v) }

  pub fn is_var(&self) -> Option<I> {
    if self.powers.len() != 1 || self.coeff != Complex::ONE {
      return None
    }
    match self.powers.first_key_value() {
      Some((&et, 1)) => Some(et),
      _ => None,
    }
  }

  pub fn is_const(&self) -> Option<Complex> {
    if self.powers.is_empty() {
      Some(self.coeff.clone())
    } else {
      None
    }
  }

  pub fn pow(&mut self, n: u32) {
    match n {
      0 => *self = Monomial::cnst(Complex::ONE),
      1 => {}
      _ => {
        self.coeff = std::mem::take(&mut self.coeff).pow(n.into());
        for i in self.powers.values_mut() {
          *i = i.checked_mul(n).unwrap();
        }
      }
    }
  }
}

#[derive(Clone)]
pub struct Polynomial<I>(
  /// sorted by Monomial::lex
  Vec<Monomial<I>>,
);

impl<I: Idx> std::fmt::Debug for Polynomial<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if self.0.is_empty() {
      write!(f, "poly 0")
    } else {
      write!(f, "poly {:?}", self.0.iter().format(" + "))
    }
  }
}

impl<I: Idx> Ord for Polynomial<I> {
  fn cmp(&self, other: &Self) -> Ordering { self.fcmp(other, |a, b| a.cmp(b)) }
}
impl<I: Idx> PartialOrd for Polynomial<I> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}
impl<I: Idx> PartialEq for Polynomial<I> {
  fn eq(&self, other: &Self) -> bool { self.cmp(other) == Ordering::Equal }
}
impl<I: Idx> Eq for Polynomial<I> {}

impl<I: Idx> std::ops::Add for Polynomial<I> {
  type Output = Self;
  fn add(self, other: Self) -> Self {
    let mut out = Polynomial::ZERO;
    for item in self.0.into_iter().merge_join_by(other.0, Monomial::lex) {
      match item {
        EitherOrBoth::Left(mon) | EitherOrBoth::Right(mon) => out.0.push(mon),
        EitherOrBoth::Both(mut m1, m2) => {
          m1.coeff += m2.coeff;
          if m1.coeff != Complex::ZERO {
            out.0.push(m1)
          }
        }
      }
    }
    out
  }
}

impl<I: Idx> std::ops::Sub for Polynomial<I> {
  type Output = Self;
  fn sub(self, other: Self) -> Self { self + other * &Complex::NEG_ONE }
}

impl<I: Idx> std::ops::Mul<&Complex> for Polynomial<I> {
  type Output = Self;
  fn mul(mut self, other: &Complex) -> Self {
    if *other == Complex::ZERO {
      return Polynomial::ZERO
    }
    if *other == Complex::ONE {
      return self
    }
    for mon in &mut self.0 {
      mon.coeff = std::mem::take(&mut mon.coeff) * other.clone()
    }
    self
  }
}

impl<I: Idx> std::ops::Mul for &Polynomial<I> {
  type Output = Polynomial<I>;
  fn mul(self, other: Self) -> Polynomial<I> {
    if self.is_zero() || other.is_zero() {
      return Polynomial::ZERO
    }
    let mut out = Polynomial::ZERO;
    for i in &self.0 {
      for j in &other.0 {
        let mon = i.mul(j);
        out.0.push(mon)
      }
    }
    out.0.sort_unstable_by(Monomial::lex);
    out.dedup();
    out
  }
}

impl<I: Idx> std::ops::Mul<&Monomial<I>> for Polynomial<I> {
  type Output = Self;
  fn mul(mut self, other: &Monomial<I>) -> Self {
    if other.coeff == Complex::ZERO {
      return Polynomial::ZERO
    }
    if other.is_const() == Some(Complex::ONE) {
      return self
    }
    for mon in &mut self.0 {
      mon.coeff = std::mem::take(&mut mon.coeff) * other.coeff.clone();
      for (&et, &i) in &other.powers {
        let v = mon.powers.entry(et).or_default();
        *v = v.checked_add(i).unwrap();
      }
    }
    self.0.sort_unstable_by(Monomial::lex);
    self
  }
}

impl<I> Polynomial<I> {
  pub const ZERO: Self = Self(Vec::new());

  pub fn single(mon: Monomial<I>) -> Self {
    if mon.coeff == Complex::ZERO {
      Self(Vec::new())
    } else {
      Self(vec![mon])
    }
  }
}

impl<I: Idx> Polynomial<I> {
  fn fcmp(&self, other: &Self, f: impl Fn(&Monomial<I>, &Monomial<I>) -> Ordering) -> Ordering {
    self.0.len().cmp(&other.0.len()).then_with(|| {
      for (a, b) in self.0.iter().zip(&other.0) {
        match f(a, b).then_with(|| a.coeff.cmp(&b.coeff)) {
          Ordering::Equal => {}
          non_eq => return non_eq,
        }
      }
      Ordering::Equal
    })
  }

  pub fn is_zero(&self) -> bool { self.0.is_empty() }

  fn dedup(&mut self) {
    let mut it = std::mem::take(&mut self.0).into_iter();
    if let Some(mut mon) = it.next() {
      for m2 in it {
        if mon == m2 {
          mon.coeff += m2.coeff;
        } else {
          if mon.coeff != Complex::ZERO {
            self.0.push(mon)
          }
          mon = m2;
        }
      }
      if mon.coeff != Complex::ZERO {
        self.0.push(mon)
      }
    }
  }

  pub fn is_var(&self) -> Option<I> {
    match *self.0 {
      [ref mon] => mon.is_var(),
      _ => None,
    }
  }

  /// Returns `Some(v)` if the polynomial is a power of `v`
  pub fn is_var_power(&self) -> Option<I> {
    match &*self.0 {
      [mon] if mon.powers.len() == 1 => Some(*mon.powers.first_key_value()?.0),
      _ => None,
    }
  }

  pub fn is_const(&self) -> Option<Complex> {
    match *self.0 {
      [] => Some(Complex::ZERO),
      [ref mon] => mon.is_const(),
      _ => None,
    }
  }

  pub fn contains(&self, v: I) -> bool { self.0.iter().any(|mon| mon.contains(v)) }

  pub fn pow(&self, n: u32) -> Self {
    match n {
      0 => Polynomial::single(Monomial::cnst(Complex::ONE)),
      1 => self.clone(),
      _ => match *self.0 {
        [] => Polynomial::ZERO,
        [ref mon] => {
          let mut mon = mon.clone();
          mon.pow(n);
          Polynomial(vec![mon])
        }
        _ => {
          let mut out = self.clone();
          for i in (0..n.ilog2()).rev() {
            out = &out * &out;
            if (n >> i) & 1 != 0 {
              out = &out * self;
            }
          }
          out
        }
      },
    }
  }

  /// computes self = self[v |-> p]
  pub fn subst(&mut self, v: I, p: &Self) {
    for mut mon in std::mem::take(&mut self.0) {
      if let Some(n) = mon.powers.remove(&v) {
        if !p.is_zero() {
          self.0.append(&mut (p.pow(n) * &mon).0);
        }
      } else {
        self.0.push(mon)
      }
    }
    self.0.sort_by(Monomial::lex);
    self.dedup()
  }
}

#[derive(Clone, Default)]
pub struct LinVar<I> {
  coeff: Complex,
  var: I,
}

// This ignores the coefficients
impl<I: Idx> Ord for LinVar<I> {
  fn cmp(&self, other: &Self) -> Ordering { self.var.cmp(&other.var) }
}
impl<I: Idx> PartialOrd for LinVar<I> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}
impl<I: Idx> PartialEq for LinVar<I> {
  fn eq(&self, other: &Self) -> bool { self.cmp(other) == Ordering::Equal }
}
impl<I: Idx> Eq for LinVar<I> {}

impl<I: Idx> std::fmt::Debug for LinVar<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if self.coeff == Complex::NEG_ONE {
      write!(f, "-")?
    } else if self.coeff != Complex::ONE {
      if self.coeff.im != Rational::ZERO {
        write!(f, "({})*", self.coeff)?
      } else {
        write!(f, "{}*", self.coeff)?
      }
    }
    write!(f, "m{:?}", self.var)
  }
}

#[derive(Clone)]
pub struct LinPoly<I> {
  cnst: Complex,
  /// sorted
  terms: Vec<LinVar<I>>,
}
impl<I> Default for LinPoly<I> {
  fn default() -> Self { Self::ZERO }
}

impl<I: Idx> std::fmt::Debug for LinPoly<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "lin ")?;
    for v in &self.terms {
      write!(f, "{v:?} + ")?
    }
    write!(f, "{:?}", self.cnst)
  }
}

impl<I: Idx> Ord for LinPoly<I> {
  fn cmp(&self, other: &Self) -> Ordering {
    for (a, b) in self.terms.iter().zip(&other.terms) {
      match a.var.cmp(&b.var).then_with(|| a.coeff.cmp(&b.coeff)) {
        Ordering::Equal => {}
        non_eq => return non_eq,
      }
    }
    self.terms.len().cmp(&other.terms.len()).then_with(|| self.cnst.cmp(&other.cnst))
  }
}
impl<I: Idx> PartialOrd for LinPoly<I> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}
impl<I: Idx> PartialEq for LinPoly<I> {
  fn eq(&self, other: &Self) -> bool { self.cmp(other) == Ordering::Equal }
}
impl<I: Idx> Eq for LinPoly<I> {}

impl<I> std::ops::MulAssign<Complex> for LinPoly<I> {
  fn mul_assign(&mut self, rhs: Complex) {
    if rhs != Complex::ONE {
      for v in &mut self.terms {
        v.coeff *= rhs.clone();
      }
      self.cnst *= rhs
    }
  }
}

impl<I> std::ops::Mul<Complex> for LinPoly<I> {
  type Output = Self;
  fn mul(mut self, rhs: Complex) -> Self {
    self *= rhs;
    self
  }
}

impl<I: Idx> std::ops::Add for LinPoly<I> {
  type Output = Self;
  fn add(self, other: Self) -> Self {
    let mut out = LinPoly::cnst(self.cnst + other.cnst);
    for item in self.terms.into_iter().merge_join_by(other.terms, Ord::cmp) {
      match item {
        EitherOrBoth::Left(mon) | EitherOrBoth::Right(mon) => out.terms.push(mon),
        EitherOrBoth::Both(mut m1, m2) => {
          m1.coeff += m2.coeff;
          if m1.coeff != Complex::ZERO {
            out.terms.push(m1)
          }
        }
      }
    }
    out
  }
}

impl<I: Idx> std::ops::Sub for LinPoly<I> {
  type Output = Self;
  fn sub(self, other: Self) -> Self { self + other * Complex::NEG_ONE }
}
impl<I: Idx> std::ops::SubAssign for LinPoly<I> {
  fn sub_assign(&mut self, rhs: Self) { *self = std::mem::take(self) - rhs }
}

impl<I> LinPoly<I> {
  const fn cnst(cnst: Complex) -> Self { Self { cnst, terms: vec![] } }
  const ZERO: Self = Self::cnst(Complex::ZERO);
}
impl<I: Idx> LinPoly<I> {
  fn dedup(&mut self) {
    let mut it = std::mem::take(&mut self.terms).into_iter();
    if let Some(mut mon) = it.next() {
      for m2 in it {
        if mon == m2 {
          mon.coeff += m2.coeff;
        } else {
          if mon.coeff != Complex::ZERO {
            self.terms.push(mon)
          }
          mon = m2;
        }
      }
      if mon.coeff != Complex::ZERO {
        self.terms.push(mon)
      }
    }
  }
}

mk_id! { MonomialId(u32), }

pub(super) fn gaussian_elimination<I: Idx>(
  vars: &mut SortedIdxVec<MonomialId, BTreeMap<I, u32>>, polys: BTreeSet<Polynomial<I>>,
) -> OrUnsat<Vec<LinPoly<MonomialId>>> {
  if polys.is_empty() {
    return Ok(vec![])
  }
  let mut eqs = BTreeSet::new();
  for p in polys {
    let mut eq = LinPoly::ZERO;
    for mon in p.0 {
      if mon.powers.is_empty() {
        eq.cnst += mon.coeff
      } else {
        let var = vars
          .find_index(|m| Monomial::lex_powers(m, &mon.powers))
          .unwrap_or_else(|idx| vars.insert_at(idx, mon.powers.clone()));
        eq.terms.push(LinVar { coeff: mon.coeff, var });
      }
    }
    eq.terms.sort_unstable();
    eq.dedup();
    if let [v, ..] = &*eq.terms {
      eq *= v.coeff.clone().inv();
      eqs.insert(eq);
    } else if eq.cnst != Complex::ZERO {
      return Err(Unsat)
    }
  }

  // GaussElimination
  if eqs.len() <= 1 {
    return Ok(eqs.into_iter().collect())
  }
  let mut eqs2 = vec![];
  while let Some(eq1) = eqs.pop_first() {
    let v1 = eq1.terms[0].var;
    while matches!(eqs.first(), Some(eq2) if eq2.terms[0].var == v1) {
      let mut eq = eq1.clone() - eqs.pop_first().unwrap();
      if let [v, ..] = &*eq.terms {
        eq *= v.coeff.clone().inv();
        eqs.insert(eq);
      } else if eq.cnst != Complex::ZERO {
        return Err(Unsat)
      }
    }
    eqs2.push(eq1);
  }
  for i in 1..eqs2.len() {
    let [lo @ .., eq1] = &mut eqs2[..=i] else { unreachable!() };
    let v1 = eq1.terms[0].var;
    for eq2 in lo {
      if let Some(lv) = eq2.terms.iter().find(|lv| v1 <= lv.var) {
        if v1 == lv.var {
          let eq1 = eq1.clone() * lv.coeff.clone();
          *eq2 = std::mem::take(eq2) - eq1;
        }
      }
    }
  }
  let mut unsat = false;
  eqs2.retain_mut(|eq| {
    !eq.terms.is_empty() || {
      unsat |= eq.cnst != Complex::ZERO;
      false
    }
  });
  if unsat {
    return Err(Unsat)
  }
  Ok(eqs2)
}

impl<I: Idx> Polynomial<I> {
  pub(super) fn reduce<J: Idx>(
    self, vars: &SortedIdxVec<J, BTreeMap<I, u32>>, eqs: &[LinPoly<J>],
  ) -> bool {
    let mut eq = LinPoly::ZERO;
    for mon in self.0 {
      if mon.powers.is_empty() {
        eq.cnst += mon.coeff
      } else if let Ok(var) = vars.find_index(|m| Monomial::lex_powers(m, &mon.powers)) {
        eq.terms.push(LinVar { coeff: mon.coeff, var })
      } else {
        return false
      }
    }
    eq.terms.sort_unstable();
    eq.dedup();

    // LinearEquationReduce
    let mut it = eqs.iter();
    'next: loop {
      let [v1, ..] = &*eq.terms else { return eq.cnst == Complex::ZERO };
      for eq2 in it.by_ref() {
        if v1.var == eq2.terms[0].var {
          eq -= eq2.clone() * v1.coeff.clone();
          continue 'next
        }
      }
      return false
    }
  }
}
