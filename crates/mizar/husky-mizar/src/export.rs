use crate::accom::SigBuilder;
use crate::analyze::Analyzer;
use crate::parser::{catch_missing, MaybeMut, PathResult};
use crate::reader::DefiniensId;
use crate::types::*;
use crate::{Assignment, LocalContext, OnVarMut, VisitMut};
use enum_map::EnumMap;
use itertools::Itertools;
use std::fmt::Debug;

const DOUBLE_CHECK: bool = false;

#[derive(Default)]
pub struct Exporter {
  pub constrs_base: ConstructorsBase,
  pub clusters_base: ClustersBase,
  pub definitions_base: DefiniensId,
  pub identify_base: u32,
  pub reductions_base: u32,
  pub properties_base: u32,
  pub theorems: Vec<Theorem>,
  pub schemes: Vec<Option<SchId>>,
}

#[allow(clippy::panic)]
fn assert_eq_iter<T: Debug + PartialEq<U>, U: Debug>(
  header: &str, mut it1: impl Iterator<Item = T> + Clone, mut it2: impl Iterator<Item = U> + Clone,
) {
  if !it1.clone().eq(it2.clone()) {
    eprintln!("failure in {header}:");
    for i in 0.. {
      match (it1.next(), it2.next()) {
        (None, None) => break,
        (Some(x1), Some(x2)) if x1 == x2 => eprintln!("{i}: both: {x1:?}"),
        (a, b) => eprintln!("{i}: mismatch:\n{a:?}\n{b:?}\n"),
      }
    }
    panic!("failure in {header}");
  }
}

struct ExportPrep<'a> {
  ctx: Option<&'a Constructors>,
  lc: &'a LocalContext,
  ic: &'a IdxVec<InferId, Assignment>,
  depth: u32,
}
impl VisitMut for ExportPrep<'_> {
  fn push_bound(&mut self, _: &mut Type) { self.depth += 1 }
  fn pop_bound(&mut self, n: u32) { self.depth -= n }
  fn visit_term(&mut self, tm: &mut Term) {
    if let Term::Infer(nr) = *tm {
      *tm = self.ic[nr].def.visit_cloned(&mut OnVarMut(|v| *v += self.depth));
    }
    self.super_visit_term(tm);
  }
  fn visit_attrs(&mut self, attrs: &mut Attrs) {
    attrs.reinsert_all(self.ctx, self.lc, true, |attr| self.visit_terms(&mut attr.args))
  }
  fn visit_attr_pair(&mut self, attrs: &mut (Attrs, Attrs)) {
    self.visit_attrs(&mut attrs.0);
    attrs.1.clone_from(&attrs.0);
  }
}

impl<'a> ExportPrep<'a> {
  fn with_ctx(&mut self, ctx: Option<&Constructors>, f: impl FnOnce(&mut ExportPrep<'_>)) {
    f(&mut ExportPrep { ctx, ..*self });
  }
}

fn mark_formats<T>(
  vocs: &Vocabularies, marked_vocs: &mut Vocabularies, fmts: &mut [T],
  get: impl Fn(&mut T) -> &mut Format,
) {
  let mut trans = EnumMap::<_, Vec<_>>::default();
  let (mut hi, mut new) = <(SymbolsBase, SymbolsBase)>::default();
  for (_, (art, counts)) in vocs.0.iter().enumerate() {
    let lo = hi;
    hi += counts;
    let used = fmts.iter_mut().any(|fmt| {
      let mut used = false;
      get(fmt).visit(|k, sym| used |= (lo.0[k]..hi.0[k]).contains(&sym));
      used
    });
    if used {
      marked_vocs.0.push((*art, *counts));
      for (kind, &count) in &counts.0 {
        trans[kind].extend((0..count).map(|i| Some(i + new.0[kind])))
      }
      new += counts;
    } else {
      for (kind, &count) in &counts.0 {
        trans[kind].extend((0..count).map(|_| None))
      }
    }
  }
  #[allow(clippy::unwrap_used, clippy::indexing_slicing)]
  for fmt in fmts {
    get(fmt).visit_mut(|k, sym| *sym = trans[k][*sym as usize].unwrap());
  }
}

#[derive(Debug)]
struct MarkConstr<'a> {
  /// Article, plus the constructor counts *excluding* this article.
  /// The current article may be in the list, in which case it is at the end.
  accum: &'a [(Article, ConstructorsBase)],
  /// The total constructor counts
  base: &'a ConstructorsBase,
  /// This is a parallel array, where `used[i]` corresponds to `accum[i]`,
  /// and `used.last()` is true if constructors from the current article are used.
  /// (Because this always contains an entry for the current article it may either
  /// be the same length or one longer than accum.)
  used: Vec<bool>,
}

impl<'a> MarkConstr<'a> {
  fn new(sig: &'a SigBuilder, n: usize) -> Self {
    Self { accum: &sig.sig.0, base: &sig.base, used: vec![false; n + 1] }
  }

  fn mark(&mut self, n: u32, key: impl Fn(&ConstructorsBase) -> u32) {
    if n < key(self.base) {
      self.used[self.accum[1..].partition_point(|(_, base)| key(base) <= n)] = true
    }
  }

  fn closure(&mut self, constrs: &mut Constructors) {
    let mut base = *self.base;
    // We skip the first two because the first is HIDDEN (which is always used)
    // and the second step can't have any effect
    for (i, (_, lo)) in self.accum.iter().enumerate().skip(2).rev() {
      if self.used[i] {
        constrs.visit_range(self, lo..&base)
      }
      base = *lo
    }
    self.used[0] = true
  }

  fn apply(&self) -> ApplyMarkConstr {
    let (mut offset, mut base) = Default::default();
    let mut apply = ApplyMarkConstr::default();
    let mut prev = true;
    for (accum, &mark) in self.accum.iter().map(|p| &p.1).chain([self.base]).zip(&self.used).skip(1)
    {
      if prev != mark {
        if mark {
          offset += *accum - base;
          apply.0.push((base, offset));
        } else {
          base = *accum;
        }
        prev = mark
      }
    }
    apply
  }

  fn apply_with(&self, f: impl FnOnce(&mut ApplyMarkConstr)) {
    let mut apply = self.apply();
    if !apply.0.is_empty() {
      f(&mut apply)
    }
  }

  fn filtered<T: Copy>(&self, ts: &[T]) -> Vec<T> {
    ts.iter().zip(&self.used).filter(|p| *p.1).map(|p| *p.0).collect_vec()
  }
}

impl VisitMut for MarkConstr<'_> {
  fn visit_mode_id(&mut self, n: &mut ModeId) { self.mark(n.0, |b| b.mode) }
  fn visit_struct_id(&mut self, n: &mut StructId) { self.mark(n.0, |b| b.struct_mode) }
  fn visit_attr_id(&mut self, n: &mut AttrId) { self.mark(n.0, |b| b.attribute) }
  fn visit_pred_id(&mut self, n: &mut PredId) { self.mark(n.0, |b| b.predicate) }
  fn visit_func_id(&mut self, n: &mut FuncId) { self.mark(n.0, |b| b.functor) }
  fn visit_sel_id(&mut self, n: &mut SelId) { self.mark(n.0, |b| b.selector) }
  fn visit_aggr_id(&mut self, n: &mut AggrId) { self.mark(n.0, |b| b.aggregate) }
}

#[derive(Default, Debug)]
struct ApplyMarkConstr(Vec<(ConstructorsBase, ConstructorsBase)>);
impl ApplyMarkConstr {
  fn apply(&mut self, n: &mut u32, key: impl Fn(&ConstructorsBase) -> u32) {
    if let Some(i) = self.0.partition_point(|(base, _)| key(base) <= *n).checked_sub(1) {
      *n -= key(&self.0[i].1)
    }
  }
}

impl VisitMut for ApplyMarkConstr {
  const MODIFY_IDS: bool = true;
  fn visit_mode_id(&mut self, n: &mut ModeId) { self.apply(&mut n.0, |b| b.mode) }
  fn visit_struct_id(&mut self, n: &mut StructId) { self.apply(&mut n.0, |b| b.struct_mode) }
  fn visit_attr_id(&mut self, n: &mut AttrId) { self.apply(&mut n.0, |b| b.attribute) }
  fn visit_pred_id(&mut self, n: &mut PredId) { self.apply(&mut n.0, |b| b.predicate) }
  fn visit_func_id(&mut self, n: &mut FuncId) { self.apply(&mut n.0, |b| b.functor) }
  fn visit_sel_id(&mut self, n: &mut SelId) { self.apply(&mut n.0, |b| b.selector) }
  fn visit_aggr_id(&mut self, n: &mut AggrId) { self.apply(&mut n.0, |b| b.aggregate) }
}

impl AccumConstructors {
  fn mark<T: for<'a> Visitable<MarkConstr<'a>> + Visitable<ApplyMarkConstr>>(
    &mut self, t: &mut T, n: usize, arts: &[Article],
  ) -> Vec<Article> {
    let mut marks = MarkConstr::new(&self.sig, n);
    t.visit(&mut marks);
    marks.closure(&mut self.constrs);
    marks.apply_with(|v| t.visit(v));
    marks.filtered(arts)
  }
}

macro_rules! try_p {
  ($self:expr, $e:expr) => {
    ok_parse_err(&mut $self.r.has_errors, $e)
  };
}
macro_rules! assert_eq_nonempty {
  ($self:expr, $ne:expr, $e:expr) => {
    if let Some(p) = try_p!($self, catch_missing($e)) {
      assert_eq!($ne, p.is_some());
    }
  };
}
fn ok_parse_err<T>(has_errors: &mut bool, val: PathResult<T>) -> Option<T> {
  match val {
    Ok(t) => Some(t),
    Err((path, e)) => {
      e.report(&path);
      *has_errors = true;
      None
    }
  }
}

impl Analyzer<'_> {
  pub fn export(&mut self) {
    // This file deals with expressions after renumbering, so the formatter is liable to panic
    self.r.lc.formatter.cfg.enable_formatter = false;
    let ep = &mut ExportPrep {
      ctx: Some(&self.r.g.constrs),
      lc: &self.r.lc,
      ic: &self.r.lc.infer_const.borrow().vec,
      depth: 0,
    };
    let new_prel = !self.g.cfg.overwrite_prel;

    // loading .sgl
    let mut arts2 = vec![];
    if let Some(accom) = &self.accom {
      arts2.extend(accom.sig.sig.0.iter().map(|p| p.0));
    } else {
      try_p!(self, self.path.read_sgl(&mut arts2));
    }
    let arts1 = if self.g.constrs.since(&self.export.constrs_base).is_empty() {
      &*arts2
    } else {
      let n = arts2.len();
      arts2.push(self.article);
      &arts2[..n]
    };

    // loading .vcl, .aco
    let (mut vocs1, mut aco) = <(Vocabularies, AccumConstructors)>::default();
    if let Some(accom) = &mut self.r.accom {
      accom.build_vocabularies(&mut vocs1);
      aco.sig = std::mem::take(&mut accom.sig);
      aco.constrs.extend(&self.g.constrs.upto(&aco.sig.base));
      aco.constrs.visit(ep);
    } else {
      self.path.read_vcl(&mut vocs1).unwrap();
      self.path.read_aco(&mut aco).unwrap();
    }
    assert_eq!(self.export.constrs_base, aco.constrs.len());
    assert_eq!(arts1.len(), aco.sig.sig.len());

    // validating .dfr
    {
      let mut dfr1 = self.lc.formatter.formats.0[self.formats_base..].to_owned();
      let nonempty = !dfr1.is_empty();
      let (mut marked_vocs, mut vocs2, mut dfr2) = Default::default();
      if self.g.cfg.verify_export {
        assert_eq_nonempty!(self, nonempty, self.path.read_dfr(false, &mut vocs2, &mut dfr2));
      }
      if nonempty {
        mark_formats(&vocs1, &mut marked_vocs, &mut dfr1, |x| x);
        if self.g.cfg.verify_export {
          assert_eq!(marked_vocs, vocs2);
          assert_eq!(dfr1, dfr2.0);
        }
        if self.g.cfg.xml_export {
          self.path.write_dfr(new_prel, &marked_vocs, &dfr1);
          if DOUBLE_CHECK {
            let (mut vocs3, mut dfr3) = Default::default();
            self.path.read_dfr(new_prel, &mut vocs3, &mut dfr3).unwrap();
            assert_eq!(marked_vocs, vocs3);
            assert_eq!(dfr1, dfr3.0);
          }
        }
      }
      if self.g.cfg.cache_prel {
        self.path.with_cache(|c| &c.dfr, (marked_vocs, dfr1));
      }
    }

    // validating .dco (also push current article to aco)
    {
      let since1 = self.r.g.constrs.since(&self.export.constrs_base);
      let nonempty = !since1.is_empty();
      let (mut dco1, mut dco2) = <(DepConstructors, _)>::default();
      if self.g.cfg.verify_export {
        assert_eq_nonempty!(self, nonempty, self.path.read_dco(false, &mut dco2, true));
      }
      if nonempty {
        dco1.constrs = since1.to_owned();
        dco1.counts = dco1.constrs.len();
        dco1.constrs.visit(ep);
        assert_eq!(aco.sig.base, aco.constrs.len());
        aco.constrs.append(&mut dco1.constrs.clone());
        let mut marks = MarkConstr::new(&aco.sig, arts1.len());
        *marks.used.last_mut().unwrap() = true;
        dco1.constrs.visit(&mut marks);
        marks.closure(&mut aco.constrs);
        marks.apply_with(|v| dco1.constrs.visit(v));
        dco1.sig = marks.filtered(arts1);
        if self.g.cfg.verify_export {
          assert_eq!(dco1.sig, dco2.sig);
          assert_eq!(dco1.counts, dco2.counts);
          macro_rules! process { ($($field:ident),*) => {$(
            assert_eq_iter(concat!("constrs.", stringify!($field)),
              dco1.constrs.$field.0.iter(), dco2.constrs.$field.0.iter());
          )*}}
          process!(mode, struct_mode, attribute, predicate, functor, selector, aggregate);
        }
        if self.g.cfg.xml_export {
          self.path.write_dco(new_prel, &aco.sig.base, &dco1);
          if DOUBLE_CHECK {
            let mut dco3 = Default::default();
            self.path.read_dco(new_prel, &mut dco3, true).unwrap();
            assert_eq!(dco1, dco3);
          }
        }
        aco.sig.sig.push((self.article, aco.sig.base));
        aco.sig.base = self.g.constrs.len();
      }
      if self.g.cfg.cache_prel {
        self.path.with_cache(|c| &c.dco, dco1);
      }
    }

    // validating .dno
    {
      let (mut dno1, mut dno2) = <(DepNotation, _)>::default();
      dno1.pats = (self.notations.iter())
        .flat_map(|(i, nota)| &nota[self.notations_base[i] as usize..])
        .map(|pat| {
          let Pattern { article, abs_nr, kind, fmt, primary, visible, pos } = pat.visit_cloned(ep);
          let fmt = self.lc.formatter.formats[fmt];
          Pattern { article, abs_nr, kind, fmt, primary, visible, pos }
        })
        .collect();
      let nonempty = !dno1.pats.is_empty();
      if self.g.cfg.verify_export {
        assert_eq_nonempty!(self, nonempty, self.path.read_dno(false, &mut dno2));
      }
      if nonempty {
        mark_formats(&vocs1, &mut dno1.vocs, &mut dno1.pats, |p| &mut p.fmt);
        let mut marks = MarkConstr::new(&aco.sig, arts1.len());
        dno1.pats.iter_mut().for_each(|p| p.visit(&mut marks));
        marks.closure(&mut aco.constrs);
        marks.apply_with(|v| dno1.pats.iter_mut().for_each(|p| p.visit(v)));
        dno1.sig = marks.filtered(&arts2);
        if self.g.cfg.verify_export {
          assert_eq!(dno1.sig, dno2.sig);
          assert_eq!(dno1.vocs, dno2.vocs);
          assert_eq_iter("notations", dno1.pats.iter(), dno2.pats.iter());
        }
        if self.g.cfg.xml_export {
          self.path.write_dno(new_prel, &dno1);
          if DOUBLE_CHECK {
            let mut dno3 = Default::default();
            self.path.read_dno(new_prel, &mut dno3).unwrap();
            assert_eq!(dno1, dno3);
          }
        }
      }
      if self.g.cfg.cache_prel {
        self.path.with_cache(|c| &c.dno, dno1);
      }
    }

    // validating .dcl
    {
      let (mut dcl1, mut dcl2) = <(DepClusters, _)>::default();
      let since1 = self.r.g.clusters.since(&self.export.clusters_base);
      let nonempty = !since1.is_empty();
      if self.g.cfg.verify_export {
        assert_eq_nonempty!(self, nonempty, self.path.read_dcl(false, &mut dcl2));
      }
      if nonempty {
        dcl1.cl = since1.to_owned();
        dcl1.cl.visit(ep);
        dcl1.sig = aco.mark(&mut dcl1.cl, arts1.len(), &arts2);
        if self.g.cfg.verify_export {
          ep.with_ctx(None, |ep| dcl2.cl.visit(ep));
          assert_eq!(dcl1.sig, dcl2.sig);
          macro_rules! process { ($($field:ident),*) => {$({
            assert_eq_iter(concat!("clusters.", stringify!($field)),
              dcl1.cl.$field.iter(), dcl2.cl.$field.iter());
          })*}}
          process!(registered, functor, conditional);
        }
        if self.g.cfg.xml_export {
          self.path.write_dcl(new_prel, &dcl1);
          if DOUBLE_CHECK {
            let mut dcl3 = Default::default();
            self.path.read_dcl(new_prel, &mut dcl3).unwrap();
            assert_eq!(dcl1, dcl3);
          }
        }
      }
      if self.g.cfg.cache_prel {
        self.path.with_cache(|c| &c.dcl, dcl1);
      }
    }

    // validating .def
    {
      let mut def1 = self.definitions.0[self.export.definitions_base.into_usize()..].to_owned();
      let nonempty = !def1.is_empty();
      let (mut sig1, mut sig, mut def2) = Default::default();
      if self.g.cfg.verify_export {
        assert_eq_nonempty!(self, nonempty, self.path.read_def(false, &mut sig, &mut def2));
      }
      if nonempty {
        def1.visit(ep);
        sig1 = aco.mark(&mut def1, arts1.len(), &arts2);
        if self.g.cfg.verify_export {
          ep.with_ctx(None, |ep| def2.visit(ep));
          assert_eq!(sig1, sig);
          assert_eq_iter("definitions", def1.iter(), def2.iter());
        }
        if self.g.cfg.xml_export {
          self.path.write_def(new_prel, &sig1, &def1);
          if DOUBLE_CHECK {
            let (mut sig3, mut def3) = Default::default();
            (self.path)
              .read_definitions(MaybeMut::None, new_prel, "def", Some(&mut sig3), &mut def3)
              .unwrap();
            ep.with_ctx(None, |ep| def3.visit(ep));
            assert_eq!(sig1, sig3);
            assert_eq!(def1, def3);
          }
        }
      }
      if self.g.cfg.cache_prel {
        self.path.with_cache(|c| &c.def, (sig1, def1));
      }
    }

    // validating .did
    {
      let mut did1 = self.identify[self.export.identify_base as usize..].to_owned();
      let nonempty = !did1.is_empty();
      let (mut sig1, mut sig, mut did2) = Default::default();
      if self.g.cfg.verify_export {
        assert_eq_nonempty!(self, nonempty, self.path.read_did(false, &mut sig, &mut did2));
      }
      if nonempty {
        did1.visit(ep);
        sig1 = aco.mark(&mut did1, arts1.len(), &arts2);
        if self.g.cfg.verify_export {
          ep.with_ctx(None, |ep| did2.visit(ep));
          assert_eq!(sig1, sig);
          assert_eq_iter("identities", did1.iter(), did2.iter());
        }
        if self.g.cfg.xml_export {
          self.path.write_did(new_prel, &sig1, &did1);
          if DOUBLE_CHECK {
            let (mut sig3, mut did3) = Default::default();
            (self.path)
              .read_identify_regs(MaybeMut::None, new_prel, "did", Some(&mut sig3), &mut did3)
              .unwrap();
            ep.with_ctx(None, |ep| did3.visit(ep));
            assert_eq!(sig1, sig3);
            assert_eq!(did1, did3);
          }
        }
      }
      if self.g.cfg.cache_prel {
        self.path.with_cache(|c| &c.did, (sig1, did1));
      }
    }

    // validating .drd
    {
      let mut drd1 = self.reductions[self.export.reductions_base as usize..].to_owned();
      let nonempty = !drd1.is_empty();
      let (mut sig1, mut sig, mut drd2) = Default::default();
      if self.g.cfg.verify_export {
        assert_eq_nonempty!(self, nonempty, self.path.read_drd(false, &mut sig, &mut drd2));
      }
      if nonempty {
        drd1.visit(ep);
        sig1 = aco.mark(&mut drd1, arts1.len(), &arts2);
        if self.g.cfg.verify_export {
          ep.with_ctx(None, |ep| drd2.visit(ep));
          assert_eq!(sig1, sig);
          assert_eq_iter("reductions", drd1.iter(), drd2.iter());
        }
        if self.g.cfg.xml_export {
          self.path.write_drd(new_prel, &sig1, &drd1);
          if DOUBLE_CHECK {
            let (mut sig3, mut drd3) = Default::default();
            (self.path)
              .read_reduction_regs(MaybeMut::None, new_prel, "drd", Some(&mut sig3), &mut drd3)
              .unwrap();
            ep.with_ctx(None, |ep| drd3.visit(ep));
            assert_eq!(sig1, sig3);
            assert_eq!(drd1, drd3);
          }
        }
      }
      if self.g.cfg.cache_prel {
        self.path.with_cache(|c| &c.drd, (sig1, drd1));
      }
    }

    // validating .dpr
    {
      let mut dpr1 = self.properties[self.export.properties_base as usize..].to_owned();
      let nonempty = !dpr1.is_empty();
      let (mut sig1, mut sig, mut dpr2) = Default::default();
      if self.g.cfg.verify_export {
        assert_eq_nonempty!(self, nonempty, self.path.read_dpr(false, &mut sig, &mut dpr2));
        dpr2.visit(ep);
      }
      if nonempty {
        dpr1.visit(ep);
        sig1 = aco.mark(&mut dpr1, arts1.len(), &arts2);
        if self.g.cfg.verify_export {
          ep.with_ctx(None, |ep| dpr2.visit(ep));
          assert_eq!(sig1, sig);
          assert_eq_iter("properties", dpr1.iter(), dpr2.iter());
        }
        if self.g.cfg.xml_export {
          self.path.write_dpr(new_prel, &sig1, &dpr1);
          if DOUBLE_CHECK {
            let (mut sig3, mut dpr3) = Default::default();
            (self.path)
              .read_properties(MaybeMut::None, new_prel, "dpr", Some(&mut sig3), &mut dpr3)
              .unwrap();
            ep.with_ctx(None, |ep| dpr3.visit(ep));
            assert_eq!(sig1, sig3);
            assert_eq!(dpr1, dpr3);
          }
        }
      }
      if self.g.cfg.cache_prel {
        self.path.with_cache(|c| &c.dpr, (sig1, dpr1));
      }
    }

    // validating .the
    {
      let (mut thms1, mut thms2) = <(DepTheorems, _)>::default();
      std::mem::swap(&mut thms1.thm, &mut self.export.theorems);
      let nonempty = !thms1.thm.is_empty();
      if self.g.cfg.verify_export {
        assert_eq_nonempty!(self, nonempty, self.path.read_the(false, &mut thms2));
      }
      if nonempty {
        thms1.thm.visit(ep);
        thms1.sig = aco.mark(&mut thms1.thm, arts1.len(), &arts2);
        if self.g.cfg.verify_export {
          ep.with_ctx(None, |ep| thms2.thm.visit(ep));
          assert_eq!(thms1.sig, thms2.sig);
          assert_eq_iter("theorems", thms1.thm.iter(), thms2.thm.iter());
        }
        if self.g.cfg.xml_export {
          self.path.write_the(new_prel, &thms1);
          if DOUBLE_CHECK {
            let mut thms3 = Default::default();
            self.path.read_the(new_prel, &mut thms3).unwrap();
            ep.with_ctx(None, |ep| thms3.thm.visit(ep));
            assert_eq!(thms1, thms3);
          }
        }
      }
      if self.g.cfg.cache_prel {
        self.path.with_cache(|c| &c.the, thms1);
      }
    }

    // validating .sch
    {
      let (mut schs1, mut schs2) = <(DepSchemes, _)>::default();
      schs1.sch = (self.export.schemes.iter())
        .map(|i| i.map(|i| self.libs.sch[&(ArticleId::SELF, i)].visit_cloned(ep)))
        .collect();
      let nonempty = !schs1.sch.is_empty();
      if self.g.cfg.verify_export {
        assert_eq_nonempty!(self, nonempty, self.path.read_sch(false, &mut schs2));
      }
      if nonempty {
        schs1.sig = aco.mark(&mut schs1.sch, arts1.len(), &arts2);
        if self.g.cfg.verify_export {
          ep.with_ctx(None, |ep| schs2.sch.visit(ep));
          assert_eq!(schs1.sig, schs2.sig);
          assert_eq_iter("schemes", schs1.sch.iter(), schs2.sch.iter());
        }
        if self.g.cfg.xml_export {
          self.path.write_sch(new_prel, &schs1);
          if DOUBLE_CHECK {
            let mut schs3 = Default::default();
            self.path.read_sch(new_prel, &mut schs3).unwrap();
            ep.with_ctx(None, |ep| schs3.sch.visit(ep));
            assert_eq!(schs1, schs3);
          }
        }
      }
      if self.g.cfg.cache_prel {
        self.path.with_cache(|c| &c.sch, schs1);
      }
    }
  }
}
