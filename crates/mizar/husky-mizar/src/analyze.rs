use crate::ast::{
  CaseKind, FormulaBinder, FormulaBinop, Pragma, PrivFuncKind, PrivPredKind, ResGroupId,
  ReservedId, VarKind,
};
use crate::error::MizError;
use crate::export::Exporter;
use crate::parser::{MizParser, MsmParser, PathResult};
use crate::reader::{DefiniensId, Reader};
use crate::types::{PatternKindClass as PKC, *};
use crate::*;
use std::ops::Range;
use std::rc::Rc;

const MAX_EXPANSIONS: usize = 20;

#[derive(Clone, Default, Debug)]
struct NameLookup {
  /// does not contain VarKind::Reserved
  var: im::HashMap<Rc<str>, VarKind>,
  func: im::HashMap<(Rc<str>, u32), PrivFuncKind>,
  pred: im::HashMap<(Rc<str>, u32), PrivPredKind>,
}

#[derive(Debug)]
struct ResGroup {
  ty: Type,
  /// Present iff name checking is enabled
  fvars: Option<IdxVec<BoundId, ReservedId>>,
}

pub struct Analyzer<'a> {
  pub r: &'a mut Reader,
  pub path: &'a MizPath,
  ident_map: HashMap<Rc<str>, IdentId>,
  idents: IdxVec<IdentId, Rc<str>>,
  sch_func_args: IdxVec<SchFuncId, Box<[Type]>>,
  priv_func_args: IdxVec<PrivPredId, Box<[Type]>>,
  priv_pred: IdxVec<PrivPredId, (Box<[Type]>, Box<Formula>)>,
  sch_pred_args: IdxVec<SchPredId, Box<[Type]>>,
  sch_names: (HashMap<Rc<str>, SchId>, SchId),
  thesis: Option<Box<Formula>>,
  thesis_stack: Vec<Option<Box<Formula>>>,
  label_names: IdxVec<LabelId, Option<Rc<str>>>,
  lookup: Rc<NameLookup>,
  reserved: IdxVec<ReservedId, (Rc<str>, ResGroupId)>,
  res_groups: IdxVec<ResGroupId, ResGroup>,
  reserved_by_name: HashMap<Rc<str>, ReservedId>,
  /// does not contain VarKind::Reserved
  reserved_lookup: im::HashMap<ReservedId, VarKind>,
  reserved_extra_depth: u32,
  /// Only used for fraenkel terms. (i, len) means that BVs above i are lifted by len;
  /// all lifts are applied cumulatively.
  reserved_more_insertion: Vec<(u32, u32)>,
  internal_selectors: HashMap<SelSymId, ConstId>,
  local_def_map: HashMap<LabelId, DefiniensId>,
  defthms: DefId,
  pub notations_base: EnumMap<PatternKindClass, u32>,
  pub export: Exporter,
}
impl<'a> std::ops::Deref for Analyzer<'a> {
  type Target = &'a mut Reader;
  fn deref(&self) -> &Self::Target { &self.r }
}
impl<'a> std::ops::DerefMut for Analyzer<'a> {
  fn deref_mut(&mut self) -> &mut Self::Target { &mut self.r }
}

macro_rules! try_p {
  ($self:expr, $e:expr) => {
    match $e {
      Ok(t) => t,
      Err((path, e)) => {
        e.report(&path);
        $self.has_errors = true;
        return
      }
    }
  };
}
impl Reader {
  fn push_parse_item(
    &mut self, parser: &mut Result<&mut MizParser<'_>, MsmParser>, buf: &mut Vec<ast::Item>,
  ) -> PathResult<bool> {
    match parser {
      Ok(parser) => {
        // parser.formats is empty, formatter.formats has .frm
        std::mem::swap(&mut parser.formats, &mut self.lc.formatter.formats);
        std::mem::swap(&mut parser.format_lookup, &mut self.formats);
        let res = parser.push_parse_item(buf);
        // move .frx formats back to formatter
        std::mem::swap(&mut self.lc.formatter.formats, &mut parser.formats);
        std::mem::swap(&mut self.formats, &mut parser.format_lookup);
        Ok(res)
      }
      Err(parser) => parser.push_parse_item(buf).map_err(|e| (parser.path.clone(), e)),
    }
  }

  pub fn run_analyzer(&mut self, path: &MizPath, parser: Option<&mut MizParser<'_>>) {
    let mut parser = match parser {
      Some(v) => Ok(v),
      None => {
        let result = if self.g.cfg.nameck_enabled { path.open_wsx() } else { path.open_msx() };
        Err(try_p!(self, result))
      }
    };
    let mut items = vec![];
    if !self.g.cfg.analyzer_enabled {
      while try_p!(self, self.push_parse_item(&mut parser, &mut items)) {
        items.clear()
      }
    }
    let mut elab = Analyzer {
      r: self,
      path,
      ident_map: Default::default(),
      idents: Default::default(),
      sch_func_args: Default::default(),
      priv_func_args: Default::default(),
      priv_pred: Default::default(),
      sch_pred_args: Default::default(),
      sch_names: Default::default(),
      thesis: None,
      thesis_stack: vec![],
      lookup: Default::default(),
      label_names: Default::default(),
      reserved: Default::default(),
      res_groups: Default::default(),
      reserved_by_name: Default::default(),
      reserved_lookup: Default::default(),
      reserved_extra_depth: 0,
      reserved_more_insertion: vec![],
      internal_selectors: Default::default(),
      local_def_map: Default::default(),
      defthms: Default::default(),
      notations_base: Default::default(),
      export: Default::default(),
    };
    #[allow(clippy::indexing_slicing)]
    for (i, pats) in &elab.r.notations {
      elab.notations_base[i] = pats.len() as u32
    }
    if elab.g.cfg.exporter_enabled {
      elab.export.constrs_base = elab.g.constrs.len();
      elab.export.clusters_base = elab.g.clusters.len();
      elab.export.definitions_base = elab.definitions.peek();
      elab.export.identify_base = elab.identify.len() as u32;
      elab.export.reductions_base = elab.reductions.len() as u32;
      elab.export.properties_base = elab.properties.len() as u32;
    }
    while try_p!(elab, elab.r.push_parse_item(&mut parser, &mut items)) {
      for it in items.iter_mut() {
        if elab.g.cfg.top_item_header {
          eprintln!("item {:?}: {:?}", it.pos, it.kind);
        }
        elab.elab_top_item(it);
      }
      items.clear()
    }
    if elab.g.cfg.xml_internals {
      path.write_idx(&elab.idents.0);
      if let Ok(parser) = parser {
        path.write_formats("frx", &elab.lc.formatter.formats, &parser.func_prio);
      }
    }
    if elab.g.cfg.exporter_enabled {
      elab.export()
    }
  }
}

impl<F> Pattern<F> {
  fn check_types<'a>(
    &self, g: &Global, lc: &LocalContext, args: &'a [TermQua],
  ) -> Option<Subst<'a>> {
    if self.primary.is_empty() {
      return Some(Subst::new(0))
    }
    let mut subst = Subst::from_essential(self.primary.len(), &self.visible, args);
    if subst.check_types(g, lc, &self.primary) {
      Some(subst)
    } else {
      None
    }
  }
}

/// Agree
fn agrees(g: &Global, lc: &LocalContext, tms: &[TermQua], tys: &[Type]) -> bool {
  let mut subst = Subst { subst_term: tms.iter().map(|tm| Some(CowBox::Borrowed(tm))).collect() };
  subst.check_types(g, lc, tys)
}

struct Scope {
  sc: crate::reader::Scope,
  priv_preds: usize,
  lookup: Rc<NameLookup>,
  reserved: im::HashMap<ReservedId, VarKind>,
}

impl Analyzer<'_> {
  #[allow(unused)]
  fn intern_id(&mut self, s: &Rc<str>) -> IdentId {
    match self.ident_map.get(s) {
      Some(&id) => id,
      None => {
        let id = self.idents.push(s.clone());
        self.ident_map.insert(s.clone(), id);
        id
      }
    }
  }

  fn open_scope(&mut self, push_label: bool, copy_thesis: bool) -> Scope {
    self.thesis_stack.push(if copy_thesis { self.thesis.clone() } else { self.thesis.take() });
    if push_label {
      self.label_names.push(None);
    }
    Scope {
      sc: self.r.open_scope(push_label),
      priv_preds: self.priv_pred.len(),
      lookup: self.lookup.clone(),
      reserved: self.reserved_lookup.clone(),
    }
  }

  #[allow(clippy::unwrap_used)]
  fn close_scope(&mut self, sc: Scope, check_for_local_const: bool) -> Descope {
    self.priv_func_args.0.truncate(sc.sc.priv_funcs);
    self.priv_pred.0.truncate(sc.priv_preds);
    self.label_names.0.truncate(sc.sc.labels);
    self.thesis = self.thesis_stack.pop().unwrap();
    self.lookup = sc.lookup;
    self.reserved_lookup = sc.reserved;
    self.r.close_scope(sc.sc, check_for_local_const)
  }

  fn scope<R: Visitable<Descope>>(
    &mut self, push_label: bool, copy_thesis: bool, check_for_local_const: bool,
    f: impl FnOnce(&mut Self) -> R,
  ) -> R {
    let sc = self.open_scope(push_label, copy_thesis);
    let mut r = f(self);
    let mut dsc = self.close_scope(sc, check_for_local_const);
    r.visit(&mut dsc);
    r
  }

  fn mk_pattern(
    &self, c: PatternKindClass, kind: PatternKind, fmt: FormatId, primary: Box<[Type]>,
    visible: Box<[LocusId]>, pos: bool,
  ) -> Pattern {
    let abs_nr = self.notations[c].vec.len() as u32 - self.notations_base[c];
    Pattern { article: self.article, abs_nr, kind, fmt, primary, visible, pos }
  }

  #[allow(clippy::too_many_arguments)]
  fn push_pattern(
    &mut self, check_access: bool, c: PatternKindClass, kind: PatternKind, fmt: FormatId,
    primary: Box<[Type]>, visible: Box<[LocusId]>, pos: bool,
  ) {
    let pat = self.mk_pattern(c, kind, fmt, primary, visible, pos);
    if check_access {
      pat.check_access();
    }
    self.r.lc.formatter.push(&self.r.g.constrs, &pat);
    self.r.notations[c].push_ext(pat)
  }

  pub fn push_prop(&mut self, label: Option<(Option<LabelId>, Rc<str>)>, prop: Formula) {
    let label = label.map(|(id2, name)| {
      let id = self.label_names.push(Some(name));
      if let Some(id2) = id2 {
        assert_eq!(id, id2)
      }
      id
    });
    self.r.push_prop(label, prop);
  }

  fn item_header(&mut self, it: &ast::Item, s: &str) {
    self.set_pos(it.pos);
    if let Some(n) = self.g.cfg.first_verbose_line {
      if self.pos.line > n && self.g.cfg.one_item {
        eprintln!("exiting");
        std::process::exit(0)
      }
      set_verbose(self.pos.line >= n);
    }
    if self.g.cfg.item_header {
      eprint!("item[{:?}]: ", self.pos);
      if self.g.cfg.always_verbose_item || verbose() {
        eprintln!("{:#?}", it.kind);
      } else {
        eprintln!("{s}")
      }
    }
  }

  fn elab_top_item(&mut self, it: &mut ast::Item) {
    match &it.kind {
      // ast::ItemKind::Section { .. } => self.item_header(it, "Section"),
      ast::ItemKind::Pragma { .. } => self.item_header(it, "Pragma"),
      ast::ItemKind::Block { .. } => self.item_header(it, "Block"),
      ast::ItemKind::SchemeBlock { .. } => self.item_header(it, "SchemeBlock"),
      ast::ItemKind::Theorem { .. } => self.item_header(it, "Theorem"),
      ast::ItemKind::Reservation { .. } => self.item_header(it, "Reservation"),
      _ => {}
    }
    match &mut it.kind {
      ast::ItemKind::Section => {}
      ast::ItemKind::Block { end, kind, items } => {
        let mut br = BlockReader::new(*kind, &self.lc);
        let check = matches!(kind, BlockKind::Definition | BlockKind::Registration);
        self.scope(false, false, check, |this| br.elab_proof(this, items, *end));
        br.after_scope(self)
      }
      ast::ItemKind::SchemeBlock(it) =>
        self.scope(false, false, false, |this| this.elab_scheme(it)),
      ast::ItemKind::Theorem { prop, just } => {
        let (f, block) = self.elab_formula_forall_reserved(&mut prop.f, true);
        Exportable.visit_formula(&f);
        if self.g.cfg.analyzer_full {
          let f = f.visit_cloned(&mut self.r.intern_const());
          let label = prop.label.as_ref().map(|l| l.id.clone());
          self.elab_justification_intro_reserved(label.is_some(), &f, just, block);
          self.push_prop(label, f)
        }
        assert!(self.reserved_extra_depth == 0);
        if self.g.cfg.exporter_enabled {
          self.export.theorems.push(Theorem { pos: it.pos, kind: TheoremKind::Thm, stmt: f })
        }
      }
      ast::ItemKind::Reservation(its) => {
        self.lc.term_cache.get_mut().open_scope();
        assert!(self.lc.bound_var.is_empty());
        for it in its {
          let ty;
          let fvars = if let Some(tys) = &it.tys {
            for ty in tys {
              let ty = self.elab_type(ty);
              self.lc.bound_var.push(ty);
            }
            ty = self.elab_type(&it.ty);
            if self.g.cfg.nameck_enabled {
              it.fvars.clone()
            } else {
              None
            }
          } else {
            assert!(self.g.cfg.nameck_enabled);
            let orig_lookup = self.reserved_lookup.clone();
            let fvars = self.collect_reserved(|cr| cr.visit_type(&mut it.ty));
            self.reserved_extra_depth = fvars.len() as u32;
            for &var in &fvars {
              #[allow(clippy::indexing_slicing)]
              let entry = &self.res_groups[self.reserved[var].1];
              #[allow(clippy::unwrap_used)]
              let fvars2 = entry.fvars.as_ref().unwrap();
              let mut ty = entry.ty.clone();
              ty.visit(&mut InstReservation(self, |i| VarKind::Reserved(fvars2[i])));
              let i = self.lc.bound_var.push(ty);
              self.reserved_lookup.insert(var, VarKind::Bound(i));
            }
            ty = self.elab_type(&it.ty);
            self.reserved_extra_depth = 0;
            self.reserved_lookup = orig_lookup;
            Some(IdxVec::from(fvars))
          };
          self.lc.bound_var.0.clear();
          Exportable.visit_type(&ty);
          let group = self.res_groups.push(ResGroup { ty, fvars });
          for v in &it.vars {
            let i = self.reserved.push((v.spelling.clone(), group));
            if self.g.cfg.nameck_enabled {
              self.reserved_by_name.insert(v.spelling.clone(), i);
            }
          }
        }
        self.lc.term_cache.get_mut().close_scope();
      }
      ast::ItemKind::Pragma(pragma) => {
        match *pragma {
          Pragma::Canceled(k, n) => self.elab_canceled(it.pos, k, n),
          Pragma::SetVerify(b) => self.no_suppress_checker = b,
          Pragma::ThmDesc(_) | Pragma::Insert(_) => {}
          // This is intentionally stricter than necessary to ensure that MML has no weird
          // pragmas. The line below should be uncommented to allow pragmas for general use.
          // Pragma::Other(_) => {}
          Pragma::Other(ref mut s) =>
            self.err(it.pos, MizError::UnexpectedPragma(std::mem::take(s))),
        }
      }
      _ => self.elab_stmt_item(it),
    }
  }

  fn elab_scheme(&mut self, ast::SchemeBlock { end, head, items }: &mut ast::SchemeBlock) {
    let ast::SchemeHead { sym, nr, groups, concl, prems } = head;
    assert!(self.sch_func_args.is_empty());
    assert!(self.sch_pred_args.is_empty());
    assert!(self.lc.sch_func_ty.is_empty());
    let infer_consts = self.lc.infer_const.get_mut().0.len();
    let mut func_id = SchFuncId::default();
    let mut pred_id = SchPredId::default();
    for group in groups {
      match group {
        ast::SchemeBinderGroup::Func { vars, tys, ret, .. } => {
          self.elab_intern_push_locus_tys(tys);
          let ret = self.elab_intern_type_no_reserve(ret);
          assert!(!vars.is_empty());
          for _ in 1..vars.len() {
            self.sch_func_args.push(self.r.lc.locus_ty.0.to_vec().into());
            self.r.lc.sch_func_ty.push(ret.clone());
          }
          self.sch_func_args.push(self.r.lc.locus_ty.0.drain(..).collect());
          self.r.lc.sch_func_ty.push(ret);
          if self.g.cfg.nameck_enabled {
            let lookup = Rc::make_mut(&mut self.lookup);
            for v in vars {
              lookup.func.insert(
                (v.spelling.clone(), tys.len() as u32),
                PrivFuncKind::SchFunc(func_id.fresh()),
              );
            }
          }
        }
        ast::SchemeBinderGroup::Pred { vars, tys, .. } => {
          self.elab_intern_push_locus_tys(tys);
          assert!(!vars.is_empty());
          for _ in 1..vars.len() {
            self.sch_pred_args.push(self.r.lc.locus_ty.0.to_vec().into());
          }
          self.sch_pred_args.push(self.r.lc.locus_ty.0.drain(..).collect());
          if self.g.cfg.nameck_enabled {
            let lookup = Rc::make_mut(&mut self.lookup);
            for v in vars {
              lookup.pred.insert(
                (v.spelling.clone(), tys.len() as u32),
                PrivPredKind::SchPred(pred_id.fresh()),
              );
            }
          }
        }
      }
    }
    let prems = (prems.iter_mut())
      .map(|prop| self.elab_proposition_forall_reserved(prop, true))
      .collect::<Box<[_]>>();
    let (mut thesis, block) = self.elab_formula_forall_reserved(concl, true);
    thesis.visit(&mut self.r.intern_const());
    if self.g.cfg.analyzer_full {
      self.elab_proof_intro_reserved(false, &thesis, items, *end, block)
    }
    let primary: Box<[_]> = self.lc.sch_func_ty.0.drain(..).collect();
    let mut sch = Scheme { sch_funcs: primary, prems, thesis };
    self.lc.expand_consts(&self.g.constrs, |c| sch.visit(c));
    sch.sch_funcs.iter().for_each(|ty| Exportable.visit_type(ty));
    sch.prems.iter().for_each(|ty| Exportable.visit_formula(ty));
    Exportable.visit_formula(&sch.thesis);
    self.lc.infer_const.get_mut().truncate(infer_consts);
    self.sch_func_args.0.clear();
    self.sch_pred_args.0.clear();
    let id = self.sch_names.1.fresh();
    if let Some(sym) = sym {
      if self.g.cfg.nameck_enabled {
        self.sch_names.0.insert(sym.clone(), id);
      }
    }
    if let Some(nr) = *nr {
      assert_eq!(nr, id)
    }
    self.libs.sch.insert((ArticleId::SELF, id), sch);
    if self.g.cfg.exporter_enabled {
      self.export.schemes.push(Some(id))
    }
  }

  fn elab_stmt_item(&mut self, it: &mut ast::Item) {
    match &it.kind {
      ast::ItemKind::Set { .. } => self.item_header(it, "Set"),
      ast::ItemKind::DefFunc { .. } => self.item_header(it, "DefFunc"),
      ast::ItemKind::DefPred { .. } => self.item_header(it, "DefPred"),
      ast::ItemKind::Reconsider { .. } => self.item_header(it, "Reconsider"),
      ast::ItemKind::Consider { .. } => self.item_header(it, "Consider"),
      ast::ItemKind::Statement { .. } => self.item_header(it, "Statement"),
      _ => {}
    }
    match &mut it.kind {
      ast::ItemKind::Set(decls) =>
        for ast::SetDecl { var, value } in decls {
          let term = self.elab_intern_term_no_reserve(value);
          let ty = term.get_type_uncached(&self.g, &self.lc);
          let c = self.lc.fixed_var.push(FixedVar { ty, def: Some((Box::new(term), true)) });
          if self.g.cfg.nameck_enabled {
            Rc::make_mut(&mut self.lookup).var.insert(var.spelling.clone(), VarKind::Const(c));
          }
        },
      ast::ItemKind::DefFunc { var, tys, value } => {
        self.lc.term_cache.get_mut().open_scope();
        self.elab_intern_push_locus_tys(tys);
        let value = self.elab_intern_term_no_reserve(value);
        let ty = value.get_type(&self.g, &self.lc, false);
        let primary = self.r.lc.locus_ty.0.drain(..).collect();
        self.lc.term_cache.get_mut().close_scope();
        let i =
          self.r.lc.priv_func.push(FuncDef { primary, ty: Box::new(ty), value: Box::new(value) });
        if self.g.cfg.nameck_enabled {
          (Rc::make_mut(&mut self.lookup).func)
            .insert((var.spelling.clone(), tys.len() as u32), PrivFuncKind::PrivFunc(i));
        }
      }
      ast::ItemKind::DefPred { var, tys, value } => {
        self.lc.term_cache.get_mut().open_scope();
        self.elab_intern_push_locus_tys(tys);
        let value = self.elab_intern_formula_forall_reserved(value, true);
        let primary = self.r.lc.locus_ty.0.drain(..).collect();
        self.lc.term_cache.get_mut().close_scope();
        let i = self.priv_pred.push((primary, Box::new(value)));
        if self.g.cfg.nameck_enabled {
          (Rc::make_mut(&mut self.lookup).pred)
            .insert((var.spelling.clone(), tys.len() as u32), PrivPredKind::PrivPred(i));
        }
      }
      ast::ItemKind::Reconsider { vars, ty, just } => {
        let ty = self.elab_intern_type_no_reserve(ty);
        let mut conjs = self.g.cfg.analyzer_full.then(Vec::new);
        for var in vars {
          if let ast::ReconsiderVar::Var(v) = var {
            *var = ast::ReconsiderVar::Equality { var: v.clone(), tm: v.to_term() }
          }
          let ast::ReconsiderVar::Equality { var, tm } = var else { unreachable!() };
          let tm = Box::new(self.elab_intern_term_no_reserve(tm));
          if let Some(conjs) = &mut conjs {
            conjs.push(Formula::Is { term: tm.clone(), ty: Box::new(ty.clone()) });
          }
          let c = self.lc.fixed_var.push(FixedVar { ty: ty.clone(), def: Some((tm, false)) });
          if self.g.cfg.nameck_enabled {
            Rc::make_mut(&mut self.lookup).var.insert(var.spelling.clone(), VarKind::Const(c));
          }
        }
        if let Some(conjs) = conjs {
          let f = Formula::mk_and(conjs);
          self.elab_justification(false, &f, just)
        }
      }
      ast::ItemKind::Consider { vars, conds, just } => {
        let start = self.lc.fixed_var.len();
        let istart = self.lc.infer_const.get_mut().len() as u32;
        for var in vars {
          self.elab_fixed_vars(var);
        }
        if self.g.cfg.analyzer_full {
          let mut to_push = vec![];
          let mut f = Formula::mk_and_with(|conjs| {
            for prop in conds {
              let f = self.elab_formula_forall_reserved(&mut prop.f, true).0;
              f.clone().append_conjuncts_to(conjs);
              to_push.push((prop.label.as_ref().map(|l| l.id.clone()), f))
            }
          })
          .mk_neg();
          let end = self.lc.fixed_var.len();
          self.lc.mk_forall(start..end, istart, false, &mut f);
          f.visit(&mut self.intern_const());
          self.elab_justification(false, &f.mk_neg(), just);
          for (label, mut f) in to_push {
            f.visit(&mut self.intern_const());
            self.push_prop(label, f);
          }
        }
      }
      ast::ItemKind::Statement(stmt) =>
        if self.g.cfg.analyzer_full {
          self.elab_stmt(stmt);
        },
      _ => unreachable!("unexpected item: {it:?}"),
    }
  }

  fn elab_stmt(&mut self, stmt: &mut ast::Statement) -> Formula {
    // vprintln!("elab_stmt (thesis = {:?}) {stmt:?}", self.thesis);
    debug_assert!(self.g.cfg.analyzer_full);
    match stmt {
      ast::Statement::Proposition { prop, just } => {
        let (mut f, block) = self.elab_formula_forall_reserved(&mut prop.f, true);
        f.visit(&mut self.r.intern_const());
        let label = prop.label.as_ref().map(|l| l.id.clone());
        self.elab_justification_intro_reserved(label.is_some(), &f, just, block);
        self.push_prop(label, f.clone());
        f
      }
      ast::Statement::IterEquality { prop, just, steps } => {
        let f = self.elab_intern_formula_forall_reserved(&mut prop.f, true);
        if let Formula::Pred { nr, ref args } = f {
          if let (nr, [lhs, rhs]) = Formula::adjust_pred(nr, args, Some(&self.g.constrs)) {
            if self.g.reqs.equals_to() == Some(nr) {
              self.elab_justification(false, &self.g.reqs.mk_eq(lhs.clone(), rhs.clone()), just);
              let mut mid = rhs.clone();
              for ast::IterStep { rhs, just, .. } in steps {
                let rhs = self.elab_intern_term_no_reserve(rhs);
                self.elab_justification(false, &self.g.reqs.mk_eq(mid, rhs.clone()), just);
                mid = rhs;
              }
              let f = self.g.reqs.mk_eq(lhs.clone(), mid);
              self.push_prop(prop.label.as_ref().map(|l| l.id.clone()), f.clone());
              return f
            }
          }
        }
        self.err(prop.f.pos(), MizError::IterEqualityNotAnEquality(Box::new(f.clone())));
        f
      }
      ast::Statement::Now { end, label, items } => {
        let label = label.as_ref().map(|l| l.id.clone());
        let f = self.scope(label.is_some(), false, true, |this| {
          ReconstructThesis { stack: vec![ProofStep::Break(true)] }.elab_proof(this, items, *end)
        });
        self.push_prop(label, f.clone());
        f
      }
    }
  }

  fn elab_proof_intro_reserved(
    &mut self, push_label: bool, thesis: &Formula, items: &mut [ast::Item], end: Position,
    reset: ReserveBlock,
  ) {
    self.scope(push_label, false, false, |this| {
      this.thesis = Some(Box::new(thesis.clone()));
      reset.intro(this);
      WithThesis.elab_proof(this, items, end)
    })
  }

  fn elab_fixed_vars(&mut self, ast::BinderGroup { vars, ty }: &mut ast::BinderGroup) {
    assert!(!vars.is_empty());
    // for i in 0..vars.len() {
    //   vprintln!("elab_fixed_var c{}: {ty:?}", self.lc.fixed_var.len() + i);
    // }
    let mut base = self.lc.fixed_var.peek();
    if let Some(ty) = ty {
      let ty = self.elab_intern_type_no_reserve(ty);
      for _ in 1..vars.len() {
        self.lc.fixed_var.push(FixedVar { ty: ty.clone(), def: None });
      }
      self.lc.fixed_var.push(FixedVar { ty, def: None });
    } else {
      assert!(vars.len() == 1);
      let Some(&nr) = self.reserved_by_name.get(&*vars[0].spelling) else {
        panic!("{:?}: variable missing type", vars[0].pos)
      };
      let ResGroup { ty, fvars } = &self.res_groups[self.reserved[nr].1];
      let subst = (fvars.as_ref().unwrap().0.iter())
        .map(|&i| {
          *self.reserved_lookup.get(&i).unwrap_or_else(|| {
            panic!("{:?}: cannot introduce reserved variable here", vars[0].pos);
          })
        })
        .collect_vec();
      let mut ty = ty.clone();
      if !subst.is_empty() {
        InstReservation(self, |i| subst[i.0 as usize]).visit_type(&mut ty)
      }
      let c = self.lc.fixed_var.push(FixedVar { ty, def: None });
      self.reserved_lookup.insert(nr, VarKind::Const(c));
    }
    if self.g.cfg.nameck_enabled {
      let lookup = Rc::make_mut(&mut self.lookup);
      for v in vars {
        lookup.var.insert(v.spelling.clone(), VarKind::Const(base.fresh()));
      }
    }
  }

  fn elab_proposition_forall_reserved(
    &mut self, prop: &mut ast::Proposition, quotable: bool,
  ) -> Formula {
    let f = self.elab_intern_formula_forall_reserved(&mut prop.f, true);
    if quotable && self.g.cfg.analyzer_full {
      self.push_prop(prop.label.as_ref().map(|l| l.id.clone()), f.clone());
    }
    f
  }

  fn elab_references(&mut self, rs: &[ast::Reference]) -> Vec<Reference> {
    let mut out = vec![];
    for r in rs {
      let kind = match r.kind {
        // The label cannot be `None` because that only occurs in the spurious anonymous
        // reference added in MSM, which we strip
        ast::ReferenceKind::Priv(id) => ReferenceKind::Priv(id.unwrap()),
        ast::ReferenceKind::UnresolvedPriv(ref name) => {
          let lab = self.label_names.enum_iter().rev().find(|p| p.1.as_deref() == Some(name));
          ReferenceKind::Priv(lab.expect("label not found").0)
        }
        ast::ReferenceKind::Global(art, ref frags) => {
          for frag in frags {
            out.push(match *frag {
              ast::RefFragment::Thm { pos, id } =>
                Reference { pos, kind: ReferenceKind::Thm((art, id)) },
              ast::RefFragment::Def { pos, id } =>
                Reference { pos, kind: ReferenceKind::Def((art, id)) },
            })
          }
          continue
        }
      };
      out.push(Reference { pos: r.pos, kind })
    }
    out
  }

  fn elab_justification(
    &mut self, push_label: bool, thesis: &Formula, just: &mut ast::Justification,
  ) {
    assert!(self.reserved_extra_depth == 0);
    self.elab_justification_intro_reserved(push_label, thesis, just, <_>::default())
  }

  fn elab_justification_intro_reserved(
    &mut self, push_label: bool, thesis: &Formula, just: &mut ast::Justification,
    block: ReserveBlock,
  ) {
    debug_assert!(self.g.cfg.analyzer_full);
    match just {
      &mut ast::Justification::Inference { pos, ref kind, ref refs } => {
        let it = Inference {
          kind: match kind {
            ast::InferenceKind::By { link } => InferenceKind::By { linked: link.is_some() },
            &ast::InferenceKind::From { sch: ast::SchRef::Resolved(art, id) } =>
              InferenceKind::From { sch: (art, id) },
            ast::InferenceKind::From { sch: ast::SchRef::UnresolvedPriv(name) } => {
              let sch = *(self.sch_names.0.get(&**name))
                .unwrap_or_else(|| panic!("local scheme '{}' not found", name));
              InferenceKind::From { sch: (ArticleId::SELF, sch) }
            }
          },
          pos,
          refs: self.elab_references(refs),
        };
        self.r.read_inference(thesis, &it)
      }
      ast::Justification::Block { pos, items } =>
        self.elab_proof_intro_reserved(push_label, thesis, items, pos.1, block),
    }
  }

  fn elab_corr_conds(
    &mut self, mut cc: CorrConds, conds: &mut [ast::CorrCond], corr: &mut Option<ast::Correctness>,
  ) {
    if self.g.cfg.analyzer_full {
      for cond in conds {
        let mut thesis = cc.0[cond.kind].take().unwrap();
        thesis.visit(&mut self.intern_const());
        self.elab_justification(false, &thesis, &mut cond.just);
      }
      if let Some(corr) = corr {
        let mut thesis = Formula::mk_and_with(|conjs| {
          for (_, stmt) in &mut cc.0 {
            if let Some(stmt) = stmt.take() {
              stmt.append_conjuncts_to(conjs);
            }
          }
        });
        thesis.visit(&mut self.intern_const());
        self.elab_justification(false, &thesis, &mut corr.just);
      }
      assert!(cc.0.iter().all(|p| p.1.is_none()));
    }
  }

  fn elab_def_case<T, U>(
    &mut self, def: &mut ast::DefCase<T>, f: impl FnOnce(&mut Self, &mut T) -> U + Copy,
  ) -> DefCase<U> {
    let case = f(self, &mut def.case);
    let it_type = self.lc.it_type.take(); // can't use 'it' inside the guard
    let guard = self.elab_formula_forall_reserved(&mut def.guard, true).0;
    self.lc.it_type = it_type;
    DefCase { case, guard }
  }

  fn elab_def_body<T, U>(
    &mut self, body: &mut ast::DefBody<T>, f: impl FnOnce(&mut Self, &mut T) -> U + Copy,
  ) -> DefBody<U> {
    DefBody {
      cases: body.cases.iter_mut().map(|c| self.elab_def_case(c, f)).collect(),
      otherwise: body.otherwise.as_deref_mut().map(|ow| f(self, ow)),
    }
  }

  fn elab_def_value(&mut self, value: &mut ast::DefValue, pos: bool) -> DefValue {
    match value {
      ast::DefValue::Term(body) => {
        let it_type = self.lc.it_type.take();
        let res = DefValue::Term(self.elab_def_body(body, |this, t| this.elab_term_no_reserve(t)));
        self.lc.it_type = it_type;
        res
      }
      ast::DefValue::Formula(body) => DefValue::Formula(
        self.elab_def_body(body, |this, f| this.elab_formula_forall_reserved(f, pos).0),
      ),
    }
  }

  fn elab_canceled(&mut self, pos: Position, kind: CancelKind, n: u32) {
    match kind {
      CancelKind::Def => {
        // canceled defs outside a block don't create a DefTheorem, so they don't increment self.defthms
        // self.defthms.0 += n;
        if self.g.cfg.exporter_enabled {
          self.export.theorems.extend((0..n).map(|_| Theorem {
            pos,
            kind: TheoremKind::CanceledDef,
            stmt: Formula::True,
          }))
        }
      }
      CancelKind::Thm =>
        if self.g.cfg.exporter_enabled {
          self.export.theorems.extend((0..n).map(|_| Theorem {
            pos,
            kind: TheoremKind::CanceledThm,
            stmt: Formula::True,
          }))
        },
      CancelKind::Sch => {
        if self.g.cfg.exporter_enabled {
          self.export.schemes.extend((0..n).map(|_| None))
        }
        self.sch_names.1 .0 += n
      }
    }
  }

  fn elab_functor_term(&mut self, fmt: FormatFunc, args: &[ast::Term]) -> Term {
    let args = self.elab_terms_qua(args);
    let fmt = self.formats[&Format::Func(fmt)];
    for pat in self.notations[PKC::Func].iter().rev() {
      if pat.fmt == fmt {
        if let Some(subst) = pat.check_types(&self.g, &self.lc, &args) {
          let PatternKind::Func(nr) = pat.kind else { unreachable!() };
          let mut args = subst.trim_to(self.g.constrs.functor[nr].primary.len());
          args.iter_mut().for_each(|t| t.strip_qua_mut());
          let tm = Term::Functor { nr, args };
          TermCollection::insert(&self.g, &self.lc, &tm, false);
          return tm
        }
      }
    }
    panic!("type error")
  }

  fn elab_pred(&mut self, fmt: FormatPred, args: Vec<TermQua>, pos: bool) -> Formula {
    let fmt = self.formats[&Format::Pred(fmt)];
    for pat in self.notations[PKC::Pred].iter().rev() {
      if pat.fmt == fmt {
        if let Some(subst) = pat.check_types(&self.g, &self.lc, &args) {
          let PatternKind::Pred(nr) = pat.kind else { unreachable!() };
          let mut args = subst.trim_to(self.g.constrs.predicate[nr].primary.len());
          args.iter_mut().for_each(|t| t.strip_qua_mut());
          return Formula::Pred { nr, args }.maybe_neg(pat.pos == pos)
        }
      }
    }
    panic!("type error")
  }

  fn elab_const(&self, nr: ConstId) -> Term {
    if let Some((ref def, true)) = self.lc.fixed_var[nr].def {
      let depth = self.lc.bound_var.len() as u32;
      (**def).visit_cloned(&mut OnVarMut(|v| *v += depth))
    } else {
      Term::Const(nr)
    }
  }

  fn elab_var_kind(&self, kind: VarKind) -> Term {
    match kind {
      VarKind::Bound(nr) => Term::Bound(self.map_bound(nr)),
      VarKind::Const(nr) => self.elab_const(nr),
      VarKind::Reserved(n) => match self.reserved_lookup[&n] {
        VarKind::Bound(nr) => Term::Bound(nr),
        VarKind::Const(nr) => self.elab_const(nr),
        VarKind::Reserved(_) => unreachable!(),
      },
    }
  }

  fn resolve_const(&self, var: &mut ast::Variable) -> ConstId {
    *var.var.get_or_insert_with(|| {
      let Some(&(mut kind)) = self.lookup.var.get(&var.spelling) else {
        panic!("{:?}: unknown variable '{}'", var.pos, var.spelling)
      };
      loop {
        match kind {
          VarKind::Bound(_) => unreachable!(),
          VarKind::Const(nr) => break nr,
          VarKind::Reserved(n) => kind = self.reserved_lookup[&n],
        }
      }
    })
  }

  fn push_nameck_scope(&mut self, scope: &NameckScope, check: impl FnMut(&mut Self, &Type)) {
    let mut vars = vec![];
    for (&k, &(kind, _)) in &scope.reserved {
      let kind = self.map_var_kind(kind);
      self.reserved_lookup.insert(k, kind);
    }
    for (&k, &bound) in &scope.uses {
      if !bound && !self.reserved_lookup.contains_key(&k) {
        vars.push(k)
      }
    }
    self.push_bound_reserved(&vars, true, check)
  }

  fn elab_fraenkel(
    &mut self, vars: &[ast::BinderGroup], scope: &ast::Term, compr: Option<&ast::Formula>,
    nameck: Option<&FraenkelNameckResult>,
  ) -> Term {
    self.lc.term_cache.get_mut().open_scope();
    let orig_len = self.lc.bound_var.len();
    let more_insertion_len = self.reserved_more_insertion.len();
    let scope_info = (self.lookup.clone(), self.reserved_lookup.clone(), self.reserved_extra_depth);
    if let Some(nameck) = nameck {
      self.push_nameck_scope(&nameck.scope, |this, dom| {
        assert!(dom.is_set(&this.g, &this.lc, &this.properties))
      })
    }
    for var in vars {
      assert!(!var.vars.is_empty());
      let dom = self.elab_type(var.ty.as_deref().unwrap());
      assert!(dom.is_set(&self.g, &self.lc, &self.properties));
      self.push_many_bound(dom, &var.vars);
    }
    let scope = Box::new(self.elab_term(scope));
    let compr_orig_len = self.lc.bound_var.len();
    if let Some(nameck) = nameck {
      self.push_nameck_scope(&nameck.compr, |_, _| {})
    }
    let mut compr = Box::new(compr.map_or(Formula::True, |f| self.elab_formula(f, true)));
    if self.lc.bound_var.len() > compr_orig_len {
      for ty in self.lc.bound_var.0.drain(compr_orig_len..).rev() {
        compr = Box::new(Formula::ForAll { dom: Box::new(ty), scope: compr })
      }
    }
    let args = self.lc.bound_var.0.split_off(orig_len).into();
    (self.lookup, self.reserved_lookup, self.reserved_extra_depth) = scope_info;
    self.reserved_more_insertion.truncate(more_insertion_len);
    self.lc.term_cache.get_mut().close_scope();
    Term::Fraenkel { args, scope, compr }
  }

  fn elab_term_qua(&mut self, tm: &ast::Term) -> TermQua {
    vprintln!("[{}] elab_term {tm:?}", self.reserved_extra_depth);
    let res = (|| match *tm {
      ast::Term::Placeholder { nr, .. } => Term::Locus(nr),
      ast::Term::Var { pos, kind, ref spelling } =>
        match kind.or_else(|| self.lookup.var.get(&**spelling).copied()) {
          Some(kind) => self.elab_var_kind(kind),
          None => panic!("{pos:?}: unresolved variable '{spelling}'"),
        },
      ast::Term::Numeral { value, .. } => Term::Numeral(value),
      ast::Term::Infix { ref sym, left, ref args, .. } => self.elab_functor_term(
        FormatFunc::Func { sym: sym.0, left, right: args.len() as u8 - left },
        args,
      ),
      ast::Term::Bracket { ref lsym, ref rsym, ref args, .. } => self.elab_functor_term(
        FormatFunc::Bracket { lsym: lsym.0, rsym: rsym.0, args: args.len() as u8 },
        args,
      ),
      ast::Term::PrivFunc { pos, kind, ref spelling, ref args } => {
        let mut args = self.elab_terms_qua(args);
        match kind.or_else(|| self.lookup.func.get(&(spelling.clone(), args.len() as u32)).copied())
        {
          Some(PrivFuncKind::PrivFunc(nr)) => {
            let def = &self.lc.priv_func[nr];
            assert!(agrees(&self.g, &self.lc, &args, &def.primary));
            args.iter_mut().for_each(|t| t.strip_qua_mut());
            let base = self.lc.bound_var.len() as u32;
            let mut inst = Inst::new(&self.g.constrs, &self.lc, &args, base);
            let value = def.value.visit_cloned(&mut inst);
            Term::PrivFunc { nr, args, value }
          }
          Some(PrivFuncKind::SchFunc(nr)) => {
            assert!(agrees(&self.g, &self.lc, &args, &self.sch_func_args[nr]));
            args.iter_mut().for_each(|t| t.strip_qua_mut());
            Term::SchFunc { nr, args }
          }
          None => panic!("{pos:?}: unresolved private functor '{spelling}'"),
        }
      }
      ast::Term::Aggregate { ref sym, ref args, .. } => {
        let args = self.elab_terms_qua(args);
        let fmt = self.formats[&Format::Aggr(FormatAggr { sym: sym.0, args: args.len() as u8 })];
        for pat in self.notations[PKC::Aggr].iter().rev() {
          if pat.fmt == fmt {
            if let Some(subst) = pat.check_types(&self.g, &self.lc, &args) {
              let PatternKind::Aggr(nr) = pat.kind else { unreachable!() };
              let mut args = subst.trim_to(self.g.constrs.aggregate[nr].primary.len());
              args.iter_mut().for_each(|t| t.strip_qua_mut());
              let tm = Term::Aggregate { nr, args };
              TermCollection::insert(&self.g, &self.lc, &tm, false);
              return tm
            }
          }
        }
        panic!("type error")
      }
      ast::Term::SubAggr { ref sym, ref arg, .. } => {
        let arg = self.elab_term_qua(arg);
        let fmt = self.formats[&Format::SubAggr(sym.0)];
        for pat in self.notations[PKC::SubAggr].iter().rev() {
          if pat.fmt == fmt
            && pat.check_types(&self.g, &self.lc, std::slice::from_ref(&arg)).is_some()
          {
            let PatternKind::SubAggr(nr) = pat.kind else { unreachable!() };
            let ty = arg.get_type_uncached(&self.g, &self.lc);
            return Term::mk_aggr(&self.g, &self.lc, nr, &arg.strip_qua(), &ty)
          }
        }
        panic!("type error")
      }
      ast::Term::Selector { ref sym, ref arg, .. } => {
        let arg = self.elab_term_qua(arg);
        let fmt = self.formats[&Format::Sel(sym.0)];
        for pat in self.notations[PKC::Sel].iter().rev() {
          if pat.fmt == fmt {
            if let Some(subst) = pat.check_types(&self.g, &self.lc, std::slice::from_ref(&arg)) {
              let PatternKind::Sel(nr) = pat.kind else { unreachable!() };
              let mut args = subst.trim_to(self.g.constrs.selector[nr].primary.len());
              args.iter_mut().for_each(|t| t.strip_qua_mut());
              let tm = Term::Selector { nr, args };
              TermCollection::insert(&self.g, &self.lc, &tm, false);
              return tm
            }
          }
        }
        panic!("type error")
      }
      ast::Term::InternalSelector { pos, ref sym, id } => {
        // only occurs inside elab_struct_def, ensured by parser
        Term::Const(
          id.or_else(|| self.internal_selectors.get(&sym.0).copied())
            .unwrap_or_else(|| panic!("{pos:?}: this struct does not have this field")),
        )
      }
      ast::Term::The { ref ty, .. } => Term::The { ty: Box::new(self.elab_type(ty)) },
      ast::Term::Fraenkel { ref vars, ref scope, ref compr, ref nameck, .. } =>
        self.elab_fraenkel(vars, scope, compr.as_deref(), nameck.as_deref()),
      ast::Term::Qua { ref term, ref ty, .. } =>
        Term::Qua { value: Box::new(self.elab_term(term)), ty: Box::new(self.elab_type(ty)) },
      ast::Term::It { .. } => {
        assert!(self.lc.it_type.is_some(), "unexpected 'it'");
        Term::It
      }
    })();
    vprintln!("elab_term {tm:?}\n -> {res:?}");
    res
  }

  fn elab_term(&mut self, tm: &ast::Term) -> Term { self.elab_term_qua(tm).strip_qua() }

  fn elab_terms_qua(&mut self, tms: &[ast::Term]) -> Box<[TermQua]> {
    tms.iter().map(|t| self.elab_term_qua(t)).collect()
  }

  /// AnalyzeAttrFrm
  fn elab_is_attr(&mut self, attr: &ast::Attr, pos: bool, tm: &TermQua) -> Formula {
    match attr {
      ast::Attr::Non { attr, .. } => self.elab_is_attr(attr, !pos, tm),
      ast::Attr::Attr { sym, args, .. } => {
        let args = (args.iter().map(|t| self.elab_term_qua(t)))
          .chain(std::iter::once(tm.clone()))
          .collect_vec();
        let fmt = self.formats[&Format::Attr(FormatAttr { sym: sym.0, args: args.len() as u8 })];
        for pat in self.notations[PKC::Attr].iter().rev() {
          if pat.fmt == fmt {
            if let Some(subst) = pat.check_types(&self.g, &self.lc, &args) {
              let PatternKind::Attr(nr) = pat.kind else { unreachable!() };
              let mut args = subst.trim_to(self.g.constrs.attribute[nr].primary.len());
              args.iter_mut().for_each(|t| t.strip_qua_mut());
              let (nr, args) = Formula::adjust_attr(nr, &args, Some(&self.g.constrs));
              return Formula::Attr { nr, args: args.to_owned().into() }.maybe_neg(pat.pos == pos)
            }
          }
        }
        panic!("type error")
      }
    }
  }

  /// AnalyzeAttribute
  fn elab_attr(&mut self, attr: &ast::Attr, mut pos: bool, ty: &mut Type) -> Attr {
    match attr {
      ast::Attr::Non { attr, .. } => self.elab_attr(attr, !pos, ty),
      ast::Attr::Attr { sym, args, .. } => {
        // vprintln!("elab_attr {attr:?} <- {ty:?}");
        let v = self.lc.bound_var.push(std::mem::take(ty));
        let args = (args.iter().map(|t| self.elab_term_qua(t)))
          .chain(std::iter::once(Term::Bound(v)))
          .collect_vec();
        let fmt = self.formats[&Format::Attr(FormatAttr { sym: sym.0, args: args.len() as u8 })];
        for pat in self.r.notations[PKC::Attr].iter().rev() {
          if pat.fmt == fmt {
            if let Some(subst) = pat.check_types(&self.g, &self.lc, &args) {
              let PatternKind::Attr(nr) = pat.kind else { unreachable!() };
              let c = &self.g.constrs.attribute[nr].c;
              let nr = c.redefines.unwrap_or(nr);
              let args = (subst.subst_term.into_vec().into_iter())
                .take(c.primary.len() - 1)
                .skip(c.superfluous as _)
                .map(|t| t.unwrap().to_owned().strip_qua())
                .collect::<Box<[_]>>();
              *ty = self.r.lc.bound_var.0.pop().unwrap();
              pos = pat.pos == pos;
              assert!(matches!(ty.attrs.0, Attrs::Consistent(_)));
              let out = Attr { nr, pos, args };
              // vprintln!("elab_attr {attr:?} <- {ty:?}\n  -> {out:?}");
              return out
            }
          }
        }
        panic!("type error")
      }
    }
  }

  fn elab_type(&mut self, ty: &ast::Type) -> Type {
    vprintln!("elab_type {ty:?}");
    let res = (|| match ty {
      ast::Type::Mode { sym, args, .. } => {
        let args = self.elab_terms_qua(args);
        let fmt = self.formats[&Format::Mode(FormatMode { sym: sym.0, args: args.len() as u8 })];
        for pat in self.notations[PKC::Mode].iter().rev() {
          if pat.fmt == fmt {
            if let Some(subst) = pat.check_types(&self.g, &self.lc, &args) {
              let mut ty;
              let base = self.lc.bound_var.len() as u32;
              match pat.kind {
                PatternKind::Mode(nr) => {
                  let mut args = subst.trim_to(self.g.constrs.mode[nr].primary.len()).to_vec();
                  args.iter_mut().for_each(|t| t.strip_qua_mut());
                  ty = Type { kind: TypeKind::Mode(nr), attrs: Default::default(), args }
                }
                PatternKind::ExpandableMode { ref expansion } => {
                  ty = (**expansion).clone();
                  let mut args = subst.finish();
                  args.iter_mut().for_each(|t| t.strip_qua_mut());
                  ty.visit(&mut Inst::new(&self.g.constrs, &self.lc, &args, base));
                }
                _ => unreachable!(),
              }
              let mut attrs = ty.attrs.1.clone();
              if let TypeKind::Mode(nr) = ty.kind {
                if nr != ModeId::ANY {
                  attrs.visit_enlarge_by(
                    &self.g.constrs,
                    &self.lc,
                    &self.g.constrs.mode[nr].ty.attrs.1,
                    &mut Inst::new(&self.g.constrs, &self.lc, &ty.args, base),
                  )
                }
              }
              attrs.round_up_with(&self.g, &self.lc, &ty, false);
              ty.attrs.1 = attrs;
              return ty
            }
          }
        }
        panic!("type error")
      }
      ast::Type::Struct { sym, args, .. } => {
        let args = self.elab_terms_qua(args);
        let fmt =
          self.formats[&Format::Struct(FormatStruct { sym: sym.0, args: args.len() as u8 })];
        for pat in self.notations[PKC::Struct].iter().rev() {
          if pat.fmt == fmt {
            if let Some(subst) = pat.check_types(&self.g, &self.lc, &args) {
              let PatternKind::Struct(nr) = pat.kind else { unreachable!() };
              let mut args = subst.trim_to(self.g.constrs.struct_mode[nr].primary.len()).to_vec();
              args.iter_mut().for_each(|t| t.strip_qua_mut());
              return Type { kind: TypeKind::Struct(nr), attrs: Default::default(), args }
            }
          }
        }
        panic!("type error")
      }
      ast::Type::Cluster { attrs, ty, .. } => {
        let mut ty = self.elab_type(ty);
        let mut ty2 = ty.clone();
        attrs.iter().rev().for_each(|attr| {
          let attr = self.elab_attr(attr, true, &mut ty2);
          ty2.attrs.0.insert(Some(&self.g.constrs), &self.lc, attr.clone());
          ty2.attrs.1.insert(Some(&self.g.constrs), &self.lc, attr);
          ty2.round_up_with_self(&self.g, &self.lc, false);
          assert!(matches!(ty2.attrs.0, Attrs::Consistent(_)));
        });
        for cl in self.g.clusters.registered.iter().rev() {
          let mut subst = Subst::new(cl.primary.len());
          if subst.eq_radices(&mut subst.eq_ctx(&self.g, &self.lc), &cl.ty, &ty)
            && (ty2.attrs.0)
              .is_subset_of(&cl.consequent.1, |a2, a1| subst.eq(&self.g, &self.lc, a1, a2))
            && subst.check_loci_types::<false>(&self.g, &self.lc, &cl.primary, false)
          {
            let mut attrs = ty2.attrs.0.clone();
            attrs.enlarge_by(&self.g.constrs, &self.lc, &ty.attrs.1);
            attrs.round_up_with(&self.g, &self.lc, &ty, false);
            assert!(matches!(ty2.attrs.0, Attrs::Consistent(_)));
            ty.attrs = (ty2.attrs.0, attrs);
            return ty
          }
        }
        panic!("non registered cluster \"{ty2:?}\"")
      }
      ast::Type::Reservation { group, subst, .. } => {
        let mut ty = self.res_groups[*group].ty.clone();
        if !subst.is_empty() {
          InstReservation(self, |i| subst[i.0 as usize]).visit_type(&mut ty)
        }
        ty
      }
    })();
    vprintln!("elab_type {ty:?}\n -> {res:?}");
    res
  }

  fn elab_flex_and(&mut self, f1: &ast::Formula, f2: &ast::Formula, pos: bool) -> Formula {
    #[derive(Clone, Copy, Default)]
    enum OneDiff<'a> {
      #[default]
      Same,
      One(&'a Term, &'a Term),
      Fail,
    }

    impl<'a> OneDiff<'a> {
      fn bool(b: bool) -> Self { Self::then(b, || Self::Same) }
      fn then(b: bool, f: impl FnOnce() -> Self) -> Self {
        if b {
          f()
        } else {
          Self::Fail
        }
      }

      fn merge(&mut self, ctx: &Constructors, other: Self) {
        match (*self, other) {
          (OneDiff::Fail, _) | (_, OneDiff::Same) => {}
          (_, OneDiff::Fail) | (OneDiff::Same, _) => *self = other,
          (OneDiff::One(a1, a2), OneDiff::One(b1, b2)) => {
            if a1.cmp(Some(ctx), None, b1, CmpStyle::Strict).is_ne()
              || a2.cmp(Some(ctx), None, b2, CmpStyle::Strict).is_ne()
            {
              *self = OneDiff::Fail
            }
          }
        }
      }

      fn merge_many(ctx: &Constructors, it: impl IntoIterator<Item = Self>) -> Self {
        let mut out = OneDiff::Same;
        for other in it {
          out.merge(ctx, other);
          if let OneDiff::Fail = out {
            break
          }
        }
        out
      }

      fn and_then(mut self, ctx: &Constructors, other: impl FnOnce() -> Self) -> Self {
        match self {
          OneDiff::Same => other(),
          OneDiff::One(..) => {
            self.merge(ctx, other());
            self
          }
          OneDiff::Fail => self,
        }
      }

      fn eq_term(
        ctx: &mut EqCtx<'_>, ic: &'a IdxVec<InferId, Assignment>, t1: &'a Term, t2: &'a Term,
      ) -> Self {
        use Term::*;
        let res = match (t1, t2) {
          (Bound(BoundId(n1)), Bound(BoundId(n2)))
          | (Const(ConstId(n1)), Const(ConstId(n2)))
          | (Numeral(n1), Numeral(n2)) => Self::bool(n1 == n2),
          (Infer(InferId(n1)), Infer(InferId(n2))) if n1 == n2 => Self::bool(true),
          (Functor { nr: n1, args: args1 }, Functor { nr: n2, args: args2 }) =>
            Self::then(n1 == n2, || Self::eq_terms(ctx, ic, args1, args2)),
          (
            SchFunc { nr: SchFuncId(n1), args: args1 },
            SchFunc { nr: SchFuncId(n2), args: args2 },
          )
          | (
            Aggregate { nr: AggrId(n1), args: args1 },
            Aggregate { nr: AggrId(n2), args: args2 },
          )
          | (Selector { nr: SelId(n1), args: args1 }, Selector { nr: SelId(n2), args: args2 })
            if n1 == n2 =>
            Self::eq_terms(ctx, ic, args1, args2),
          (The { ty: ty1 }, The { ty: ty2 }) => Self::eq_type(ctx, ic, ty1, ty2),
          (
            Fraenkel { args: args1, scope: sc1, compr: c1 },
            Fraenkel { args: args2, scope: sc2, compr: c2 },
          ) => Self::then(args1.len() == args2.len(), || {
            Self::merge_many(
              &ctx.g.constrs,
              args1.iter().zip(&**args2).map(|(ty1, ty2)| Self::eq_type(ctx, ic, ty1, ty2)),
            )
            .and_then(&ctx.g.constrs, || Self::eq_term(ctx, ic, sc1, sc2))
            .and_then(&ctx.g.constrs, || Self::eq_formula(ctx, ic, c1, c2))
          }),
          (It, It) => Self::bool(true),
          (_, &Infer(nr)) => ctx.lift2(|ctx| Self::eq_term(ctx, ic, t1, &ic[nr].def)),
          (&Infer(nr), _) => ctx.lift1(|ctx| Self::eq_term(ctx, ic, &ic[nr].def, t2)),
          (Locus(_), _) | (_, Locus(_)) => unreachable!(),
          (Qua { .. }, _) | (_, Qua { .. }) => unreachable!(),
          (EqMark(_), _) | (_, EqMark(_)) => unreachable!(),
          (EqClass(_), _) | (_, EqClass(_)) => unreachable!(),
          (FreeVar(_), _) | (_, FreeVar(_)) => unreachable!(),
          (PrivFunc { .. }, _) | (_, PrivFunc { .. }) =>
            Self::eq_term(ctx, ic, t1.skip_priv_func(None), t2.skip_priv_func(None)),
          _ => Self::bool(false),
        };
        if let Self::Fail = res {
          Self::One(t1, t2)
        } else {
          res
        }
      }

      fn eq_terms(
        ctx: &mut EqCtx<'_>, ic: &'a IdxVec<InferId, Assignment>, t1: &'a [Term], t2: &'a [Term],
      ) -> Self {
        Self::then(t1.len() == t2.len(), || {
          Self::merge_many(
            &ctx.g.constrs,
            t1.iter().zip(t2).map(|(t1, t2)| Self::eq_term(ctx, ic, t1, t2)),
          )
        })
      }

      fn eq_type(
        ctx: &mut EqCtx<'_>, ic: &'a IdxVec<InferId, Assignment>, ty1: &'a Type, ty2: &'a Type,
      ) -> Self {
        Self::then(().eq_attrs(ctx, ty1, ty2) && ty1.kind == ty2.kind, || {
          Self::eq_terms(ctx, ic, &ty1.args, &ty2.args)
        })
      }

      fn eq_formula(
        ctx: &mut EqCtx<'_>, ic: &'a IdxVec<InferId, Assignment>, f1: &'a Formula, f2: &'a Formula,
      ) -> Self {
        use Formula::*;
        match (f1.skip_priv_pred(), f2.skip_priv_pred()) {
          (True, True) => Self::bool(true),
          (Neg { f: f1 }, Neg { f: f2 }) => Self::eq_formula(ctx, ic, f1, f2),
          (Is { term: t1, ty: ty1 }, Is { term: t2, ty: ty2 }) => Self::eq_term(ctx, ic, t1, t2)
            .and_then(&ctx.g.constrs, || Self::eq_type(ctx, ic, ty1, ty2)),
          (And { args: args1 }, And { args: args2 }) =>
            Self::then(args1.len() == args2.len(), || {
              Self::merge_many(
                &ctx.g.constrs,
                args1.iter().zip(args2).map(|(f1, f2)| Self::eq_formula(ctx, ic, f1, f2)),
              )
            }),
          (
            SchPred { nr: SchPredId(n1), args: args1 },
            SchPred { nr: SchPredId(n2), args: args2 },
          )
          | (
            PrivPred { nr: PrivPredId(n1), args: args1, .. },
            PrivPred { nr: PrivPredId(n2), args: args2, .. },
          )
          | (Attr { nr: AttrId(n1), args: args1 }, Attr { nr: AttrId(n2), args: args2 })
          | (Pred { nr: PredId(n1), args: args1 }, Pred { nr: PredId(n2), args: args2 })
            if n1 == n2 =>
            Self::eq_terms(ctx, ic, args1, args2),
          (ForAll { dom: dom1, scope: sc1 }, ForAll { dom: dom2, scope: sc2 }) =>
            Self::eq_type(ctx, ic, dom1, dom2)
              .and_then(&ctx.g.constrs, || Self::eq_formula(ctx, ic, sc1, sc2)),
          #[allow(clippy::explicit_auto_deref)]
          (FlexAnd { terms: t1, scope: sc1, .. }, FlexAnd { terms: t2, scope: sc2, .. }) =>
            Self::eq_terms(ctx, ic, &**t1, &**t2)
              .and_then(&ctx.g.constrs, || Self::eq_formula(ctx, ic, sc1, sc2)),
          _ => Self::bool(false),
        }
      }
    }

    struct Apply<'a> {
      t1: &'a Term,
      t2: &'a Term,
      base: u32,
    }

    impl<'a> Apply<'a> {
      fn fail() -> ! { panic!("flex-and construction failed") }
      fn term(&mut self, ctx: &mut EqCtx<'a>, t1: &Term, t2: &Term) -> Term {
        use Term::*;
        if ().eq_term(ctx, t1, t2) {
          return t1.visit_cloned(&mut OnVarMut(|nr| {
            if *nr >= self.base {
              *nr += 1
            }
          }))
        }
        if ().eq_term(ctx, self.t1, t1) && ().eq_term(ctx, self.t2, t2) {
          return Term::Bound(BoundId(self.base))
        }
        match (t1, t2) {
          (Functor { nr: n1, args: args1 }, Functor { nr: n2, args: args2 }) if n1 == n2 =>
            Functor { nr: *n1, args: self.terms(ctx, args1, args2) },
          (SchFunc { nr: n1, args: args1 }, SchFunc { nr: n2, args: args2 }) if n1 == n2 =>
            SchFunc { nr: *n1, args: self.terms(ctx, args1, args2) },
          (Aggregate { nr: n1, args: args1 }, Aggregate { nr: n2, args: args2 }) if n1 == n2 =>
            Aggregate { nr: *n1, args: self.terms(ctx, args1, args2) },
          (Selector { nr: n1, args: args1 }, Selector { nr: n2, args: args2 }) if n1 == n2 =>
            Selector { nr: *n1, args: self.terms(ctx, args1, args2) },
          (The { ty: ty1 }, The { ty: ty2 }) => The { ty: Box::new(self.ty(ctx, ty1, ty2)) },
          (
            Fraenkel { args: args1, scope: sc1, compr: c1 },
            Fraenkel { args: args2, scope: sc2, compr: c2 },
          ) => {
            let (depth1, depth2) = (ctx.depth1, ctx.depth2);
            let args = args1
              .iter()
              .zip(&**args2)
              .map(|(t1, t2)| {
                let t = self.ty(ctx, t1, t2);
                ctx.depth1 += 1;
                ctx.depth2 += 1;
                t
              })
              .collect();
            let scope = Box::new(self.term(ctx, sc1, sc2));
            let compr = Box::new(self.formula(ctx, c1, c2));
            (ctx.depth1, ctx.depth2) = (depth1, depth2);
            Fraenkel { args, scope, compr }
          }
          (_, &Infer(nr)) =>
            ctx.lift2(|ctx| self.term(ctx, t1, &ctx.lc.infer_const.borrow()[nr].def)),
          (&Infer(nr), _) =>
            ctx.lift1(|ctx| self.term(ctx, &ctx.lc.infer_const.borrow()[nr].def, t2)),
          (Locus(_), _) | (_, Locus(_)) => unreachable!(),
          (Qua { .. }, _) | (_, Qua { .. }) => unreachable!(),
          (EqMark(_), _) | (_, EqMark(_)) => unreachable!(),
          (EqClass(_), _) | (_, EqClass(_)) => unreachable!(),
          (FreeVar(_), _) | (_, FreeVar(_)) => unreachable!(),
          (PrivFunc { .. }, _) | (_, PrivFunc { .. }) =>
            self.term(ctx, t1.skip_priv_func(None), t2.skip_priv_func(None)),
          _ => Self::fail(),
        }
      }

      fn terms(&mut self, ctx: &mut EqCtx<'a>, args1: &[Term], args2: &[Term]) -> Box<[Term]> {
        args1.iter().zip(args2).map(|(t1, t2)| self.term(ctx, t1, t2)).collect()
      }

      fn ty(&mut self, ctx: &mut EqCtx<'a>, ty1: &Type, ty2: &Type) -> Type {
        assert!(().eq_attrs(ctx, ty1, ty2) && ty1.kind == ty2.kind);
        let attrs = ty1.attrs.visit_cloned(&mut OnVarMut(|nr| {
          if *nr >= self.base {
            *nr += 1
          }
        }));
        Type { attrs, kind: ty1.kind, args: self.terms(ctx, &ty1.args, &ty2.args).into() }
      }

      fn formula(&mut self, ctx: &mut EqCtx<'a>, f1: &Formula, f2: &Formula) -> Formula {
        use Formula::*;
        if ().eq_formula(ctx, f1, f2) {
          return f1.visit_cloned(&mut OnVarMut(|nr| {
            if *nr >= self.base {
              *nr += 1
            }
          }))
        }
        match (f1.skip_priv_pred(), f2.skip_priv_pred()) {
          (Neg { f: f1 }, Neg { f: f2 }) => Neg { f: Box::new(self.formula(ctx, f1, f2)) },
          (Is { term: t1, ty: ty1 }, Is { term: t2, ty: ty2 }) =>
            Is { term: Box::new(self.term(ctx, t1, t2)), ty: Box::new(self.ty(ctx, ty1, ty2)) },
          (And { args: args1 }, And { args: args2 }) => And {
            args: args1.iter().zip(args2).map(|(f1, f2)| self.formula(ctx, f1, f2)).collect(),
          },
          (SchPred { nr: n1, args: args1 }, SchPred { nr: n2, args: args2 }) if n1 == n2 =>
            SchPred { nr: *n1, args: self.terms(ctx, args1, args2) },
          (
            PrivPred { nr: n1, args: args1, value: v1 },
            PrivPred { nr: n2, args: args2, value: v2 },
          ) if n1 == n2 => PrivPred {
            nr: *n1,
            args: self.terms(ctx, args1, args2),
            value: Box::new(self.formula(ctx, v1, v2)),
          },
          (Attr { nr: n1, args: args1 }, Attr { nr: n2, args: args2 }) if n1 == n2 =>
            Attr { nr: *n1, args: self.terms(ctx, args1, args2) },
          (Pred { nr: n1, args: args1 }, Pred { nr: n2, args: args2 }) if n1 == n2 =>
            Pred { nr: *n1, args: self.terms(ctx, args1, args2) },
          (ForAll { dom: dom1, scope: sc1 }, ForAll { dom: dom2, scope: sc2 }) => {
            let dom = Box::new(self.ty(ctx, dom1, dom2));
            let scope = ctx.enter(1, |ctx| Box::new(self.formula(ctx, sc1, sc2)));
            ForAll { dom, scope }
          }
          #[allow(clippy::explicit_auto_deref)]
          (FlexAnd { terms: t1, scope: sc1, nat, le }, FlexAnd { terms: t2, scope: sc2, .. }) => {
            let terms = Box::new([self.term(ctx, &t1[0], &t2[0]), self.term(ctx, &t1[1], &t2[1])]);
            let scope = ctx.enter(1, |ctx| Box::new(self.formula(ctx, sc1, sc2)));
            FlexAnd { terms, scope, nat: nat.clone(), le: *le }
          }
          _ => Self::fail(),
        }
      }
    }

    let Some(natural) = self.g.reqs.natural() else { panic!("requirement NUMERALS missing") };
    let Some(le) = self.g.reqs.less_or_equal() else { panic!("requirement REAL missing") };
    let f1 = self.elab_formula(f1, pos);
    let f2 = self.elab_formula(f2, pos);
    let (t1, t2) = self.g.with_eq(&self.lc, |ctx| {
      let ic = self.lc.infer_const.borrow();
      let OneDiff::One(t1, t2) = OneDiff::eq_formula(ctx, &ic, &f1, &f2) else {
        panic!("can't abstract flex-and term, must have exactly one difference")
      };
      (t1.clone(), t2.clone())
    });
    let base = self.lc.bound_var.len() as u32;
    let only_constant = !CheckBound::get(base..u32::MAX, |cb| {
      cb.visit_term(&t1);
      cb.visit_term(&t2);
    });
    assert!(only_constant, "can't abstract flex-and term, contains bound variables");
    let natural = Attrs::Consistent(vec![Attr::new0(natural, true)]);
    let is_natural = |t: &Term| {
      let attrs = &t.get_type(&self.g, &self.lc, false).attrs.1;
      natural.is_subset_of(attrs, |a1, a2| self.g.eq(&self.lc, a1, a2))
    };
    assert!(
      is_natural(&t1) && is_natural(&t2),
      "can't abstract flex-and term, boundary variable does not have 'natural' adjective"
    );
    let mut nat =
      Box::new(Type { kind: ModeId::SET.into(), attrs: (natural.clone(), natural), args: vec![] });
    nat.round_up_with_self(&self.g, &self.lc, false);
    let scope =
      self.g.with_eq(&self.lc, |ctx| Apply { t1: &t1, t2: &t2, base }.formula(ctx, &f1, &f2));
    Formula::FlexAnd { terms: Box::new([t1, t2]), scope: Box::new(scope), nat, le }
  }

  fn elab_push_conjuncts(&mut self, f: &ast::Formula, conjs: &mut Vec<Formula>, pos: bool) {
    match f {
      ast::Formula::Not { f, .. } => self.elab_push_conjuncts(f, conjs, !pos),
      ast::Formula::Binop { kind: FormulaBinop::And, f1, f2, .. } if pos => {
        self.elab_push_conjuncts(f1, conjs, pos);
        self.elab_push_conjuncts(f2, conjs, pos);
      }
      ast::Formula::Binop { kind: FormulaBinop::Or, f1, f2, .. } if !pos => {
        self.elab_push_conjuncts(f1, conjs, pos);
        self.elab_push_conjuncts(f2, conjs, pos);
      }
      ast::Formula::Binop { kind: FormulaBinop::Imp, f1, f2, .. } if !pos => {
        self.elab_push_conjuncts(f1, conjs, !pos);
        self.elab_push_conjuncts(f2, conjs, pos);
      }
      _ => self.elab_formula(f, pos).append_conjuncts_to(conjs),
    }
  }

  fn elab_and(&mut self, f1: &ast::Formula, f2: &ast::Formula, pos1: bool, pos2: bool) -> Formula {
    Formula::mk_and_with(|conjs| {
      self.elab_push_conjuncts(f1, conjs, pos1);
      self.elab_push_conjuncts(f2, conjs, pos2);
    })
  }

  fn push_many_bound(&mut self, mut dom: Type, vars: &[ast::Variable]) {
    for v in vars {
      let i = self.lc.bound_var.push(dom.clone());
      // vprintln!("push_bound b{i:?}[{}]: {dom:?}", v.spelling);
      dom.visit(&mut OnVarMut(|nr| {
        if *nr >= i.0 {
          *nr += 1
        }
      }));
      if self.g.cfg.nameck_enabled {
        Rc::make_mut(&mut self.lookup).var.insert(v.spelling.clone(), VarKind::Bound(i));
      }
    }
  }

  fn elab_forall(
    &mut self, vars: &[ast::BinderGroup], st: Option<&ast::Formula>, scope: &ast::Formula,
    pos: bool,
  ) -> Formula {
    self.lc.term_cache.get_mut().open_scope();
    let lookup = self.lookup.clone();
    for var in vars {
      assert!(!var.vars.is_empty());
      let dom = self.elab_type(var.ty.as_deref().expect("names must be resolved here"));
      self.push_many_bound(dom, &var.vars);
    }
    let mut scope = if let Some(st) = st {
      Formula::mk_and_with(|conjs| {
        self.elab_formula(st, true).append_conjuncts_to(conjs);
        self.elab_formula(scope, !pos).append_conjuncts_to(conjs);
      })
      .mk_neg()
    } else {
      self.elab_formula(scope, pos)
    };
    for var in vars {
      for _ in 0..var.vars.len() {
        scope = Formula::forall(self.lc.bound_var.0.pop().unwrap(), scope)
      }
    }
    self.lc.term_cache.get_mut().close_scope();
    self.lookup = lookup;
    scope
  }

  fn elab_formula(&mut self, f: &ast::Formula, positive: bool) -> Formula {
    // vprintln!("elab_formula {positive:?} {f:?}");
    let res = match f {
      ast::Formula::Not { f, .. } => self.elab_formula(f, !positive),
      ast::Formula::Binop { kind: FormulaBinop::And, f1, f2, .. } =>
        self.elab_and(f1, f2, true, true).maybe_neg(positive),
      ast::Formula::Binop { kind: FormulaBinop::Or, f1, f2, .. } =>
        self.elab_and(f1, f2, false, false).maybe_neg(!positive),
      ast::Formula::Binop { kind: FormulaBinop::FlexAnd, f1, f2, .. } =>
        self.elab_flex_and(f1, f2, true).maybe_neg(positive),
      ast::Formula::Binop { kind: FormulaBinop::FlexOr, f1, f2, .. } =>
        self.elab_flex_and(f1, f2, false).maybe_neg(!positive),
      ast::Formula::Binop { kind: FormulaBinop::Imp, f1, f2, .. } =>
        self.elab_and(f1, f2, true, false).maybe_neg(!positive),
      ast::Formula::Binop { kind: FormulaBinop::Iff, f1, f2, .. } =>
        self.elab_formula(f1, true).mk_iff(self.elab_formula(f2, true)).maybe_neg(positive),
      ast::Formula::Pred(pred) => {
        let args = self.elab_terms_qua(&pred.args);
        let right = args.len() as u8 - pred.left;
        let fmt = FormatPred { sym: pred.sym.0, left: pred.left, right };
        self.elab_pred(fmt, args.into(), pred.positive == positive)
      }
      ast::Formula::ChainPred { first, rest, .. } => {
        let mut args = self.elab_terms_qua(&first.args).into_vec();
        let mut sym = first.sym.0;
        let mut left = first.left;
        Formula::mk_and_with(|conjs| {
          for rhs in rest {
            let mut mid: Vec<_> = args[left as usize..].into();
            let right = mid.len() as u8;
            let fmt = FormatPred { sym, left, right };
            self.elab_pred(fmt, args, rhs.positive).append_conjuncts_to(conjs);
            mid.extend(rhs.right.iter().map(|t| self.elab_term_qua(t)));
            (args, sym, left) = (mid, rhs.sym.0, right);
          }
          let right = args.len() as u8 - left;
          let fmt = FormatPred { sym, left, right };
          self.elab_pred(fmt, args, first.positive).append_conjuncts_to(conjs);
        })
        .maybe_neg(positive)
      }
      &ast::Formula::PrivPred { pos, kind, ref spelling, ref args } => {
        let mut args = self.elab_terms_qua(args);
        match kind.or_else(|| self.lookup.pred.get(&(spelling.clone(), args.len() as u32)).copied())
        {
          Some(PrivPredKind::PrivPred(nr)) => {
            let (ty, value) = &self.priv_pred[nr];
            assert!(agrees(&self.g, &self.lc, &args, ty));
            args.iter_mut().for_each(|t| t.strip_qua_mut());
            let base = self.lc.bound_var.len() as u32;
            let value = value.visit_cloned(&mut Inst::new(&self.g.constrs, &self.lc, &args, base));
            Formula::PrivPred { nr, args, value }.maybe_neg(positive)
          }
          Some(PrivPredKind::SchPred(nr)) => {
            assert!(agrees(&self.g, &self.lc, &args, &self.sch_pred_args[nr]));
            args.iter_mut().for_each(|t| t.strip_qua_mut());
            Formula::SchPred { nr, args }.maybe_neg(positive)
          }
          None => panic!("{pos:?}: unresolved private predicate '{spelling}'"),
        }
      }
      ast::Formula::Attr { positive: positive2, term, attrs, .. } => {
        let term = self.elab_term_qua(term);
        Formula::mk_and_with(|conjs| {
          for attr in attrs {
            self.elab_is_attr(attr, true, &term).append_conjuncts_to(conjs)
          }
        })
        .maybe_neg(*positive2 == positive)
      }
      ast::Formula::Is { positive: positive2, term, ty, .. } =>
        Formula::Is { term: Box::new(self.elab_term(term)), ty: Box::new(self.elab_type(ty)) }
          .maybe_neg(*positive2 == positive),
      ast::Formula::Binder { kind: FormulaBinder::ForAll, vars, st, scope, .. } =>
        self.elab_forall(vars, st.as_deref(), scope, true).maybe_neg(positive),
      ast::Formula::Binder { kind: FormulaBinder::Exists, vars, st, scope, .. } =>
        self.elab_forall(vars, st.as_deref(), scope, false).maybe_neg(!positive),
      ast::Formula::False { .. } => Formula::True.maybe_neg(!positive),
      ast::Formula::Thesis { pos: loc } => self
        .thesis
        .as_deref()
        // this step is super sketchy, but Mizar actually lets you access the
        // `thesis` of outer `proof` scopes in a `now` block
        .or_else(|| self.thesis_stack.iter().rev().find_map(Option::as_deref))
        .unwrap_or_else(|| panic!("at {}:{loc:?}: thesis is not known", self.article))
        .clone()
        .maybe_neg(positive),
    };
    vprintln!("elab_formula {positive:?} {f:?}\n -> {res:?}");
    res
  }

  /// AnalizeArgTypeList
  fn elab_intern_push_locus_tys(&mut self, tys: &[ast::Type]) {
    assert!(self.lc.locus_ty.is_empty());
    for ty in tys {
      let mut ty = self.elab_type(ty);
      ty.visit(&mut self.r.intern_const());
      self.lc.locus_ty.push(ty);
    }
  }

  fn elab_term_no_reserve(&mut self, tm: &mut ast::Term) -> Term {
    assert!(self.collect_reserved(|cr| cr.visit_term(tm)).is_empty());
    self.elab_term(tm)
  }

  fn elab_type_no_reserve(&mut self, tm: &mut ast::Type) -> Type {
    assert!(self.collect_reserved(|cr| cr.visit_type(tm)).is_empty());
    self.elab_type(tm)
  }

  fn elab_intern_term_no_reserve(&mut self, tm: &mut ast::Term) -> Term {
    let mut tm = self.elab_term_no_reserve(tm);
    tm.visit(&mut self.r.intern_const());
    tm
  }

  fn elab_intern_type_no_reserve(&mut self, ty: &mut ast::Type) -> Type {
    let mut ty = self.elab_type_no_reserve(ty);
    ty.visit(&mut self.r.intern_const());
    ty
  }

  fn elab_intern_formula_forall_reserved(&mut self, f: &mut ast::Formula, pos: bool) -> Formula {
    let mut f = self.elab_formula_forall_reserved(f, pos).0;
    f.visit(&mut self.r.intern_const());
    f
  }

  fn try_unfold(
    &self, up: bool, def: &Definiens, pos: bool, kind: ConstrKind, args: &[Term],
  ) -> Option<(bool, Formula)> {
    let subst = def.matches(&self.g, &self.lc, kind, args)?.finish();
    let mut inst = Inst::new(&self.g.constrs, &self.lc, &subst, 0);
    let DefValue::Formula(value) = &def.value else { unreachable!() };
    let (pos2, f2) = if value.cases.is_empty() {
      (pos, value.otherwise.as_ref().unwrap().visit_cloned(&mut inst))
    } else {
      let f = Formula::mk_and_with(|disjs| {
        let mut otherwise = vec![];
        for case in &*value.cases {
          let guard = case.guard.visit_cloned(&mut inst);
          if value.otherwise.is_some() {
            guard.clone().mk_neg().append_conjuncts_to(&mut otherwise)
          }
          let f = Formula::mk_and_with(|conj| {
            guard.append_conjuncts_to(conj);
            case.case.visit_cloned(&mut inst).maybe_neg(pos).append_conjuncts_to(conj);
          });
          f.mk_neg().append_conjuncts_to(disjs)
        }
        if let Some(ow) = &value.otherwise {
          ow.visit_cloned(&mut inst).maybe_neg(pos).append_conjuncts_to(&mut otherwise);
          Formula::mk_and(otherwise).mk_neg().append_conjuncts_to(disjs)
        }
      });
      (false, f)
    };
    if matches!(def.assumptions, Formula::True) {
      Some((pos2, f2))
    } else {
      let f = Formula::mk_and_with(|conjs| {
        def.assumptions.visit_cloned(&mut inst).append_conjuncts_to(conjs);
        f2.maybe_neg(pos2 != up).append_conjuncts_to(conjs);
      });
      Some((!up, f))
    }
  }

  /// Replaces `f` with the normalized version,
  /// satisfying `f -> new_f` (up = true) or `new_f -> f` (up = false).
  /// Swapping both `up` and `f.0` produces almost the same result,
  /// but when unfolding conditional definitions we prefer to produce a
  /// CNF formula regardless of whether we are unfolding `f` or `!f`.
  ///
  /// For example, if `foo` is defined to be `if C { P } else { Q }`, then
  /// * `whnf(up, 1, foo)` yields `(C /\ P) \/ (!C /\ Q)` (for both values of `up`)
  /// * `whnf(up, 1, !foo)` yields `(C /\ !P) \/ (!C /\ !Q)` (for both values of `up`)
  ///
  /// `up` makes a difference for definitions with assumptions, to determine whether
  /// the assumptions should be written as `assum /\ unfolded` or `assum -> unfolded`.
  fn whnf(&self, up: bool, mut atomic: usize, f: &mut (bool, Box<Formula>)) -> usize {
    'start: loop {
      vprintln!("whnf (up = {up}, atomic = {atomic}) <- {f:?}");
      let mut args_buf;
      let (kind, args) = match &mut *f.1 {
        Formula::Neg { f: f2 } => {
          *f = (!f.0, std::mem::take(f2));
          continue 'start
        }
        Formula::PrivPred { value, .. } => {
          f.1 = std::mem::take(value);
          continue 'start
        }
        Formula::FlexAnd { .. } => {
          let Formula::FlexAnd { nat, le, terms, scope } = std::mem::take(&mut *f.1) else {
            unreachable!()
          };
          *f.1 = Global::expand_flex_and(nat, le, *terms, scope, 0);
          continue 'start
        }
        Formula::Pred { nr, args } if atomic > 0 => {
          let (n, args) = Formula::adjust_pred(*nr, args, Some(&self.g.constrs));
          (ConstrKind::Pred(n), args)
        }
        Formula::Attr { nr, args } if atomic > 0 => {
          let (n, args) = Formula::adjust_attr(*nr, args, Some(&self.g.constrs));
          (ConstrKind::Attr(n), args)
        }
        Formula::Is { term, ty } if atomic > 0 => {
          let TypeKind::Mode(n) = ty.kind else { break };
          if !matches!(&ty.attrs.0, Attrs::Consistent(attrs) if attrs.is_empty()) {
            break
          }
          let (n, args) = Type::adjust(n, &ty.args, &self.g.constrs);
          args_buf = args.to_vec();
          args_buf.push((**term).clone());
          (ConstrKind::Mode(n), &*args_buf)
        }
        _ => break,
      };
      for def in self.definitions.0.iter().rev() {
        if let Some((pos2, f2)) = self.try_unfold(up, def, f.0, kind, args) {
          f.0 = pos2;
          *f.1 = f2;
          // vprintln!("expanded {def:?}\n  -> {:?}", f);
          atomic -= 1;
          continue 'start
        }
      }
      break
    }
    atomic
  }

  /// ChopVars(Conclusion = !up)
  /// Attempts to instantiate `for x holds P[x]` to `P[term]`.
  /// * If `up = true` then `f -> (for x holds P[x])` (used for unfolding in hyps)
  /// * If `up = false` then `(for x holds P[x]) -> f` (unfolding thesis)
  fn inst_forall(&self, term: &Term, widenable: bool, up: bool, f: &mut (bool, Box<Formula>)) {
    self.whnf(up, MAX_EXPANSIONS, f);
    vprintln!("inst_forall {term:?}, {widenable}, {up}, {f:?}");
    let (true, Formula::ForAll { dom, scope }) = (f.0, &mut *f.1) else { panic!("not a forall") };
    let ty = term.get_type(&self.g, &self.lc, false);
    vprintln!("inst_forall {term:?}: {ty:?}");
    Inst0(0, term).visit_formula(scope);
    let ok = if !widenable {
      self.g.eq(&self.lc, &**dom, &ty)
    } else if up {
      dom.is_wider_than(&self.g, &self.lc, &ty)
    } else {
      ty.is_wider_than(&self.g, &self.lc, dom)
    };
    *f.1 = if ok {
      std::mem::take(scope)
    } else {
      assert!(ty.is_wider_than(&self.g, &self.lc, dom));
      assert!(self.g.eq_attrs(&self.lc, &ty, dom));
      Formula::mk_and_with(|conds| {
        loop {
          conds.push(Formula::Is { term: Box::new(term.clone()), ty: dom.clone() });
          *dom = dom.widening(&self.g, &self.lc).unwrap();
          if self.g.eq_radices(&self.lc, &ty, dom) {
            break
          }
        }
        conds.reverse();
        std::mem::take(scope).mk_neg().append_conjuncts_to(conds);
      })
      .mk_neg()
    }
  }

  /// ChopVars(Conclusion = !up)
  /// Attempts to unfold `for x1..xn holds P[x1..xn]` to `P[c1..cn]`
  /// where `c1..cn` are the fixed_vars starting at `start`.
  /// * If `up = true` then `f -> (for x1..xn holds P[x1..xn])` (used for unfolding in hyps)
  /// * If `up = false` then `(for x1..xn holds P[x1..xn]) -> f` (unfolding thesis)
  fn forall_telescope(
    &self, start: usize, widenable: bool, up: bool, f: &mut (bool, Box<Formula>),
  ) {
    for v in (start..self.lc.fixed_var.len()).map(ConstId::from_usize) {
      self.inst_forall(&Term::Const(v), widenable, up, f)
    }
  }

  /// Chopped(Conclusion = !up)
  /// Attempts to rewrite `conjs := tgt /\ conjs2` to `conjs2`.
  /// * If `up = true` then `conjs -> tgt /\ conjs2` (used for unfolding in hyps)
  /// * If `up = false` then `tgt /\ conjs2 -> conjs` (unfolding thesis)
  fn and_telescope(
    &mut self, tgt: Vec<Formula>, up: bool, conjs: Vec<Formula>,
  ) -> Result<Vec<Formula>, Box<TypeMismatch>> {
    vprintln!("and_telescope {tgt:?} <- {conjs:?}");
    let mut iter1 = UnfoldConjIter::new(tgt);
    let mut iter2 = UnfoldConjIter::new(conjs);
    'ok: loop {
      let mut f1 = match iter1.next() {
        Some(f1) => (true, f1),
        None => break 'ok,
      };
      let mut f2 = (true, Box::new(iter2.next().unwrap_or(Formula::True)));
      loop {
        if f1.0 == f2.0 && self.g.eq(&self.lc, &f1.1, &f2.1) {
          continue 'ok
        }
        match (&mut f1.1, &mut *f2.1) {
          (Formula::True, _) if f1.0 =>
            f1.1 = match iter1.next() {
              Some(f1) => f1,
              None => break 'ok,
            },
          (Formula::And { args }, _) if f1.0 => {
            let mut iter = std::mem::take(args).into_iter();
            f1.1 = iter.next().unwrap();
            iter1.push_front(iter);
          }
          (_, Formula::And { args }) if f2.0 => {
            let mut iter = std::mem::take(args).into_iter();
            *f2.1 = iter.next().unwrap();
            iter2.push_front(iter);
          }
          (Formula::Neg { f }, _) => {
            f1.1 = std::mem::take(f);
            f1.0 = !f1.0;
          }
          (_, Formula::Neg { f }) => {
            f2.1 = std::mem::take(f);
            f2.0 = !f2.0;
          }
          (
            Formula::PrivPred { nr: n1, value: v1, .. },
            Formula::PrivPred { nr: n2, value: v2, .. },
          ) => match (*n1).cmp(n2) {
            Ordering::Less => f2.1 = std::mem::take(v2),
            Ordering::Equal =>
              return Err(Box::new(TypeMismatch {
                got: f1.1.maybe_neg(f1.0),
                want: f2.1.maybe_neg(f2.0),
              })),
            Ordering::Greater => f1.1 = std::mem::take(v1),
          },
          (Formula::PrivPred { value, .. }, _) => f1.1 = std::mem::take(value),
          (_, Formula::PrivPred { value, .. }) => f2.1 = std::mem::take(value),
          (_, Formula::True) if f2.0 && !iter2.is_empty() =>
            *f2.1 = iter2.next().unwrap_or(Formula::True),
          _ => {
            vprintln!("compare: {f1:?} <> {f2:?}");
            if self.whnf(up, 1, &mut f2) != 0 {
              return Err(Box::new(TypeMismatch {
                got: f1.1.maybe_neg(f1.0),
                want: f2.1.maybe_neg(f2.0),
              }))
            }
          }
        }
      }
    }
    Ok(iter2.into_vec())
  }

  fn elab_spec(&mut self, spec: Option<&ast::Type>, tgt: &Type) -> Type {
    if let Some(ty) = spec {
      let ty = self.elab_type(ty);
      assert!(tgt.is_wider_than(&self.g, &self.lc, &ty));
      ty
    } else {
      tgt.clone()
    }
  }

  /// RClusterObj.RegisterCluster
  fn register_cluster(&mut self, mut attrs: Attrs, primary: Box<[Type]>, mut ty: Type) {
    let mut attrs1 = attrs.clone();
    attrs1.enlarge_by(&self.g.constrs, &self.lc, &ty.attrs.1); // lower cluster? [from original]
    attrs.enlarge_by(&self.g.constrs, &self.lc, &ty.attrs.1);
    attrs.round_up_with(&self.g, &self.lc, &ty, false);
    let Attrs::Consistent(_) = attrs else { panic!("inconsistent existential cluster") };
    ty.attrs = (Attrs::EMPTY, Attrs::EMPTY);
    self.read_registered_cluster(RegisteredCluster {
      cl: Cluster { primary, consequent: (attrs1, attrs) },
      ty: Box::new(ty),
    });
  }
}

struct TypeMismatch {
  got: Formula,
  want: Formula,
}
impl std::fmt::Debug for TypeMismatch {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "type mismatch: got {:?}, expected {:?}", self.got, self.want)
  }
}

struct UnfoldConjIter {
  stack: Vec<std::vec::IntoIter<Formula>>,
  iter: std::vec::IntoIter<Formula>,
}
impl UnfoldConjIter {
  fn new(conjs: Vec<Formula>) -> UnfoldConjIter {
    UnfoldConjIter { stack: vec![], iter: conjs.into_iter() }
  }
  fn push_front(&mut self, front: std::vec::IntoIter<Formula>) {
    self.stack.push(std::mem::replace(&mut self.iter, front))
  }
  fn into_vec(self) -> Vec<Formula> {
    self.iter.chain(self.stack.into_iter().rev().flatten()).collect()
  }
  fn is_empty(&self) -> bool { self.iter.as_slice().is_empty() && self.stack.is_empty() }
}
impl Iterator for UnfoldConjIter {
  type Item = Formula;
  fn next(&mut self) -> Option<Self::Item> {
    loop {
      if let Some(f) = self.iter.next() {
        if !matches!(f, Formula::True) {
          return Some(f)
        }
      }
      self.iter = self.stack.pop()?;
    }
  }
}

struct InstReservation<'a, 'b, F: Fn(BoundId) -> VarKind>(&'b Analyzer<'a>, F);

impl<F: Fn(BoundId) -> VarKind> VisitMut for InstReservation<'_, '_, F> {
  fn visit_term(&mut self, tm: &mut Term) {
    match *tm {
      Term::Bound(nr) => *tm = self.0.elab_var_kind(self.1(nr)),
      _ => self.super_visit_term(tm),
    }
  }
}

#[derive(Debug)]
struct InstConst {
  depth: u32,
  base: u32,
}

impl VisitMut for InstConst {
  fn visit_term(&mut self, tm: &mut Term) {
    match *tm {
      Term::Bound(ref mut nr) =>
        if nr.0 >= self.depth {
          nr.0 -= self.depth
        } else {
          *tm = Term::Const(ConstId(self.base + nr.0))
        },
      _ => self.super_visit_term(tm),
    }
  }
}

#[derive(Clone, Copy, Debug)]
enum Args {
  Unary([ConstId; 1]),
  Binary([ConstId; 2]),
}

#[derive(Debug)]
struct InstN<const N: usize> {
  from: [ConstId; N],
  to: [BoundId; N],
  it: BoundId,
  lift: u32,
}

impl<const N: usize> InstN<N> {
  fn new(args: [ConstId; N], lift: u32, it: u32, tgt: [u32; N]) -> InstN<N> {
    InstN { from: args, to: tgt.map(BoundId), it: BoundId(it), lift }
  }
}

impl<const N: usize> VisitMut for InstN<N> {
  fn visit_term(&mut self, tm: &mut Term) {
    match *tm {
      Term::Bound(ref mut nr) => nr.0 += self.lift,
      Term::Const(c) =>
        for i in 0..N {
          if c == self.from[i] {
            *tm = Term::Bound(self.to[i]);
            return
          }
        },
      Term::It => *tm = Term::Bound(self.it),
      _ => self.super_visit_term(tm),
    }
  }
}

#[derive(Clone, Copy, Debug)]
enum PropertyDeclKind<'a> {
  Func(Option<(FuncId, &'a [ConstId])>, Args),
  Pred(Args, bool),
  Mode(&'a DefBody<Formula>),
  None,
}

#[derive(Debug)]
struct PropertiesBuilder<'a> {
  visible: &'a [LocusId],
  kind: PropertyDeclKind<'a>,
  props: Properties,
  formula: Option<Box<Formula>>,
}

impl<'a> PropertiesBuilder<'a> {
  const fn new(visible: &'a [LocusId]) -> Self {
    Self { visible, kind: PropertyDeclKind::None, props: Properties::EMPTY, formula: None }
  }

  fn load_args(
    &mut self, g: &Global, lc: &LocalContext, to_const: &IdxVec<LocusId, ConstId>,
    f: impl FnOnce(Args) -> PropertyDeclKind<'a>,
  ) {
    match *self.visible {
      [v1] => {
        let c1 = to_const[v1];
        self.props.arg1 = v1.0;
        self.kind = f(Args::Unary([c1]));
      }
      [v1, v2] => {
        let c1 = to_const[v1];
        let c2 = to_const[v2];
        if g.eq(lc, &lc.fixed_var[c1].ty, &lc.fixed_var[c2].ty) {
          self.props.arg1 = v1.0;
          self.props.arg2 = v2.0;
          self.kind = f(Args::Binary([c1, c2]));
        }
      }
      _ => {}
    }
  }

  fn the_formula<const N: usize>(
    &self, args: [ConstId; N], lift: u32, it: u32, tgt: [u32; N],
  ) -> Formula {
    let f = self.formula.as_deref().unwrap();
    f.visit_cloned(&mut InstN::new(args, lift, it, tgt))
  }

  fn reflexivity(&self, lc: &LocalContext, args: [ConstId; 2], pos: bool) -> Formula {
    let ty = &lc.fixed_var[args[0]].ty;
    Formula::forall(ty.clone(), self.the_formula(args, 1, 0, [0, 0]).maybe_neg(pos))
  }

  fn asymmetry(&self, lc: &LocalContext, args: [ConstId; 2], pos1: bool, pos2: bool) -> Formula {
    let ty = &lc.fixed_var[args[0]].ty;
    let f = Formula::mk_and_with(|conj| {
      self.the_formula(args, 2, 0, [0, 1]).maybe_neg(pos1).append_conjuncts_to(conj);
      self.the_formula(args, 2, 0, [1, 0]).maybe_neg(pos2).append_conjuncts_to(conj);
    });
    Formula::forall(ty.clone(), Formula::forall(ty.clone(), f.mk_neg()))
  }

  fn elab_properties(&mut self, elab: &mut Analyzer<'_>, props: &mut [ast::Property]) {
    for prop in props {
      let (g, lc) = (&elab.g, &elab.lc);
      if g.cfg.analyzer_full {
        let thesis = match (prop.kind, &self.kind) {
          (PropertyKind::Symmetry, &PropertyDeclKind::Pred(Args::Binary(args), pos)) =>
            self.asymmetry(lc, args, pos, !pos),
          (PropertyKind::Reflexivity, &PropertyDeclKind::Pred(Args::Binary(args), pos)) =>
            self.reflexivity(lc, args, pos),
          (PropertyKind::Irreflexivity, &PropertyDeclKind::Pred(Args::Binary(args), pos)) =>
            self.reflexivity(lc, args, !pos),
          (PropertyKind::Connectedness, &PropertyDeclKind::Pred(Args::Binary(args), pos)) =>
            self.asymmetry(lc, args, !pos, !pos),
          (PropertyKind::Asymmetry, &PropertyDeclKind::Pred(Args::Binary(args), pos)) =>
            self.asymmetry(lc, args, pos, pos),
          (PropertyKind::Commutativity, &PropertyDeclKind::Func(t, Args::Binary(args))) => {
            let ty = &lc.fixed_var[args[0]].ty;
            if let Some((nr, args2)) = t {
              let term = |tgt| {
                let inst = &mut InstN::new(args, 2, 0, tgt);
                let args = args2.iter().map(|&c| Term::Const(c).visit_cloned(inst)).collect();
                Term::Functor { nr, args }
              };
              Formula::forall(
                ty.clone(),
                Formula::forall(ty.clone(), g.reqs.mk_eq(term([0, 1]), term([1, 0]))),
              )
            } else {
              let f = Formula::mk_and_with(|conj| {
                self.the_formula(args, 3, 0, [1, 2]).append_conjuncts_to(conj);
                self.the_formula(args, 3, 0, [2, 1]).mk_neg().append_conjuncts_to(conj);
              });
              Formula::forall(
                lc.it_type.as_deref().unwrap().clone(),
                Formula::forall(ty.clone(), Formula::forall(ty.clone(), f.mk_neg())),
              )
            }
          }
          (PropertyKind::Idempotence, &PropertyDeclKind::Func(_, Args::Binary(args))) => {
            let ty = &lc.fixed_var[args[0]].ty;
            assert!(lc.it_type.as_ref().unwrap().is_wider_than(g, lc, ty));
            Formula::forall(ty.clone(), self.the_formula(args, 1, 0, [0, 0]))
          }
          (PropertyKind::Involutiveness, &PropertyDeclKind::Func(_, Args::Unary(args))) => {
            let ty = &lc.fixed_var[args[0]].ty;
            let it_ty = lc.it_type.as_deref().unwrap();
            assert!(g.eq(lc, it_ty, ty));
            let f = Formula::mk_and_with(|conj| {
              self.the_formula(args, 2, 0, [1]).append_conjuncts_to(conj);
              self.the_formula(args, 2, 1, [0]).mk_neg().append_conjuncts_to(conj);
            });
            Formula::forall(it_ty.clone(), Formula::forall(it_ty.clone(), f.mk_neg()))
          }
          (PropertyKind::Projectivity, &PropertyDeclKind::Func(_, Args::Unary(args))) => {
            let ty = &lc.fixed_var[args[0]].ty;
            let it_ty = lc.it_type.as_deref().unwrap();
            let wty = &*ty.widening_of(g, lc, it_ty).unwrap();
            assert!(g.eq_radices(lc, wty, ty));
            assert!(ty.attrs.0.is_subset_of(&wty.attrs.1, |a1, a2| g.eq(lc, a1, a2)));
            let f = Formula::mk_and_with(|conj| {
              self.the_formula(args, 2, 0, [1]).append_conjuncts_to(conj);
              self.the_formula(args, 2, 0, [0]).mk_neg().append_conjuncts_to(conj);
            });
            Formula::forall(it_ty.clone(), Formula::forall(ty.clone(), f.mk_neg()))
          }
          (PropertyKind::Sethood, PropertyDeclKind::Mode(value)) => {
            let it_ty = lc.it_type.as_deref().unwrap();
            let f = value.by_cases(0, |case| {
              let f = Formula::mk_and_with(|conj| {
                case.visit_cloned(&mut AbstractIt(1, 2)).append_conjuncts_to(conj);
                let f = Formula::Pred {
                  nr: g.reqs.belongs_to().unwrap(),
                  args: Box::new([Term::Bound(BoundId(1)), Term::Bound(BoundId(0))]),
                };
                conj.push(f.mk_neg())
              });
              Formula::forall(Type::SET, Formula::forall(it_ty.clone(), f.mk_neg()).mk_neg())
                .mk_neg()
            });
            f.mk_neg()
          }
          (PropertyKind::Associativity, PropertyDeclKind::Func(_, Args::Binary(_))) =>
            panic!("associativity declarations are not supported"),
          (PropertyKind::Transitivity, &PropertyDeclKind::Pred(Args::Binary(_), _)) =>
            panic!("transitivity declarations are not supported"),
          (PropertyKind::Abstractness, _) => unreachable!(),
          (k, tgt) => panic!("property {k:?} is not applicable to {tgt:?}"),
        };
        elab.elab_justification(false, &thesis, &mut prop.just);
      }
      self.props.set(match self.kind {
        PropertyDeclKind::Pred(_, false) => prop.kind.flip(),
        _ => prop.kind,
      });
    }
    self.props.trim()
  }
}

struct Exportable;

impl Visit for Exportable {
  fn visit_term(&mut self, tm: &Term) {
    match tm {
      Term::Const(_) => panic!("local constant in exportable item"),
      Term::PrivFunc { .. } => panic!("private function in exportable item"),
      _ => self.super_visit_term(tm),
    }
  }

  fn visit_formula(&mut self, f: &Formula) {
    match f {
      Formula::PrivPred { .. } => panic!("private predicate in exportable item"),
      _ => self.super_visit_formula(f),
    }
  }
}

/// This type represents a list of reserved variables auto-generalized by
/// `elab_formula_forall_reserved`. Using `intro` adds a block of implicit `let` statements
/// corresponding to the collected variables (this is done for theorems and statements followed
/// by a `proof` block).
#[derive(Default)]
struct ReserveBlock(Vec<ReservedId>);

impl ReserveBlock {
  fn intro(self, elab: &mut Analyzer<'_>) {
    if !self.0.is_empty() {
      let mut thesis = elab.thesis.take().expect("must be called in a proof block");
      let mut inst = InstConst { depth: 0, base: elab.lc.fixed_var.len() as u32 };
      let lookup = Rc::make_mut(&mut elab.lookup);
      for &nr in &self.0 {
        let Formula::ForAll { mut dom, scope } = *thesis else { unreachable!() };
        if inst.depth != 0 {
          inst.visit_type(&mut dom);
        }
        let c = elab.r.lc.fixed_var.push(FixedVar { ty: *dom, def: None });
        elab.reserved_lookup.insert(nr, VarKind::Const(c));
        lookup.var.insert(elab.reserved[nr].0.clone(), VarKind::Const(c));
        thesis = scope;
        inst.depth += 1;
      }
      inst.visit_formula(&mut thesis);
      elab.thesis = Some(thesis);
    }
  }
}

#[derive(Clone, Debug)]
pub struct NameckScope {
  uses: im::OrdMap<ReservedId, bool>,
  reserved: im::HashMap<ReservedId, (VarKind, u32)>,
}

#[derive(Clone, Debug)]
pub struct FraenkelNameckResult {
  /// The scope for the scope part of the term
  pub scope: NameckScope,
  /// The scope for the compr part of the term
  pub compr: NameckScope,
}

struct CollectReserved<'a> {
  reserved: &'a IdxVec<ReservedId, (Rc<str>, ResGroupId)>,
  res_groups: &'a IdxVec<ResGroupId, ResGroup>,
  reserved_by_name: &'a HashMap<Rc<str>, ReservedId>,
  /// does not contain VarKind::Reserved
  reserved_lookup: &'a im::HashMap<ReservedId, VarKind>,
  /// does not contain VarKind::Reserved
  local_lookup: im::HashMap<ReservedId, (VarKind, u32)>,
  uses: im::OrdMap<ReservedId, bool>,
  lookup: Rc<NameLookup>,
  depth: BoundId,
  level: u32,
}

impl Analyzer<'_> {
  fn collect_reserved(&mut self, f: impl FnOnce(&mut CollectReserved<'_>)) -> Vec<ReservedId> {
    if !self.g.cfg.nameck_enabled {
      return vec![]
    }
    // for (i, (name, gp)) in self.reserved.enum_iter() {
    //   eprintln!(
    //     "{name}[{i:?}]{}: {:?}",
    //     if self.reserved_by_name.get(name) == Some(&i) { "!" } else { "" },
    //     self.res_groups[*gp]
    //   );
    // }
    assert!(self.lc.bound_var.is_empty());
    let mut cr = CollectReserved {
      reserved: &self.reserved,
      res_groups: &self.res_groups,
      reserved_by_name: &self.reserved_by_name,
      reserved_lookup: &self.reserved_lookup,
      uses: Default::default(),
      lookup: self.lookup.clone(),
      local_lookup: Default::default(),
      depth: Default::default(),
      level: 0,
    };
    f(&mut cr);
    let mut vars = vec![];
    for (&k, &bound) in &cr.uses {
      if !bound && !self.reserved_lookup.contains_key(&k) {
        vars.push(k)
      }
    }
    vars
  }

  fn map_bound(&self, mut i: BoundId) -> BoundId {
    // eprint!(
    //   "map_bound({:?}, {:?}): {i:?} -> ",
    //   self.reserved_extra_depth, self.reserved_more_insertion
    // );
    i.0 += self.reserved_extra_depth;
    for &(lo, len) in &self.reserved_more_insertion {
      if i.0 < lo {
        break
      }
      i.0 += len
    }
    // eprintln!("{i:?}");
    i
  }

  fn map_var_kind(&self, kind: VarKind) -> VarKind {
    match kind {
      VarKind::Bound(i) => VarKind::Bound(self.map_bound(i)),
      VarKind::Reserved(_) => unreachable!(),
      kind => kind,
    }
  }

  fn push_bound_reserved(
    &mut self, fvars: &[ReservedId], inner: bool, mut check: impl FnMut(&mut Self, &Type),
  ) {
    if !fvars.is_empty() {
      for &v in fvars {
        let ResGroup { ty, fvars } = &self.res_groups[self.reserved[v].1];
        let fvars = fvars.as_ref().unwrap();
        let mut ty = ty.clone();
        ty.visit(&mut InstReservation(self, |i| VarKind::Reserved(fvars[i])));
        check(self, &ty);
        let i = self.lc.bound_var.push(ty);
        self.reserved_lookup.insert(v, VarKind::Bound(i));
      }
      let len = fvars.len() as u32;
      if inner {
        self.reserved_more_insertion.push((self.lc.bound_var.len() as u32 - len, len))
      } else {
        self.reserved_extra_depth += len
      }
    }
  }

  fn elab_formula_forall_reserved(
    &mut self, f: &mut ast::Formula, pos: bool,
  ) -> (Formula, ReserveBlock) {
    let fvars = self.collect_reserved(|cr| cr.visit_formula(f));
    if fvars.is_empty() {
      return (self.elab_formula(f, pos), ReserveBlock::default())
    }
    assert!(self.lc.bound_var.is_empty());
    self.lc.term_cache.get_mut().open_scope();
    let lookup = self.reserved_lookup.clone();
    self.push_bound_reserved(&fvars, false, |_, _| {});
    let mut f2 = self.elab_formula(f, pos);
    for ty in self.lc.bound_var.0.drain(..).rev() {
      f2 = Formula::ForAll { dom: Box::new(ty), scope: Box::new(f2) }
    }
    self.reserved_extra_depth = 0;
    self.reserved_lookup = lookup;
    self.lc.term_cache.get_mut().close_scope();
    (f2, ReserveBlock(fvars))
  }
}

impl CollectReserved<'_> {
  fn use_reserved(&mut self, pos: Position, n: ReservedId) {
    for &m in &self.res_groups[self.reserved[n].1].fvars.as_ref().unwrap().0 {
      if let Some(k) = self.local_lookup.get(&m) {
        assert!(
          k.1 < self.level,
          "{pos:?}: can't use reserved variable because it would result in an invalid type"
        )
      } else if !self.reserved_lookup.contains_key(&m) {
        self.uses.insert(m, false);
      }
    }
    self.uses.insert(n, false);
  }

  fn merge_bound_uses(
    uses: &mut im::OrdMap<ReservedId, bool>, bound: &im::OrdMap<ReservedId, bool>,
  ) {
    for (&k, _) in bound {
      match uses.entry(k) {
        im::ordmap::Entry::Occupied(mut e) => {
          e.insert(false);
        }
        im::ordmap::Entry::Vacant(e) => {
          e.insert(true);
        }
      }
    }
  }

  fn push_bound(&mut self, vars: &mut [ast::BinderGroup]) {
    for ast::BinderGroup { vars, ty } in vars {
      if let Some(ty) = ty {
        self.visit_type(ty);
        for v in &*vars {
          (Rc::make_mut(&mut self.lookup).var)
            .insert(v.spelling.clone(), VarKind::Bound(self.depth.fresh()));
        }
      } else {
        assert!(vars.len() == 1);
        let Some(&i) = self.reserved_by_name.get(&*vars[0].spelling) else {
          panic!("{:?}: variable missing type", vars[0].pos)
        };
        let group = self.reserved[i].1;
        let subst = (self.res_groups[group].fvars.as_ref().unwrap().0.iter())
          .map(|&i| {
            if let Some(&(kind, _)) = self.local_lookup.get(&i) {
              kind
            } else if let Some(&kind) = self.reserved_lookup.get(&i) {
              kind
            } else {
              self.use_reserved(vars[0].pos, i);
              VarKind::Reserved(i)
            }
          })
          .collect_vec();
        *ty = Some(Box::new(ast::Type::Reservation { pos: vars[0].pos, group, subst }));
        let v = self.depth.fresh();
        Rc::make_mut(&mut self.lookup).var.insert(vars[0].spelling.clone(), VarKind::Bound(v));
        self.local_lookup.insert(i, (VarKind::Bound(v), self.level));
      }
    }
  }

  fn scope(&mut self, f: impl FnOnce(&mut Self)) {
    let sc = (self.lookup.clone(), self.local_lookup.clone(), self.depth);
    f(self);
    (self.lookup, self.local_lookup, self.depth) = sc;
  }

  fn visit_term(&mut self, tm: &mut ast::Term) {
    match tm {
      ast::Term::Var { pos, kind, ref spelling } =>
        if kind.is_none() {
          let Some(kind2) = (self.lookup.var.get(&**spelling).copied())
            .or_else(|| Some(VarKind::Reserved(*self.reserved_by_name.get(&**spelling)?)))
          else {
            panic!("{pos:?}: unresolved variable '{spelling}'")
          };
          if let VarKind::Reserved(n) = kind2 {
            self.use_reserved(*pos, n)
          }
          *kind = Some(kind2)
        },
      ast::Term::Placeholder { .. }
      | ast::Term::Numeral { .. }
      | ast::Term::InternalSelector { .. }
      | ast::Term::It { .. } => {}
      ast::Term::PrivFunc { args, .. }
      | ast::Term::Infix { args, .. }
      | ast::Term::Bracket { args, .. }
      | ast::Term::Aggregate { args, .. } => self.visit_terms(args),
      ast::Term::SubAggr { arg, .. } | ast::Term::Selector { arg, .. } => self.visit_term(arg),
      ast::Term::Qua { term, ty, .. } => {
        self.visit_term(term);
        self.visit_type(ty)
      }
      ast::Term::The { ty, .. } => self.visit_type(ty),
      ast::Term::Fraenkel { vars, scope, compr, nameck, .. } => self.scope(|this| {
        this.push_bound(vars);
        let orig = std::mem::take(&mut this.uses);
        let scope_reserved = this.local_lookup.clone();
        this.level += 1;
        this.visit_term(scope);
        let mut scope_uses = std::mem::take(&mut this.uses);
        let compr_reserved = this.local_lookup.clone();
        if let Some(f) = compr {
          this.visit_formula(f)
        }
        this.level -= 1;
        let compr_uses = std::mem::replace(&mut this.uses, orig);
        Self::merge_bound_uses(&mut scope_uses, &compr_uses);
        Self::merge_bound_uses(&mut this.uses, &scope_uses);
        if !scope_uses.is_empty() {
          *nameck = Some(Box::new(FraenkelNameckResult {
            scope: NameckScope { uses: scope_uses, reserved: scope_reserved },
            compr: NameckScope { uses: compr_uses, reserved: compr_reserved },
          }))
        }
      }),
    }
  }

  fn visit_terms(&mut self, tms: &mut [ast::Term]) {
    tms.iter_mut().for_each(|tm| self.visit_term(tm))
  }

  fn visit_attr(&mut self, attr: &mut ast::Attr) {
    match attr {
      ast::Attr::Attr { args, .. } => self.visit_terms(args),
      ast::Attr::Non { attr, .. } => self.visit_attr(attr),
    }
  }

  fn visit_attrs(&mut self, attrs: &mut [ast::Attr]) {
    attrs.iter_mut().for_each(|attr| self.visit_attr(attr))
  }

  fn visit_type(&mut self, ty: &mut ast::Type) {
    match ty {
      ast::Type::Mode { args, .. } | ast::Type::Struct { args, .. } => self.visit_terms(args),
      ast::Type::Cluster { attrs, ty, .. } => {
        self.visit_attrs(attrs);
        self.visit_type(ty)
      }
      ast::Type::Reservation { .. } => unreachable!(),
    }
  }

  fn visit_formula(&mut self, f: &mut ast::Formula) {
    match f {
      ast::Formula::Not { f, .. } => self.visit_formula(f),
      ast::Formula::Binop { f1, f2, .. } => {
        self.visit_formula(f1);
        self.visit_formula(f2)
      }
      ast::Formula::Pred(pred) => self.visit_terms(&mut pred.args),
      ast::Formula::ChainPred { first, rest, .. } => {
        self.visit_terms(&mut first.args);
        rest.iter_mut().for_each(|rhs| self.visit_terms(&mut rhs.right));
      }
      ast::Formula::PrivPred { args, .. } => self.visit_terms(args),
      ast::Formula::Attr { term, attrs, .. } => {
        self.visit_term(term);
        self.visit_attrs(attrs)
      }
      ast::Formula::Is { term, ty, .. } => {
        self.visit_term(term);
        self.visit_type(ty)
      }
      ast::Formula::Binder { vars, st, scope, .. } => self.scope(|this| {
        this.push_bound(vars);
        if let Some(st) = st {
          this.visit_formula(st)
        }
        this.visit_formula(scope)
      }),
      ast::Formula::False { .. } | ast::Formula::Thesis { .. } => {}
    }
  }
}

trait ReadProof {
  type CaseIter;
  type SupposeRecv;
  type Output: Visitable<Descope>;

  /// Changes the thesis from `for x1..xn holds P` to `P`
  /// where `x1..xn` are the fixed_vars introduced since `start`
  fn intro(&mut self, elab: &mut Analyzer, start: usize, istart: u32);

  /// Changes the thesis from `!(conj1 & ... & conjn & rest)` to `!rest`
  fn assume(&mut self, elab: &mut Analyzer, conjs: Vec<Formula>);

  /// Changes the thesis from `!(!(for x1..xn holds !f) & rest)` to `!rest`
  /// (that is, `(ex x1..xn st f) -> rest` to `rest`)
  /// where `x1..xn` are the fixed_vars introduced since `start`
  fn given(&mut self, elab: &mut Analyzer, start: usize, istart: u32, f: Formula);

  /// Changes the thesis from `ex x st P(x)` to `P(term)`
  fn take(&mut self, elab: &mut Analyzer, term: Term);

  /// Changes the thesis from `ex x st P(x)` to `P(v)`,
  /// where `v` is the last `fixed_var` to be introduced
  fn take_as_var(&mut self, elab: &mut Analyzer, v: ConstId) { self.take(elab, Term::Const(v)); }

  /// Changes the thesis from `conjs & rest` to `rest`
  fn thus(&mut self, elab: &mut Analyzer, conjs: Vec<Formula>);

  /// Unfold the definitions `refs` in the thesis
  fn unfold(&mut self, elab: &mut Analyzer, refs: &[ast::Reference]);

  fn new_cases(&mut self, elab: &mut Analyzer) -> Self::CaseIter;

  fn new_case(&mut self, _: &mut Analyzer, _: &mut Self::CaseIter, _: &[Formula]) {}

  fn end_case(&mut self, _: &mut Analyzer, _: &mut Self::CaseIter, _: Self::Output) {}

  fn end_cases(&mut self, _: &mut Analyzer, _: Self::CaseIter) {}

  fn new_supposes(&mut self, elab: &mut Analyzer) -> Self::SupposeRecv;

  fn new_suppose(&mut self, _: &mut Analyzer, _: &mut Self::SupposeRecv, _: &[Formula]) {}

  fn end_suppose(&mut self, _: &mut Analyzer, _: &mut Self::SupposeRecv, _: Self::Output) {}

  fn end_supposes(&mut self, _: &mut Analyzer, _: Self::SupposeRecv) {}

  fn end_block(&mut self, elab: &mut Analyzer, end: Position) -> Self::Output;

  fn super_elab_item(&mut self, elab: &mut Analyzer, it: &mut ast::Item) -> bool {
    match &it.kind {
      ast::ItemKind::Let { .. } => elab.item_header(it, "Let"),
      ast::ItemKind::Assume { .. } => elab.item_header(it, "Assume"),
      ast::ItemKind::Unfold { .. } => elab.item_header(it, "Unfold"),
      ast::ItemKind::Given { .. } => elab.item_header(it, "Given"),
      ast::ItemKind::Take { .. } => elab.item_header(it, "Take"),
      ast::ItemKind::Thus { .. } => elab.item_header(it, "Thus"),
      ast::ItemKind::PerCases { .. } => elab.item_header(it, "PerCases"),
      _ => {}
    }
    // eprintln!("[{:?}] thesis = {:?}", it.pos, elab.thesis);
    match &mut it.kind {
      ast::ItemKind::Let { vars, conds } => {
        let n = elab.lc.fixed_var.len();
        let istart = elab.lc.infer_const.get_mut().len() as u32;
        for var in vars {
          elab.elab_fixed_vars(var);
        }
        self.intro(elab, n, istart);
        if !conds.is_empty() {
          let mut conjs = vec![];
          for prop in conds {
            elab.elab_proposition_forall_reserved(prop, true).append_conjuncts_to(&mut conjs);
          }
          self.assume(elab, conjs)
        }
      }
      ast::ItemKind::Assume(asm) => {
        let mut conjs = vec![];
        for prop in asm.conds() {
          elab.elab_proposition_forall_reserved(prop, true).append_conjuncts_to(&mut conjs);
        }
        self.assume(elab, conjs)
      }
      ast::ItemKind::Given { vars, conds } => {
        let n = elab.lc.fixed_var.len();
        let istart = elab.lc.infer_const.get_mut().len() as u32;
        for var in vars {
          elab.elab_fixed_vars(var);
        }
        let f = Formula::mk_and_with(|conjs| {
          conds.iter_mut().for_each(|prop| {
            elab.elab_proposition_forall_reserved(prop, true).append_conjuncts_to(conjs)
          })
        });
        self.given(elab, n, istart, f);
      }
      ast::ItemKind::Take(its) =>
        for ast::TakeDecl { var, term } in its {
          let term = elab.elab_intern_term_no_reserve(term);
          if let Some(var) = var {
            let ty = term.get_type(&elab.g, &elab.lc, false);
            let v = elab.lc.fixed_var.push(FixedVar { ty, def: Some((Box::new(term), false)) });
            if elab.g.cfg.nameck_enabled {
              Rc::make_mut(&mut elab.lookup).var.insert(var.spelling.clone(), VarKind::Const(v));
            }
            self.take_as_var(elab, v)
          } else {
            self.take(elab, term)
          }
        },
      ast::ItemKind::Thus(stmt) =>
        if elab.g.cfg.analyzer_full {
          let f = elab.elab_stmt(stmt);
          self.thus(elab, f.into_conjuncts())
        },
      ast::ItemKind::PerCases { just, kind: CaseKind::Case, blocks } => {
        let mut iter = self.new_cases(elab);
        let f = Formula::mk_and_with(|disjs| {
          for bl in blocks {
            let (case, o) = elab.scope(false, true, false, |elab| {
              let case = Formula::mk_and_with(|conjs| {
                for prop in bl.hyp.conds() {
                  elab.elab_proposition_forall_reserved(prop, true).append_conjuncts_to(conjs);
                }
                self.new_case(elab, &mut iter, conjs)
              });
              (case, self.elab_proof(elab, &mut bl.items, bl.end))
            });
            case.mk_neg().append_conjuncts_to(disjs);
            self.end_case(elab, &mut iter, o);
          }
          self.end_cases(elab, iter);
        });
        elab.elab_justification(false, &f.mk_neg(), just);
        return false
      }
      ast::ItemKind::Unfold(refs) => self.unfold(elab, refs),
      ast::ItemKind::PerCases { just, kind: CaseKind::Suppose, blocks } => {
        let f = Formula::mk_and_with(|disjs| {
          let mut recv = self.new_supposes(elab);
          for bl in blocks {
            let (case, o) = elab.scope(false, true, false, |elab| {
              let case = Formula::mk_and_with(|conjs| {
                for prop in bl.hyp.conds() {
                  elab.elab_proposition_forall_reserved(prop, true).append_conjuncts_to(conjs);
                }
                self.new_suppose(elab, &mut recv, conjs);
              });
              (case, self.elab_proof(elab, &mut bl.items, bl.end))
            });
            case.mk_neg().append_conjuncts_to(disjs);
            self.end_suppose(elab, &mut recv, o);
          }
          self.end_supposes(elab, recv)
        });
        elab.elab_justification(false, &f.mk_neg(), just);
        return false
      }
      _ => elab.elab_stmt_item(it),
    }
    true
  }

  fn elab_item(&mut self, elab: &mut Analyzer, item: &mut ast::Item) -> bool {
    self.super_elab_item(elab, item)
  }

  fn elab_proof(
    &mut self, elab: &mut Analyzer, items: &mut [ast::Item], end: Position,
  ) -> Self::Output {
    for item in items {
      if !self.elab_item(elab, item) {
        break
      }
    }
    self.end_block(elab, end)
  }
}

struct CorrConds(EnumMap<CorrCondKind, Option<Box<Formula>>>);

impl CorrConds {
  const fn new() -> Self { Self(EnumMap::from_array([None, None, None, None, None, None])) }
}

struct AbstractIt(u32, u32);
impl AbstractIt {
  fn forall(it_type: &Type, mut f: Formula, pos: bool) -> Formula {
    AbstractIt(0, 1).visit_formula(&mut f);
    Formula::forall(it_type.clone(), f.maybe_neg(pos))
  }
}

impl VisitMut for AbstractIt {
  fn visit_term(&mut self, tm: &mut Term) {
    match tm {
      Term::Bound(n) => n.0 += self.1,
      Term::It => *tm = Term::Bound(BoundId(self.0)),
      _ => self.super_visit_term(tm),
    }
  }
}

struct AbstractLocus(u32);
impl VisitMut for AbstractLocus {
  fn visit_term(&mut self, tm: &mut Term) {
    match tm {
      Term::Bound(n) => n.0 += self.0,
      Term::Const(_) => panic!("unexpected local constant"),
      Term::Locus(LocusId(n)) => *tm = Term::Bound(BoundId(*n as _)),
      Term::It => panic!("unexpected 'it'"),
      _ => self.super_visit_term(tm),
    }
  }
}

pub trait BodyKind {
  fn it_eq(&self, g: &Global) -> Formula;
  fn mk_eq(&self, g: &Global, other: &Self) -> Formula;
}
impl BodyKind for Term {
  fn it_eq(&self, g: &Global) -> Formula { g.reqs.mk_eq(Term::It, self.clone()) }
  fn mk_eq(&self, g: &Global, other: &Self) -> Formula { g.reqs.mk_eq(self.clone(), other.clone()) }
}
impl BodyKind for Formula {
  fn it_eq(&self, _: &Global) -> Formula { self.clone() }
  fn mk_eq(&self, _: &Global, other: &Self) -> Formula { self.clone().mk_iff(other.clone()) }
}

impl<T: BodyKind> DefBody<T> {
  fn mk_consistency(&self, g: &Global, it_type: Option<&Type>) -> Option<Box<Formula>> {
    if self.cases.is_empty() {
      return None
    }
    let f = Formula::mk_and_with(|conjs| {
      for (i, j) in self.cases.iter().tuple_combinations() {
        let f = Formula::mk_and_with(|disj| {
          i.guard.clone().append_conjuncts_to(disj);
          j.guard.clone().append_conjuncts_to(disj);
          i.case.it_eq(g).mk_iff(j.case.it_eq(g)).mk_neg().append_conjuncts_to(disj);
        });
        f.mk_neg().append_conjuncts_to(conjs);
      }
    });
    Some(Box::new(match it_type {
      Some(it_type) => AbstractIt::forall(it_type, f, true),
      None => f,
    }))
  }

  fn by_cases(&self, lift: u32, neg_f: impl Fn(&T) -> Formula) -> Box<Formula> {
    let mut els = self.otherwise.as_ref().map(|_| vec![]);
    Box::new(Formula::mk_and_with(|conjs| {
      for def in &*self.cases {
        let f = Formula::mk_and_with(|disj| {
          let mut guard = def.guard.clone();
          if lift != 0 {
            OnVarMut(|n| *n += lift).visit_formula(&mut guard);
          }
          if let Some(els) = &mut els {
            guard.clone().mk_neg().append_conjuncts_to(els)
          }
          guard.append_conjuncts_to(disj);
          neg_f(&def.case).append_conjuncts_to(disj);
        });
        f.mk_neg().append_conjuncts_to(conjs);
      }
      if let (Some(mut els), Some(ow)) = (els, &self.otherwise) {
        neg_f(ow).append_conjuncts_to(&mut els);
        Formula::mk_and(els).mk_neg().append_conjuncts_to(conjs)
      }
    }))
  }

  fn iffthm_for(&self, g: &Global, defines: &Formula) -> Box<Formula> {
    self.by_cases(0, |case| defines.clone().mk_iff(case.it_eq(g)).mk_neg())
  }

  fn defthm_for(&self, g: &Global, defines: &T) -> Box<Formula> {
    self.by_cases(0, |case| defines.mk_eq(g, case).mk_neg())
  }

  fn mk_compatibility(
    &self, g: &Global, it_type: Option<&Type>, defines: &Formula,
  ) -> Box<Formula> {
    let mut f = self.iffthm_for(g, defines);
    if let Some(it_type) = it_type {
      *f = AbstractIt::forall(it_type, std::mem::take(&mut *f), true)
    }
    f
  }
}

impl DefBody<Formula> {
  fn mk_existence(&self, it_type: &Type) -> Box<Formula> {
    self.by_cases(0, |case| AbstractIt::forall(it_type, case.clone(), false))
  }

  fn mk_uniqueness(&self, g: &Global, it_type: &Type) -> Box<Formula> {
    let scope = self.by_cases(2, |case| {
      Formula::mk_and_with(|conjs| {
        case.visit_cloned(&mut AbstractIt(0, 2)).append_conjuncts_to(conjs);
        case.visit_cloned(&mut AbstractIt(1, 2)).append_conjuncts_to(conjs);
        conjs.push(g.reqs.mk_eq(Term::Bound(BoundId(0)), Term::Bound(BoundId(1))).mk_neg())
      })
    });
    let it_type2 = it_type.visit_cloned(&mut AbstractIt(0, 1));
    Box::new(Formula::forall(it_type.clone(), Formula::forall(it_type2, *scope)))
  }
}

impl DefBody<Term> {
  fn mk_coherence(&self, it_type: &Type) -> Box<Formula> {
    self.by_cases(0, |case| {
      Formula::Is { term: Box::new(case.clone()), ty: Box::new(it_type.clone()) }.mk_neg()
    })
  }
}

fn mk_mode_coherence(
  ctx: &Constructors, lc: &LocalContext, nr: ModeId, attrs: &Attrs, args: Vec<Term>, it_type: &Type,
) -> Box<Formula> {
  Box::new(Formula::forall(
    Type {
      kind: TypeKind::Mode(nr),
      attrs: (Attrs::EMPTY, attrs.visit_cloned(&mut Inst::new(ctx, lc, &args, 0))),
      args,
    },
    Formula::Is { term: Box::new(Term::Bound(BoundId(0))), ty: Box::new(it_type.clone()) },
  ))
}

fn mk_func_coherence(nr: FuncId, args: Box<[Term]>, it_type: &Type) -> Box<Formula> {
  Box::new(Formula::Is {
    term: Box::new(Term::Functor { nr, args }),
    ty: Box::new(it_type.clone()),
  })
}

impl DefValue {
  fn mk_consistency(&self, g: &Global, it_type: Option<&Type>) -> Option<Box<Formula>> {
    match self {
      DefValue::Term(value) => value.mk_consistency(g, it_type),
      DefValue::Formula(value) => value.mk_consistency(g, it_type),
    }
  }

  fn mk_compatibility(
    &self, g: &Global, it_type: Option<&Type>, defines: &Formula,
  ) -> Box<Formula> {
    match self {
      DefValue::Term(value) => value.mk_compatibility(g, it_type, defines),
      DefValue::Formula(value) => value.mk_compatibility(g, it_type, defines),
    }
  }

  fn as_formula(&self, g: &Global) -> Box<Formula> {
    match self {
      DefValue::Term(value) => value.by_cases(0, |case| case.it_eq(g).mk_neg()),
      DefValue::Formula(value) => value.by_cases(0, |case| case.it_eq(g).mk_neg()),
    }
  }
}

struct WithThesis;

impl ReadProof for WithThesis {
  type CaseIter = UnfoldConjIter;
  type SupposeRecv = ();
  type Output = ();

  fn intro(&mut self, elab: &mut Analyzer, start: usize, _: u32) {
    let mut thesis = (true, elab.thesis.take().unwrap());
    elab.forall_telescope(start, false, false, &mut thesis);
    elab.thesis = Some(Box::new(thesis.1.maybe_neg(thesis.0)));
  }

  fn assume(&mut self, elab: &mut Analyzer, conjs: Vec<Formula>) {
    let thesis = elab.thesis.take().unwrap().mk_neg().into_conjuncts();
    let args = elab.and_telescope(conjs, true, thesis).unwrap_or_else(|t| panic!("{t:?}"));
    elab.thesis = Some(Box::new(Formula::mk_and(args).mk_neg()))
  }

  fn given(&mut self, elab: &mut Analyzer, start: usize, istart: u32, f: Formula) {
    let mut f = f.mk_neg();
    let end = elab.lc.fixed_var.len();
    elab.lc.mk_forall(start..end, istart, false, &mut f);
    self.assume(elab, vec![f.mk_neg()]);
  }

  fn take(&mut self, elab: &mut Analyzer, term: Term) {
    let mut thesis = (false, elab.thesis.take().unwrap());
    elab.inst_forall(&term, true, true, &mut thesis);
    elab.thesis = Some(Box::new(thesis.1.maybe_neg(!thesis.0)));
  }

  fn thus(&mut self, elab: &mut Analyzer, f: Vec<Formula>) {
    let thesis = elab.thesis.take().unwrap().into_conjuncts();
    let args = elab.and_telescope(f, false, thesis).unwrap_or_else(|t| panic!("{t:?}"));
    elab.thesis = Some(Box::new(Formula::mk_and(args)))
  }

  fn unfold(&mut self, elab: &mut Analyzer, refs: &[ast::Reference]) {
    let mut f = (true, elab.thesis.take().unwrap());
    for r in elab.elab_references(refs) {
      let def = match r.kind {
        ReferenceKind::Priv(label) => elab.local_def_map.get(&label),
        ReferenceKind::Thm(_) => None,
        ReferenceKind::Def(r) => elab.def_map.get(&r),
      };
      let def = &elab.definitions[*def.expect("not a definition reference")];
      let fail = || panic!("thesis is not the specified definition");
      let mut args_buf;
      let (kind, args) = loop {
        match &mut *f.1 {
          Formula::Neg { f: f2 } => f = (!f.0, std::mem::take(f2)),
          Formula::PrivPred { value, .. } => f.1 = std::mem::take(value),
          Formula::Pred { nr, args } => {
            let (n, args) = Formula::adjust_pred(*nr, args, Some(&elab.g.constrs));
            break (ConstrKind::Pred(n), args)
          }
          Formula::Attr { nr, args } => {
            let (n, args) = Formula::adjust_attr(*nr, args, Some(&elab.g.constrs));
            break (ConstrKind::Attr(n), args)
          }
          Formula::Is { term, ty } => {
            let TypeKind::Mode(n) = ty.kind else { fail() };
            if !matches!(&ty.attrs.0, Attrs::Consistent(attrs) if attrs.is_empty()) {
              fail()
            }
            let (n, args) = Type::adjust(n, &ty.args, &elab.g.constrs);
            args_buf = args.to_vec();
            args_buf.push((**term).clone());
            break (ConstrKind::Mode(n), &*args_buf)
          }
          _ => fail(),
        }
      };
      let Some((pos, f2)) = elab.try_unfold(false, def, true, kind, args) else { fail() };
      f.0 = pos;
      *f.1 = f2;
    }
    elab.thesis = Some(Box::new(f.1.maybe_neg(f.0)));
  }

  fn new_cases(&mut self, elab: &mut Analyzer) -> Self::CaseIter {
    UnfoldConjIter::new(elab.thesis.as_ref().unwrap().clone().mk_neg().into_conjuncts())
  }

  fn new_case(&mut self, elab: &mut Analyzer, iter: &mut Self::CaseIter, f: &[Formula]) {
    let mut err = None;
    let args = 'next: loop {
      let mut thesis = (false, Box::new(iter.next().unwrap_or(Formula::True)));
      loop {
        let conjs = thesis.1.clone().maybe_neg(thesis.0).into_conjuncts();
        match elab.and_telescope(f.to_vec(), false, conjs) {
          Ok(args) => break 'next args,
          Err(e) => {
            let e = err.get_or_insert(e);
            assert!(elab.whnf(false, 1, &mut thesis) == 0, "{e:?}");
            if !thesis.0 && matches!(*thesis.1, Formula::And { .. }) {
              iter.push_front(thesis.1.into_conjuncts().into_iter());
              continue 'next
            }
          }
        }
      }
    };
    elab.thesis = Some(Box::new(Formula::mk_and(args)));
  }

  fn end_cases(&mut self, elab: &mut Analyzer, mut case: Self::CaseIter) {
    assert!(case.next().is_none());
    **elab.thesis.as_mut().unwrap() = Formula::True;
  }

  fn new_supposes(&mut self, _: &mut Analyzer) {}

  fn end_supposes(&mut self, elab: &mut Analyzer, _: ()) {
    **elab.thesis.as_mut().unwrap() = Formula::True;
  }

  fn end_block(&mut self, elab: &mut Analyzer, end: Position) {
    let f = elab.thesis.as_deref().unwrap();
    if !matches!(f, Formula::True) {
      eprintln!(
        "error at {}:{end:?}: block incomplete; thesis at end of block:\n  {f:?}",
        elab.article
      );
      panic!("{end:?}: block incomplete")
    }
  }
}

#[derive(Debug)]
enum ProofStep {
  Let { range: Range<usize>, istart: u32 },
  Assume { conjs: Vec<Formula> },
  Given { range: Range<usize>, istart: u32, not_f: Formula },
  TakeAsVar { range: Range<usize>, istart: u32 },
  Thus { conjs: Vec<Formula> },
  Break(bool),
}

struct ReconstructThesis {
  stack: Vec<ProofStep>,
}

impl ReconstructThesis {
  fn reconstruct(&mut self, elab: &mut Analyzer, pos: bool) -> Formula {
    struct Reconstruction {
      pos: bool,
      conjs: Vec<Formula>,
    }
    impl Reconstruction {
      fn as_pos(&mut self, pos: bool) -> &mut Vec<Formula> {
        if self.pos != pos {
          self.pos = pos;
          self.conjs = Formula::mk_and(std::mem::take(&mut self.conjs)).mk_neg().into_conjuncts();
        }
        &mut self.conjs
      }
    }
    let conjs = elab.thesis.take().map_or_else(Vec::new, |f| f.into_conjuncts());
    let mut rec = Reconstruction { pos, conjs };
    loop {
      match self.stack.pop().unwrap() {
        ProofStep::Let { range, istart } => {
          let mut f = Formula::mk_and(std::mem::take(rec.as_pos(true)));
          elab.lc.mk_forall(range, istart, true, &mut f);
          rec.conjs = vec![f];
        }
        ProofStep::Assume { mut conjs } => {
          let rest = rec.as_pos(false);
          std::mem::swap(&mut conjs, rest);
          rest.append(&mut conjs)
        }
        ProofStep::Given { range, istart, mut not_f } => {
          let rest = rec.as_pos(false);
          elab.lc.mk_forall(range, istart, true, &mut not_f);
          rest.insert(0, not_f.mk_neg())
        }
        ProofStep::TakeAsVar { range, istart } => {
          let mut f = Formula::mk_and(std::mem::take(rec.as_pos(false)));
          elab.lc.mk_forall(range, istart, true, &mut f);
          rec.conjs = vec![f];
        }
        ProofStep::Thus { mut conjs } => {
          let rest = rec.as_pos(true);
          std::mem::swap(&mut conjs, rest);
          rest.append(&mut conjs)
        }
        ProofStep::Break(pos2) => {
          assert_eq!(pos, pos2);
          return Formula::mk_and(std::mem::take(rec.as_pos(pos)))
        }
      }
    }
  }
}

impl ReadProof for ReconstructThesis {
  type CaseIter = ();
  type SupposeRecv = Option<Box<Formula>>;
  type Output = Formula;

  fn intro(&mut self, elab: &mut Analyzer, start: usize, istart: u32) {
    match self.stack.last_mut() {
      Some(ProofStep::Let { range, .. }) if range.end == start =>
        range.end = elab.lc.fixed_var.len(),
      _ => self.stack.push(ProofStep::Let { range: start..elab.lc.fixed_var.len(), istart }),
    }
  }

  fn assume(&mut self, _: &mut Analyzer, mut conjs: Vec<Formula>) {
    if let Some(ProofStep::Assume { conjs: rest }) = self.stack.last_mut() {
      rest.append(&mut conjs)
    } else {
      self.stack.push(ProofStep::Assume { conjs })
    }
  }

  fn given(&mut self, elab: &mut Analyzer, start: usize, istart: u32, f: Formula) {
    self.stack.push(ProofStep::Given {
      range: start..elab.lc.fixed_var.len(),
      istart,
      not_f: f.mk_neg(),
    })
  }

  fn take(&mut self, _: &mut Analyzer, _: Term) { panic!("take steps are not reconstructible") }

  fn take_as_var(&mut self, elab: &mut Analyzer, v: ConstId) {
    match self.stack.last_mut() {
      Some(ProofStep::TakeAsVar { range, .. }) if range.end == v.0 as usize =>
        range.end = elab.lc.fixed_var.len(),
      _ => self.stack.push(ProofStep::TakeAsVar {
        range: v.0 as usize..elab.lc.fixed_var.len(),
        istart: elab.lc.infer_const.get_mut().len() as u32,
      }),
    }
  }

  fn thus(&mut self, _: &mut Analyzer, mut f: Vec<Formula>) {
    if let Some(ProofStep::Thus { conjs }) = self.stack.last_mut() {
      conjs.append(&mut f)
    } else {
      self.stack.push(ProofStep::Thus { conjs: f })
    }
  }

  fn unfold(&mut self, _: &mut Analyzer, _: &[ast::Reference]) {
    panic!("unfolding steps are not reconstructable")
  }

  fn new_cases(&mut self, _: &mut Analyzer) { self.stack.push(ProofStep::Break(false)) }
  fn new_case(&mut self, _: &mut Analyzer, _: &mut (), conjs: &[Formula]) {
    self.stack.push(ProofStep::Break(true));
    self.stack.push(ProofStep::Thus { conjs: conjs.to_vec() })
  }

  fn end_case(&mut self, elab: &mut Analyzer, _: &mut Self::CaseIter, f: Formula) {
    self.assume(elab, f.mk_neg().into_conjuncts());
  }

  fn end_cases(&mut self, elab: &mut Analyzer, _: ()) {
    let f = self.reconstruct(elab, false);
    self.thus(elab, f.mk_neg().into_conjuncts())
  }

  fn new_supposes(&mut self, _: &mut Analyzer) -> Self::SupposeRecv { None }

  fn new_suppose(&mut self, _: &mut Analyzer, _: &mut Self::SupposeRecv, _: &[Formula]) {
    self.stack.push(ProofStep::Break(true))
  }

  fn end_suppose(&mut self, elab: &mut Analyzer, recv: &mut Self::SupposeRecv, f: Formula) {
    if let Some(thesis) = recv {
      assert!(elab.eq(&**thesis, &f))
    } else {
      *recv = Some(Box::new(f))
    }
  }

  fn end_supposes(&mut self, elab: &mut Analyzer, recv: Self::SupposeRecv) {
    self.thus(elab, recv.unwrap().into_conjuncts())
  }

  fn end_block(&mut self, elab: &mut Analyzer, _: Position) -> Formula {
    self.reconstruct(elab, true)
  }
}

struct ToLocus<'a> {
  infer_const: &'a IdxVec<InferId, Assignment>,
  to_locus: &'a IdxVec<ConstId, Option<LocusId>>,
  it: LocusId,
}

impl ToLocus<'_> {
  fn get(&self, c: ConstId) -> LocusId {
    self.to_locus.get(c).and_then(|l| *l).expect("local constant in exported item")
  }
}

impl VisitMut for ToLocus<'_> {
  fn visit_term(&mut self, tm: &mut Term) {
    loop {
      match *tm {
        Term::Const(c) => *tm = Term::Locus(self.get(c)),
        Term::Infer(n) => {
          *tm = self.infer_const[n].def.clone();
          continue
        }
        Term::It => *tm = Term::Locus(self.it),
        _ => self.super_visit_term(tm),
      }
      break
    }
  }
}

struct MakeSelector<'a> {
  base: u8,
  fixed_vars: u32,
  to_const: &'a IdxVec<LocusId, ConstId>,
  terms: Vec<Result<Box<Term>, SelId>>,
}

impl VisitMut for MakeSelector<'_> {
  fn visit_term(&mut self, tm: &mut Term) {
    if let Term::Const(c) = tm {
      if let Some(i) = c.0.checked_sub(self.fixed_vars) {
        *tm = match self.terms[i as usize] {
          Ok(ref t) => (**t).clone(),
          Err(nr) => Term::Selector {
            nr,
            args: (0..=self.base).map(|i| Term::Const(self.to_const[LocusId(i)])).collect(),
          },
        }
      }
    } else {
      self.super_visit_term(tm)
    }
  }
}

struct PendingDef {
  kind: ConstrKind,
  df: Box<Definiens>,
  label: Option<(Option<LabelId>, Rc<str>)>,
  thm: Box<Formula>,
}

enum ReconstructAssum {
  Let { start: LocusId },
  Assum(Vec<Formula>),
}

struct BlockReader {
  kind: BlockKind,
  to_locus: IdxVec<ConstId, Option<LocusId>>,
  to_const: IdxVec<LocusId, ConstId>,
  primary: IdxVec<LocusId, Type>,
  assums: Vec<ReconstructAssum>,
  defs: Vec<(Position, Option<PendingDef>)>,
  needs_round_up: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct CheckAccess(u64);
impl CheckAccess {
  const fn new(n: usize) -> Self {
    assert!(n < u64::BITS as usize);
    Self(0)
  }

  const fn get(&self, n: LocusId) -> bool { self.0 & (1 << n.0 as u64) != 0 }
  fn set(&mut self, n: LocusId) { self.0 |= 1 << n.0 as u64 }

  fn with(primary: &[Type], f: impl FnOnce(&mut Self)) {
    let mut occ = Self::new(primary.len());
    f(&mut occ);
    for (i, ty) in primary.iter().enumerate().rev() {
      assert!(occ.get(LocusId::from_usize(i)));
      occ.visit_type(ty)
    }
  }
}
impl<F> Pattern<F> {
  fn check_access(&self) {
    CheckAccess::with(&self.primary, |occ| self.visible.iter().for_each(|&v| occ.set(v)))
  }
}

impl Visit for CheckAccess {
  fn visit_term(&mut self, tm: &Term) {
    match *tm {
      Term::Locus(i) => self.set(i),
      _ => self.super_visit_term(tm),
    }
  }
}

#[derive(Debug)]
struct PatternFuncResult {
  nr: FuncId,
  args: Box<[Term]>,
  var_set: CheckAccess,
}

impl BlockReader {
  fn new(kind: BlockKind, lc: &LocalContext) -> Self {
    Self {
      kind,
      to_locus: IdxVec::from_default(lc.fixed_var.len()),
      to_const: IdxVec::default(),
      primary: Default::default(),
      assums: vec![],
      defs: vec![],
      needs_round_up: false,
    }
  }

  fn to_locus<R>(&self, elab: &Analyzer, f: impl FnOnce(&mut ToLocus<'_>) -> R) -> R {
    f(&mut ToLocus {
      infer_const: &elab.lc.infer_const.borrow(),
      to_locus: &self.to_locus,
      it: LocusId(self.primary.len() as _),
    })
  }

  fn locus(&self, c: ConstId) -> LocusId {
    self.to_locus.get(c).and_then(|l| *l).expect("local constant in exported item")
  }

  fn after_scope(self, elab: &mut Analyzer) {
    elab.notations.iter_mut().for_each(|nota| nota.1.up());
    for (pos, def) in self.defs {
      if let Some(PendingDef { kind, df, label, thm }) = def {
        let id = elab.r.definitions.peek();
        elab.r.read_definiens(&df);
        if elab.g.cfg.analyzer_full {
          let thm2 = (*thm).visit_cloned(&mut elab.intern_const());
          if let Some((Some(label), _)) = label {
            elab.local_def_map.insert(label, id);
          }
          elab.push_prop(label, thm2);
        }
        if elab.g.cfg.exporter_enabled {
          elab.export.theorems.push(Theorem { pos, kind: TheoremKind::Def(kind), stmt: *thm })
        }
      } else if elab.g.cfg.exporter_enabled {
        elab.export.theorems.push(Theorem {
          pos,
          kind: TheoremKind::CanceledDef,
          stmt: Formula::True,
        })
      }
    }
  }

  fn has_assums(&self) -> bool {
    self.assums.iter().any(|f| matches!(f, ReconstructAssum::Assum(_)))
  }

  fn assums(&self) -> Formula {
    Formula::mk_and_with(|conjs| {
      for a in &self.assums {
        if let ReconstructAssum::Assum(f) = a {
          conjs.extend_from_slice(f)
        }
      }
    })
  }

  fn forall_locus(&self, elab: &Analyzer, mut f: Box<Formula>) -> Box<Formula> {
    self.to_locus(elab, |l| {
      let mut al = AbstractLocus(self.primary.len() as u32);
      for assum in self.assums.iter().rev() {
        match assum {
          ReconstructAssum::Let { start } =>
            while al.0 as u8 > start.0 {
              al.0 -= 1;
              let mut ty = elab.lc.fixed_var[self.to_const[LocusId(al.0 as u8)]].ty.visit_cloned(l);
              ty.visit(&mut al);
              f = Box::new(Formula::ForAll { dom: Box::new(ty), scope: f })
            },
          ReconstructAssum::Assum(assums) => {
            let f2 = f.mk_neg();
            *f = Formula::mk_and_with(|conjs| {
              conjs.extend(assums.iter().map(|f| f.visit_cloned(&mut al)));
              f2.append_conjuncts_to(conjs);
            })
            .mk_neg()
          }
        }
      }
      f
    })
  }

  fn check_compatible_args(&self, lc: &LocalContext, subst: &Subst<'_>) {
    let n = self.primary.len().checked_sub(subst.subst_term.len()).expect("too many args");
    let ic = lc.infer_const.borrow();
    'next: for ((i, _), tm) in self.primary.enum_iter().skip(n).zip(&*subst.subst_term) {
      if let Some(mut tm) = tm.as_deref() {
        loop {
          match *tm {
            Term::Const(c) if self.locus(c) == i => continue 'next,
            Term::Infer(n) => tm = &ic[n].def,
            _ => break,
          }
        }
      }
      panic!("incorrect args to redefinition")
    }
  }

  fn elab_func_def(
    &mut self, elab: &mut Analyzer, loc: Position, pat: &mut ast::PatternFunc,
    it: &mut ast::DefinitionBody, spec: Option<&ast::Type>, def: &mut Option<Box<ast::Definiens>>,
  ) {
    let fmt = elab.formats[&Format::Func(pat.to_format())];
    let mut cc = CorrConds::new();
    let primary: Box<[_]> = self.primary.0.iter().cloned().collect();
    let args = pat.args_mut();
    let visible: Box<[_]> = args.iter_mut().map(|v| self.locus(elab.resolve_const(v))).collect();
    let mut properties = PropertiesBuilder::new(&visible);
    let (redefines, superfluous, it_type);
    if it.redef {
      let args: Box<[_]> = args.iter().map(|v| Term::Const(v.var())).collect();
      let pat = elab.notations[PKC::Func].iter().rev().find(|pat| {
        pat.fmt == fmt
          && !matches!(pat.kind, PatternKind::Func(nr)
            if elab.g.constrs.functor[nr].redefines.is_some())
          && matches!(pat.check_types(&elab.g, &elab.lc, &args),
            Some(subst) if { self.check_compatible_args(&elab.lc, &subst); true })
      });
      let pat = pat.expect("type error");
      let PatternKind::Func(nr) = pat.kind else { unreachable!() };
      let c = &elab.g.constrs.functor[nr];
      (redefines, superfluous) = (Some(nr), (self.primary.len() - c.primary.len()) as u8);
      properties.props = c.properties.offset(superfluous);
      let args2: Box<[_]> =
        self.to_const.0[superfluous as usize..].iter().map(|c| Term::Const(*c)).collect();
      let mut inst = Inst::new(&elab.g.constrs, &elab.lc, &args2, 0);
      it_type = elab.elab_spec(spec, &c.ty.visit_cloned(&mut inst));
      if spec.is_some() {
        cc.0[CorrCondKind::Coherence] = Some(mk_func_coherence(nr, args2, &it_type));
      }
    } else {
      (redefines, superfluous) = (None, 0);
      it_type = spec.map_or(Type::ANY, |ty| elab.elab_type(ty));
    }
    elab.lc.it_type = Some(Box::new(it_type));
    let value = def.as_mut().map(|def| elab.elab_def_value(&mut def.kind, true));
    let mut it_type = elab.lc.it_type.take().unwrap();
    if let Some(value) = &value {
      cc.0[CorrCondKind::Consistency] = value.mk_consistency(&elab.g, Some(&it_type));
      if let Some(nr) = redefines {
        let args =
          self.to_const.0[superfluous as usize..].iter().map(|c| Term::Const(*c)).collect();
        let defined = Term::Functor { nr, args }.it_eq(&elab.g);
        cc.0[CorrCondKind::Compatibility] =
          Some(value.mk_compatibility(&elab.g, Some(&it_type), &defined));
      }
      properties.formula = Some(value.as_formula(&elab.g))
    }
    if !it.redef {
      match value.as_ref().unwrap() {
        DefValue::Term(value) => cc.0[CorrCondKind::Coherence] = Some(value.mk_coherence(&it_type)),
        DefValue::Formula(value) => {
          cc.0[CorrCondKind::Existence] = Some(value.mk_existence(&it_type));
          cc.0[CorrCondKind::Uniqueness] = Some(value.mk_uniqueness(&elab.g, &it_type));
        }
      }
    }
    elab.elab_corr_conds(cc, &mut it.conds, &mut it.corr);
    if !self.has_assums() {
      properties.load_args(&elab.g, &elab.lc, &self.to_const, |args| {
        PropertyDeclKind::Func(
          redefines
            .filter(|_| value.is_none())
            .map(|t| (t, &self.to_const.0[superfluous as usize..])),
          args,
        )
      });
    }
    elab.lc.it_type = Some(it_type);
    properties.elab_properties(elab, &mut it.props);
    it_type = elab.lc.it_type.take().unwrap();
    self.to_locus(elab, |l| it_type.visit(l));
    let n;
    if it.redef && spec.is_none() && superfluous == 0 && it.props.is_empty() {
      n = redefines.unwrap()
    } else {
      let primary = primary.clone();
      let mut c = TyConstructor {
        c: Constructor { primary, redefines, superfluous, properties: properties.props },
        ty: (*it_type).clone(),
      };
      c.visit(&mut elab.intern_const());
      n = elab.g.constrs.functor.push(c);
      elab.push_constr(ConstrKind::Func(n));
    }
    if let Some(mut value) = value {
      self.to_locus(elab, |l| value.visit(l));
      let formals: Box<[_]> = self.primary.enum_iter().map(|(i, _)| Term::Locus(i)).collect();
      let mut it_type2 = (*it_type).clone();
      it_type2.adjust_mut(&elab.g.constrs);
      let primary: Box<[_]> = self.primary.0.iter().cloned().chain([it_type2]).collect();
      it_type.visit(&mut Inst::new(&elab.g.constrs, &elab.lc, &formals, 0));
      let defined = Term::Functor { nr: n, args: formals };
      let depth = self.primary.len() as u32;
      let mut f;
      match &value {
        DefValue::Term(value) => {
          f = value.defthm_for(&elab.g, &defined);
          AbstractLocus(depth).visit_formula(&mut f);
        }
        DefValue::Formula(value) => {
          let itvar = LocusId(depth as _);
          f = value.iffthm_for(&elab.g, &elab.g.reqs.mk_eq(Term::Locus(itvar), defined));
          AbstractLocus(depth + 1).visit_formula(&mut f);
          AbstractLocus(depth).visit_type(&mut it_type);
          f = Box::new(Formula::ForAll { dom: it_type, scope: f });
        }
      };
      let thm = self.forall_locus(elab, f);
      let df = Box::new(Definiens {
        essential: (superfluous..primary.len() as u8).map(LocusId).collect(),
        c: ConstrDef {
          article: elab.article,
          def_nr: elab.defthms.fresh(),
          constr: ConstrKind::Func(redefines.unwrap_or(n)),
          primary,
        },
        assumptions: self.assums(),
        value,
      });
      let label = def.as_ref().unwrap().label.as_ref().map(|l| l.id.clone());
      self.defs.push((loc, Some(PendingDef { kind: ConstrKind::Func(n), df, label, thm })));
    }
    elab.push_pattern(true, PKC::Func, PatternKind::Func(n), fmt, primary, visible, true)
  }

  fn elab_pred_def(
    &mut self, elab: &mut Analyzer, loc: Position, pat: &mut ast::PatternPred,
    it: &mut ast::DefinitionBody, def: &mut Option<Box<ast::Definiens>>,
  ) {
    let fmt = elab.formats[&Format::Pred(pat.to_format())];
    let mut cc = CorrConds::new();
    let primary: Box<[_]> = self.primary.0.iter().cloned().collect();
    let visible: Box<[_]> =
      pat.args.iter_mut().map(|v| self.locus(elab.resolve_const(v))).collect();
    let mut properties = PropertiesBuilder::new(&visible);
    let (redefines, superfluous, pos);
    if it.redef {
      let args: Box<[_]> = pat.args.iter().map(|v| Term::Const(v.var())).collect();
      let pat = elab.notations[PKC::Pred].iter().rev().find(|pat| {
        pat.fmt == fmt
          && !matches!(pat.kind, PatternKind::Pred(nr)
            if elab.g.constrs.predicate[nr].redefines.is_some())
          && matches!(pat.check_types(&elab.g, &elab.lc, &args),
            Some(subst) if { self.check_compatible_args(&elab.lc, &subst); true })
      });
      let pat = pat.expect("type error");
      let PatternKind::Pred(nr) = pat.kind else { unreachable!() };
      let c = &elab.g.constrs.predicate[nr];
      (redefines, superfluous, pos) =
        (Some(nr), (self.primary.len() - c.primary.len()) as u8, pat.pos);
      properties.props = c.properties.offset(superfluous)
    } else {
      (redefines, superfluous, pos) = (None, 0, true)
    }
    let value = def.as_deref_mut().map(|def| elab.elab_def_value(&mut def.kind, pos));
    if let Some(value) = &value {
      cc.0[CorrCondKind::Consistency] = value.mk_consistency(&elab.g, None);
      if let Some(nr) = redefines {
        let args =
          self.to_const.0[superfluous as usize..].iter().map(|c| Term::Const(*c)).collect();
        cc.0[CorrCondKind::Compatibility] =
          Some(value.mk_compatibility(&elab.g, None, &Formula::Pred { nr, args }));
      }
      properties.formula = Some(value.as_formula(&elab.g))
    } else if let Some(nr) = redefines {
      let args = self.to_const.0[superfluous as usize..].iter().map(|c| Term::Const(*c)).collect();
      properties.formula = Some(Box::new(Formula::Pred { nr, args }))
    }
    elab.elab_corr_conds(cc, &mut it.conds, &mut it.corr);
    if !self.has_assums() {
      properties
        .load_args(&elab.g, &elab.lc, &self.to_const, |args| PropertyDeclKind::Pred(args, pos));
    }
    properties.elab_properties(elab, &mut it.props);
    let n;
    if it.redef && superfluous == 0 && it.props.is_empty() {
      n = redefines.unwrap()
    } else {
      let p = primary.clone();
      let mut c = Constructor { primary: p, redefines, superfluous, properties: properties.props };
      c.visit(&mut elab.intern_const());
      n = elab.g.constrs.predicate.push(c);
      elab.push_constr(ConstrKind::Pred(n));
    }
    if let Some(value) = value {
      let DefValue::Formula(mut value) = value else { panic!("expected formula") };
      self.to_locus(elab, |l| value.visit(l));
      let formals = self.primary.enum_iter().map(|(i, _)| Term::Locus(i)).collect();
      let mut f = value.defthm_for(&elab.g, &Formula::Pred { nr: n, args: formals });
      AbstractLocus(self.primary.len() as u32).visit_formula(&mut f);
      let thm = self.forall_locus(elab, f);
      let df = Box::new(Definiens {
        essential: (superfluous..primary.len() as u8).map(LocusId).collect(),
        c: ConstrDef {
          article: elab.article,
          def_nr: elab.defthms.fresh(),
          constr: ConstrKind::Pred(redefines.unwrap_or(n)),
          primary: primary.clone(),
        },
        assumptions: self.assums(),
        value: DefValue::Formula(value),
      });
      let label = def.as_deref_mut().unwrap().label.as_ref().map(|l| l.id.clone());
      self.defs.push((loc, Some(PendingDef { kind: ConstrKind::Pred(n), df, label, thm })));
    }
    elab.push_pattern(true, PKC::Pred, PatternKind::Pred(n), fmt, primary, visible, pos)
  }

  fn elab_mode_def(
    &mut self, elab: &mut Analyzer, loc: Position, pat: &mut ast::PatternMode,
    kind: &mut ast::DefModeKind, it: &mut ast::DefinitionBody,
  ) {
    let fmt = elab.formats[&Format::Mode(pat.to_format())];
    let mut cc = CorrConds::new();
    let primary: Box<[_]> = self.primary.0.iter().cloned().collect();
    let visible: Box<[_]> =
      pat.args.iter_mut().map(|v| self.locus(elab.resolve_const(v))).collect();
    let mut properties = PropertiesBuilder::new(&visible);
    match kind {
      ast::DefModeKind::Expandable { expansion } => {
        let mut expansion = Box::new(elab.elab_type(expansion));
        elab.elab_corr_conds(cc, &mut it.conds, &mut it.corr);
        properties.elab_properties(elab, &mut it.props);
        self.to_locus(elab, |l| expansion.visit(l));
        let kind = PatternKind::ExpandableMode { expansion };
        elab.push_pattern(true, PKC::Mode, kind, fmt, primary, visible, true)
      }
      ast::DefModeKind::Standard { spec, def } => {
        let (redefines, superfluous, it_type);
        if it.redef {
          let args: Box<[_]> = pat.args.iter().map(|v| Term::Const(v.var())).collect();
          let pat = elab.notations[PKC::Mode].iter().rev().find(|pat| {
            pat.fmt == fmt
              && !matches!(pat.kind, PatternKind::Mode(nr)
              if elab.g.constrs.mode[nr].redefines.is_some())
              && matches!(pat.check_types(&elab.g, &elab.lc, &args),
              Some(subst) if { self.check_compatible_args(&elab.lc, &subst); true })
          });
          let pat = pat.expect("type error");
          let PatternKind::Mode(nr) = pat.kind else { panic!("redefining expandable mode") };
          let c = &elab.g.constrs.mode[nr];
          (redefines, superfluous) = (Some(nr), (self.primary.len() - c.primary.len()) as u8);
          if c.properties.get(PropertyKind::Sethood) {
            properties.props.set(PropertyKind::Sethood)
          }
          let args2: Vec<_> =
            self.to_const.0[superfluous as usize..].iter().map(|c| Term::Const(*c)).collect();
          let tgt = c.ty.visit_cloned(&mut Inst::new(&elab.g.constrs, &elab.lc, &args2, 0));
          it_type = elab.elab_spec(spec.as_deref(), &tgt);
          if spec.is_some() {
            cc.0[CorrCondKind::Coherence] =
              Some(mk_mode_coherence(&elab.g.constrs, &elab.lc, nr, &tgt.attrs.1, args2, &it_type));
          }
        } else {
          (redefines, superfluous) = Default::default();
          it_type = spec.as_deref().map_or(Type::ANY, |ty| elab.elab_type(ty));
        }
        elab.lc.it_type = Some(Box::new(it_type));
        let value = def.as_deref_mut().map(|def| elab.elab_def_value(&mut def.kind, true));
        let mut it_type = elab.lc.it_type.take().unwrap();
        if let Some(value) = &value {
          cc.0[CorrCondKind::Consistency] = value.mk_consistency(&elab.g, Some(&it_type));
          if let Some(nr) = redefines {
            let args =
              self.to_const.0[superfluous as usize..].iter().map(|c| Term::Const(*c)).collect();
            let ty = Box::new(Type {
              kind: TypeKind::Mode(nr),
              attrs: (Attrs::EMPTY, it_type.attrs.1.clone()),
              args,
            });
            let defined = Formula::Is { term: Box::new(Term::It), ty };
            cc.0[CorrCondKind::Compatibility] =
              Some(value.mk_compatibility(&elab.g, Some(&it_type), &defined));
          }
        }
        if !it.redef {
          let Some(DefValue::Formula(value)) = &value else { panic!("expected formula") };
          cc.0[CorrCondKind::Existence] = Some(value.mk_existence(&it_type));
          properties.kind = PropertyDeclKind::Mode(value);
        }
        if let TypeKind::Mode(nr) = it_type.kind {
          if elab.g.constrs.mode[nr].properties.get(PropertyKind::Sethood) {
            properties.props.set(PropertyKind::Sethood)
          }
        }
        elab.elab_corr_conds(cc, &mut it.conds, &mut it.corr);
        elab.lc.it_type = Some(it_type);
        properties.elab_properties(elab, &mut it.props);
        it_type = elab.lc.it_type.take().unwrap();
        self.to_locus(elab, |l| it_type.visit(l));
        let n;
        if it.redef && spec.is_none() && superfluous == 0 {
          n = redefines.unwrap()
        } else {
          let primary = primary.clone();
          let mut c = TyConstructor {
            c: Constructor { primary, redefines, superfluous, properties: properties.props },
            ty: (*it_type).clone(),
          };
          c.visit(&mut elab.intern_const());
          n = elab.g.constrs.mode.push(c);
          elab.push_constr(ConstrKind::Mode(n));
        }
        if let Some(value) = value {
          let DefValue::Formula(mut value) = value else { panic!("expected formula") };
          self.to_locus(elab, |l| value.visit(l));
          let formals = self.primary.enum_iter().map(|(i, _)| Term::Locus(i)).collect_vec();
          let mut it_type2 = (*it_type).clone();
          it_type2.adjust_mut(&elab.g.constrs);
          let primary: Box<[_]> = self.primary.0.iter().cloned().chain([it_type2]).collect();
          it_type.visit(&mut Inst::new(&elab.g.constrs, &elab.lc, &formals, 0));
          let ty = Box::new(Type {
            kind: TypeKind::Mode(n),
            attrs: (Attrs::EMPTY, it_type.attrs.1.clone()),
            args: formals,
          });
          let depth = self.primary.len() as u32;
          let defined = Formula::Is { term: Box::new(Term::Locus(LocusId(depth as _))), ty };
          let mut f = value.defthm_for(&elab.g, &defined);
          AbstractLocus(depth + 1).visit_formula(&mut f);
          AbstractLocus(depth).visit_type(&mut it_type);
          let thm = self.forall_locus(elab, Box::new(Formula::ForAll { dom: it_type, scope: f }));
          let df = Box::new(Definiens {
            essential: (superfluous..primary.len() as u8).map(LocusId).collect(),
            c: ConstrDef {
              article: elab.article,
              def_nr: elab.defthms.fresh(),
              constr: ConstrKind::Mode(redefines.unwrap_or(n)),
              primary,
            },
            assumptions: self.assums(),
            value: DefValue::Formula(value),
          });
          let label = def.as_ref().unwrap().label.as_ref().map(|l| l.id.clone());
          self.defs.push((loc, Some(PendingDef { kind: ConstrKind::Mode(n), df, label, thm })));
        }
        elab.push_pattern(true, PKC::Mode, PatternKind::Mode(n), fmt, primary, visible, true)
      }
    }
  }

  fn elab_attr_def(
    &mut self, elab: &mut Analyzer, loc: Position, pat: &mut ast::PatternAttr,
    it: &mut ast::DefinitionBody, mut def: Option<&mut ast::Definiens>,
  ) {
    let fmt = elab.formats[&Format::Attr(pat.to_format())];
    let mut cc = CorrConds::new();
    let primary: Box<[_]> = self.primary.0.iter().cloned().collect();
    let visible: Box<[_]> =
      pat.args.iter_mut().map(|v| self.locus(elab.resolve_const(v))).collect();
    let (redefines, superfluous, pos) = if it.redef {
      let args: Box<[_]> = pat.args.iter().map(|v| Term::Const(v.var())).collect();
      let pat = elab.notations[PKC::Attr].iter().rev().find(|pat| {
        pat.fmt == fmt
          && !matches!(pat.kind, PatternKind::Attr(nr)
              if elab.g.constrs.attribute[nr].redefines.is_some())
          && matches!(pat.check_types(&elab.g, &elab.lc, &args),
            Some(subst) if { self.check_compatible_args(&elab.lc, &subst); true })
      });
      let pat = pat.expect("type error");
      let PatternKind::Attr(nr) = pat.kind else { unreachable!() };
      let c = &elab.g.constrs.attribute[nr];
      (Some(nr), (self.primary.len() - c.primary.len()) as u8, pat.pos)
    } else {
      (None, 0, true)
    };
    let value = def.as_mut().map(|def| elab.elab_def_value(&mut def.kind, pos));
    if let Some(value) = &value {
      cc.0[CorrCondKind::Consistency] = value.mk_consistency(&elab.g, None);
      if let Some(nr) = redefines {
        let args =
          self.to_const.0[superfluous as usize..].iter().map(|c| Term::Const(*c)).collect();
        cc.0[CorrCondKind::Compatibility] =
          Some(value.mk_compatibility(&elab.g, None, &Formula::Attr { nr, args }));
      }
    }
    elab.elab_corr_conds(cc, &mut it.conds, &mut it.corr);
    let mut properties = PropertiesBuilder::new(&visible);
    properties.elab_properties(elab, &mut it.props);
    let n;
    if it.redef && superfluous == 0 && it.props.is_empty() {
      n = redefines.unwrap()
    } else {
      let p = primary.clone();
      let mut c = TyConstructor {
        c: Constructor { primary: p, redefines, superfluous, properties: properties.props },
        ty: self.primary.0.last().unwrap().clone(),
      };
      c.visit(&mut elab.intern_const());
      n = elab.g.constrs.attribute.push(c);
      elab.push_constr(ConstrKind::Attr(n));
    }
    if let Some(value) = value {
      let DefValue::Formula(mut value) = value else { panic!("expected formula") };
      self.to_locus(elab, |l| value.visit(l));
      let formals = (superfluous..self.primary.len() as u8).map(LocusId).map(Term::Locus).collect();
      let mut f =
        value.defthm_for(&elab.g, &Formula::Attr { nr: redefines.unwrap_or(n), args: formals });
      AbstractLocus(self.primary.len() as u32).visit_formula(&mut f);
      let thm = self.forall_locus(elab, f);
      let df = Box::new(Definiens {
        essential: (superfluous..primary.len() as u8).map(LocusId).collect(),
        c: ConstrDef {
          article: elab.article,
          def_nr: elab.defthms.fresh(),
          constr: ConstrKind::Attr(redefines.unwrap_or(n)),
          primary: primary.clone(),
        },
        assumptions: self.assums(),
        value: DefValue::Formula(value),
      });
      let label = def.as_ref().unwrap().label.as_ref().map(|l| l.id.clone());
      self.defs.push((loc, Some(PendingDef { kind: ConstrKind::Attr(n), df, label, thm })));
    }
    elab.push_pattern(true, PKC::Attr, PatternKind::Attr(n), fmt, primary, visible, pos)
  }

  fn elab_struct_def(&mut self, elab: &mut Analyzer, it: &mut ast::DefStruct) {
    let mut parents: Box<[_]> = it.parents.iter().map(|ty| elab.elab_type(ty)).collect();
    let formals = self.primary.enum_iter().map(|(i, _)| Term::Locus(i)).collect_vec();
    let struct_primary: Box<[_]> = self.primary.0.iter().cloned().collect();
    let struct_id = elab.g.constrs.struct_mode.peek();
    let struct_pat = elab.mk_pattern(
      PKC::Struct,
      PatternKind::Struct(struct_id),
      elab.formats[&Format::Struct(it.pat.to_mode_format())],
      struct_primary.clone(),
      it.pat.args.iter_mut().map(|v| self.locus(elab.resolve_const(v))).collect(),
      true,
    );
    struct_pat.check_access();

    let struct_ty = Type {
      kind: TypeKind::Struct(struct_id),
      attrs: (Attrs::EMPTY, Attrs::EMPTY),
      args: formals.clone(),
    };
    let fixed_vars = elab.lc.fixed_var.len();
    let base = self.primary.len() as u8;
    self.to_locus.0.resize(fixed_vars, None);
    let mut cur_locus = LocusId(base);
    for group in &it.fields {
      let ty = elab.elab_type(&group.ty);
      for field in &group.vars {
        let c = elab.lc.fixed_var.push(FixedVar { ty: ty.clone(), def: None });
        self.to_locus.0.push(Some(cur_locus.fresh()));
        if elab.g.cfg.nameck_enabled {
          elab.internal_selectors.insert(field.sym.0, c);
        }
      }
    }
    elab.internal_selectors.clear();
    let field_tys = elab.lc.fixed_var.0.drain(fixed_vars..).map(|v| v.ty).collect_vec();
    let aggr_primary: Box<[_]> = self.to_locus(elab, |l| {
      let field_tys2 = field_tys.iter().map(|ty| {
        let mut ty = ty.clone();
        ty.adjust_mut(&elab.g.constrs);
        ty.visit(l);
        ty
      });
      self.primary.0.iter().cloned().chain(field_tys2).collect()
    });
    let aggr_id = elab.g.constrs.aggregate.peek();
    let aggr_pat = elab.mk_pattern(
      PKC::Aggr,
      PatternKind::Aggr(aggr_id),
      elab.formats[&Format::Aggr(it.pat.to_aggr_format(field_tys.len()))],
      aggr_primary.clone(),
      (base..cur_locus.0).map(LocusId).collect(),
      true,
    );
    aggr_pat.check_access();
    self.to_locus.0.truncate(fixed_vars);

    let mut prefixes = vec![];
    for ty in &*parents {
      assert!(ty.attrs.0.attrs().is_empty());
      let TypeKind::Struct(s) = ty.kind else { panic!("not a struct type") };
      prefixes.push(elab.g.constrs.struct_mode[s].fields.clone().into_vec());
    }

    let sel_primary_it = self.primary.0.iter().chain([&struct_ty]).cloned();
    elab.push_pattern(
      false,
      PKC::SubAggr,
      PatternKind::SubAggr(struct_id),
      elab.formats[&Format::SubAggr(it.pat.to_subaggr_format())],
      sel_primary_it.clone().collect(),
      Box::new([LocusId(base)]),
      true,
    );

    self.to_locus.push(Some(self.to_const.push(ConstId::from_usize(fixed_vars))));
    let mut mk_sel =
      MakeSelector { base, fixed_vars: fixed_vars as u32, to_const: &self.to_const, terms: vec![] };
    let mut fields = vec![];
    let mut field_fmt = vec![];
    let mut new_fields = vec![];
    for (v, mut ty) in it.fields.iter().flat_map(|group| &group.vars).zip(field_tys) {
      let fmt = elab.formats[&Format::Sel(v.sym.0)];
      assert!(!field_fmt.contains(&fmt), "duplicate field name");
      field_fmt.push(fmt);
      ty.visit(&mut mk_sel);
      let mut iter = parents.iter().zip(&mut prefixes).filter_map(|(parent, fields)| {
        let arg = Term::Const(elab.lc.fixed_var.push(FixedVar { ty: parent.clone(), def: None }));
        for pat in elab.notations[PKC::Sel].iter().rev() {
          if pat.fmt == fmt {
            if let Some(subst) = pat.check_types(&elab.g, &elab.lc, std::slice::from_ref(&arg)) {
              let PatternKind::Sel(nr) = pat.kind else { unreachable!() };
              let args = subst.trim_to(elab.g.constrs.selector[nr].primary.len());
              let mut inst = Inst::new(&elab.g.constrs, &elab.lc, &args, 0);
              let ty2 = elab.g.constrs.selector[nr].ty.visit_cloned(&mut inst);
              assert!(elab.eq(&ty, &ty2), "field type mismatch:\n  {ty:?} =?=\n  {ty2:?}");
              elab.lc.fixed_var.0.pop();
              fields.retain(|&x| x != nr);
              return Some((nr, args))
            }
          }
        }
        elab.lc.fixed_var.0.pop();
        None
      });
      if let Some((sel_id, args)) = iter.next() {
        assert!(iter.all(|(nr, _)| sel_id == nr), "overlapping parent fields");
        mk_sel.terms.push(Ok(Box::new(Term::Selector { nr: sel_id, args })));
        fields.push(sel_id);
      } else {
        self.to_locus(elab, |l| ty.visit(l));
        let mut sel = TyConstructor { c: Constructor::new(sel_primary_it.clone().collect()), ty };
        sel.visit(&mut elab.intern_const());
        let sel_id = elab.g.constrs.selector.push(sel);
        elab.push_pattern(
          true,
          PKC::Sel,
          PatternKind::Sel(sel_id),
          fmt,
          sel_primary_it.clone().collect(),
          Box::new([LocusId(base)]),
          true,
        );
        new_fields.push(sel_id);
        mk_sel.terms.push(Err(sel_id));
        fields.push(sel_id);
      }
    }
    self.to_locus.0.pop();
    self.to_const.0.pop();

    assert!(prefixes.iter().all(|prefix| prefix.is_empty()), "structure does not extend parent");
    self.to_locus(elab, |l| parents.visit(l));

    let mut c = TyConstructor {
      c: Constructor::new(sel_primary_it.clone().collect()),
      ty: struct_ty.clone(),
    };
    c.c.properties.set(PropertyKind::Abstractness);
    c.visit(&mut elab.intern_const());
    let attr_id = elab.g.constrs.attribute.push(c);
    let attr_primary = sel_primary_it.collect();
    elab.push_pattern(
      false,
      PKC::Attr,
      PatternKind::Attr(attr_id),
      FormatId::STRICT,
      attr_primary,
      Box::new([LocusId(base)]),
      true,
    );

    elab.push_constr(ConstrKind::Attr(attr_id));
    elab.push_constr(ConstrKind::Struct(struct_id));
    elab.push_constr(ConstrKind::Aggr(aggr_id));
    new_fields.into_iter().for_each(|sel_id| elab.push_constr(ConstrKind::Sel(sel_id)));

    let attrs = Attrs::Consistent(vec![Attr { nr: attr_id, pos: true, args: formals.into() }]);
    std::mem::swap(&mut self.primary, &mut elab.lc.locus_ty);
    elab.register_cluster(attrs.clone(), struct_primary.clone(), struct_ty.clone());
    std::mem::swap(&mut self.primary, &mut elab.lc.locus_ty);

    let mut sorted_fields: Box<[_]> = fields.clone().into();
    sorted_fields.sort_unstable();
    let mut c = StructMode {
      c: Constructor::new(struct_primary),
      parents,
      aggr: aggr_id,
      fields: sorted_fields,
    };
    c.visit(&mut elab.intern_const());
    let struct_id2 = elab.g.constrs.struct_mode.push(c);
    assert!(struct_id == struct_id2);
    elab.r.lc.formatter.push(&elab.r.g.constrs, &struct_pat);
    elab.r.notations[PKC::Struct].push_ext(struct_pat);

    let mut aggr_ty = struct_ty;
    aggr_ty.attrs = (attrs.clone(), attrs);
    let mut c = Aggregate {
      c: TyConstructor { c: Constructor::new(aggr_primary), ty: aggr_ty },
      base,
      fields: fields.into(),
    };
    c.visit(&mut elab.intern_const());
    let aggr_id2 = elab.g.constrs.aggregate.push(c);
    assert!(aggr_id == aggr_id2);
    elab.r.lc.formatter.push(&elab.r.g.constrs, &aggr_pat);
    elab.r.notations[PKC::Aggr].push_ext(aggr_pat);
  }

  fn elab_exist_reg(
    &mut self, elab: &mut Analyzer, concl: &[ast::Attr], ty: &ast::Type,
    conds: &mut [ast::CorrCond], corr: &mut Option<ast::Correctness>,
  ) {
    let mut ty = elab.elab_type(ty);
    let mut attrs = ty.attrs.0.clone();
    let f = Formula::mk_and_with(|conjs| {
      for attr in concl {
        let attr = elab.elab_attr(attr, true, &mut ty);
        let args = attr.args.iter().cloned().chain([Term::Bound(BoundId(0))]).collect();
        conjs.push(Formula::Attr { nr: attr.nr, args }.maybe_neg(attr.pos));
        attrs.insert(Some(&elab.g.constrs), &elab.lc, attr);
      }
    });
    let (kind, args) = match ty.kind {
      TypeKind::Mode(nr) => {
        let (n, args) = Type::adjust(nr, &ty.args, &elab.g.constrs);
        (TypeKind::Mode(n), args)
      }
      _ => (ty.kind, &*ty.args),
    };
    let mut ty2 = Type { kind, attrs: ty.attrs.clone(), args: args.to_vec() };
    let mut cc = CorrConds::new();
    cc.0[CorrCondKind::Existence] = Some(Box::new(Formula::forall(ty, f.mk_neg()).mk_neg()));
    elab.elab_corr_conds(cc, conds, corr);

    let primary: Box<[_]> = self.primary.0.iter().cloned().collect();
    self.to_locus(elab, |l| {
      attrs.visit(l);
      ty2.visit(l);
    });
    CheckAccess::with(&primary, |occ| {
      occ.visit_attrs(&attrs);
      occ.visit_terms(&ty2.args);
    });
    std::mem::swap(&mut self.primary, &mut elab.lc.locus_ty);
    elab.register_cluster(attrs, primary, ty2);
    std::mem::swap(&mut self.primary, &mut elab.lc.locus_ty);
  }

  fn elab_cond_reg(
    &mut self, elab: &mut Analyzer, antecedent: &[ast::Attr], concl: &[ast::Attr], ty: &ast::Type,
    conds: &mut [ast::CorrCond], corr: &mut Option<ast::Correctness>,
  ) {
    let mut ty = elab.elab_type(ty);
    let (kind, args) = match ty.kind {
      TypeKind::Mode(nr) => {
        let (n, args) = Type::adjust(nr, &ty.args, &elab.g.constrs);
        (TypeKind::Mode(n), args)
      }
      _ => (ty.kind, &*ty.args),
    };
    let mut ty2 = Type { kind, attrs: (Attrs::EMPTY, Attrs::EMPTY), args: args.to_vec() };

    let (mut attrs1, mut attrs2) = (ty.attrs.0.clone(), ty.attrs.0.clone());
    let f = Formula::mk_and_with(|conjs| {
      for attr in antecedent {
        let attr = elab.elab_attr(attr, true, &mut ty);
        let args = attr.args.iter().cloned().chain([Term::Bound(BoundId(0))]).collect();
        conjs.push(Formula::Attr { nr: attr.nr, args }.maybe_neg(attr.pos));
        attrs1.insert(Some(&elab.g.constrs), &elab.lc, attr);
      }
      let f = Formula::mk_and_with(|conjs| {
        for attr in concl {
          let attr = elab.elab_attr(attr, true, &mut ty);
          let args = attr.args.iter().cloned().chain([Term::Bound(BoundId(0))]).collect();
          conjs.push(Formula::Attr { nr: attr.nr, args }.maybe_neg(attr.pos));
          attrs2.insert(Some(&elab.g.constrs), &elab.lc, attr);
        }
      });
      f.mk_neg().append_conjuncts_to(conjs)
    });
    let mut cc = CorrConds::new();
    cc.0[CorrCondKind::Coherence] = Some(Box::new(Formula::forall(ty, f.mk_neg())));
    elab.elab_corr_conds(cc, conds, corr);

    let primary: Box<[_]> = self.primary.0.iter().cloned().collect();
    self.to_locus(elab, |l| {
      attrs1.visit(l);
      attrs2.visit(l);
      ty2.visit(l);
    });
    CheckAccess::with(&primary, |occ| {
      occ.visit_attrs(&attrs1);
      occ.visit_terms(&ty2.args);
    });
    let Attrs::Consistent(_) = attrs1 else { panic!("inconsistent cluster antecedent") };
    let Attrs::Consistent(_) = attrs2 else { panic!("inconsistent cluster consequent") };
    elab.read_conditional_cluster(ConditionalCluster {
      cl: Cluster { primary, consequent: (attrs2.clone(), attrs2) },
      ty: Box::new(ty2),
      antecedent: attrs1,
    });
    self.needs_round_up = true;
  }

  fn elab_func_reg(
    &mut self, elab: &mut Analyzer, term: &ast::Term, concl: &[ast::Attr], oty: Option<&ast::Type>,
    conds: &mut [ast::CorrCond], corr: &mut Option<ast::Correctness>,
  ) {
    let term = elab.elab_term(term);
    let mut term2 = match term {
      Term::Functor { nr, ref args } => {
        let (nr, args) = Term::adjust(nr, args, Some(&elab.g.constrs));
        Term::Functor { nr, args: args.to_vec().into() }
      }
      Term::Aggregate { .. } | Term::Selector { .. } => term.clone(),
      _ => panic!("invalid functor registration target"),
    };
    let mut ty = match oty {
      None => term2.get_type(&elab.g, &elab.lc, false),
      Some(ty) => elab.elab_type(ty),
    };
    let concl = concl.iter().map(|attr| elab.elab_attr(attr, true, &mut ty)).collect_vec();
    let mut cc = CorrConds::new();
    cc.0[CorrCondKind::Coherence] = Some(Box::new(if oty.is_some() {
      let f = Formula::mk_and_with(|conj| {
        conj.push(elab.g.reqs.mk_eq(Term::Bound(BoundId(0)), term));
        let f = Formula::mk_and_with(|conj| {
          for attr in &concl {
            let args = attr.args.iter().cloned().chain([Term::Bound(BoundId(0))]).collect();
            conj.push(Formula::Attr { nr: attr.nr, args }.maybe_neg(attr.pos))
          }
        });
        f.mk_neg().append_conjuncts_to(conj)
      });
      Formula::forall(ty.clone(), f.mk_neg())
    } else {
      Formula::mk_and_with(|conj| {
        for attr in &concl {
          let args = attr.args.iter().chain([&term]).cloned().collect();
          conj.push(Formula::Attr { nr: attr.nr, args }.maybe_neg(attr.pos))
        }
      })
    }));
    elab.elab_corr_conds(cc, conds, corr);

    let mut attrs = ty.attrs.0.clone();
    for attr in concl {
      attrs.insert(Some(&elab.g.constrs), &elab.lc, attr);
    }

    let primary: Box<[_]> = self.primary.0.iter().cloned().collect();
    let mut attrs1 = attrs.clone();
    attrs.enlarge_by(&elab.g.constrs, &elab.lc, &ty.attrs.1);
    attrs.round_up_with(&elab.g, &elab.lc, &ty, false);
    let Attrs::Consistent(_) = attrs else { panic!("inconsistent functor registration") };
    self.to_locus(elab, |l| {
      term2.visit(l);
      attrs1.visit(l);
      attrs.visit(l);
      ty.visit(l);
    });
    CheckAccess::with(&primary, |occ| occ.visit_term(&term2));
    elab.read_functor_cluster(FunctorCluster {
      cl: Cluster { primary, consequent: (attrs1, attrs) },
      ty: oty.map(|_| Box::new(ty)),
      term: Box::new(term2),
    });
    self.needs_round_up = true;
    std::mem::swap(&mut self.primary, &mut elab.lc.locus_ty);
    elab.r.g.round_up_term_cache(&mut elab.r.lc);
    std::mem::swap(&mut self.primary, &mut elab.lc.locus_ty);
  }

  fn elab_identify_pattern_func(
    &self, elab: &Analyzer, pat: &mut ast::PatternFunc,
  ) -> PatternFuncResult {
    let fmt = elab.formats[&Format::Func(pat.to_format())];
    let visible: Box<[_]> =
      pat.args_mut().iter_mut().map(|v| self.locus(elab.resolve_const(v))).collect();
    let args: Box<[_]> = pat.args().iter().map(|v| Term::Const(v.var())).collect();
    for pat in elab.notations[PKC::Func].iter().rev() {
      if pat.fmt == fmt {
        if let Some(subst) = pat.check_types(&elab.g, &elab.lc, &args) {
          let PatternKind::Func(nr) = pat.kind else { unreachable!() };
          let args = subst.trim_to(elab.g.constrs.functor[nr].primary.len());
          let mut var_set = CheckAccess::new(self.primary.len());
          for &i in &*visible {
            var_set.set(i);
            var_set.visit_type(&self.primary[i]);
          }
          return PatternFuncResult { nr, args, var_set }
        }
      }
    }
    panic!("type error")
  }

  fn elab_identify_func(&mut self, elab: &mut Analyzer, it: &mut ast::IdentifyFunc) {
    let lhs_pat = self.elab_identify_pattern_func(elab, &mut it.lhs);
    let rhs_pat = self.elab_identify_pattern_func(elab, &mut it.rhs);
    assert!(
      (lhs_pat.var_set.0 | rhs_pat.var_set.0) == (1 << self.primary.len() as u64) - 1,
      "Unused locus"
    );
    let mut eq_args = vec![];
    let mut occ = CheckAccess::new(self.primary.len() as _);
    occ.0 |= lhs_pat.var_set.0 & rhs_pat.var_set.0;
    for (v1, v2) in &mut it.eqs {
      let (c1, c2) = (elab.resolve_const(v1), elab.resolve_const(v2));
      let (v1, v2) = (self.locus(c1), self.locus(c2));
      let (c1, c2, v1) = match (
        lhs_pat.var_set.get(v1) as i8 - rhs_pat.var_set.get(v1) as i8,
        lhs_pat.var_set.get(v2) as i8 - rhs_pat.var_set.get(v2) as i8,
      ) {
        (1, -1) => (c1, c2, v1),
        (-1, 1) => (c2, c1, v2),
        (1, 1) | (-1, -1) => panic!("Cannot mix left and right pattern arguments"),
        _ => panic!("The argument(s) must belong to the left or right pattern"),
      };
      eq_args.push((c1, c2));
      occ.set(v1);
      assert!(elab.lc.fixed_var[c2].ty.is_wider_than(&elab.g, &elab.lc, &elab.lc.fixed_var[c1].ty));
    }
    eq_args.sort_unstable();

    for (i, ty) in self.primary.enum_iter() {
      if occ.get(i) {
        occ.visit_type(ty)
      }
    }
    assert!(
      occ == lhs_pat.var_set,
      "Left and right pattern must have the same number of arguments"
    );
    let mut cc = CorrConds::new();
    let (lhs_args, rhs_args) =
      self.to_locus(elab, |l| (lhs_pat.args.visit_cloned(l), rhs_pat.args.visit_cloned(l)));
    let f = Formula::mk_and_with(|conjs| {
      conjs.extend(
        eq_args.iter().map(|&(c1, c2)| elab.g.reqs.mk_eq(Term::Const(c1), Term::Const(c2))),
      );
      let f = elab.g.reqs.mk_eq(
        Term::Functor { nr: lhs_pat.nr, args: lhs_pat.args },
        Term::Functor { nr: rhs_pat.nr, args: rhs_pat.args },
      );
      conjs.push(f.mk_neg());
    });
    cc.0[CorrCondKind::Compatibility] = Some(Box::new(f.mk_neg()));
    elab.elab_corr_conds(cc, &mut it.conds, &mut it.corr);
    elab.r.push_identify(&IdentifyFunc {
      primary: self.primary.0.iter().cloned().collect(),
      lhs: Term::Functor { nr: lhs_pat.nr, args: lhs_args },
      rhs: Term::Functor { nr: rhs_pat.nr, args: rhs_args },
      eq_args: eq_args.into_iter().map(|(c1, c2)| (self.locus(c1), self.locus(c2))).collect(),
    });
  }

  fn elab_reduction(&mut self, elab: &mut Analyzer, it: &mut ast::Reduction) {
    fn is_ssubterm(ctx: &mut EqCtx<'_>, sup: &Term, sub: &Term) -> bool {
      use Term::*;
      match sup {
        Numeral(_) | Locus(_) | Bound(_) | Const(_) | Infer(_) => ().eq_term(ctx, sup, sub),
        PrivFunc { value, .. } => is_ssubterm(ctx, value, sub),
        Functor { args, .. }
        | SchFunc { args, .. }
        | Aggregate { args, .. }
        | Selector { args, .. } => subterm_list(ctx, args, sub),
        The { .. } | Fraenkel { .. } => false,
        EqClass(_) | EqMark(_) | Qua { .. } | FreeVar(_) | It => unreachable!(),
      }
    }
    fn subterm_list(ctx: &mut EqCtx<'_>, args: &[Term], sub: &Term) -> bool {
      args.iter().any(|t| ().eq_term(ctx, t, sub) || is_ssubterm(ctx, t, sub))
    }

    let mut from = elab.elab_term(&it.from);
    let mut to = elab.elab_term(&it.to);
    let reduction_allowed = {
      let Term::Functor { nr, ref args } = from else {
        panic!("reduction must have a functor term on the LHS")
      };
      let args = Term::adjust(nr, args, Some(&elab.g.constrs)).1;
      elab.with_eq(|ctx| subterm_list(ctx, args, to.skip_priv_func(Some(ctx.lc))))
    };
    assert!(reduction_allowed, "Right term must be a subterm of the left term");
    let mut cc = CorrConds::new();
    cc.0[CorrCondKind::Reducibility] = Some(Box::new(elab.g.reqs.mk_eq(from.clone(), to.clone())));
    elab.elab_corr_conds(cc, &mut it.conds, &mut it.corr);
    self.to_locus(elab, |l| {
      from.visit(l);
      to.visit(l)
    });
    let primary: Box<[_]> = self.primary.0.iter().cloned().collect();
    CheckAccess::with(&primary, |occ| occ.visit_term(&from));
    elab.r.reductions.push(Reduction { primary, terms: [from, to] });
  }

  fn elab_sethood_registration(
    &mut self, elab: &mut Analyzer, ty: &ast::Type, just: &mut ast::Justification,
  ) {
    let mut ty = elab.elab_type(ty);
    let primary: Box<[_]> = self.primary.0.iter().cloned().collect();
    if elab.g.cfg.analyzer_full {
      let mut property = Formula::mk_neg(Formula::forall(
        Type::SET,
        Formula::mk_neg(Formula::forall(
          ty.clone(),
          Formula::Pred {
            nr: elab.g.reqs.belongs_to().unwrap(),
            args: Box::new([Term::Bound(BoundId(1)), Term::Bound(BoundId(0))]),
          },
        )),
      ));
      property.visit(&mut elab.intern_const());
      elab.elab_justification(false, &property, just);
    }
    self.to_locus(elab, |l| ty.visit(l));
    CheckAccess::with(&primary, |occ| occ.visit_type(&ty));
    elab.properties.push(Property { primary, ty, kind: PropertyKind::Sethood })
  }

  fn elab_pred_notation(
    &mut self, elab: &mut Analyzer, new: &mut ast::PatternPred, orig: &mut ast::PatternPred,
    pos: bool,
  ) {
    let fmt_orig = elab.formats[&Format::Pred(orig.to_format())];
    let primary: Box<[_]> = self.primary.0.iter().cloned().collect();
    CheckAccess::with(&primary, |occ| {
      orig.args.iter_mut().for_each(|v| occ.set(self.locus(elab.resolve_const(v))))
    });
    let args: Box<[_]> = orig.args.iter().map(|v| Term::Const(v.var())).collect();
    let pat = elab.notations[PKC::Pred].iter().rev().find(|pat| {
      pat.fmt == fmt_orig
        && !matches!(pat.kind, PatternKind::Pred(nr)
            if elab.g.constrs.predicate[nr].redefines.is_some())
        && matches!(pat.check_types(&elab.g, &elab.lc, &args),
            Some(subst) if { self.check_compatible_args(&elab.lc, &subst); true })
    });
    let pat = pat.expect("type error");
    let PatternKind::Pred(nr) = pat.kind else { unreachable!() };
    let c = &elab.g.constrs.predicate[nr];
    elab.push_pattern(
      true,
      PKC::Pred,
      PatternKind::Pred(c.redefines.unwrap_or(nr)),
      elab.formats[&Format::Pred(new.to_format())],
      primary,
      new.args.iter_mut().map(|v| self.locus(elab.resolve_const(v))).collect(),
      pos == pat.pos,
    )
  }

  fn elab_func_notation(
    &mut self, elab: &mut Analyzer, new: &mut ast::PatternFunc, orig: &mut ast::PatternFunc,
  ) {
    let fmt_orig = elab.formats[&Format::Func(orig.to_format())];
    let primary: Box<[_]> = self.primary.0.iter().cloned().collect();
    CheckAccess::with(&primary, |occ| {
      orig.args_mut().iter_mut().for_each(|v| occ.set(self.locus(elab.resolve_const(v))))
    });
    let args: Box<[_]> = orig.args().iter().map(|v| Term::Const(v.var())).collect();
    let pat = elab.notations[PKC::Func].iter().rev().find(|pat| {
      pat.fmt == fmt_orig
        && !matches!(pat.kind, PatternKind::Func(nr)
            if elab.g.constrs.functor[nr].redefines.is_some())
        && matches!(pat.check_types(&elab.g, &elab.lc, &args),
            Some(subst) if { self.check_compatible_args(&elab.lc, &subst); true })
    });
    let pat = pat.expect("type error");
    let PatternKind::Func(nr) = pat.kind else { unreachable!() };
    let c = &elab.g.constrs.functor[nr];
    elab.push_pattern(
      true,
      PKC::Func,
      PatternKind::Func(c.redefines.unwrap_or(nr)),
      elab.formats[&Format::Func(new.to_format())],
      primary,
      new.args_mut().iter_mut().map(|v| self.locus(elab.resolve_const(v))).collect(),
      true,
    )
  }

  fn elab_mode_notation(
    &mut self, elab: &mut Analyzer, new: &mut ast::PatternMode, orig: &mut ast::PatternMode,
  ) {
    let fmt_orig = elab.formats[&Format::Mode(orig.to_format())];
    let primary: Box<[_]> = self.primary.0.iter().cloned().collect();
    CheckAccess::with(&primary, |occ| {
      orig.args.iter_mut().for_each(|v| occ.set(self.locus(elab.resolve_const(v))))
    });
    let args: Box<[_]> = orig.args.iter().map(|v| Term::Const(v.var())).collect();
    let pat = elab.notations[PKC::Mode].iter().rev().find(|pat| {
      pat.fmt == fmt_orig
        && !matches!(pat.kind, PatternKind::Mode(nr)
            if elab.g.constrs.mode[nr].redefines.is_some())
        && matches!(pat.check_types(&elab.g, &elab.lc, &args),
            Some(subst) if { self.check_compatible_args(&elab.lc, &subst); true })
    });
    let pat = pat.expect("type error");
    let PatternKind::Mode(nr) = pat.kind else { panic!("redefining expandable mode") };
    let c = &elab.g.constrs.mode[nr];
    elab.push_pattern(
      true,
      PKC::Mode,
      PatternKind::Mode(c.redefines.unwrap_or(nr)),
      elab.formats[&Format::Mode(new.to_format())],
      primary,
      new.args.iter_mut().map(|v| self.locus(elab.resolve_const(v))).collect(),
      true,
    )
  }

  fn elab_attr_notation(
    &mut self, elab: &mut Analyzer, new: &mut ast::PatternAttr, orig: &mut ast::PatternAttr,
    pos: bool,
  ) {
    let fmt_orig = elab.formats[&Format::Attr(orig.to_format())];
    let primary: Box<[_]> = self.primary.0.iter().cloned().collect();
    CheckAccess::with(&primary, |occ| {
      orig.args.iter_mut().for_each(|v| occ.set(self.locus(elab.resolve_const(v))))
    });
    let args: Box<[_]> = orig.args.iter().map(|v| Term::Const(v.var())).collect();
    let pat = elab.notations[PKC::Attr].iter().rev().find(|pat| {
      pat.fmt == fmt_orig
        && !matches!(pat.kind, PatternKind::Attr(nr)
            if elab.g.constrs.attribute[nr].redefines.is_some())
        && matches!(pat.check_types(&elab.g, &elab.lc, &args),
            Some(subst) if { self.check_compatible_args(&elab.lc, &subst); true })
    });
    let pat = pat.expect("type error");
    let PatternKind::Attr(nr) = pat.kind else { unreachable!() };
    elab.push_pattern(
      true,
      PKC::Attr,
      PatternKind::Attr(elab.g.constrs.attribute[nr].redefines.unwrap_or(nr)),
      elab.formats[&Format::Attr(new.to_format())],
      primary,
      new.args.iter_mut().map(|v| self.locus(elab.resolve_const(v))).collect(),
      pos == pat.pos,
    )
  }

  fn elab_canceled_def(&mut self, elab: &mut Analyzer, loc: Position, n: u32) {
    elab.defthms.0 += n;
    if elab.g.cfg.exporter_enabled {
      self.defs.extend((0..n).map(|_| (loc, None)))
    }
  }
}

impl ReadProof for BlockReader {
  type CaseIter = std::convert::Infallible;
  type SupposeRecv = std::convert::Infallible;
  type Output = ();

  fn intro(&mut self, elab: &mut Analyzer, start: usize, _: u32) {
    self.to_locus.0.resize(start, None);
    if !matches!(self.assums.last(), Some(ReconstructAssum::Let { .. })) {
      self.assums.push(ReconstructAssum::Let { start: self.primary.peek() })
    }
    for fv in &elab.r.lc.fixed_var.0[start..] {
      let mut ty = fv.ty.clone();
      ty.adjust_mut(&elab.g.constrs);
      self.to_locus(elab, |l| ty.visit(l));
      Exportable.visit_type(&ty);
      let i = self.primary.push(ty);
      self.to_const.0.push(self.to_locus.push(Some(i)));
    }
  }

  fn assume(&mut self, elab: &mut Analyzer, mut conjs: Vec<Formula>) {
    if !conjs.is_empty() {
      self.to_locus(elab, |l| conjs.visit(l));
      conjs.iter().for_each(|f| Exportable.visit_formula(f));
      if let Some(ReconstructAssum::Assum(assums)) = self.assums.last_mut() {
        assums.append(&mut conjs)
      } else {
        self.assums.push(ReconstructAssum::Assum(conjs))
      }
    }
  }

  fn given(&mut self, elab: &mut Analyzer, start: usize, istart: u32, f: Formula) {
    let mut f = f.mk_neg();
    let end = elab.lc.fixed_var.len();
    elab.lc.mk_forall(start..end, istart, false, &mut f);
    self.assume(elab, vec![f.mk_neg()]);
  }

  fn take(&mut self, _: &mut Analyzer, _: Term) { panic!("invalid item") }
  fn thus(&mut self, _: &mut Analyzer, _: Vec<Formula>) { panic!("invalid item") }
  fn unfold(&mut self, _: &mut Analyzer, _: &[ast::Reference]) { panic!("invalid item") }
  fn new_cases(&mut self, _: &mut Analyzer) -> Self::CaseIter { panic!("invalid item") }

  fn new_supposes(&mut self, _: &mut Analyzer) -> Self::SupposeRecv { panic!("invalid item") }

  fn end_block(&mut self, elab: &mut Analyzer, _: Position) {
    if self.needs_round_up {
      let mut attrs = elab.g.numeral_type.attrs.1.clone();
      attrs.round_up_with(&elab.g, &elab.lc, &elab.g.numeral_type, false);
      elab.g.numeral_type.attrs.1 = attrs;
      for i in 0..elab.g.clusters.registered.len() {
        let cl = &elab.r.g.clusters.registered[i];
        let mut attrs = cl.consequent.1.clone();
        elab.r.lc.with_locus_tys(&cl.primary, |lc| {
          attrs.round_up_with(&elab.r.g, lc, &cl.ty, false);
        });
        elab.r.g.clusters.registered[i].consequent.1 = attrs;
      }
    }
  }

  fn elab_item(&mut self, elab: &mut Analyzer, it: &mut ast::Item) -> bool {
    match &it.kind {
      ast::ItemKind::Definition { .. } => elab.item_header(it, "Definition"),
      ast::ItemKind::DefStruct { .. } => elab.item_header(it, "DefStruct"),
      ast::ItemKind::PatternRedef { .. } => elab.item_header(it, "PatternRedef"),
      ast::ItemKind::Cluster { .. } => elab.item_header(it, "Cluster"),
      ast::ItemKind::Reduction { .. } => elab.item_header(it, "Reduction"),
      ast::ItemKind::IdentifyFunc { .. } => elab.item_header(it, "IdentifyFunc"),
      ast::ItemKind::SethoodRegistration { .. } => elab.item_header(it, "SethoodRegistration"),
      _ => {}
    }
    match (self.kind, &mut it.kind) {
      (BlockKind::Definition, ast::ItemKind::Definition(df)) => match &mut df.kind {
        ast::DefinitionKind::Func { pat, spec, def } =>
          self.elab_func_def(elab, it.pos, pat, &mut df.body, spec.as_deref(), def),
        ast::DefinitionKind::Pred { pat, def } =>
          self.elab_pred_def(elab, it.pos, pat, &mut df.body, def),
        ast::DefinitionKind::Mode { pat, kind } =>
          self.elab_mode_def(elab, it.pos, pat, kind, &mut df.body),
        ast::DefinitionKind::Attr { pat, def } =>
          self.elab_attr_def(elab, it.pos, pat, &mut df.body, def.as_deref_mut()),
      },
      (BlockKind::Definition, ast::ItemKind::DefStruct(it)) => self.elab_struct_def(elab, it),
      (BlockKind::Notation, ast::ItemKind::PatternRedef(it)) => match it {
        ast::PatternRedef::Pred { new, orig, pos } =>
          self.elab_pred_notation(elab, new, orig, *pos),
        ast::PatternRedef::Func { new, orig } => self.elab_func_notation(elab, new, orig),
        ast::PatternRedef::Mode { new, orig } => self.elab_mode_notation(elab, new, orig),
        ast::PatternRedef::Attr { new, orig, pos } =>
          self.elab_attr_notation(elab, new, orig, *pos),
      },
      (BlockKind::Registration, ast::ItemKind::Cluster(it)) => match &mut it.kind {
        ast::ClusterDeclKind::Exist { concl, ty } =>
          self.elab_exist_reg(elab, concl, ty, &mut it.conds, &mut it.corr),
        ast::ClusterDeclKind::Func { term, concl, ty } =>
          self.elab_func_reg(elab, term, concl, ty.as_deref(), &mut it.conds, &mut it.corr),
        ast::ClusterDeclKind::Cond { antecedent, concl, ty } =>
          self.elab_cond_reg(elab, antecedent, concl, ty, &mut it.conds, &mut it.corr),
      },
      (BlockKind::Registration, ast::ItemKind::IdentifyFunc(it)) =>
        self.elab_identify_func(elab, it),
      (BlockKind::Registration, ast::ItemKind::Reduction(it)) => self.elab_reduction(elab, it),
      (BlockKind::Registration, ast::ItemKind::SethoodRegistration { ty, just }) =>
        self.elab_sethood_registration(elab, ty, just),
      (BlockKind::Definition, &mut ast::ItemKind::Pragma(Pragma::Canceled(CancelKind::Def, n))) =>
        self.elab_canceled_def(elab, it.pos, n),
      (
        BlockKind::Registration | BlockKind::Definition,
        ast::ItemKind::Pragma(Pragma::ThmDesc(_)),
      ) => {}
      _ => return self.super_elab_item(elab, it),
    }
    true
  }
}
