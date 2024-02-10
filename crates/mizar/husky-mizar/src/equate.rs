use self::polynomial::{Monomial, Polynomial};
use crate::bignum::{Complex, Rational};
use crate::checker::{Atoms, Checker, Conjunct, Dnf, OrUnsat, Unsat};
use crate::types::*;
use crate::{
  vprintln, CheckBound, CmpStyle, EqCtx, Equate, ExpandPrivFunc, Global, Inst, LocalContext,
  OnVarMut, Visit, VisitMut, WithGlobalLocal,
};
use enum_map::EnumMap;
use itertools::Itertools;
use std::borrow::Cow;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};

mod polynomial;

pub struct EqTerm {
  pub id: EqClassId,
  /// Term is EqMark(mark)
  pub mark: EqMarkId,
  pub eq_class: Vec<EqMarkId>,
  pub ty_class: Vec<Type>,
  pub supercluster: Attrs,
  pub number: Option<Complex>,
  pub eq_polys: BTreeSet<Polynomial<EqTermId>>,
}

impl std::fmt::Debug for EqTerm {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?} = ", Term::EqMark(self.mark))?;
    f.debug_list()
      .entries(self.eq_class.iter().map(|&m| Term::EqMark(m)))
      .entries(&self.eq_polys)
      .finish()?;
    if let Some(n) = &self.number {
      write!(f, " = {n}")?
    }
    write!(f, ": {:?}{:?}", &self.supercluster, &self.ty_class)
  }
}

#[derive(Default)]
struct ConstrMap<I>(BTreeMap<I, Vec<EqMarkId>>);

impl<I: Idx> ConstrMap<I> {
  fn insert(&mut self, nr: I, mark: EqMarkId) { self.0.entry(nr).or_default().push(mark) }

  fn find(&self, g: &Global, lc: &LocalContext, nr: I, args: &[Term]) -> Option<EqMarkId> {
    let entry = self.0.get(&nr)?;
    entry.iter().copied().find(|&m| g.eq(lc, args, lc.marks[m].0.args().unwrap()))
  }
}

impl Attrs {
  fn try_attrs(&self) -> OrUnsat<&[Attr]> {
    match self {
      Attrs::Inconsistent => Err(Unsat),
      Attrs::Consistent(attrs) => Ok(attrs),
    }
  }
  fn try_insert(&mut self, ctx: &Constructors, lc: &LocalContext, item: Attr) -> OrUnsat<bool> {
    // vprintln!("insert {item:?} -> {self:?}");
    let changed = self.insert(Some(ctx), lc, item);
    self.try_attrs()?;
    Ok(changed)
  }
}

struct AllowedClusters {
  ccl: Vec<(usize, Attrs)>,
  fcl: Vec<(usize, Attrs)>,
}

#[derive(Default)]
struct ConstrMaps {
  functor: ConstrMap<FuncId>,
  aggregate: ConstrMap<AggrId>,
  selector: ConstrMap<SelId>,
  priv_func: ConstrMap<PrivFuncId>,
  sch_func: ConstrMap<SchFuncId>,
  choice: Vec<EqMarkId>,
  fraenkel: Vec<EqMarkId>,
}

pub struct Equalizer<'a> {
  pub g: &'a Global,
  pub lc: &'a mut LocalContext,
  reductions: &'a [Reduction],
  infers: IdxVec<InferId, Option<EqMarkId>>,
  constrs: ConstrMaps,
  /// TrmS
  pub terms: IdxVec<EqTermId, EqTerm>,
  pub next_eq_class: EqClassId,
  clash: bool,
}
impl WithGlobalLocal for Equalizer<'_> {
  fn global(&self) -> &Global { self.g }
  fn local(&self) -> &LocalContext { self.lc }
}

struct CheckE<'a> {
  marks: &'a IdxVec<EqMarkId, (Term, EqTermId)>,
  found: bool,
}

impl<'a> CheckE<'a> {
  fn with(marks: &'a IdxVec<EqMarkId, (Term, EqTermId)>, f: impl FnOnce(&mut CheckE<'a>)) -> bool {
    let mut ce = CheckE { marks, found: false };
    f(&mut ce);
    ce.found
  }
}

impl Visit for CheckE<'_> {
  fn abort(&self) -> bool { self.found }
  fn visit_term(&mut self, tm: &Term) {
    match *tm {
      Term::EqClass(_) => self.found = true,
      Term::EqMark(m) if matches!(self.marks[m].0, Term::EqClass(_)) => self.found = true,
      _ => self.super_visit_term(tm),
    }
  }
}

struct EqMarks;

impl Equate for EqMarks {
  const IGNORE_MARKS: bool = false;
  fn eq_pred(
    &mut self, ctx: &mut EqCtx<'_>, n1: PredId, n2: PredId, args1: &[Term], args2: &[Term],
  ) -> bool {
    let (n1_adj, args1_adj) = Formula::adjust_pred(n1, args1, Some(&ctx.g.constrs));
    let (n2_adj, args2_adj) = Formula::adjust_pred(n2, args2, Some(&ctx.g.constrs));
    n1_adj == n2_adj
      && (self.eq_terms(ctx, args1_adj, args2_adj)
        || {
          let c = &ctx.g.constrs.predicate[n1];
          c.properties.get(PropertyKind::Symmetry) && {
            let mut args1 = args1.iter().collect_vec();
            args1.swap(c.properties.arg1 as usize, c.properties.arg2 as usize);
            args1[c.superfluous as usize..]
              .iter()
              .zip(args2_adj)
              .all(|(&t1, t2)| self.eq_term(ctx, t1, t2))
          }
        }
        || {
          let c = &ctx.g.constrs.predicate[n2];
          c.properties.get(PropertyKind::Symmetry) && {
            let mut args2 = args2.iter().collect_vec();
            args2.swap(c.properties.arg1 as usize, c.properties.arg2 as usize);
            args1_adj
              .iter()
              .zip(&args2[c.superfluous as usize..])
              .all(|(t1, &t2)| self.eq_term(ctx, t1, t2))
          }
        })
  }

  // EqMarks.eq_term: EqTrms
  // EqMarks.eq_formula: EqFrms
}

impl Term {
  pub fn mark(&self) -> Option<EqMarkId> {
    match *self {
      Term::EqMark(m) => Some(m),
      _ => None,
    }
  }

  pub fn unmark<'a>(&'a self, lc: &'a LocalContext) -> &'a Term {
    match *self {
      Term::EqMark(m) => &lc.marks[m].0,
      _ => self,
    }
  }

  pub fn class(&self) -> Option<EqClassId> {
    match *self {
      Term::EqClass(ec) => Some(ec),
      _ => None,
    }
  }
}

impl Equalizer<'_> {
  /// YEqClass
  fn new_eq_class(&mut self, tm: &mut Term) -> (EqMarkId, EqTermId) {
    let id = self.next_eq_class.fresh();
    // vprintln!("new_eq_class e{id:?}: {tm:?}");
    let et = self.terms.push(EqTerm {
      id,
      mark: Default::default(),
      eq_class: vec![],
      ty_class: vec![Type::ANY],
      supercluster: Attrs::default(),
      number: None,
      eq_polys: Default::default(),
    });
    let m = self.lc.marks.push((std::mem::take(tm), et));
    *tm = Term::EqMark(m);
    self.terms[et].mark = self.lc.marks.push((Term::EqClass(id), et));
    self.terms[et].eq_class.push(m);
    (m, et)
  }

  fn insert_type(&mut self, mut new: Type, et: EqTermId) -> OrUnsat<bool> {
    self.y(|y| y.visit_type(&mut new))?;
    let mut eq_term = &mut self.terms[et];
    // vprintln!("insert type e{:?}: {new:?}", eq_term.id);
    let mut i;
    let mut added = false;
    loop {
      if (eq_term.ty_class.iter())
        .any(|old| old.kind == new.kind && self.g.eq(self.lc, &*old.args, &new.args))
      {
        if !new.attrs.1.is_subset_of(&eq_term.supercluster, |a1, a2| self.g.eq(self.lc, a1, a2)) {
          for attr in new.attrs.1.try_attrs().unwrap() {
            added |= eq_term.supercluster.try_insert(&self.g.constrs, self.lc, attr.clone())?;
          }
        }
        return Ok(added)
      }
      self.y(|y| y.visit_type(&mut new))?; // is this okay? we already visited it
      let Attrs::Consistent(attrs) = std::mem::take(&mut new.attrs).1 else { unreachable!() };
      eq_term = &mut self.terms[et];
      for attr in attrs {
        eq_term.supercluster.try_insert(&self.g.constrs, self.lc, attr)?;
      }
      if matches!(new.kind, TypeKind::Mode(_)) {
        if let Some(new2) = new.widening(self.g, self.lc) {
          eq_term.ty_class.push(std::mem::replace(&mut new, *new2));
          added = true;
          continue
        }
      }
      i = eq_term.ty_class.len();
      eq_term.ty_class.push(new);
      break
    }
    if let TypeKind::Struct(mut m) = eq_term.ty_class[i].kind {
      loop {
        let parents = self.g.constrs.struct_mode[m].parents.clone();
        for mut ty in parents.into_vec() {
          ty.visit(&mut Inst::new(&self.g.constrs, self.lc, &eq_term.ty_class[i].args, 0));
          self.y(|y| y.visit_type(&mut ty))?;
          eq_term = &mut self.terms[et];
          ty.attrs = Default::default();
          if !(eq_term.ty_class.iter())
            .any(|old| old.kind == ty.kind && EqMarks.eq(self.g, self.lc, &*old.args, &ty.args))
          {
            eq_term.ty_class.push(ty)
          }
        }
        i += 1;
        let Some(new) = eq_term.ty_class.get(i) else { return Ok(true) };
        let TypeKind::Struct(m2) = new.kind else { unreachable!() };
        m = m2;
      }
    }
    Ok(true)
  }
}

/// Not sure why it's called this but all the mizar functions here
/// are called YSomething so there it is.
struct Y<'a, 'b> {
  eq: &'b mut Equalizer<'a>,
  unsat: OrUnsat<()>,
  depth: u32,
}
impl<'a, 'b> std::ops::Deref for Y<'a, 'b> {
  type Target = &'b mut Equalizer<'a>;
  fn deref(&self) -> &Self::Target { &self.eq }
}
impl<'a, 'b> std::ops::DerefMut for Y<'a, 'b> {
  fn deref_mut(&mut self) -> &mut Self::Target { &mut self.eq }
}

impl<'a> Equalizer<'a> {
  fn y<'b, R>(&'b mut self, f: impl FnOnce(&mut Y<'a, 'b>) -> R) -> OrUnsat<R> {
    let mut y = Y { eq: self, unsat: Ok(()), depth: 0 };
    let r = f(&mut y);
    y.unsat?;
    Ok(r)
  }

  fn prep_binder(
    &mut self, tm: &mut Term, depth: u32, coll: fn(&mut ConstrMaps) -> &mut Vec<EqMarkId>,
  ) -> Option<Result<EqTermId, usize>> {
    if CheckBound::get(0..depth, |cb| cb.visit_term(tm)) {
      return None
    }
    OnVarMut(|n| *n -= depth).visit_term(tm);
    let vec = coll(&mut self.constrs);
    match vec.binary_search_by(|&m| {
      self.lc.marks[m].0.cmp(Some(&self.g.constrs), Some(self.lc), tm, CmpStyle::Red)
    }) {
      Ok(i) => Some(Ok(self.lc.marks[vec[i]].1)),
      Err(i) => Some(Err(i)),
    }
  }
}

macro_rules! y_try {
  ($self:expr, $e:expr) => {
    match $e {
      Ok(e) => e,
      Err(Unsat) => {
        $self.unsat = Err(Unsat);
        return
      }
    }
  };
}

impl<'a, 'b> Y<'a, 'b> {
  fn visit_args(&mut self, tms: &mut [Term]) -> bool {
    self.visit_terms(tms);
    tms.iter().all(|tm| matches!(tm, Term::EqMark(_)))
  }

  fn add_binder_into(
    &mut self, tm: &mut Term, coll: fn(&mut ConstrMaps) -> &mut Vec<EqMarkId>,
  ) -> Option<EqTermId> {
    let depth = self.depth;
    match self.prep_binder(tm, depth, coll)? {
      Ok(i) => {
        *tm = Term::EqMark(self.terms[i].mark);
        None
      }
      Err(i) => {
        let (m, et) = self.new_eq_class(tm);
        coll(&mut self.constrs).insert(i, m);
        Some(et)
      }
    }
  }
}

impl VisitMut for Y<'_, '_> {
  fn abort(&self) -> bool { self.unsat.is_err() }
  fn push_bound(&mut self, _: &mut Type) { self.depth += 1 }
  fn pop_bound(&mut self, n: u32) { self.depth -= n }

  /// YTerm
  fn visit_term(&mut self, tm: &mut Term) {
    if self.abort() {
      return
    }
    // vprintln!("y term <- {tm:?}");
    let et = match *tm {
      Term::Bound(_) | Term::EqClass(_) => return,
      Term::Infer(nr) => {
        if let Some(&Some(em)) = self.infers.get(nr) {
          *tm = Term::EqMark(em);
        } else {
          let et = self.new_eq_class(tm).1;
          *self.eq.infers.get_mut_extending(nr) = Some(self.eq.terms[et].mark);
          let ty = {
            let ic = self.eq.lc.infer_const.borrow();
            self.eq.terms[et].number = ic[nr].number.clone();
            ic[nr].ty.visit_cloned(&mut ExpandPrivFunc(&self.eq.g.constrs, self.eq.lc))
          };
          y_try!(self, self.insert_type(ty, et));
          *tm = Term::EqMark(self.terms[et].mark);
        }
        return
      }
      Term::Functor { nr, ref mut args } => {
        let orig = args.clone();
        if !self.visit_args(args) {
          return
        }
        let mut args1 = args.clone();
        let ty = if CheckE::with(&self.lc.marks, |ce| ce.visit_terms(&orig)) {
          Term::Functor { nr, args: orig }.get_type_uncached(self.g, self.lc)
        } else {
          *Term::Functor { nr, args: orig }.round_up_type(self.g, self.lc, true).to_owned()
        };
        let (nr2, args2) = Term::adjust(nr, args, Some(&self.g.constrs));
        if let Some(m) = self.constrs.functor.find(self.g, self.lc, nr2, args2) {
          *tm = Term::EqMark(self.terms[self.lc.marks[m].1].mark);
          return
        }
        *tm = Term::Functor { nr: nr2, args: args2.to_vec().into() };
        let (m, et) = self.new_eq_class(tm);
        self.constrs.functor.insert(nr2, m);
        y_try!(self, self.insert_type(ty, et));
        if self.g.reqs.zero_number() == Some(Term::adjusted_nr(nr2, &self.g.constrs)) {
          self.terms[et].number = Some(Complex::ZERO);
        }
        let constr = &self.g.constrs.functor[nr];
        if constr.properties.get(PropertyKind::Commutativity) {
          args1.swap(constr.properties.arg1 as usize, constr.properties.arg2 as usize);
          let (nr3, comm_args) = Term::adjust(nr, &args1, Some(&self.g.constrs));
          let m =
            self.lc.marks.push((Term::Functor { nr: nr3, args: comm_args.to_vec().into() }, et));
          self.terms[et].eq_class.push(m);
          self.constrs.functor.insert(nr3, m)
        }
        *tm = Term::EqMark(self.terms[et].mark);
        return
      }
      Term::SchFunc { ref mut args, .. } => {
        if !self.visit_args(args) {
          return
        }
        self.new_eq_class(tm).1
      }
      Term::PrivFunc { nr, ref mut args, .. } => {
        if !self.visit_args(args) {
          return
        }
        let (m, et) = self.new_eq_class(tm);
        self.constrs.priv_func.insert(nr, m);
        et
      }
      Term::Aggregate { nr, ref mut args, .. } => {
        if !self.visit_args(args) {
          return
        }
        if let Some(m) = self.constrs.aggregate.find(self.g, self.lc, nr, args) {
          *tm = Term::EqMark(self.terms[self.lc.marks[m].1].mark);
          return
        }
        let (m, et) = self.new_eq_class(tm);
        self.constrs.aggregate.insert(nr, m);
        et
      }
      Term::Selector { nr, ref mut args, .. } => {
        if !self.visit_args(args) {
          return
        }
        if let Some(m) = self.constrs.selector.find(self.g, self.lc, nr, args) {
          *tm = Term::EqMark(self.terms[self.lc.marks[m].1].mark);
          return
        }
        let (m, et) = self.new_eq_class(tm);
        self.constrs.selector.insert(nr, m);
        et
      }
      Term::Fraenkel { ref mut args, ref mut scope, ref mut compr } => {
        for ty in &mut **args {
          self.visit_type(ty);
          self.push_bound(ty);
        }
        self.visit_term(scope);
        self.visit_formula(compr);
        self.pop_bound(args.len() as u32);
        let Some(et) = self.add_binder_into(tm, |c| &mut c.fraenkel) else { return };
        et
      }
      Term::The { ref mut ty } => {
        self.visit_type(ty);
        let Some(et) = self.add_binder_into(tm, |c| &mut c.choice) else { return };
        et
      }
      Term::EqMark(m) => match self.lc.marks[m].0 {
        Term::Bound(_) | Term::EqClass(_) => return,
        _ => unreachable!("already marked"),
      },
      Term::Locus(_)
      | Term::Const(_)
      | Term::FreeVar(_)
      | Term::Numeral(_)
      | Term::Qua { .. }
      | Term::It => unreachable!("{tm:?}"),
    };
    let ty = tm.get_type_uncached(self.g, self.lc);
    y_try!(self, self.insert_type(ty, et));
    *tm = Term::EqMark(self.terms[et].mark);
    // vprintln!("y term -> {tm:?} -> {:?}", tm.mark().map(|m| &self.lc.marks[m]));
  }
}

impl Equalizer<'_> {
  fn yy_binder(
    &mut self, mut term: Term, fi: EqTermId, coll: fn(&mut ConstrMaps) -> &mut Vec<EqMarkId>,
  ) -> EqTermId {
    match self.prep_binder(&mut term, 0, coll) {
      None => fi,
      Some(Ok(et)) => et,
      Some(Err(i)) => {
        let et = self.lc.marks[self.terms[fi].mark].1;
        let m = self.lc.marks.push((term, fi));
        self.terms[et].eq_class.push(m);
        coll(&mut self.constrs).insert(i, m);
        fi
      }
    }
  }

  /// YYTerm(fTrm = term, fi = fi)
  fn yy_term(&mut self, mut term: Term, fi: EqTermId) -> OrUnsat<EqTermId> {
    // vprintln!("yy term {term:?} <- {:?}", self.terms[fi]);
    macro_rules! func_like {
      ($k:ident: $nr:expr, $args:expr) => {{
        self.y(|y| y.visit_terms($args))?;
        if let Some(m) = self.constrs.$k.find(self.g, self.lc, $nr, $args) {
          return Ok(self.lc.marks[m].1)
        }
        let et = self.lc.marks[self.terms[fi].mark].1;
        let m = self.lc.marks.push((term, fi));
        self.terms[et].eq_class.push(m);
        self.constrs.$k.insert($nr, m);
        Ok(fi)
      }};
    }
    match term {
      Term::Numeral(n) => {
        let c = n.into();
        for etm in self.terms.0.iter() {
          if !etm.eq_class.is_empty() && etm.number.as_ref() == Some(&c) {
            return Ok(self.lc.marks[etm.mark].1)
          }
        }
        let et = self.lc.marks[self.terms[fi].mark].1;
        if matches!(self.terms[et].number.replace(c.clone()), Some(c2) if c != c2) {
          return Err(Unsat)
        }
        Ok(fi)
      }
      Term::Functor { nr, ref mut args } => {
        self.y(|y| y.visit_terms(args))?;
        let c = &self.g.constrs.functor[nr];
        let (nr1, args1) = Term::adjust(nr, args, Some(&self.g.constrs));
        if let Some(m) = self.constrs.functor.find(self.g, self.lc, nr1, args1) {
          return Ok(self.lc.marks[m].1)
        }
        let comm_args = if c.properties.get(PropertyKind::Commutativity) {
          let mut args = args.clone();
          args.swap(c.properties.arg1 as usize, c.properties.arg2 as usize);
          if let Some(m) = self.constrs.functor.find(self.g, self.lc, nr1, &args) {
            return Ok(self.lc.marks[m].1)
          }
          Some(args)
        } else {
          None
        };
        let et = self.lc.marks[self.terms[fi].mark].1;
        match self.g.reqs.rev.get(nr1) {
          Some(Some(Requirement::ZeroNumber)) => self.terms[et].number = Some(Complex::ZERO),
          Some(Some(Requirement::ImaginaryUnit)) => self.terms[et].number = Some(Complex::I),
          _ => {}
        }
        let m = self.lc.marks.push((Term::Functor { nr: nr1, args: args1.to_vec().into() }, fi));
        self.constrs.functor.insert(nr1, m);
        self.terms[et].eq_class.push(m);
        if let Some(args) = comm_args {
          let (nr2, args2) = Term::adjust(nr, &args, Some(&self.g.constrs));
          let m = self.lc.marks.push((Term::Functor { nr: nr2, args: args2.to_vec().into() }, fi));
          self.constrs.functor.insert(nr2, m);
          self.terms[et].eq_class.push(m);
        }
        Ok(fi)
      }
      Term::SchFunc { nr, ref mut args } => func_like!(sch_func: nr, args),
      Term::PrivFunc { nr, ref mut args, .. } => func_like!(priv_func: nr, args),
      Term::Selector { nr, ref mut args } => func_like!(selector: nr, args),
      Term::Aggregate { nr, ref mut args } => {
        self.y(|y| y.visit_terms(args))?;
        if let Some(vec) = self.constrs.aggregate.0.get(&nr) {
          let base = self.g.constrs.aggregate[nr].base as usize;
          let args = &args[base..];
          for &m in vec {
            if self.eq(args, &self.lc.marks[m].0.args().unwrap()[base..]) {
              return Ok(self.lc.marks[m].1)
            }
          }
        }
        let et = self.lc.marks[self.terms[fi].mark].1;
        let m = self.lc.marks.push((term, fi));
        self.terms[et].eq_class.push(m);
        self.constrs.aggregate.insert(nr, m);
        Ok(fi)
      }
      Term::Fraenkel { ref mut args, ref mut scope, ref mut compr } => {
        self.y(move |y| {
          for ty in &mut **args {
            y.visit_type(ty);
            y.push_bound(ty);
          }
          y.visit_term(scope);
          y.visit_formula(compr);
          y.pop_bound(args.len() as u32);
        })?;
        Ok(self.yy_binder(term, fi, |c| &mut c.fraenkel))
      }
      Term::The { ref mut ty } => {
        self.y(|y| y.visit_type(ty))?;
        Ok(self.yy_binder(term, fi, |c| &mut c.choice))
      }
      Term::Infer(_) | Term::Const(_) => Ok(fi),
      Term::Locus(_)
      | Term::Bound(_)
      | Term::EqClass(_)
      | Term::EqMark(_)
      | Term::FreeVar(_)
      | Term::Qua { .. }
      | Term::It => unreachable!(),
    }
  }
}

#[derive(Debug, Default)]
struct Equals(BTreeSet<(EqTermId, EqTermId)>);

impl Equals {
  #[inline]
  fn insert(&mut self, et1: EqTermId, et2: EqTermId) {
    match et1.cmp(&et2) {
      Ordering::Less => self.0.insert((et1, et2)),
      Ordering::Equal => false,
      Ordering::Greater => self.0.insert((et2, et1)),
    };
  }
}

struct HasInfer<'a> {
  infers: &'a IdxVec<InferId, Option<EqMarkId>>,
  found: bool,
}
impl<'a> HasInfer<'a> {
  pub fn get(infers: &'a IdxVec<InferId, Option<EqMarkId>>, f: impl FnOnce(&mut Self)) -> bool {
    let mut cb = Self { infers, found: false };
    f(&mut cb);
    cb.found
  }
}
impl Visit for HasInfer<'_> {
  fn abort(&self) -> bool { self.found }
  fn visit_term(&mut self, tm: &Term) {
    match *tm {
      Term::Infer(n) => self.found |= self.infers.get(n).map_or(true, |i| i.is_none()),
      _ => self.super_visit_term(tm),
    }
  }
}

impl Attr {
  fn is_strict(&self, ctx: &Constructors) -> bool {
    self.pos && ctx.attribute[self.nr].properties.get(PropertyKind::Abstractness)
  }
}

struct Instantiate<'a> {
  g: &'a Global,
  lc: &'a LocalContext,
  terms: &'a IdxVec<EqTermId, EqTerm>,
  subst: &'a [Type],
}

impl Instantiate<'_> {
  /// InstantiateTerm(fCluster = self.subst, eTrm = tgt, aTrm = src)
  fn inst_term(&self, src: &Term, tgt: &Term) -> Dnf<LocusId, EqClassId> {
    // vprintln!("inst_term {:?} <- {src:?} = {tgt:?}", self.subst);
    match (tgt.unmark(self.lc), src) {
      (Term::Numeral(n), Term::Numeral(n2)) => Dnf::mk_bool(n == n2),
      (Term::Functor { nr: n1, args: args1 }, Term::Functor { nr: n2, args: args2 }) => {
        let (n1, args1) = Term::adjust(*n1, args1, Some(&self.g.constrs));
        let (n2, args2) = Term::adjust(*n2, args2, Some(&self.g.constrs));
        if n1 == n2 {
          let mut res = Dnf::True;
          for (a, b) in args1.iter().zip(args2) {
            res.mk_and_then(|| Ok(self.inst_term(a, b))).unwrap()
          }
          res
        } else {
          Dnf::FALSE
        }
      }
      (
        Term::Selector { nr: SelId(n1), args: args1 },
        Term::Selector { nr: SelId(n2), args: args2 },
      )
      | (
        Term::Aggregate { nr: AggrId(n1), args: args1 },
        Term::Aggregate { nr: AggrId(n2), args: args2 },
      ) if n1 == n2 => {
        let mut res = Dnf::True;
        for (a, b) in args1.iter().zip(&**args2) {
          res.mk_and_then(|| Ok(self.inst_term(a, b))).unwrap()
        }
        res
      }
      (
        Term::Numeral(_) | Term::Functor { .. } | Term::Selector { .. } | Term::Aggregate { .. },
        _,
      ) => Dnf::FALSE,
      (Term::EqClass(_), _) => {
        let et = self.lc.marks[self.terms[self.lc.marks[tgt.mark().unwrap()].1].mark].1;
        match src {
          &Term::Locus(v) => {
            let mut z = self.inst_type(&self.subst[v.0 as usize], et);
            z.mk_and_then(|| Ok(Dnf::single(Conjunct::single(v, self.terms[et].id)))).unwrap();
            z
          }
          &Term::Numeral(n) => Dnf::mk_bool(self.terms[et].number == Some(n.into())),
          Term::Functor { nr: n1, args: args1 } => {
            let (n1, args1) = Term::adjust(*n1, args1, Some(&self.g.constrs));
            let mut res = Dnf::FALSE;
            for &m in &self.terms[et].eq_class {
              if let Term::Functor { nr: n2, args: args2 } = &self.lc.marks[m].0 {
                let (n2, args2) = Term::adjust(*n2, args2, Some(&self.g.constrs));
                if n1 == n2 {
                  res.mk_or_else(|| Ok(self.inst_terms(args1, args2))).unwrap()
                }
              }
            }
            res
          }
          Term::Selector { nr: n1, args: args1 } => {
            let mut res = Dnf::FALSE;
            for &m in &self.terms[et].eq_class {
              if let Term::Selector { nr: n2, args: args2 } = &self.lc.marks[m].0 {
                if n1 == n2 {
                  res.mk_or_else(|| Ok(self.inst_terms(args1, args2))).unwrap()
                }
              }
            }
            res
          }
          Term::Aggregate { nr: n1, args: args1 } => {
            let mut res = Dnf::FALSE;
            for &m in &self.terms[et].eq_class {
              if let Term::Aggregate { nr: n2, args: args2 } = &self.lc.marks[m].0 {
                if n1 == n2 {
                  res.mk_or_else(|| Ok(self.inst_terms(args1, args2))).unwrap()
                }
              }
            }
            res
          }
          _ => unreachable!(),
        }
      }
      r => unreachable!("{r:?}"),
    }
    // vprintln!("inst_term {:?} -> {src:?} = {tgt:?} -> {res:?}", self.subst);
  }

  fn inst_terms(&self, args1: &[Term], args2: &[Term]) -> Dnf<LocusId, EqClassId> {
    assert!(args1.len() == args2.len());
    let mut res = Dnf::True;
    for (a, b) in args1.iter().zip(args2) {
      res.mk_and_then(|| Ok(self.inst_term(a, b))).unwrap()
    }
    res
  }

  /// InstantiateType(cCluster = self.subst, enr = et, aTyp = ty)
  fn inst_type(&self, ty: &Type, et: EqTermId) -> Dnf<LocusId, EqClassId> {
    let et = self.lc.marks[self.terms[et].mark].1;
    let mut res = Dnf::FALSE;
    match ty.kind {
      TypeKind::Struct(_) =>
        for ty2 in &self.terms[et].ty_class {
          if ty.kind == ty2.kind {
            res.mk_or(self.inst_terms(&ty.args, &ty2.args)).unwrap();
            if let Dnf::True = res {
              break
            }
          }
        },
      TypeKind::Mode(n) => {
        let (n, args) = Type::adjust(n, &ty.args, &self.g.constrs);
        for ty2 in &self.terms[et].ty_class {
          if let TypeKind::Mode(n2) = ty2.kind {
            let (n2, args2) = Type::adjust(n2, &ty2.args, &self.g.constrs);
            if n == n2 {
              res.mk_or(self.inst_terms(args, args2)).unwrap();
              if let Dnf::True = res {
                break
              }
            }
          }
        }
      }
    }
    self.and_inst_attrs(&ty.attrs.0, et, &mut res);
    res
  }

  fn and_inst_attrs(&self, attrs: &Attrs, et: EqTermId, res: &mut Dnf<LocusId, EqClassId>) {
    let Attrs::Consistent(attrs) = attrs else { unreachable!() };
    let Attrs::Consistent(sc) = &self.terms[et].supercluster else { unreachable!() };
    // vprintln!("and_inst {attrs:?} <> {:?}", self.terms[et]);
    'next: for a1 in attrs {
      let (n1, args1) = a1.adjust(Some(&self.g.constrs));
      let mut z = Dnf::FALSE;
      for a2 in sc {
        let (n2, args2) = a2.adjust(Some(&self.g.constrs));
        if n1 == n2 && a1.pos == a2.pos {
          z.mk_or(self.inst_terms(args1, args2)).unwrap();
          if let Dnf::True = z {
            continue 'next
          }
        }
      }
      res.mk_and(z).unwrap();
    }
    // vprintln!("and_inst {attrs:?} <> {:?} -> {:?}", self.terms[et], res);
  }
}

fn is_empty_set(g: &Global, lc: &LocalContext, terms: &[EqMarkId]) -> bool {
  let empty = g.reqs.empty_set().unwrap();
  terms.iter().any(|&m| matches!(lc.marks[m].0, Term::Functor { nr, .. } if nr == empty))
}

impl Attrs {
  fn try_enlarge_by(
    &mut self, ctx: &Constructors, lc: &LocalContext, other: &Attrs,
  ) -> OrUnsat<bool> {
    let c = self.attrs().len();
    self.enlarge_by(ctx, lc, other);
    Ok(self.try_attrs()?.len() != c)
  }
}

impl<'a> Equalizer<'a> {
  pub fn new(ck: &'a mut Checker<'_>) -> Self {
    Self {
      g: ck.g,
      lc: ck.lc,
      reductions: ck.reductions,
      infers: Default::default(),
      constrs: Default::default(),
      terms: Default::default(),
      next_eq_class: Default::default(),
      clash: false,
    }
  }

  fn filter_allowed(&self, attrs: &Attrs) -> Attrs {
    match attrs {
      Attrs::Inconsistent => Attrs::Inconsistent,
      Attrs::Consistent(attrs) => {
        let attrs =
          attrs.iter().filter(|a| !HasInfer::get(&self.infers, |ci| ci.visit_terms(&a.args)));
        Attrs::Consistent(attrs.cloned().collect())
      }
    }
  }

  fn add_symm(&self, pos: &Atoms, neg: &mut Atoms, prop: PropertyKind) {
    for f in &pos.0 .0 {
      if let Formula::Pred { nr, args } = f {
        let pred = &self.g.constrs.predicate[*nr];
        // Why are we searching for f in neg_bas here?
        if pred.properties.get(prop) && neg.find(self.g, self.lc, f).is_none() {
          let mut args = args.clone();
          args.swap(pred.properties.arg1 as usize, pred.properties.arg2 as usize);
          neg.insert(self.g, self.lc, Cow::Owned(Formula::Pred { nr: *nr, args }));
        }
      }
    }
  }

  fn check_refl(&self, atoms: &Atoms, prop: PropertyKind, ineqs: &mut Ineqs) -> OrUnsat<()> {
    for f in &atoms.0 .0 {
      if let Formula::Pred { nr, args } = f {
        let pred = self.g.constrs.predicate[*nr].properties;
        if pred.get(prop) {
          let et1 = self.lc.marks[args[pred.arg1 as usize].mark().unwrap()].1;
          let et2 = self.lc.marks[args[pred.arg2 as usize].mark().unwrap()].1;
          if et1 == et2 {
            return Err(Unsat)
          }
          ineqs.push(self.terms[et1].mark, self.terms[et2].mark);
        }
      }
    }
    Ok(())
  }

  fn drain_pending(
    &mut self, to_y_term: &mut Vec<(EqTermId, Term)>, eq_pendings: &mut Equals,
  ) -> OrUnsat<()> {
    for (i, mut tm) in to_y_term.drain(..) {
      self.y(|y| y.visit_term(&mut tm))?;
      eq_pendings.insert(i, self.lc.marks[tm.mark().unwrap()].1)
    }
    Ok(())
  }

  /// UnionTrms
  fn union_terms(&mut self, x: EqTermId, y: EqTermId) -> OrUnsat<()> {
    let (x, y) = (self.lc.marks[self.terms[x].mark].1, self.lc.marks[self.terms[y].mark].1);
    let (from, to) = match x.cmp(&y) {
      Ordering::Less => (y, x),
      Ordering::Equal => return Ok(()),
      Ordering::Greater => (x, y),
    };
    // vprintln!(
    //   "union {:?} <=> {:?}",
    //   self.terms[x].eq_class.iter().map(|&x| Term::EqMark(x)).collect_vec(),
    //   self.terms[y].eq_class.iter().map(|&x| Term::EqMark(x)).collect_vec(),
    // );
    self.clash = true;
    if let Some(n1) = self.terms[from].number.clone() {
      if matches!(self.terms[to].number.replace(n1.clone()), Some(n2) if n1 != n2) {
        return Err(Unsat)
      }
    }
    for &m in &self.terms[from].eq_class {
      let m = self.terms[self.lc.marks[m].1].mark;
      self.lc.marks[m].1 = to;
    }
    let eq_class = std::mem::take(&mut self.terms[from].eq_class);
    self.terms[to].eq_class.append(&mut { eq_class });
    let Attrs::Consistent(attrs) = std::mem::take(&mut self.terms[from].supercluster) else {
      unreachable!()
    };
    for attr in attrs {
      self.terms[to].supercluster.try_insert(&self.g.constrs, self.lc, attr)?;
    }
    for ty in std::mem::take(&mut self.terms[from].ty_class) {
      self.insert_type(ty, to)?;
    }
    let eq_polys = std::mem::take(&mut self.terms[from].eq_polys);
    self.terms[to].eq_polys.extend(eq_polys);
    if self.terms[to].eq_polys.len() == 2 {
      let mut it = std::mem::take(&mut self.terms[to].eq_polys).into_iter();
      let (p1, p2) = (it.next().unwrap(), it.next().unwrap());
      if let Some((i, p)) = match (p1.is_var(), p2.is_var()) {
        (Some(v1), Some(v2)) if v1 < v2 => Some((v2, p1)),
        (Some(v1), _) => Some((v1, p2)),
        (_, Some(v2)) => Some((v2, p1)),
        (None, None) => {
          self.terms[to].eq_polys.insert(p1);
          self.terms[to].eq_polys.insert(p2);
          None
        }
      } {
        let mut pending = BTreeSet::new();
        self.subst_var(i, &p, Some(&mut pending))?;
        self.terms[to].eq_polys.insert(p);
        self.subst_pending_vars(pending)?;
      }
    }
    Ok(())
  }

  fn instantiate<'b>(&'b self, subst: &'b [Type]) -> Instantiate<'b> {
    Instantiate { g: self.g, lc: self.lc, terms: &self.terms, subst }
  }

  fn locate_terms(
    &self, inst: &Conjunct<LocusId, EqClassId>, args1: &[Term], args2: &[Term],
  ) -> Option<()> {
    assert!(args1.len() == args2.len());
    // vprintln!("locate_terms {args1:?}, {args2:?}");
    for (t1, t2) in args1.iter().zip(args2) {
      let m1 = self.locate_term(inst, t1)?;
      matches!(*t2, Term::EqMark(m2) if self.lc.marks[m1].1 == self.lc.marks[m2].1).then_some(())?;
    }
    Some(())
  }

  fn locate_term(&self, inst: &Conjunct<LocusId, EqClassId>, tm: &Term) -> Option<EqMarkId> {
    // vprintln!("locate_term {inst:?}, {tm:?}");
    match *tm {
      Term::Locus(n) => {
        let id = *inst.0.get(&n)?;
        Some(self.terms.0.iter().find(|&et| et.id == id && !et.eq_class.is_empty())?.mark)
      }
      Term::Infer(n) => (self.terms.0.iter())
        .find(|et| {
          et.eq_class.iter().any(|&m| matches!(self.lc.marks[m].0, Term::Infer(n2) if n == n2))
        })
        .map(|et| et.mark),
      Term::Numeral(nr) => {
        let c = nr.into();
        self.terms.0.iter().find(|et| et.number.as_ref() == Some(&c)).map(|et| et.mark)
      }
      Term::Functor { nr, ref args } => (self.terms.0.iter())
        .find(|et| {
          et.eq_class.iter().any(|&m| {
            matches!(&self.lc.marks[m].0, Term::Functor { nr: nr2, args: args2 }
              if nr == *nr2 && self.locate_terms(inst, args, args2).is_some())
          })
        })
        .map(|et| et.mark),
      Term::Selector { nr, ref args } => (self.terms.0.iter())
        .find(|et| {
          et.eq_class.iter().any(|&m| {
            matches!(&self.lc.marks[m].0, Term::Selector { nr: nr2, args: args2 }
              if nr == *nr2 && self.locate_terms(inst, args, args2).is_some())
          })
        })
        .map(|et| et.mark),
      Term::Aggregate { nr, ref args } => (self.terms.0.iter())
        .find(|et| {
          et.eq_class.iter().any(|&m| {
            matches!(&self.lc.marks[m].0, Term::Aggregate { nr: nr2, args: args2 }
              if nr == *nr2 && self.locate_terms(inst, args, args2).is_some())
          })
        })
        .map(|et| et.mark),
      _ => None,
    }
    // vprintln!("locate_term {inst:?}, {tm:?} -> {:?}", res.map(Term::EqMark));
  }

  fn locate_attrs(&self, inst: &Conjunct<LocusId, EqClassId>, attrs: &Attrs) -> Attrs {
    match attrs {
      Attrs::Inconsistent => Attrs::Inconsistent,
      Attrs::Consistent(attrs) => {
        let mut res = vec![];
        for attr in attrs {
          if let Some(args) =
            attr.args.iter().map(|tm| self.locate_term(inst, tm).map(Term::EqMark)).collect()
          {
            res.push(Attr { nr: attr.nr, pos: attr.pos, args })
          }
        }
        res.sort_by(|a1, a2| a1.cmp_abs(Some(&self.g.constrs), Some(self.lc), a2, CmpStyle::Attr));
        Attrs::Consistent(res)
      }
    }
  }

  /// ProcessReductions
  fn process_reductions(&mut self) -> OrUnsat<()> {
    let mut i = 0;
    while let Some(m) = self.infers.0.get(i) {
      if let Some(m) = *m {
        let et = self.lc.marks[m].1;
        // vprintln!("reducing: {et:?}'e{:#?}", self.terms[et].id);
        if !self.terms[et].eq_class.is_empty() {
          for red in self.reductions {
            let inst = self
              .instantiate(&red.primary)
              .inst_term(&red.terms[0], &Term::EqMark(self.terms[et].mark));
            // if !matches!(&inst, Dnf::Or(conjs) if conjs.is_empty()) {
            //   vprintln!("found reduction {et:?}'e{:#?} by {red:#?}", self.terms[et].id);
            //   vprintln!("inst = {inst:#?}");
            // }
            if let Some(conj) = match inst {
              Dnf::True => Some(Conjunct::TRUE),
              Dnf::Or(conjs) => conjs.into_iter().next(),
            } {
              let m = if let Term::Functor { nr, args } = &red.terms[1] {
                let (nr, args) = Term::adjust(*nr, args, Some(&self.g.constrs));
                self.locate_term(&conj, &Term::Functor { nr, args: args.to_vec().into() })
              } else {
                self.locate_term(&conj, &red.terms[1])
              };
              self.union_terms(et, self.lc.marks[m.unwrap()].1)?;
            }
          }
        }
      }
      i += 1;
    }
    Ok(())
  }

  fn set_number(&mut self, et: EqTermId, val: Complex) -> OrUnsat<()> {
    if let Some(n) = &self.terms[et].number {
      if val != *n {
        return Err(Unsat)
      }
    } else {
      for et2 in (0..self.terms.len()).map(EqTermId::from_usize) {
        if !self.terms[et2].eq_class.is_empty() && self.terms[et2].number.as_ref() == Some(&val) {
          vprintln!("[{et:?}] = [{et2:?}] = {val}");
          self.union_terms(et, et2)?
        }
      }
      self.clash = true;
      // vprintln!("set_number[{et:?}] := {val}");
      self.terms[et].number = Some(val);
      // for (et, etm) in self.terms.enum_iter() {
      //   vprintln!("state: {et:?}' {:#?}", etm);
      // }
    }
    Ok(())
  }

  /// Identities(aArithmIncl = arith)
  fn identities(&mut self, arith: bool) -> OrUnsat<()> {
    let mut to_union = vec![];
    let mut to_number = vec![];
    loop {
      for marks in self.constrs.aggregate.0.values() {
        let mut iter = marks.iter().copied();
        while let Some(m1) = iter.next() {
          let et1 = self.lc.marks[self.terms[self.lc.marks[m1].1].mark].1;
          if let Some(m2) =
            iter.clone().find(|&m| self.lc.marks[self.terms[self.lc.marks[m].1].mark].1 == et1)
          {
            let Term::Aggregate { nr, args: args1 } = &self.lc.marks[m1].0 else { unreachable!() };
            let Term::Aggregate { args: args2, .. } = &self.lc.marks[m2].0 else { unreachable!() };
            let base = self.g.constrs.aggregate[*nr].base as usize;
            assert!(args1.len() == args2.len());
            for (a1, a2) in args1.iter().zip(&**args2).skip(base) {
              let m1 = self.lc.marks[a1.mark().unwrap()].1;
              let m2 = self.lc.marks[a2.mark().unwrap()].1;
              if m1 != m2 {
                to_union.push((m1, m2))
              }
            }
          }
        }
      }
      for (x, y) in to_union.drain(..) {
        self.union_terms(x, y)?;
      }

      for (&i, marks) in &self.constrs.functor.0 {
        let props = self.g.constrs.functor[i].properties;
        if props.get(PropertyKind::Idempotence) {
          for &m in marks {
            let (Term::Functor { ref args, .. }, et) = self.lc.marks[m] else { unreachable!() };
            let et1 = self.lc.marks[args[props.arg1 as usize].mark().unwrap()].1;
            let et2 = self.lc.marks[args[props.arg2 as usize].mark().unwrap()].1;
            if self.lc.marks[self.terms[et1].mark].1 == self.lc.marks[self.terms[et2].mark].1 {
              to_union.push((self.lc.marks[self.terms[et].mark].1, et1))
            }
          }
        }
        if props.get(PropertyKind::Involutiveness) {
          for &m in marks {
            let (Term::Functor { ref args, .. }, et) = self.lc.marks[m] else { unreachable!() };
            assert!(props.arg1 as usize + 1 == args.len());
            let et1 = self.lc.marks[args[props.arg1 as usize].mark().unwrap()].1;
            let args1 = &args[..props.arg1 as usize];
            for &m2 in &self.terms[self.lc.marks[self.terms[et1].mark].1].eq_class {
              if let Term::Functor { nr, args: ref args2 } = self.lc.marks[m2].0 {
                if nr == i && EqMarks.eq(self.g, self.lc, args1, &args2[..props.arg1 as usize]) {
                  let et2 = self.lc.marks[args2[props.arg1 as usize].mark().unwrap()].1;
                  to_union.push((self.lc.marks[self.terms[et].mark].1, et2))
                }
              }
            }
          }
        }
        if props.get(PropertyKind::Projectivity) {
          for &m in marks {
            let (Term::Functor { ref args, .. }, et) = self.lc.marks[m] else { unreachable!() };
            assert!(props.arg1 as usize + 1 == args.len());
            let et1 = self.lc.marks[args[props.arg1 as usize].mark().unwrap()].1;
            let args1 = &args[..props.arg1 as usize];
            for &m2 in &self.terms[self.lc.marks[self.terms[et1].mark].1].eq_class {
              if let Term::Functor { nr, args: ref args2 } = self.lc.marks[m2].0 {
                if nr == i && EqMarks.eq(self.g, self.lc, args1, &args2[..props.arg1 as usize]) {
                  to_union.push((self.lc.marks[self.terms[et].mark].1, et1))
                }
              }
            }
          }
        }
        macro_rules! op {
          (|$x:ident| $e:expr) => {
            for &m in marks {
              let (Term::Functor { ref args, .. }, et) = self.lc.marks[m] else { unreachable!() };
              let et1 = self.lc.marks[args[0].mark().unwrap()].1;
              if let Some($x) = &self.terms[et1].number {
                to_number.push((self.lc.marks[self.terms[et].mark].1, $e))
              }
            }
          };
          (|$x:ident, $y:ident| $e:expr) => {
            for &m in marks {
              let (Term::Functor { ref args, .. }, et) = self.lc.marks[m] else { unreachable!() };
              let et1 = self.lc.marks[args[0].mark().unwrap()].1;
              let et2 = self.lc.marks[args[1].mark().unwrap()].1;
              if let (Some($x), Some($y)) = (&self.terms[et1].number, &self.terms[et2].number) {
                to_number.push((self.lc.marks[self.terms[et].mark].1, $e))
              }
            }
          };
        }
        match self.g.reqs.rev.get(i).copied().flatten() {
          Some(Requirement::Union) =>
            for &m in marks {
              let (Term::Functor { ref args, .. }, et) = self.lc.marks[m] else { unreachable!() };
              let et1 = self.lc.marks[args[0].mark().unwrap()].1;
              if is_empty_set(self.g, self.lc, &self.terms[et1].eq_class) {
                let et2 = self.lc.marks[args[1].mark().unwrap()].1;
                to_union.push((self.lc.marks[self.terms[et].mark].1, et2))
              }
            },
          Some(Requirement::Intersection) =>
            for &m in marks {
              let (Term::Functor { ref args, .. }, et) = self.lc.marks[m] else { unreachable!() };
              let et1 = self.lc.marks[args[0].mark().unwrap()].1;
              if is_empty_set(self.g, self.lc, &self.terms[et1].eq_class) {
                to_union.push((self.lc.marks[self.terms[et].mark].1, et1))
              }
            },
          Some(Requirement::Subtraction) =>
            for &m in marks {
              let (Term::Functor { ref args, .. }, et) = self.lc.marks[m] else { unreachable!() };
              let et1 = self.lc.marks[args[0].mark().unwrap()].1;
              if is_empty_set(self.g, self.lc, &self.terms[et1].eq_class) || {
                let et2 = self.lc.marks[args[1].mark().unwrap()].1;
                is_empty_set(self.g, self.lc, &self.terms[et2].eq_class)
              } {
                to_union.push((self.lc.marks[self.terms[et].mark].1, et1))
              }
            },
          Some(Requirement::SymmetricDifference) =>
            for &m in marks {
              let (Term::Functor { ref args, .. }, et) = self.lc.marks[m] else { unreachable!() };
              let et2 = self.lc.marks[args[1].mark().unwrap()].1;
              if is_empty_set(self.g, self.lc, &self.terms[et2].eq_class) {
                let et1 = self.lc.marks[args[0].mark().unwrap()].1;
                to_union.push((self.lc.marks[self.terms[et].mark].1, et1))
              }
            },
          Some(Requirement::Succ) => op!(|x| x.clone() + Complex::ONE),
          Some(Requirement::RealAdd) if arith =>
            for &m in marks {
              let (Term::Functor { ref args, .. }, et) = self.lc.marks[m] else { unreachable!() };
              let et1 = self.lc.marks[args[0].mark().unwrap()].1;
              if let Some(x1) = &self.terms[et1].number {
                let et2 = self.lc.marks[args[1].mark().unwrap()].1;
                if *x1 == Complex::ZERO {
                  to_union.push((self.lc.marks[self.terms[et].mark].1, et2))
                } else if let Some(x2) = &self.terms[et2].number {
                  to_number.push((self.lc.marks[self.terms[et].mark].1, x1.clone() + x2.clone()))
                }
              }
            },
          Some(Requirement::RealMult) if arith =>
            for &m in marks {
              let (Term::Functor { ref args, .. }, et) = self.lc.marks[m] else { unreachable!() };
              let et1 = self.lc.marks[args[0].mark().unwrap()].1;
              if let Some(x1) = &self.terms[et1].number {
                let et2 = self.lc.marks[args[1].mark().unwrap()].1;
                if *x1 == Complex::ZERO {
                  to_union.push((self.lc.marks[self.terms[et].mark].1, et1))
                } else if *x1 == Complex::ONE {
                  to_union.push((self.lc.marks[self.terms[et].mark].1, et2))
                } else if let Some(x2) = &self.terms[et2].number {
                  to_number.push((self.lc.marks[self.terms[et].mark].1, x1.clone() * x2.clone()))
                }
              }
            },
          Some(Requirement::RealNeg) if arith => op!(|x| -x.clone()),
          Some(Requirement::RealInv) if arith =>
            for &m in marks {
              let (Term::Functor { ref args, .. }, et) = self.lc.marks[m] else { unreachable!() };
              let et1 = self.lc.marks[args[0].mark().unwrap()].1;
              if let Some(x) = &self.terms[et1].number {
                if *x != Complex::ZERO {
                  to_number.push((self.lc.marks[self.terms[et].mark].1, x.clone().inv()))
                }
              }
            },
          Some(Requirement::RealDiff) if arith =>
            for &m in marks {
              let (Term::Functor { ref args, .. }, et) = self.lc.marks[m] else { unreachable!() };
              let et2 = self.lc.marks[args[1].mark().unwrap()].1;
              if let Some(x2) = &self.terms[et2].number {
                let et1 = self.lc.marks[args[0].mark().unwrap()].1;
                if *x2 == Complex::ZERO {
                  to_union.push((self.lc.marks[self.terms[et].mark].1, et1))
                } else if let Some(x1) = &self.terms[et1].number {
                  to_number.push((self.lc.marks[self.terms[et].mark].1, x1.clone() - x2.clone()))
                }
              }
            },
          Some(Requirement::RealDiv) if arith =>
            for &m in marks {
              let (Term::Functor { ref args, .. }, et) = self.lc.marks[m] else { unreachable!() };
              let et1 = self.lc.marks[args[0].mark().unwrap()].1;
              let et2 = self.lc.marks[args[1].mark().unwrap()].1;
              match (&self.terms[et1].number, &self.terms[et2].number) {
                (Some(x1), _) if *x1 == Complex::ZERO =>
                  to_union.push((self.lc.marks[self.terms[et].mark].1, et1)),
                (_, Some(x2)) if *x2 == Complex::ONE =>
                  to_union.push((self.lc.marks[self.terms[et].mark].1, et1)),
                (Some(x1), Some(x2)) =>
                  if *x2 != Complex::ZERO {
                    to_number.push((self.lc.marks[self.terms[et].mark].1, x1.clone() / x2.clone()))
                  },
                _ => {}
              }
            },
          _ => {}
        }
      }
      for (x, y) in to_union.drain(..) {
        self.union_terms(x, y)?;
      }
      for (x, y) in to_number.drain(..) {
        self.set_number(x, y)?;
      }
      if !self.clash {
        return Ok(())
      }

      loop {
        self.clash = false;
        let mut f = |vec: &Vec<EqMarkId>| {
          for (m1, m2) in vec.iter().copied().tuple_combinations() {
            let (ref tm1, et1) = self.lc.marks[m1];
            let (ref tm2, et2) = self.lc.marks[m2];
            if et1 != et2 {
              match (tm1, tm2) {
                (
                  Term::Functor { nr: nr1, args: args1 },
                  Term::Functor { nr: nr2, args: args2 },
                ) => {
                  let args1 = Term::adjust(*nr1, args1, Some(&self.g.constrs)).1;
                  let args2 = Term::adjust(*nr2, args2, Some(&self.g.constrs)).1;
                  if EqMarks.eq(self.g, self.lc, args1, args2) {
                    to_union.push((et1, et2))
                  }
                }
                (Term::SchFunc { args: args1, .. }, Term::SchFunc { args: args2, .. })
                | (Term::PrivFunc { args: args1, .. }, Term::PrivFunc { args: args2, .. }) =>
                  if EqMarks.eq(self.g, self.lc, args1, args2) {
                    to_union.push((et1, et2))
                  },
                (Term::Aggregate { args: args1, .. }, Term::Aggregate { nr, args: args2 }) => {
                  let base = self.g.constrs.aggregate[*nr].base as usize;
                  if EqMarks.eq(self.g, self.lc, &args1[base..], &args2[base..]) {
                    to_union.push((et1, et2))
                  }
                }
                (Term::Selector { args: args1, .. }, Term::Selector { args: args2, .. }) =>
                  if EqMarks.eq(self.g, self.lc, args1.last().unwrap(), args2.last().unwrap()) {
                    to_union.push((et1, et2))
                  },
                (
                  Term::Fraenkel { args: args1, scope: sc1, compr: compr1 },
                  Term::Fraenkel { args: args2, scope: sc2, compr: compr2 },
                ) =>
                  if args1.len() == args2.len()
                    && args1
                      .iter()
                      .zip(&**args2)
                      .all(|(ty1, ty2)| EqMarks.eq(self.g, self.lc, ty1, ty2))
                    && EqMarks.eq(self.g, self.lc, sc1, sc2)
                    && EqMarks.eq(self.g, self.lc, compr1, compr2)
                  {
                    to_union.push((et1, et2))
                  },
                (Term::The { ty: ty1 }, Term::The { ty: ty2 }) =>
                  if EqMarks.eq(self.g, self.lc, ty1, ty2) {
                    to_union.push((et1, et2))
                  },
                _ => unreachable!(),
              }
            }
          }
        };
        self.constrs.functor.0.values().for_each(&mut f);
        self.constrs.aggregate.0.values().for_each(&mut f);
        self.constrs.selector.0.values().for_each(&mut f);
        self.constrs.priv_func.0.values().for_each(&mut f);
        self.constrs.sch_func.0.values().for_each(&mut f);
        f(&self.constrs.fraenkel);
        f(&self.constrs.choice);
        for (x, y) in to_union.drain(..) {
          self.union_terms(x, y)?;
        }
        if !self.clash {
          break
        }
      }
      self.process_reductions()?;
    }
  }

  /// SubstituteVariable
  fn subst_var(
    &mut self, from: EqTermId, p: &Polynomial<EqTermId>,
    mut pending: Option<&mut BTreeSet<EqTermId>>,
  ) -> OrUnsat<bool> {
    if p.contains(from) {
      return Ok(false)
    }
    // vprintln!("subst {from:?}' := {p:?}");
    // let from2 = self.lc.marks[self.terms[from].mark].1;
    let mut progress = false;
    for (v, etm) in self.terms.enum_iter_mut() {
      if v != from && !etm.eq_class.is_empty() {
        for mut p2 in std::mem::take(&mut etm.eq_polys) {
          if p2.contains(from) {
            p2.subst(from, p);
            if let Some(c) = p2.is_const() {
              if let Some(c2) = &etm.number {
                if c != *c2 {
                  return Err(Unsat)
                }
              } else {
                if let Some(pending) = &mut pending {
                  pending.insert(from);
                }
                etm.number = Some(c)
              }
            }
            progress = true;
            etm.eq_polys.insert(p2);
          } else {
            etm.eq_polys.insert(p2);
          }
        }
      }
    }
    Ok(progress)
  }

  /// ClearPolynomialValues
  fn clear_polynomial_values(&mut self) -> OrUnsat<()> {
    let mut to_subst = vec![];
    for (et, etm) in self.terms.enum_iter() {
      let et2 = self.lc.marks[etm.mark].1;
      if et2 != et && !self.terms[et2].eq_polys.is_empty() {
        to_subst.push((et, et2))
      }
    }
    for (et, v) in to_subst {
      self.subst_var(et, &Polynomial::single(Monomial::atom(v)), None)?;
    }
    Ok(())
  }

  /// EquatePolynomialValues
  fn equate_polynomial_values(&mut self, eqs: &mut Equals) -> OrUnsat<()> {
    if self.g.reqs.complex().is_none() {
      return Ok(())
    }
    let mut to_subst = vec![];
    let mut pending = BTreeSet::new();
    loop {
      for (v1, etm) in self.terms.enum_iter() {
        if !etm.eq_class.is_empty() {
          if let Some(p) = etm.eq_polys.first() {
            if let Some(v2) = p.is_var() {
              if v2 != v1 {
                if let Some(q) = self.terms[self.lc.marks[self.terms[v2].mark].1].eq_polys.first() {
                  if let Some(v3) = q.is_var() {
                    if v3 == v2 {
                      if v1 > v2 {
                        to_subst.push((v1, v2, q.clone()));
                      } else {
                        to_subst.push((v2, v1, Polynomial::single(Monomial::atom(v1))));
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
      for (v1, v2, p) in to_subst.drain(..) {
        self.subst_var(v1, &p, Some(&mut pending))?;
        self.union_terms(v1, v2)?;
      }
      self.subst_pending_vars(std::mem::take(&mut pending))?;
      'next: for (x, y) in std::mem::take(&mut eqs.0) {
        let et1 = self.lc.marks[self.terms[x].mark].1;
        let et2 = self.lc.marks[self.terms[y].mark].1;
        if let (Some(q1), Some(q2)) =
          (self.terms[et1].eq_polys.first(), self.terms[et2].eq_polys.first())
        {
          for p1 in &self.terms[et1].eq_polys {
            for p2 in &self.terms[et2].eq_polys {
              if p1 == p2 {
                self.union_terms(x, y)?;
                continue 'next
              }
            }
          }
          let (u1, u2) = (q1.is_var_power(), q2.is_var_power());
          if u1 == Some(et1) {
            if u2 == Some(et2) {
              self.union_terms(x, y)?;
              continue
            }
            if self.terms[et2].number.is_some() && self.terms[et2].eq_polys.len() == 1 {
              self.subst_var(x, &q2.clone(), Some(&mut pending))?;
              self.union_terms(x, y)?;
              continue
            }
          } else if self.terms[et1].number.is_some() && self.terms[et1].eq_polys.len() == 1 {
            if u2 == Some(y) {
              self.subst_var(y, &q2.clone(), Some(&mut pending))?;
              self.union_terms(x, y)?;
              continue
            } else if let Some(c2) = q2.is_const() {
              assert!(self.terms[et1].number.as_ref() != Some(&c2));
              return Err(Unsat)
            }
          }
        }
        eqs.0.insert((x, y));
      }
      if pending.is_empty() {
        break
      }
    }
    Ok(())
  }

  /// EquatePolynomials
  fn equate_polynomials(&mut self) -> OrUnsat<()> {
    let mut to_union = vec![];
    for ((et1, etm1), (et2, etm2)) in (self.terms.enum_iter())
      .filter(|(_, etm)| !etm.eq_class.is_empty() && !etm.eq_polys.is_empty())
      .tuple_combinations()
    {
      for p1 in &etm1.eq_polys {
        for p2 in &etm2.eq_polys {
          if p1 == p2 {
            to_union.push((et1, et2))
          }
        }
      }
    }
    for (x, y) in to_union {
      self.union_terms(x, y)?
    }
    Ok(())
  }

  /// SubstitutePendingVars
  fn subst_pending_vars(&mut self, mut pending: BTreeSet<EqTermId>) -> OrUnsat<()> {
    // vprintln!("subst_pending_vars {pending:?}");
    let mut first = true;
    loop {
      let mut progress = false;
      while let Some(et) = pending.pop_first() {
        let et = self.lc.marks[self.terms[et].mark].1;
        let etm = &self.terms[et];
        assert!(!etm.eq_class.is_empty());
        if etm.eq_polys.len() == 1 {
          progress |=
            self.subst_var(et, &etm.eq_polys.first().unwrap().clone(), Some(&mut pending))?;
        }
      }
      if !first && !progress {
        return Ok(())
      }
      first = false;
      for (et, etm) in self.terms.enum_iter() {
        if !etm.eq_class.is_empty() && etm.number.is_some() && etm.eq_polys.len() == 1 {
          pending.insert(et);
        }
      }
    }
  }

  /// ProcessLinearEquations
  fn process_linear_equations(&mut self, eqs: &mut Equals) -> OrUnsat<()> {
    if eqs.0.is_empty() {
      return Ok(())
    }
    let mut polys = BTreeSet::new();
    for &(x, y) in &eqs.0 {
      let et1 = self.lc.marks[self.terms[x].mark].1;
      let et2 = self.lc.marks[self.terms[y].mark].1;
      if let (Some(q1), Some(q2)) =
        (self.terms[et1].eq_polys.first(), self.terms[et2].eq_polys.first())
      {
        let p = q1.clone() - q2.clone();
        if let Some(c) = p.is_const() {
          if c != Complex::ZERO {
            return Err(Unsat)
          }
        } else {
          polys.insert(p);
        }
      }
    }
    let mut vars = Default::default();
    let eqs2 = polynomial::gaussian_elimination(&mut vars, polys)?;
    for ((et1, etm1), (et2, etm2)) in (self.terms.enum_iter())
      .filter(|(_, etm)| !etm.eq_polys.is_empty() && !etm.eq_polys.is_empty())
      .tuple_combinations()
    {
      for p1 in &etm1.eq_polys {
        for p2 in &etm2.eq_polys {
          let q = p1.clone() - p2.clone();
          if q.reduce(&vars, &eqs2) {
            eqs.insert(et1, et2)
          }
        }
      }
    }
    Ok(())
  }

  fn insert_non_attr0(&mut self, et1: EqTermId, et2: EqTermId, nr: AttrId) -> OrUnsat<()> {
    if self.terms[et1].supercluster.find0(&self.g.constrs, nr, true) {
      self.terms[et2].supercluster.try_insert(&self.g.constrs, self.lc, Attr::new0(nr, false))?;
    }
    Ok(())
  }

  fn nonempty_nonzero_of_ne(&mut self, et1: EqTermId, et2: EqTermId) -> OrUnsat<()> {
    if let Some(empty) = self.g.reqs.empty() {
      // a != b, a is empty => b is non empty
      self.insert_non_attr0(et1, et2, empty)?;
      // a != b, b is empty => a is non empty
      self.insert_non_attr0(et2, et1, empty)?;
    }
    if let Some(zero) = self.g.reqs.zero() {
      // a != b, a is zero => b is non zero
      self.insert_non_attr0(et1, et2, zero)?;
      // a != b, b is zero => a is non zero
      self.insert_non_attr0(et2, et1, zero)?;
    }
    Ok(())
  }

  fn check_neg_attr(&self, nr: AttrId, args: &[Term]) -> OrUnsat<()> {
    let (last, args1) = args.split_last().unwrap();
    if let Some(attr) = self.terms[self.lc.marks[last.mark().unwrap()].1].supercluster.find(
      &self.g.constrs,
      self.lc,
      &Attr { nr, pos: true, args: args1.to_vec().into() },
    ) {
      if attr.pos {
        return Err(Unsat)
      }
    }
    Ok(())
  }

  fn match_formulas(&self, neg: &Formula, pos_bas: &Atoms) -> OrUnsat<()> {
    for pos in &pos_bas.0 .0 {
      match (neg, pos) {
        (
          Formula::Attr { nr: AttrId(n1), args: args1 },
          Formula::Attr { nr: AttrId(n2), args: args2 },
        )
        | (
          Formula::SchPred { nr: SchPredId(n1), args: args1 },
          Formula::SchPred { nr: SchPredId(n2), args: args2 },
        )
        | (
          Formula::PrivPred { nr: PrivPredId(n1), args: args1, .. },
          Formula::PrivPred { nr: PrivPredId(n2), args: args2, .. },
        ) if n1 == n2 && EqMarks.eq(self.g, self.lc, args1, args2) => return Err(Unsat),
        _ => {}
      }
    }
    Ok(())
  }

  fn depends_on(&self, etm: &EqTerm, tgt: EqTermId) -> bool {
    assert!(!self.terms[tgt].eq_class.is_empty());
    !etm.eq_class.is_empty() && {
      struct CheckEqTerm<'a> {
        marks: &'a IdxVec<EqMarkId, (Term, EqTermId)>,
        terms: &'a IdxVec<EqTermId, EqTerm>,
        tgt: EqTermId,
        found: bool,
      }
      impl Visit for CheckEqTerm<'_> {
        fn abort(&self) -> bool { self.found }
        fn visit_term(&mut self, tm: &Term) {
          match *tm {
            Term::EqClass(_) => self.found = true,
            Term::EqMark(m) => {
              let (ref tm, et) = self.marks[m];
              if matches!(tm, Term::EqClass(_)) {
                self.found |= self.marks[self.terms[et].mark].1 == self.tgt
              } else {
                self.super_visit_term(tm);
              }
            }
            _ => self.super_visit_term(tm),
          }
        }
      }

      let mut ck = CheckEqTerm { marks: &self.lc.marks, terms: &self.terms, tgt, found: false };
      for &m in &etm.eq_class {
        ck.visit_term(&Term::EqMark(m))
      }
      ck.visit_types(&etm.ty_class);
      ck.visit_attrs(&etm.supercluster);
      ck.found
    }
  }

  fn round_up_one_supercluster(
    &mut self, et: EqTermId, attrs: &Attrs, inst: &Dnf<LocusId, EqClassId>,
  ) -> OrUnsat<bool> {
    match inst {
      Dnf::True => {
        let attrs = self.locate_attrs(&Conjunct::TRUE, attrs);
        self.terms[et].supercluster.try_enlarge_by(&self.g.constrs, self.lc, &attrs)
      }
      Dnf::Or(conjs) => {
        let mut added = false;
        for conj in conjs {
          let attrs = self.locate_attrs(conj, attrs);
          added |= self.terms[et].supercluster.try_enlarge_by(&self.g.constrs, self.lc, &attrs)?;
        }
        Ok(added)
      }
    }
  }

  pub fn run(
    &mut self, atoms: &Atoms, conj: &Conjunct<AtomId, bool>,
  ) -> OrUnsat<EnumMap<bool, Atoms>> {
    self.lc.marks.0.clear();
    let mut eqs = Equals::default();
    let mut bas = EnumMap::<bool, Atoms>::default();
    // for (i, fv) in self.lc.fixed_var.enum_iter() {
    //   vprintln!("c{i:?} := {:#?}", fv);
    // }
    // for (i, asgn) in self.lc.infer_const.get_mut().enum_iter() {
    //   vprintln!("?{i:?} := {:#?}", asgn);
    // }
    for pos in [true, false] {
      for (i, f) in atoms.0.enum_iter() {
        // vprintln!("y pass atom {f:?}");
        if conj.0.get(&i).copied() == Some(pos) {
          // vprintln!("{}: {f:?}", if pos { "assume" } else { "goal" });
          match f {
            Formula::Is { term, ty } if pos => {
              let x_type = self.y(|y| (**ty).visit_cloned(y))?;
              let x_term = self.y(|y| (**term).visit_cloned(y))?;
              self.insert_type(x_type, self.lc.marks[x_term.mark().unwrap()].1)?;
            }
            Formula::Attr { nr, args } => {
              let mut args = self.y(|y| args.visit_cloned(y))?.into_vec();
              let term = args.pop().unwrap();
              let et = self.lc.marks[term.mark().unwrap()].1;
              let et = self.lc.marks[self.terms[et].mark].1;
              let attr = Attr { nr: *nr, pos, args: args.into() };
              self.terms[et].supercluster.try_insert(&self.g.constrs, self.lc, attr)?;
              self.terms[et].supercluster.try_attrs()?;
            }
            Formula::Pred { nr, args } if pos => {
              let (nr, args) = Formula::adjust_pred(*nr, args, Some(&self.g.constrs));
              if self.g.reqs.equals_to() == Some(nr) {
                let [arg1, arg2] = args else { unreachable!() };
                let m1 = self.y(|y| arg1.visit_cloned(y))?.mark().unwrap();
                let m2 = self.y(|y| arg2.visit_cloned(y))?.mark().unwrap();
                eqs.insert(self.lc.marks[m1].1, self.lc.marks[m2].1);
              } else {
                bas[pos].0.push(self.y(|y| f.visit_cloned(y))?);
              }
            }
            _ => {
              bas[pos].0.push(self.y(|y| f.visit_cloned(y))?);
            }
          }
        }
      }
    }

    // vprintln!("start");
    // for (et, etm) in self.terms.enum_iter() {
    //   vprintln!("state: {et:?}' {:#?}", etm);
    // }
    // vprintln!("eqs = {:?}", eqs.0);

    let [mut neg_bas, mut pos_bas] = bas.into_array();
    self.add_symm(&pos_bas, &mut neg_bas, PropertyKind::Asymmetry);
    self.add_symm(&neg_bas, &mut pos_bas, PropertyKind::Connectedness);

    let mut to_y_term = vec![];
    let mut to_yy_term = vec![];
    let mut settings = Equals::default();
    let mut i = EqTermId::default();
    // This cannot be a for loop because the terms list grows due to y_term() and yy_term()
    while let Some(ets) = self.terms.get(i) {
      for &m in &ets.eq_class {
        if let Term::Infer(id) = self.lc.marks[m].0 {
          let asgn = &self.lc.infer_const.borrow()[id];
          for &z in &asgn.eq_const {
            to_y_term.push((i, Term::Infer(z)));
          }
          to_yy_term.push((i, asgn.def.visit_cloned(&mut ExpandPrivFunc(&self.g.constrs, self.lc))))
        }
      }
      self.drain_pending(&mut to_y_term, &mut eqs)?;
      for (i, tm) in to_yy_term.drain(..) {
        settings.insert(i, self.yy_term(tm, i)?)
      }
      i.0 += 1;
    }

    // InitEmptyInEqClass
    if let Some(empty_set) = self.g.reqs.empty_set() {
      let empty = self.g.reqs.empty().unwrap();
      for (i, ets) in self.terms.enum_iter() {
        assert!(!ets.eq_class.is_empty()); // TODO: is this true?
        if !ets.eq_class.is_empty() && ets.supercluster.find0(&self.g.constrs, empty, true) {
          to_y_term.push((i, Term::Functor { nr: empty_set, args: Box::new([]) }))
        }
      }
      self.drain_pending(&mut to_y_term, &mut eqs)?;
    }
    if let Some(zero_number) = self.g.reqs.zero_number() {
      let zero = self.g.reqs.zero().unwrap();
      for (i, ets) in self.terms.enum_iter() {
        assert!(!ets.eq_class.is_empty()); // TODO: is this true?
        if !ets.eq_class.is_empty() && ets.supercluster.find0(&self.g.constrs, zero, true) {
          to_y_term.push((i, Term::Functor { nr: zero_number, args: Box::new([]) }))
        }
      }
      self.drain_pending(&mut to_y_term, &mut eqs)?;
    }

    // InitStructuresInEqClass
    for (i, mut tm) in to_y_term.drain(..) {
      self.y(|y| y.visit_term(&mut tm))?;
      eqs.insert(i, self.lc.marks[tm.mark().unwrap()].1)
    }

    for ets in self.terms.0.iter() {
      assert!(!ets.eq_class.is_empty()); // TODO: is this true?
      if !ets.eq_class.is_empty() {
        let ei = self.lc.marks[ets.mark].1;
        let mut strict_struct = None;
        for attr in ets.supercluster.try_attrs().unwrap() {
          if attr.is_strict(&self.g.constrs) {
            let TypeKind::Struct(s) = self.g.constrs.attribute[attr.nr].ty.kind else { panic!() };
            if matches!(strict_struct.replace(s), Some(old) if old != s) {
              return Err(Unsat)
            }
          }
        }
        if let Some(s) = strict_struct {
          for ty in &ets.ty_class {
            if ty.kind == TypeKind::Struct(s) {
              to_y_term.push((ei, Term::mk_aggr(self.g, self.lc, s, &Term::EqMark(ets.mark), ty)))
            }
          }
        }
      }
    }
    let mut eqs2 = Equals::default();
    self.drain_pending(&mut to_y_term, &mut eqs2)?;
    for (x, y) in eqs2.0 {
      self.union_terms(x, y)?
    }
    // vprintln!("drain_pending -> {eqs:?}");

    self.process_reductions()?;

    // InitSuperClusterForComplex
    if let Some(complex) = self.g.reqs.complex() {
      let mut to_complex = vec![];
      for (et, etm) in self.terms.enum_iter() {
        for &m in &etm.eq_class {
          match self.lc.marks[m].0 {
            Term::Infer(_) if etm.number.is_some() => to_complex.push(et),
            Term::Functor { nr, ref args } =>
              if let Some(Some(
                Requirement::ImaginaryUnit
                | Requirement::RealNeg
                | Requirement::RealInv
                | Requirement::RealAdd
                | Requirement::RealMult
                | Requirement::RealDiff
                | Requirement::RealDiv,
              )) = self.g.reqs.rev.get(nr)
              {
                for arg1 in &**args {
                  let et1 = self.lc.marks[arg1.mark().unwrap()].1;
                  to_complex.push(self.lc.marks[self.terms[et1].mark].1);
                }
                to_complex.push(et);
              },
            _ => {}
          }
        }
      }
      for et in to_complex {
        let attr = Attr::new0(complex, true);
        self.terms[et].supercluster.try_insert(&self.g.constrs, self.lc, attr)?;
      }
    }

    // UnionEqualsForNonComplex
    let eqs_orig = eqs.0.len();
    if let Some(complex) = self.g.reqs.complex() {
      let mut unsat = Ok(());
      eqs.0.retain(|&(x, y)| {
        let et1 = self.lc.marks[self.terms[x].mark].1;
        let et2 = self.lc.marks[self.terms[y].mark].1;
        if self.terms[et1].supercluster.find0(&self.g.constrs, complex, true)
          || self.terms[et2].supercluster.find0(&self.g.constrs, complex, true)
        {
          unsat = unsat.and_then(|_| {
            let attr = Attr::new0(complex, true);
            self.terms[et1].supercluster.try_insert(&self.g.constrs, self.lc, attr)?;
            let attr = Attr::new0(complex, true);
            self.terms[et2].supercluster.try_insert(&self.g.constrs, self.lc, attr)?;
            Ok(())
          });
          true
        } else {
          unsat = unsat.and_then(|_| self.union_terms(x, y));
          false
        }
      });
      unsat?;
    } else {
      for (x, y) in std::mem::take(&mut eqs.0) {
        self.union_terms(x, y)?;
      }
    }
    if eqs.0.len() != eqs_orig {
      self.identities(false)?
    }

    // InitPolynomialValues
    if let Some(complex) = self.g.reqs.complex() {
      let mut pending = BTreeSet::new();
      for et in (0..self.terms.len()).map(EqTermId::from_usize) {
        let etm = &mut self.terms[et];
        if !etm.eq_class.is_empty() {
          for i in 0..etm.eq_class.len() {
            let etm = &mut self.terms[et];
            let m = etm.eq_class[i];
            match self.lc.marks[m].0 {
              Term::SchFunc { .. }
              | Term::Aggregate { .. }
              | Term::PrivFunc { .. }
              | Term::Selector { .. }
              | Term::Fraenkel { .. }
              | Term::EqClass(_) =>
                if etm.supercluster.find0(&self.g.constrs, complex, true) {
                  etm.eq_polys.insert(Polynomial::single(Monomial::atom(et)));
                },
              Term::Infer(_) =>
                if let Some(n) = &etm.number {
                  etm.eq_polys.insert(Polynomial::single(Monomial::cnst(n.clone())));
                  pending.insert(et);
                } else if etm.supercluster.find0(&self.g.constrs, complex, true)
                  && etm.eq_polys.is_empty()
                {
                  etm.eq_polys.insert(Polynomial::single(Monomial::atom(et)));
                },
              Term::Functor { nr, ref args } => {
                macro_rules! op {
                  (@poly $m:expr) => {
                    Polynomial::single(Monomial::atom(
                      self.lc.marks[self.terms[self.lc.marks[$m].1].mark].1,
                    ))
                  };
                  (|$x:ident| $e:expr) => {{
                    let [Term::EqMark(m1)] = **args else { unreachable!() };
                    let $x = op!(@poly m1);
                    self.terms[et].eq_polys.insert($e);
                    pending.insert(et);
                  }};
                  (|$x:ident, $y:ident| $e:expr) => {{
                    let [Term::EqMark(m1), Term::EqMark(m2)] = **args else { unreachable!() };
                    let ($x, $y) = (op!(@poly m1), op!(@poly m2));
                    self.terms[et].eq_polys.insert($e);
                    pending.insert(et);
                  }};
                }
                match self.g.reqs.rev.get(nr).copied().flatten() {
                  Some(Requirement::ImaginaryUnit) => {
                    assert!(etm.number.is_some());
                    etm.eq_polys.insert(Polynomial::single(Monomial::cnst(Complex::I)));
                    pending.insert(et);
                  }
                  Some(Requirement::RealAdd) => op!(|p1, p2| p1 + p2),
                  Some(Requirement::RealMult) => op!(|p1, p2| &p1 * &p2),
                  Some(Requirement::RealDiff) => op!(|p1, p2| p1 - p2),
                  Some(Requirement::RealNeg) => op!(|p1| p1 * &Complex::NEG_ONE),
                  Some(Requirement::RealInv) => {
                    etm.eq_polys.insert(Polynomial::single(Monomial::atom(et)));
                  }
                  Some(Requirement::RealDiv) => {
                    let [Term::EqMark(m1), Term::EqMark(m2)] = **args else { unreachable!() };
                    let et1 = self.lc.marks[self.terms[self.lc.marks[m1].1].mark].1;
                    let et2 = self.lc.marks[self.terms[self.lc.marks[m2].1].mark].1;
                    if let Some(p) = self.terms[et2].number.clone().filter(|p| *p != Complex::ZERO)
                    {
                      let q = Polynomial::single(Monomial::atom(et1)) * &p.inv();
                      self.terms[et].eq_polys.insert(q);
                      pending.insert(et);
                    } else {
                      self.terms[et].eq_polys.insert(Polynomial::single(Monomial::atom(et)));
                    }
                  }
                  _ =>
                    if let Some(n) = &etm.number {
                      etm.eq_polys.insert(Polynomial::single(Monomial::cnst(n.clone())));
                    } else if etm.supercluster.find0(&self.g.constrs, complex, true)
                      && etm.eq_polys.is_empty()
                    {
                      etm.eq_polys.insert(Polynomial::single(Monomial::atom(et)));
                    },
                }
              }
              Term::The { .. } => {}
              _ => unreachable!(),
            }
          }
          if self.terms[et].eq_polys.len() > 1 {
            self.terms[et].eq_polys.retain(|p| p.is_var() != Some(et))
          }
        }
      }
      self.subst_pending_vars(pending)?
    }
    // vprintln!("after init polynomials");
    // for (et, etm) in self.terms.enum_iter() {
    //   vprintln!("state: {et:?}' {:#?}", etm);
    // }

    // SubstituteSettings
    let mut pending = BTreeSet::new();
    for (x, y) in settings.0 {
      let et1 = self.lc.marks[self.terms[x].mark].1;
      let et2 = self.lc.marks[self.terms[y].mark].1;
      if !self.terms[et1].eq_polys.is_empty() {
        if let Some(p) = self.terms[et2].eq_polys.first() {
          self.subst_var(x, &p.clone(), Some(&mut pending))?;
        }
      }
      self.union_terms(x, y)?
    }
    self.subst_pending_vars(pending)?;

    self.clear_polynomial_values()?;
    self.equate_polynomial_values(&mut eqs)?;
    self.equate_polynomials()?;
    self.clear_polynomial_values()?;

    self.process_linear_equations(&mut eqs)?;

    for (x, y) in eqs.0 {
      self.union_terms(x, y)?
    }
    self.equate_polynomials()?;
    loop {
      self.clear_polynomial_values()?;
      self.identities(true)?;
      self.equate_polynomials()?;
      if !self.clash {
        break
      }
    }

    // vprintln!("before renumber");
    // for (et, etm) in self.terms.enum_iter() {
    //   vprintln!("state: {et:?}' {:#?}", etm);
    // }

    // RenumEqClasses
    let mut eq_class = EqClassId::default();
    for etm in &mut self.terms.0 {
      if !etm.eq_class.is_empty() {
        // let old = etm.id;
        etm.id = eq_class.fresh();
        // vprintln!("renumber e{old:?} -> e{:?}", etm.id);
        self.lc.marks[etm.mark].0 = Term::EqClass(etm.id)
      }
    }
    for etm in &self.terms.0 {
      let et = self.lc.marks[etm.mark].1;
      let Term::EqClass(ec) = self.lc.marks[self.terms[et].mark].0 else { unreachable!() };
      self.lc.marks[etm.mark].0 = Term::EqClass(ec);
    }
    for i in 0..self.terms.0.len() {
      let etm = &mut self.terms.0[i];
      if !etm.eq_class.is_empty() {
        let Attrs::Consistent(sc) = std::mem::take(&mut etm.supercluster) else { unreachable!() };
        for mut a in sc {
          for tm in a.args.iter_mut() {
            let Term::EqMark(m) = tm else { unreachable!() };
            *m = self.terms[self.lc.marks[*m].1].mark
          }
          self.terms.0[i].supercluster.try_insert(&self.g.constrs, self.lc, a)?;
        }
      }
    }

    // vprintln!("after renumber");
    // for (et, etm) in self.terms.enum_iter() {
    //   vprintln!("state: {et:?}' {:#?}", etm);
    // }

    // ContradictionVerify
    for neg in &neg_bas.0 .0 {
      match neg {
        Formula::Attr { nr, args } => self.check_neg_attr(*nr, args)?,
        Formula::Pred { nr, args } => {
          let props = self.g.constrs.predicate[*nr].properties;
          if props.get(PropertyKind::Reflexivity)
            && self.lc.marks[args[props.arg1 as usize].mark().unwrap()].1
              == self.lc.marks[args[props.arg2 as usize].mark().unwrap()].1
          {
            return Err(Unsat)
          }
        }
        _ => {}
      }
      match neg {
        Formula::Attr { .. }
        | Formula::SchPred { .. }
        | Formula::PrivPred { .. }
        | Formula::Pred { .. } => {
          if pos_bas.0 .0.iter().any(|pos| EqMarks.eq(self.g, self.lc, pos, neg)) {
            return Err(Unsat)
          }
        }
        Formula::Is { term, ty } => {
          for ty2 in &self.terms[self.lc.marks[term.mark().unwrap()].1].ty_class {
            if self.with_eq(|ctx| EqMarks.eq_radices(ctx, ty2, ty)) {
              return Err(Unsat)
            }
          }
        }
        _ => {}
      }
    }

    for neg in &neg_bas.0 .0 {
      if let Formula::Pred { nr, args } = neg {
        let props = self.g.constrs.predicate[*nr].properties;
        if props.get(PropertyKind::Reflexivity) {
          let et1 = self.lc.marks[args[props.arg1 as usize].mark().unwrap()].1;
          let et2 = self.lc.marks[args[props.arg2 as usize].mark().unwrap()].1;
          self.nonempty_nonzero_of_ne(et1, et2)?;
        }
      }
    }

    loop {
      let mut added = false;
      // vprintln!("start pos loop");
      // for (et, etm) in self.terms.enum_iter() {
      //   vprintln!("state: {et:?}' {:#?}", etm);
      // }
      for pos in &pos_bas.0 .0 {
        if let Formula::Pred { nr, args } = pos {
          let (nr, args) = Formula::adjust_pred(*nr, args, Some(&self.g.constrs));
          if self.g.reqs.less_or_equal() == Some(nr) {
            let [arg1, arg2] = args else { unreachable!() };
            let et1 = self.lc.marks[arg1.mark().unwrap()].1;
            let et2 = self.lc.marks[arg2.mark().unwrap()].1;
            if let (Some(positive), Some(negative)) =
              (self.g.reqs.positive(), self.g.reqs.negative())
            {
              // a <= b, a is positive => b is positive
              let pos1 = self.terms[et1].supercluster.find0(&self.g.constrs, positive, true);
              added |= pos1
                && self.terms[et2].supercluster.try_insert(
                  &self.g.constrs,
                  self.lc,
                  Attr::new0(positive, true),
                )?;
              // a <= b, b is negative => a is negative
              let neg2 = self.terms[et2].supercluster.find0(&self.g.constrs, negative, true);
              added |= neg2
                && self.terms[et1].supercluster.try_insert(
                  &self.g.constrs,
                  self.lc,
                  Attr::new0(negative, true),
                )?;
              // a <= b, a is non negative => b is non negative
              let nonneg1 = self.terms[et1].supercluster.find0(&self.g.constrs, negative, false);
              added |= nonneg1
                && self.terms[et2].supercluster.try_insert(
                  &self.g.constrs,
                  self.lc,
                  Attr::new0(negative, false),
                )?;
              // a <= b, b is non positive => a is non positive
              let nonpos2 = self.terms[et2].supercluster.find0(&self.g.constrs, positive, false);
              added |= nonpos2
                && self.terms[et1].supercluster.try_insert(
                  &self.g.constrs,
                  self.lc,
                  Attr::new0(positive, false),
                )?;
              if let Some(zero) = self.g.reqs.zero() {
                // a <= b, a is non negative, b is non zero => b is positive
                if nonneg1 && self.terms[et2].supercluster.find0(&self.g.constrs, zero, false) {
                  added |= self.terms[et2].supercluster.try_insert(
                    &self.g.constrs,
                    self.lc,
                    Attr::new0(positive, true),
                  )?;
                }
                // a <= b, b is non positive, a is non zero => a is negative
                if nonpos2 && self.terms[et1].supercluster.find0(&self.g.constrs, zero, false) {
                  added |= self.terms[et2].supercluster.try_insert(
                    &self.g.constrs,
                    self.lc,
                    Attr::new0(negative, true),
                  )?;
                }
              }
            }
            if let (Some(n1), Some(n2)) = (&self.terms[et1].number, &self.terms[et2].number) {
              if n1.im == Rational::ZERO && n2.im == Rational::ZERO && n1.re > n2.re {
                return Err(Unsat)
              }
            }
          } else if self.g.reqs.belongs_to() == Some(nr) {
            let [arg1, arg2] = args else { unreachable!() };
            let et1 = self.lc.marks[arg1.mark().unwrap()].1;
            let et2 = self.lc.marks[arg2.mark().unwrap()].1;
            if let Some(empty) = self.g.reqs.empty() {
              // A in B => B is non empty
              added |= self.terms[et2].supercluster.try_insert(
                &self.g.constrs,
                self.lc,
                Attr::new0(empty, false),
              )?;
            }
            if let Some(element) = self.g.reqs.element() {
              // A in B => A: Element of B
              let ty = Type { args: vec![arg2.clone()], ..Type::new(element.into()) };
              self.insert_type(ty, et1)?;
            }
          } else if self.g.reqs.inclusion() == Some(nr) {
            if let (Some(element), Some(pw)) = (self.g.reqs.element(), self.g.reqs.power_set()) {
              let [arg1, arg2] = args else { unreachable!() };
              // A c= B => A: Element of bool B
              let mut tm = Term::Functor { nr: pw, args: Box::new([arg2.clone()]) };
              self.y(|y| y.visit_term(&mut tm))?;
              let ty = Type { args: vec![tm], ..Type::new(element.into()) };
              self.insert_type(ty, self.lc.marks[arg1.mark().unwrap()].1)?;
            }
          }
        }
      }
      if !added {
        break
      }
    }

    loop {
      let mut added = false;
      // vprintln!("start element transitivity loop");
      // for (et, etm) in self.terms.enum_iter() {
      //   vprintln!("state: {et:?}' {:#?}", etm);
      // }
      for pos2 in &pos_bas.0 .0 {
        if let Formula::Pred { nr, args } = pos2 {
          let (nr, args) = Formula::adjust_pred(*nr, args, Some(&self.g.constrs));
          if self.g.reqs.belongs_to() == Some(nr) {
            let [arg1, arg2] = args else { unreachable!() };
            let et2 = self.lc.marks[arg2.mark().unwrap()].1;
            let mut to_push = vec![];
            for ty in &self.terms[et2].ty_class {
              if let TypeKind::Mode(n) = ty.kind {
                let (n, args) = Type::adjust(n, &ty.args, &self.g.constrs);
                if self.g.reqs.element() == Some(n) {
                  let [arg3] = args else { unreachable!() };
                  for &m in &self.terms[self.lc.marks[arg3.mark().unwrap()].1].eq_class {
                    if let Term::Functor { nr, args } = &self.lc.marks[m].0 {
                      if self.g.reqs.power_set() == Some(*nr) {
                        let [arg4] = &**args else { unreachable!() };
                        // a in b, b: Element of bool C => C is non empty, a: Element of C
                        to_push.push(arg4.mark().unwrap());
                      }
                    }
                  }
                }
              }
            }
            if let Some(empty) = self.g.reqs.empty() {
              for &m in &to_push {
                let et = self.lc.marks[m].1;
                self.terms[et].supercluster.try_insert(
                  &self.g.constrs,
                  self.lc,
                  Attr::new0(empty, false),
                )?;
              }
            }
            if let Some(element) = self.g.reqs.element() {
              let et1 = self.lc.marks[arg1.mark().unwrap()].1;
              for &m in &to_push {
                let ty = Type { args: vec![Term::EqMark(m)], ..Type::new(element.into()) };
                added |= self.insert_type(ty, et1)?;
              }
            }
          }
        }
      }
      if !added {
        break
      }
    }

    loop {
      let mut added = false;
      // vprintln!("start neg loop");
      // for (et, etm) in self.terms.enum_iter() {
      //   vprintln!("state: {et:?}' {:#?}", etm);
      // }
      for neg in &neg_bas.0 .0 {
        match neg {
          Formula::Attr { nr, args } => {
            self.check_neg_attr(*nr, args)?;
            self.match_formulas(neg, &pos_bas)?
          }
          Formula::SchPred { .. } | Formula::PrivPred { .. } =>
            self.match_formulas(neg, &pos_bas)?,
          Formula::Pred { nr, args } => {
            let (nr, args) = Formula::adjust_pred(*nr, args, Some(&self.g.constrs));
            if self.g.reqs.less_or_equal() == Some(nr) {
              let [arg1, arg2] = args else { unreachable!() };
              let et1 = self.lc.marks[arg1.mark().unwrap()].1;
              let et2 = self.lc.marks[arg2.mark().unwrap()].1;
              if let (Some(positive), Some(negative)) =
                (self.g.reqs.positive(), self.g.reqs.negative())
              {
                // b < a, a is non positive => b is negative
                added |= self.terms[et1].supercluster.find0(&self.g.constrs, positive, false)
                  && self.terms[et2].supercluster.try_insert(
                    &self.g.constrs,
                    self.lc,
                    Attr::new0(negative, true),
                  )?;
                // b < a, b is non negative => a is positive
                added |= self.terms[et2].supercluster.find0(&self.g.constrs, negative, false)
                  && self.terms[et1].supercluster.try_insert(
                    &self.g.constrs,
                    self.lc,
                    Attr::new0(positive, true),
                  )?;
              }
              if let (Some(n1), Some(n2)) = (&self.terms[et1].number, &self.terms[et2].number) {
                if n1.im == Rational::ZERO && n2.im == Rational::ZERO && n1.re <= n2.re {
                  return Err(Unsat)
                }
              }
            } else if self.g.reqs.belongs_to() == Some(nr) {
              if let (Some(element), Some(empty)) = (self.g.reqs.element(), self.g.reqs.empty()) {
                let [arg1, arg2] = args else { unreachable!() };
                let et1 = self.lc.marks[arg1.mark().unwrap()].1;
                let et2 = self.lc.marks[arg2.mark().unwrap()].1;
                if self.terms[et2].supercluster.find0(&self.g.constrs, empty, false) {
                  let ty = Type { args: vec![arg2.clone()], ..Type::new(element.into()) };
                  // B is non empty, A: Element of B => A in B
                  if self.terms[et1].ty_class.iter().any(|ty2| {
                    ty2.decreasing_attrs(&ty, |a1, a2| EqMarks.eq(self.g, self.lc, a1, a2))
                      && self.with_eq(|ctx| EqMarks.eq_radices(ctx, &ty, ty2))
                  }) {
                    return Err(Unsat)
                  }
                }
              }
            } else if self.g.reqs.inclusion() == Some(nr) {
              if let (Some(element), Some(pw)) = (self.g.reqs.element(), self.g.reqs.power_set()) {
                let [arg1, arg2] = args else { unreachable!() };
                let et1 = self.lc.marks[arg1.mark().unwrap()].1;
                let mut tm = Term::Functor { nr: pw, args: Box::new([arg2.clone()]) };
                self.y(|y| y.visit_term(&mut tm))?;
                let ty = Type { args: vec![tm], ..Type::new(element.into()) };
                // A: Element of bool B => A c= B
                if self.terms[et1].ty_class.iter().any(|ty2| {
                  ty2.decreasing_attrs(&ty, |a1, a2| EqMarks.eq(self.g, self.lc, a1, a2))
                    && self.with_eq(|ctx| EqMarks.eq_radices(ctx, &ty, ty2))
                }) {
                  return Err(Unsat)
                }
              }
            }
            for pos in &pos_bas.0 .0 {
              if EqMarks.eq(self.g, self.lc, neg, pos) {
                return Err(Unsat)
              }
            }
          }
          Formula::Is { term, ty } => {
            let et = self.lc.marks[term.mark().unwrap()].1;
            if self.with_eq(|ctx| {
              self.terms[et].ty_class.iter().any(|ty2| EqMarks.eq_radices(ctx, ty, ty2))
            }) {
              return Err(Unsat)
            }
          }
          _ => {}
        }
      }
      if !added {
        break
      }
    }

    let mut eq_stack: BTreeSet<EqTermId> =
      self.terms.enum_iter().filter(|p| !p.1.eq_class.is_empty()).map(|p| p.0).collect();

    // InitAllowedClusters
    let allowed = AllowedClusters {
      ccl: (self.g.clusters.conditional.iter())
        .map(|cl| self.filter_allowed(&cl.consequent.1))
        .enumerate()
        .filter(|attrs| !attrs.1.attrs().is_empty())
        .collect(),
      fcl: (self.g.clusters.functor.vec.0.iter())
        .map(|cl| self.filter_allowed(&cl.consequent.1))
        .enumerate()
        .filter(|attrs| !attrs.1.attrs().is_empty())
        .collect(),
    };

    while let Some(i) = eq_stack.pop_first() {
      // RoundUpSuperCluster
      if self.terms[i].eq_class.is_empty() {
        continue
      }
      // vprintln!("round up superclusters {i:?}' {:#?}", self.terms[i]);
      let mut progress = false;
      loop {
        let mut added = false;
        for &(j, ref attrs) in &allowed.ccl {
          let cl = &self.g.clusters.conditional.vec[j];
          // vprintln!("\nround up [{j}] = {cl:?}\n in {:?}", self.terms[i]);
          let inst = self.instantiate(&cl.primary);
          let mut r = inst.inst_type(&cl.ty, i);
          inst.and_inst_attrs(&cl.antecedent, i, &mut r);
          added |= self.round_up_one_supercluster(i, attrs, &r)?;
        }
        for &(j, ref attrs) in &allowed.fcl {
          let cl = &self.g.clusters.functor.vec[j];
          // vprintln!("\nround up [{j}] = {cl:#?}\n in {:?}", self.terms[i]);
          let inst = self.instantiate(&cl.primary);
          let mut r = inst.inst_term(&cl.term, &Term::EqMark(self.terms[i].mark));
          if let Some(ty) = &cl.ty {
            r.mk_and_then(|| Ok(inst.inst_type(ty, i))).unwrap()
          }
          added |= self.round_up_one_supercluster(i, attrs, &r)?;
        }
        if !added {
          break
        }
        progress = true
      }

      if progress {
        for (j, etm) in self.terms.enum_iter() {
          if self.depends_on(etm, i) {
            eq_stack.insert(j);
          }
        }
      }
    }
    // vprintln!("after round up");
    // for (et, etm) in self.terms.enum_iter() {
    //   vprintln!("state: {et:?}' {:#?}", etm);
    // }

    // PreUnification
    let mut ineqs = Ineqs::default();
    for f in &neg_bas.0 .0 {
      if let Formula::Pred { nr, args } = f {
        if self.g.reqs.equals_to() == Some(*nr) {
          let [arg1, arg2] = &**args else { unreachable!() };
          ineqs.push(arg1.mark().unwrap(), arg2.mark().unwrap());
        }
      }
    }
    ineqs.base = ineqs.ineqs.len();
    self.check_refl(&pos_bas, PropertyKind::Irreflexivity, &mut ineqs)?;
    self.check_refl(&neg_bas, PropertyKind::Reflexivity, &mut ineqs)?;
    ineqs.process(self, &mut neg_bas)?;
    for (etm1, etm2) in (self.terms.0.iter())
      .filter(|etm| !etm.eq_class.is_empty() && !etm.supercluster.attrs().is_empty())
      .tuple_combinations()
    {
      if etm1.supercluster.contradicts(&self.g.constrs, self.lc, &etm2.supercluster) {
        ineqs.push(etm1.mark, etm2.mark)
      }
    }
    for f in &neg_bas.0 .0 {
      match f {
        Formula::Pred { nr, args } => {
          let (nr, args) = Formula::adjust_pred(*nr, args, Some(&self.g.constrs));
          let props = self.g.constrs.predicate[nr].properties;
          if props.get(PropertyKind::Reflexivity) {
            ineqs.process_ineq(
              self,
              args[props.arg1 as usize].mark().unwrap(),
              args[props.arg2 as usize].mark().unwrap(),
            );
          }
          if self.g.reqs.equals_to() != Some(nr) {
            for f2 in &pos_bas.0 .0 {
              if let Formula::Pred { nr: nr2, args: args2 } = f2 {
                let (nr2, args2) = Formula::adjust_pred(*nr2, args2, Some(&self.g.constrs));
                if nr == nr2 {
                  ineqs.push_if_one_diff(&self.lc.marks, args, args2)
                }
              }
            }
          }
        }
        Formula::SchPred { .. } | Formula::Attr { .. } | Formula::PrivPred { .. } => {
          for f2 in &pos_bas.0 .0 {
            let (args1, args2) = match (f, f2) {
              (
                Formula::SchPred { nr: n1, args: args1 },
                Formula::SchPred { nr: n2, args: args2 },
              ) if n1 == n2 => (args1, args2),
              (Formula::Attr { nr: n1, args: args1 }, Formula::Attr { nr: n2, args: args2 })
                if n1 == n2 =>
                (args1, args2),
              (
                Formula::PrivPred { nr: n1, args: args1, .. },
                Formula::PrivPred { nr: n2, args: args2, .. },
              ) if n1 == n2 => (args1, args2),
              _ => continue,
            };
            ineqs.push_if_one_diff(&self.lc.marks, args1, args2)
          }
        }
        Formula::Is { term, ty } => {
          let adj1 = match ty.kind {
            TypeKind::Mode(n) => Some(Type::adjust(n, &ty.args, &self.g.constrs)),
            TypeKind::Struct(_) => None,
          };
          let m1 = term.mark().unwrap();
          let et1 = self.lc.marks[m1].1;
          for ty2 in &self.terms[et1].ty_class {
            if let (Some((n1, args1)), TypeKind::Mode(n2)) = (adj1, ty2.kind) {
              let (n2, args2) = Type::adjust(n2, &ty2.args, &self.g.constrs);
              if n1 == n2 {
                ineqs.push_if_one_diff(&self.lc.marks, args1, args2)
              }
            }
          }
          for (et2, etm2) in self.terms.enum_iter() {
            if et2 != et1
              && !etm2.eq_class.is_empty()
              && self
                .with_eq(|ctx| etm2.ty_class.iter().any(|ty2| EqMarks.eq_radices(ctx, ty, ty2)))
            {
              ineqs.push(m1, etm2.mark);
            }
          }
        }
        _ => {}
      }
    }
    ineqs.process(self, &mut neg_bas)?;

    Ok(EnumMap::from_array([neg_bas, pos_bas]))
  }
}

#[derive(Default)]
struct Ineqs {
  ineqs: Vec<(EqMarkId, EqMarkId)>,
  processed: usize,
  base: usize,
}

impl Ineqs {
  fn push(&mut self, a: EqMarkId, b: EqMarkId) {
    let (a, b) = match a.cmp(&b) {
      Ordering::Less => (a, b),
      Ordering::Equal => unreachable!(),
      Ordering::Greater => (b, a),
    };
    if !self.ineqs.contains(&(a, b)) {
      // vprintln!("push: {:?} != {:?}", Term::EqMark(a), Term::EqMark(b));
      self.ineqs.push((a, b));
    }
  }

  fn push_if_one_diff(
    &mut self, marks: &IdxVec<EqMarkId, (Term, EqTermId)>, tms1: &[Term], tms2: &[Term],
  ) {
    let mut it = tms1
      .iter()
      .zip(tms2)
      .map(|(a, b)| (a.mark().unwrap(), b.mark().unwrap()))
      .filter(|&(a, b)| marks[a].1 != marks[b].1);
    if let (Some((a, b)), None) = (it.next(), it.next()) {
      self.push(a, b)
    }
  }

  fn process_ineq(&mut self, eq: &Equalizer<'_>, a: EqMarkId, b: EqMarkId) {
    // vprintln!("process: {:?} != {:?}", Term::EqMark(a), Term::EqMark(b));
    // for (et, etm) in eq.terms.enum_iter() {
    //   vprintln!("process {et:?}' {:#?}", etm);
    // }
    let et1 = eq.lc.marks[a].1;
    let et2 = eq.lc.marks[b].1;
    for &m1 in &eq.terms[et1].eq_class {
      let tm1 = &eq.lc.marks[m1].0;
      match tm1 {
        Term::Functor { .. }
        | Term::SchFunc { .. }
        | Term::PrivFunc { .. }
        | Term::Aggregate { .. }
        | Term::Selector { .. } => {}
        _ => continue,
      }
      for &m2 in &eq.terms[et2].eq_class {
        let (args1, args2) = match (tm1, &eq.lc.marks[m2].0) {
          (Term::Functor { nr: n1, args: args1 }, Term::Functor { nr: n2, args: args2 })
            if n1 == n2 =>
            (args1, args2),
          (Term::SchFunc { nr: n1, args: args1 }, Term::SchFunc { nr: n2, args: args2 })
            if n1 == n2 =>
            (args1, args2),
          (
            Term::PrivFunc { nr: n1, args: args1, .. },
            Term::PrivFunc { nr: n2, args: args2, .. },
          ) if n1 == n2 => (args1, args2),
          (Term::Aggregate { nr: n1, args: args1 }, Term::Aggregate { nr: n2, args: args2 })
            if n1 == n2 =>
            (args1, args2),
          (Term::Selector { nr: n1, args: args1 }, Term::Selector { nr: n2, args: args2 })
            if n1 == n2 =>
            (args1, args2),
          _ => continue,
        };
        self.push_if_one_diff(&eq.lc.marks, args1, args2)
      }
    }
  }

  fn process(&mut self, eq: &mut Equalizer<'_>, neg_bas: &mut Atoms) -> OrUnsat<()> {
    while let Some(&(a, b)) = self.ineqs.get(self.processed) {
      eq.nonempty_nonzero_of_ne(eq.lc.marks[a].1, eq.lc.marks[b].1)?;
      if self.processed >= self.base {
        neg_bas.0.push(Formula::Pred {
          nr: eq.g.reqs.equals_to().unwrap(),
          args: Box::new([Term::EqMark(a), Term::EqMark(b)]),
        });
      }
      self.processed += 1;
      self.process_ineq(eq, a, b);
    }
    Ok(())
  }
}
