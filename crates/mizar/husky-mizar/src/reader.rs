use crate::accom::Accomodator;
use crate::checker::Checker;
use crate::error::MizError;
use crate::parser::MizParser;
use crate::types::*;
use crate::*;
use std::io;

enum PendingDef {
  Constr(ConstrKind),
  Cluster(ClusterKind, usize),
}

mk_id! {
  DefiniensId(u32),
}

pub struct Reader {
  pub g: Global,
  pub lc: LocalContext,
  pub libs: Libraries,
  pub article: Article,
  treat_thm_as_axiom: bool,
  pub no_suppress_checker: bool,
  pub accom: Option<Box<Accomodator>>,
  /// gFormatsColl
  #[allow(clippy::box_collection)]
  pub formats: Box<HashMap<Format, FormatId>>,
  pub has_errors: bool,
  pub formats_base: usize,
  /// Notat
  pub notations: EnumMap<PatternKindClass, ExtVec<Pattern>>,
  /// Definientia
  pub definitions: IdxVec<DefiniensId, Definiens>,
  /// EqDefinientia
  pub equalities: Vec<Definiens>,
  /// ExDefinientia
  pub expansions: Vec<Definiens>,
  /// gPropertiesList
  pub properties: Vec<Property>,
  /// gIdentifications
  pub identify: Vec<IdentifyFunc>,
  /// gReductions
  pub reductions: Vec<Reduction>,
  pub equals: BTreeMap<ConstrKind, Vec<EqualsDef>>,
  pub func_ids: BTreeMap<ConstrKind, Vec<usize>>,
  props: Vec<Formula>,
  labels: IdxVec<LabelId, Option<usize>>,
  pending_defs: Vec<PendingDef>,
  pub def_map: HashMap<DefRef, DefiniensId>,
  pub pos: Position,
  pub progress: Option<ProgressBar>,
}
impl WithGlobalLocal for Reader {
  fn global(&self) -> &Global { &self.g }
  fn local(&self) -> &LocalContext { &self.lc }
}

impl MizPath {
  pub fn with_reader(
    &self, cfg: &Config, progress: Option<&ProgressBar>, mml_vct: &[u8],
    f: &mut dyn FnMut(&mut Reader, Option<&mut MizParser<'_>>),
  ) -> io::Result<bool> {
    let mut accom = cfg.accom_enabled.then(Box::<Accomodator>::default);
    let data;
    let mut parser = if cfg.parser_enabled {
      data = self.read_miz().unwrap();
      Some(Box::new(MizParser::new(self.art, progress, &data)))
    } else {
      None
    };
    if let Some(accom) = &mut accom {
      if let Some(parser) = &mut parser {
        parser.parse_env(&mut accom.dirs)
      } else {
        self.read_evl(&mut accom.dirs).unwrap();
      }
    }

    // MizPBlockObj.InitPrepData
    let mut refs = References::default();
    let refs = if cfg.analyzer_enabled || cfg.accom_enabled {
      None
    } else {
      if cfg.checker_enabled {
        self.read_ref(&mut refs).unwrap();
      }
      Some(&refs)
    };

    // Load_EnvConstructors
    let mut v = Reader::new(cfg, progress.cloned(), accom, self.art);
    v.lc.attr_sort_bug = cfg.attr_sort_bug;
    v.lc.formatter.dump = cfg.dump.formatter;
    let old = v.lc.start_stash();
    if let Some(accom) = &mut v.accom {
      accom.accom_constructors(&mut v.g.constrs).unwrap();
      accom.accom_requirements(&v.g.constrs, &mut v.g.reqs).unwrap();
      if cfg.xml_internals {
        self.write_atr(&accom.sig, &v.g.constrs)
      }
    } else {
      self.read_atr(&mut v.g.constrs).unwrap();
      self.read_ere(&mut v.g.reqs).unwrap();
    }
    v.g.reqs.init_rev();
    let mut has_omega = false;
    if let (Some(element), Some(omega)) = (v.g.reqs.element(), v.g.reqs.omega()) {
      has_omega = true;
      v.g.numeral_type = Type {
        kind: TypeKind::Mode(element),
        attrs: Default::default(),
        args: vec![Term::Functor { nr: omega, args: Box::new([]) }],
      }
    }
    let symbols = v.accom.as_deref_mut().map(|accom| {
      let mut symbols = Default::default();
      let mut inf = parser.as_ref().map(|_| Default::default());
      let mut priority = vec![];
      accom.accom_symbols(mml_vct, &mut symbols, &mut priority, inf.as_mut());
      if cfg.checker_enabled || cfg.parser_enabled {
        accom.accom_articles()
      }
      if let Some(parser) = &mut parser {
        parser.load_symbols(&symbols, &inf.unwrap(), &priority);
      }
      symbols
    });
    let mut notations = Default::default();
    let needs_formats =
      cfg.analyzer_enabled || v.lc.formatter.cfg.enable_formatter || cfg.exporter_enabled;
    if needs_formats && !cfg.parser_enabled {
      let mut fmts = Default::default();
      self.read_formats("frx", &mut fmts).unwrap();
      v.formats.extend(fmts.formats.enum_iter().map(|(id, f)| (*f, id)));
      *v.lc.formatter.formats = fmts.formats;
    }
    if let Some(accom) = &mut v.accom {
      if let Some(parser) = &mut parser {
        let mut fmts = Default::default();
        let mut fmt_map = Default::default();
        accom.accom_notations(&mut fmt_map, Some(&mut fmts), &mut notations).unwrap();
        fmts.formats.0.iter().for_each(|fmt| parser.read_format(fmt));
        v.formats.extend(fmts.formats.enum_iter().map(|(id, f)| (*f, id)));
        *v.lc.formatter.formats = fmts.formats;
      } else {
        accom.accom_notations(&mut v.formats, None, &mut notations).unwrap();
      }
    } else {
      self.read_eno(&mut notations).unwrap();
    }
    if cfg.exporter_enabled {
      v.formats_base = if cfg.parser_enabled {
        v.lc.formatter.formats.len()
      } else {
        let mut f = Default::default();
        self.read_formats("frm", &mut f).unwrap();
        f.formats.len()
      }
    }
    if cfg.dump.notations {
      notations.iter().for_each(|pat| eprintln!("{pat:?}"))
    }
    v.lc.formatter.init_symbols(self, symbols);
    if v.accom.is_some() {
      self.write_dcx(&v.lc.formatter.symbols);
    }
    v.lc.formatter.init();
    v.lc.formatter.extend(&v.g.constrs, &notations);
    if cfg.dump.constructors {
      v.g.constrs.dump()
    }
    if cfg.dump.requirements {
      v.g.reqs.dump(&v.g.constrs)
    }
    if let Some(accom) = &mut v.accom {
      accom.accom_clusters(&v.g.constrs, &mut v.g.clusters).unwrap();
    } else {
      self.read_ecl(&v.g.constrs, &mut v.g.clusters).unwrap();
    }
    v.g.clusters.functor.sort_all(|a, b| FunctorCluster::cmp_term(&a.term, &v.g.constrs, &b.term));
    if cfg.dump.clusters {
      v.g.clusters.dump()
    }

    let mut attrs = Attrs::default();
    if let Some(zero) = v.g.reqs.zero() {
      attrs.push(Attr::new0(zero, false))
    }
    if has_omega {
      if let Some(positive) = v.g.reqs.positive() {
        attrs.push(Attr::new0(positive, true))
      }
    }
    attrs.round_up_with(&v.g, &v.lc, &v.g.numeral_type, false);
    v.g.numeral_type.attrs.1 = attrs;
    v.lc.clear_term_cache();
    v.g.round_up_clusters(&mut v.lc);

    if cfg.analyzer_enabled {
      // LoadSGN
      for mut pat in notations {
        if let PatternKind::ExpandableMode { expansion } = &mut pat.kind {
          v.lc.load_locus_tys(&pat.primary);
          expansion.round_up_with_self(&v.g, &v.lc, false);
          v.lc.unload_locus_tys();
        }
        v.notations[pat.kind.class()].push(pat)
      }
      if cfg.xml_internals && v.accom.is_some() {
        self.write_eno(&v.notations);
      }
    }

    // LoadDefinitions
    if cfg.analyzer_enabled {
      if let Some(accom) = &mut v.accom {
        accom
          .accom_definitions(&v.g.constrs, DirectiveKind::Definitions, &mut v.definitions.0)
          .unwrap();
        if cfg.xml_internals {
          self.write_dfs(&v.definitions.0)
        }
      } else {
        self.read_definitions(&v.g.constrs, false, "dfs", None, &mut v.definitions.0).unwrap();
      }
      if cfg.dump.definitions {
        for d in &v.definitions.0 {
          eprintln!("definition: {d:?}");
        }
      }
    }

    // LoadEqualities
    if cfg.checker_enabled {
      if let Some(accom) = &mut v.accom {
        accom
          .accom_definitions(&v.g.constrs, DirectiveKind::Equalities, &mut v.equalities)
          .unwrap();
      } else {
        self.read_definitions(&v.g.constrs, false, "dfe", None, &mut v.equalities).unwrap();
      }
      if cfg.dump.definitions {
        for d in &v.equalities {
          eprintln!("equality: {d:?}");
        }
      }
    }

    // LoadExpansions
    if cfg.checker_enabled {
      if let Some(accom) = &mut v.accom {
        accom
          .accom_definitions(&v.g.constrs, DirectiveKind::Expansions, &mut v.expansions)
          .unwrap();
      } else {
        self.read_definitions(&v.g.constrs, false, "dfx", None, &mut v.expansions).unwrap();
      }
      if cfg.dump.definitions {
        for d in &v.expansions {
          eprintln!("expansion: {d:?}");
        }
      }
    }

    // LoadPropertiesReg
    if let Some(accom) = &mut v.accom {
      accom.accom_properties(&v.g.constrs, &mut v.properties).unwrap();
    } else {
      self.read_properties(&v.g.constrs, false, "epr", None, &mut v.properties).unwrap();
    }

    // LoadIdentify, LoadReductions
    if cfg.checker_enabled || cfg.exporter_enabled {
      if let Some(accom) = &mut v.accom {
        accom.accom_identify_regs(&v.g.constrs, &mut v.identify).unwrap();
        accom.accom_reduction_regs(&v.g.constrs, &mut v.reductions).unwrap();
      } else {
        self.read_identify_regs(&v.g.constrs, false, "eid", None, &mut v.identify).unwrap();
        self.read_reduction_regs(&v.g.constrs, false, "erd", None, &mut v.reductions).unwrap();
      }
    }

    // in mizar this was done inside the parser
    RoundUpTypes::with(&v.g, &mut v.lc, |rr| {
      v.definitions.visit(rr);
      v.equalities.visit(rr);
      v.expansions.visit(rr);
      v.properties.visit(rr);
      v.identify.visit(rr);
      v.reductions.visit(rr);
    });

    for df in &v.equalities {
      if let Some(func) = df.equals_expansion() {
        let nr = func.pattern.0;
        if !func.expansion.has_func(&v.g.constrs, nr) {
          v.equals.entry(df.constr).or_default().push(func);
        }
      }
    }

    for id in &mut v.identify {
      for i in 0..id.primary.len() {
        v.lc.load_locus_tys(&id.primary);
        id.primary[i].round_up_with_self(&v.g, &v.lc, false);
        v.lc.unload_locus_tys();
      }
    }

    for (i, id) in v.identify.iter().enumerate() {
      let Term::Functor { nr, .. } = id.lhs else { unreachable!() };
      let k = ConstrKind::Func(Term::adjusted_nr(nr, &v.g.constrs));
      v.func_ids.entry(k).or_default().push(i);
    }

    // CollectConstInEnvConstructors
    let cc = &mut v.intern_const();
    let numeral_type = v.g.numeral_type.visit_cloned(cc);
    let mut constrs = v.g.constrs.clone();
    constrs.mode.visit(cc);
    constrs.struct_mode.visit(cc);
    // constrs.attribute.visit(cc); // no collection in attributes?
    constrs.predicate.visit(cc);
    constrs.functor.visit(cc);
    constrs.selector.visit(cc);
    constrs.aggregate.visit(cc);
    let mut clusters = v.g.clusters.clone();
    clusters.registered.iter_mut().for_each(|c| c.consequent.1.visit(cc));
    clusters.conditional.iter_mut().for_each(|c| c.consequent.1.visit(cc));
    // note: collecting in the functor term breaks the sort order
    clusters.functor.vec.0.iter_mut().for_each(|c| c.consequent.1.visit(cc));
    v.g.numeral_type = numeral_type;
    v.g.constrs = constrs;
    v.g.clusters = clusters;

    // InLibraries
    if cfg.checker_enabled {
      if let Some(accom) = &mut v.accom {
        accom.accom_theorems(cfg.xml_internals, &v.g.constrs, &mut v.def_map, &mut v.libs).unwrap();
      } else {
        self.read_eth(&v.g.constrs, refs, &mut v.libs).unwrap();
      }
      let cc = &mut InternConst::new(&v.g, &v.lc, &v.equals, &v.identify, &v.func_ids);
      v.libs.thm.values_mut().for_each(|f| f.visit(cc));
      v.libs.def.values_mut().for_each(|f| f.visit(cc));
      if let Some(accom) = &mut v.accom {
        accom.accom_schemes(cfg.xml_internals, &v.g.constrs, &mut v.libs).unwrap();
      } else {
        self.read_esh(&v.g.constrs, refs, &mut v.libs).unwrap();
      }
      RoundUpTypes::with(&v.g, &mut v.lc, |rr| v.libs.visit(rr));

      if cfg.dump.libraries {
        for (&(ar, nr), th) in &v.libs.thm {
          eprintln!("art {ar:?}:{nr:?} = {th:?}");
        }
        for (&(ar, nr), th) in &v.libs.def {
          eprintln!("art {ar:?}:def {nr:?} = {th:?}");
        }
        for (&(ar, nr), sch) in &v.libs.sch {
          eprintln!("art {ar:?}:sch {nr:?} = {sch:?}");
        }
      }
    }

    if let (Some(accom), Some(parser)) = (&mut v.accom, &mut parser) {
      std::mem::swap(&mut parser.articles, &mut accom.articles)
    }

    f(&mut v, parser.as_deref_mut());

    LocalContext::end_stash(old);
    Ok(v.has_errors)
  }
}

pub struct Scope {
  pub fixed_var: usize,
  pub props: usize,
  pub labels: usize,
  pub priv_funcs: usize,
  pub infer_const: usize,
  pub pending_defs: usize,
}

impl Reader {
  pub fn new(
    cfg: &Config, progress: Option<ProgressBar>, accom: Option<Box<Accomodator>>, article: Article,
  ) -> Self {
    Reader {
      g: Global {
        cfg: cfg.clone(),
        reqs: Default::default(),
        constrs: Default::default(),
        clusters: Default::default(),
        numeral_type: Type::SET,
      },
      lc: LocalContext::default(),
      libs: Libraries::default(),
      article,
      treat_thm_as_axiom: matches!(article.as_str(), "tarski_0" | "tarski_a"),
      has_errors: accom.as_deref().map_or(false, |acc| acc.has_errors),
      accom,
      formats: Default::default(),
      formats_base: 0,
      notations: Default::default(),
      definitions: Default::default(),
      equalities: Default::default(),
      expansions: Default::default(),
      properties: Default::default(),
      identify: Default::default(),
      reductions: Default::default(),
      equals: Default::default(),
      func_ids: Default::default(),
      props: Default::default(),
      labels: Default::default(),
      pending_defs: Default::default(),
      def_map: Default::default(),
      pos: Default::default(),
      no_suppress_checker: true,
      progress,
    }
  }

  pub fn err(&mut self, pos: Position, msg: MizError) {
    self.has_errors |= msg.report(self.article, pos, &self.g, &self.lc);
  }

  pub fn intern<'a, T: Clone + Visitable<InternConst<'a>>>(&'a self, t: &T) -> T {
    let mut t = t.clone();
    t.visit(&mut self.intern_const());
    t
  }

  pub fn intern_const(&self) -> InternConst<'_> {
    InternConst::new(&self.g, &self.lc, &self.equals, &self.identify, &self.func_ids)
  }

  pub fn push_prop(&mut self, label: Option<LabelId>, prop: Formula) {
    // eprintln!("push_prop {label:?}: {prop:?}");
    if let Some(label) = label {
      assert_eq!(label, self.labels.push(Some(self.props.len())));
    }
    self.props.push(prop);
  }

  fn read_proposition(&mut self, prop: &Proposition) {
    self.push_prop(prop.label, self.intern(&prop.f))
  }

  fn push_fixed_var(&mut self, ty: &Type) {
    self.lc.fixed_var.push(FixedVar { ty: self.intern(ty), def: None });
  }

  fn read_fixed_vars(&mut self, vars: &[Type]) {
    vars.iter().for_each(|ty| self.push_fixed_var(ty))
  }

  pub(crate) fn open_scope(&mut self, push_label: bool) -> Scope {
    let labels = self.labels.len();
    // eprintln!("new block {:?}, labels = {labels}", label);
    if push_label {
      self.labels.push(None);
      // eprintln!("push_prop open {l:?}");
    }
    let fixed_var = self.lc.fixed_var.len();
    let props = self.props.len();
    let pending_defs = self.pending_defs.len();
    let priv_funcs = self.lc.priv_func.len();
    let infer_const = self.lc.infer_const.get_mut().len();
    // let sc = self.lc.term_cache.get_mut().scope;
    // vprintln!("open scope {:?}", (labels, fixed_var, props, priv_funcs, infer_const, sc));
    self.lc.term_cache.get_mut().open_scope();
    Scope { fixed_var, props, labels, priv_funcs, infer_const, pending_defs }
  }

  pub(crate) fn close_scope(&mut self, sc: Scope, check_for_local_const: bool) -> Descope {
    self.lc.term_cache.get_mut().close_scope();
    // let labels2 = self.labels.len();
    // let fixed_var2 = self.lc.fixed_var.len();
    // let props2 = self.props.len();
    // let priv_funcs2 = self.lc.priv_func.len();
    // let infer_const2 = self.lc.infer_const.get_mut().len();
    // let sc2 = self.lc.term_cache.get_mut().scope;
    // vprintln!(
    //   "close scope {:?} <- {:?}",
    //   (sc.labels, sc.fixed_var, sc.props, sc.priv_funcs, sc.infer_const, sc2),
    //   (labels2, fixed_var2, props2, priv_funcs2, infer_const2, sc2 + 1)
    // );
    self.lc.fixed_var.0.truncate(sc.fixed_var);
    self.props.truncate(sc.props);
    // eprintln!("push_prop reset {} / {}", sc.props, sc.labels);
    self.labels.0.truncate(sc.labels);
    self.lc.priv_func.0.truncate(sc.priv_funcs);
    let mut descope =
      self.lc.truncate_infer_const(&self.g.constrs, check_for_local_const, sc.infer_const);
    // let infer_const3 = self.lc.infer_const.get_mut().len();
    // if infer_const3 > sc.infer_const {
    //   vprintln!("reinserted {:?} -> {:?}", sc.infer_const, infer_const3);
    // }
    if descope.remap.is_empty() {
      self.pending_defs.truncate(sc.pending_defs)
    } else {
      for x in self.pending_defs.drain(sc.pending_defs..) {
        match x {
          PendingDef::Constr(k) => self.g.constrs.visit_at(&mut descope, k),
          PendingDef::Cluster(ClusterKind::R, i) =>
            self.g.clusters.registered[i].visit(&mut descope),
          PendingDef::Cluster(ClusterKind::F, i) => self.g.clusters.functor[i].visit(&mut descope),
          PendingDef::Cluster(ClusterKind::C, i) =>
            self.g.clusters.conditional.vec[i].visit(&mut descope),
        }
      }
    }

    // self._dbg_scope_check();
    descope
  }

  fn scope<R>(
    &mut self, label: Option<LabelId>, check_for_local_const: bool, f: impl FnOnce(&mut Self) -> R,
  ) -> R {
    let sc = self.open_scope(label.is_some());
    let r = f(self);
    self.close_scope(sc, check_for_local_const);
    r
  }

  /// Prepare
  pub fn run_checker(&mut self, path: &MizPath) {
    let result = path.read_xml(|it| {
      assert!(matches!(
        it,
        Item::AuxiliaryItem(_)
          | Item::Scheme(_)
          | Item::Theorem { .. }
          | Item::DefTheorem { .. }
          | Item::Reservation { .. }
          | Item::Canceled(_)
          | Item::Definiens(_)
          | Item::Block { .. }
      ));
      // stat(s);
      if self.g.cfg.top_item_header {
        eprintln!("item: {it:?}");
      }
      self.read_item(&it);
    });
    if let Err((path, e)) = result {
      e.report(&path);
      self.has_errors = true;
    }
  }

  pub fn set_pos(&mut self, pos: Position) {
    self.pos = pos;
    if let Some(progress) = &self.progress {
      progress.set_position(pos.line.into())
    }
  }

  pub fn read_item(&mut self, it: &Item) {
    if let Some(pos) = it.pos() {
      self.set_pos(pos);
    }
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
        eprintln!("{it:#?}");
      } else {
        match it {
          Item::Let(_) => eprintln!("Let"),
          Item::Given(it) => eprintln!("Given @ {:?}", it.prop.pos),
          Item::Thus(it) => eprintln!("Thus @ {:?}", it.pos()),
          Item::Assume(it) => eprintln!("Assume @ {:?}", it[0].pos),
          Item::Take(_) => eprintln!("Take"),
          Item::TakeAsVar(_, _) => eprintln!("TakeAsVar"),
          Item::PerCases(it) => eprintln!("PerCases @ {:?}", it.pos.0),
          Item::AuxiliaryItem(it) => match it {
            AuxiliaryItem::Statement(it) => match it {
              Statement::Proposition { .. } => eprintln!("Proposition @ {:?}", it.pos()),
              Statement::IterEquality { .. } => eprintln!("IterEquality @ {:?}", it.pos()),
              Statement::Now { .. } => eprintln!("Now @ {:?}", it.pos()),
            },
            AuxiliaryItem::Consider { prop, .. } => eprintln!("Consider @ {:?}", prop.pos),
            AuxiliaryItem::Set { .. } => eprintln!("Set"),
            AuxiliaryItem::Reconsider { prop, .. } => eprintln!("Reconsider @ {:?}", prop.pos),
            AuxiliaryItem::DefFunc { .. } => eprintln!("DefFunc"),
            AuxiliaryItem::DefPred { .. } => eprintln!("DefPred"),
          },
          Item::Registration(_) => eprintln!("Registration"),
          Item::Scheme(it) => eprintln!("Scheme @ {:?}", it.pos.0),
          Item::Theorem { prop, .. } => eprintln!("Theorem @ {:?}", prop.pos),
          Item::DefTheorem { kind, prop } => eprintln!("DefTheorem {kind:?} @ {:?}", prop.pos),
          Item::Reservation { .. } => eprintln!("Reservation"),
          Item::Canceled(_) => eprintln!("Canceled"),
          Item::Definition(it) => eprintln!("Definition @ {:?}", it.pos),
          Item::DefStruct(it) => eprintln!("DefStruct @ {:?}", it.pos),
          Item::Definiens(_) => eprintln!("Definiens"),
          Item::Block { kind, pos, .. } => eprintln!("{kind:?} block @ {:?}", pos.0),
          Item::Pattern(_) => eprintln!("Pattern"),
        }
      }
    }
    match it {
      // reservations not handled by checker
      Item::Reservation { .. } => {}
      Item::Let(vars) => self.read_fixed_vars(vars),
      Item::Given(GivenItem { prop, fixed, intro }) => {
        self.read_proposition(prop);
        self.read_fixed_vars(fixed);
        intro.iter().for_each(|prop| self.read_proposition(prop));
      }
      Item::Assume(intro) => intro.iter().for_each(|prop| self.read_proposition(prop)),
      Item::Take(_) => {}
      Item::TakeAsVar(ty, tm) => {
        let fv = FixedVar { ty: self.intern(ty), def: Some((Box::new(self.intern(tm)), false)) };
        self.lc.fixed_var.push(fv);
      }
      Item::PerCases(PerCases { label, block_thesis, cases, prop, just, .. }) => {
        self.scope(*label, false, |this| {
          for CaseBlock { cs, items, .. } in cases {
            this.scope(None, false, |this| {
              let (CaseKind::Case(props) | CaseKind::Suppose(props)) = cs;
              props.iter().for_each(|p| this.read_proposition(p));
              for it in items {
                this.read_item(&it.0);
                // this is only needed if we want to match mizar numbering
                // if let Some(thesis) = &it.1 {
                //   let _ = this.intern(&thesis.f);
                // }
              }
            });
          }
          this.read_just_prop(prop, just, false)
        });
        self.push_prop(*label, self.intern(block_thesis))
      }
      Item::AuxiliaryItem(AuxiliaryItem::Statement(it)) | Item::Thus(it) => self.read_stmt(it),
      Item::AuxiliaryItem(AuxiliaryItem::Consider { prop, just, fixed, intro }) => {
        self.read_just_prop(prop, just, false);
        self.read_fixed_vars(fixed);
        intro.iter().for_each(|prop| self.read_proposition(prop));
      }
      Item::AuxiliaryItem(AuxiliaryItem::Set { ty, .. }) => self.push_fixed_var(ty),
      Item::AuxiliaryItem(AuxiliaryItem::Reconsider { terms, prop, just }) => {
        for (ty, tm) in terms {
          let fv = FixedVar { ty: self.intern(ty), def: Some((Box::new(self.intern(tm)), false)) };
          self.lc.fixed_var.push(fv);
        }
        self.read_just_prop(prop, just, false);
      }
      Item::AuxiliaryItem(AuxiliaryItem::DefFunc { args, ty, value }) => {
        self.lc.priv_func.push(FuncDef {
          primary: self.intern(args),
          ty: Box::new(self.intern(ty)),
          value: Box::new(self.intern(value)),
        });
      }
      Item::AuxiliaryItem(AuxiliaryItem::DefPred { .. }) => {}
      Item::Registration(reg) => match reg {
        Registration::Cluster(cl) => self.read_cluster_decl(cl),
        Registration::Identify { kind, conds, corr } => {
          self.read_corr_conds(conds, corr);
          self.push_identify(kind)
        }
        Registration::Reduction { kind, conds, corr } => {
          self.read_corr_conds(conds, corr);
          self.reductions.push(kind.clone())
        }
        Registration::Property { prop, just, .. } => self.read_just_prop(prop, just, false),
      },
      Item::Scheme(sch) => self.read_scheme(sch),
      Item::Theorem { prop, just } => self.read_just_prop(prop, just, true),
      Item::DefTheorem { prop, .. } => self.read_proposition(prop),
      Item::Canceled(_) => {}
      Item::Definition(Definition { conds, corr, props, constr, patts, .. }) => {
        self.read_corr_conds(conds, corr);
        for JustifiedProperty { prop, just, .. } in props {
          self.read_just_prop(prop, just, false)
        }
        if let Some(constr) = constr {
          let id = self.g.constrs.push(self.intern(constr));
          self.push_constr(id);
        }
        self.lc.formatter.extend(&self.g.constrs, patts)
      }
      Item::DefStruct(DefStruct { constrs, cl, conds, corr, patts, .. }) => {
        for c in constrs {
          let id = self.g.constrs.push(self.intern(c));
          self.push_constr(id);
        }
        self.read_cluster_decl(cl);
        self.read_corr_conds(conds, corr);
        self.lc.formatter.extend(&self.g.constrs, patts)
      }
      Item::Definiens(df) => self.read_definiens(df),
      Item::Block { kind, label, items, .. } => {
        let check = matches!(kind, BlockKind::Definition | BlockKind::Registration);
        self.scope(*label, check, |this| {
          for it in items {
            this.read_item(it);
          }
        });
      }
      Item::Pattern(pat) => self.lc.formatter.push(&self.g.constrs, pat),
    }
  }

  fn read_scheme(&mut self, SchemeBlock { nr, decls, prems, thesis, just, .. }: &SchemeBlock) {
    self.scope(None, false, |this| {
      assert!(this.lc.sch_func_ty.is_empty());
      let infer_consts = this.lc.infer_const.get_mut().0.len();
      for decl in decls {
        if let SchemeDecl::Func { ty, .. } = decl {
          this.lc.sch_func_ty.0.push(this.intern(ty))
        }
      }
      let mut prems = prems
        .iter()
        .map(|prem| {
          let f = this.intern(&prem.f);
          this.push_prop(prem.label, f.clone());
          f
        })
        .collect();
      let mut thesis = this.intern(&thesis.f);
      this.read_justification(&thesis, just);
      let mut primary = this.lc.sch_func_ty.0.drain(..).collect();
      this.lc.expand_consts(&this.g.constrs, |c| {
        (&mut primary, &mut prems).visit(c);
        thesis.visit(c)
      });
      this.lc.infer_const.get_mut().truncate(infer_consts);
      this.libs.sch.insert((ArticleId::SELF, *nr), Scheme { sch_funcs: primary, prems, thesis });
    });
  }

  pub fn read_definiens(&mut self, df: &Definiens) {
    if self.g.cfg.analyzer_enabled {
      self.definitions.push(df.clone());
    }
    if self.g.cfg.checker_enabled {
      self.equalities.push(df.clone());
      self.expansions.push(df.clone());
      if let Some(func) = df.equals_expansion() {
        let f = func.pattern.0;
        if !func.expansion.has_func(&self.g.constrs, f) {
          let mut i = 0;
          loop {
            let mut ic = self.lc.infer_const.borrow_mut();
            let Some(asgn) = ic.0.get_mut(i) else { break };
            if let Term::Functor { nr, args } = &asgn.def {
              let (nr, args) = Term::adjust(*nr, args, Some(&self.g.constrs));
              if f == nr {
                let args = &args.to_owned();
                drop(ic);
                if let Some(mut t) = func.expand_if_equal(&self.g, &self.lc, args, 0) {
                  t.visit(&mut self.intern_const());
                  let Term::Infer(nr) = t else { unreachable!() };
                  self.lc.infer_const.borrow_mut().0[i].insert_eq_const(nr);
                }
              }
            }
            i += 1;
          }
          self.equals.entry(df.constr).or_default().push(func);
        }
      }
    }
  }

  fn read_justification(&mut self, thesis: &Formula, just: &Justification) {
    match just {
      Justification::Simple(it) => self.read_inference(thesis, it),
      Justification::Proof { label, thesis: block_thesis, items, .. } => {
        let block_thesis = self.intern(block_thesis);
        assert!(self.g.eq(&self.lc, thesis, &block_thesis), "\n{thesis:?}\n !=\n{block_thesis:?}");
        self.scope(*label, false, |this| {
          for it in items {
            this.read_item(&it.0);
            // this is only needed if we want to match mizar numbering
            // if let Some(thesis) = &it.1 {
            //   let _ = this.intern(&thesis.f);
            // }
          }
        });
      }
      Justification::SkippedProof => assert!(matches!(thesis, Formula::True)),
    }
  }

  fn read_just_prop(&mut self, prop: &Proposition, just: &Justification, quotable: bool) {
    let f = self.intern(&prop.f);
    self.read_justification(&f, just);
    if quotable {
      self.push_prop(prop.label, f);
    }
  }

  fn read_stmt(&mut self, it: &Statement) {
    match it {
      Statement::Proposition { prop, just } => self.read_just_prop(prop, just, true),
      Statement::IterEquality { label, lhs, steps, .. } => {
        let mut lhs = self.intern(lhs);
        let llhs = lhs.clone();
        for (rhs, step) in steps {
          let rhs = self.intern(rhs);
          self.read_inference(&self.g.reqs.mk_eq(lhs, rhs.clone()), step);
          lhs = rhs;
        }
        self.push_prop(*label, self.g.reqs.mk_eq(llhs, lhs))
      }
      Statement::Now { label, thesis, items, .. } => {
        self.scope(*label, true, |this| {
          for it in &**items {
            this.read_item(it);
          }
        });
        self.push_prop(*label, self.intern(thesis));
      }
    }
  }

  fn read_corr_conds(&mut self, conds: &[CorrCond], corr: &Option<Correctness>) {
    conds.iter().for_each(|c| self.read_just_prop(&c.prop, &c.just, false));
    if let Some(c) = corr {
      self.read_just_prop(&c.prop, &c.just, false)
    }
  }

  pub fn push_constr(&mut self, id: ConstrKind) { self.pending_defs.push(PendingDef::Constr(id)) }

  pub fn push_identify(&mut self, id: &IdentifyFunc) {
    let mut ic = self.lc.infer_const.borrow();
    // Note that the infer_const array can grow in the loop,
    // but the loop itself only goes over the elements which are there at the beginning.
    // This is probably a bug, but MML depends on it so ¯\_(ツ)_/¯
    for i in 0..ic.len() {
      let asgn = &ic.0[i];
      if matches!(asgn.def, Term::Functor { .. }) {
        if let Some(subst) = id.try_apply_lhs(&self.g, &self.lc, &id.lhs, &asgn.def) {
          let mut tm = subst.inst_term(&self.g.constrs, &self.lc, &id.rhs, 0);
          drop(ic);
          tm.visit(&mut self.intern_const());
          let Term::Infer(n) = tm else { unreachable!() };
          self.lc.infer_const.borrow_mut().0[i].insert_eq_const(n);
          ic = self.lc.infer_const.borrow();
        }
      }
    }
    let Term::Functor { nr, .. } = id.lhs else { unreachable!() };
    let k = ConstrKind::Func(Term::adjusted_nr(nr, &self.g.constrs));
    self.func_ids.entry(k).or_default().push(self.identify.len());
    self.identify.push(id.clone());
  }

  /// RoundUpFurther
  fn round_up_further<T>(&mut self, rounded_up: BTreeMap<InferId, T>) {
    if rounded_up.is_empty() {
      return
    }
    let mut i = *rounded_up.first_key_value().unwrap().0;
    let mut ic = self.lc.infer_const.borrow();
    while let Some(asgn) = ic.get(i) {
      match &asgn.def {
        Term::Functor { args, .. }
        | Term::Selector { args, .. }
        | Term::Aggregate { args, .. }
        | Term::SchFunc { args, .. } => {
          if args.iter().any(|t| matches!(t, Term::Infer(i) if rounded_up.contains_key(i))) {
            let mut ty = asgn.def.round_up_type(&self.g, &self.lc, false).to_owned();
            ty.attrs.1.round_up_with(&self.g, &self.lc, &asgn.ty, false);
            drop(ic);
            ty.attrs.1.visit(&mut self.intern_const());
            self.lc.infer_const.borrow_mut()[i].ty = *ty;
            ic = self.lc.infer_const.borrow();
          }
        }
        Term::Numeral(_) => {
          let mut attrs = asgn.ty.attrs.1.clone();
          attrs.round_up_with(&self.g, &self.lc, &asgn.ty, false);
          attrs.visit(&mut self.intern_const());
          drop(ic);
          self.lc.infer_const.borrow_mut()[i].ty.attrs.1 = attrs;
          ic = self.lc.infer_const.borrow();
        }
        _ => {}
      }
      i.0 += 1;
    }
    //
  }

  fn read_cluster_decl(&mut self, cl: &ClusterDecl) {
    self.read_corr_conds(&cl.conds, &cl.corr);
    match &cl.kind {
      ClusterDeclKind::R(cl) => self.read_registered_cluster(cl.clone()),
      ClusterDeclKind::F(cl) => self.read_functor_cluster(cl.clone()),
      ClusterDeclKind::C(cl) => self.read_conditional_cluster(cl.clone()),
    }
  }

  pub fn read_registered_cluster(&mut self, mut cl: RegisteredCluster) {
    cl.consequent.1.visit(&mut self.intern_const());
    let i = self.g.clusters.registered.len();
    self.g.clusters.registered.push(cl);
    self.pending_defs.push(PendingDef::Cluster(ClusterKind::R, i))
  }

  pub fn read_conditional_cluster(&mut self, mut cl: ConditionalCluster) {
    cl.consequent.1.visit(&mut self.intern_const());
    let mut rounded_up = BTreeMap::new();
    for (i, asgn) in self.lc.infer_const.borrow().enum_iter() {
      if let Some(subst) = cl.try_apply(&self.g, &self.lc, &asgn.ty.attrs.1, &asgn.ty, false) {
        let mut attrs = asgn.ty.attrs.1.clone();
        let orig = attrs.attrs().len();
        // eprintln!("enlarge {:?} by {:?}", self, cl.consequent.1);
        let mut inst = Inst::new(&self.g.constrs, &self.lc, &subst, 0);
        attrs.visit_enlarge_by(&self.g.constrs, &self.lc, &cl.consequent.1, &mut inst);
        assert!(matches!(attrs, Attrs::Consistent(_)));
        attrs.round_up_with(&self.g, &self.lc, &asgn.ty, false);
        if attrs.attrs().len() > orig {
          rounded_up.insert(i, attrs);
        }
      }
    }
    let i = self.g.clusters.conditional.len();
    self.g.clusters.conditional.push(&self.g.constrs, cl);
    for (&i, attrs) in &mut rounded_up {
      attrs.visit(&mut self.intern_const());
      self.lc.infer_const.get_mut()[i].ty.attrs.1 = std::mem::take(attrs);
    }
    self.round_up_further(rounded_up);
    self.pending_defs.push(PendingDef::Cluster(ClusterKind::C, i))
  }

  pub fn read_functor_cluster(&mut self, mut cl: FunctorCluster) {
    cl.consequent.1.visit(&mut self.intern_const());
    let mut rounded_up = BTreeMap::new();
    for (i, asgn) in self.lc.infer_const.borrow().enum_iter() {
      let mut attrs = asgn.ty.attrs.1.clone();
      let orig = attrs.attrs().len();
      cl.round_up_with(&self.g, &self.lc, &asgn.def, &asgn.ty, &mut attrs, false);
      assert!(matches!(attrs, Attrs::Consistent(_)));
      attrs.round_up_with(&self.g, &self.lc, &asgn.ty, false);
      if attrs.attrs().len() > orig {
        rounded_up.insert(i, attrs);
      }
    }
    let (_, i) = (self.g.clusters.functor)
      .equal_range(|a| FunctorCluster::cmp_term(&a.term, &self.g.constrs, &cl.term));
    self.g.clusters.functor.insert_at(i, cl);
    for (&i, attrs) in &mut rounded_up {
      attrs.visit(&mut self.intern_const());
      self.lc.infer_const.get_mut()[i].ty.attrs.1 = std::mem::take(attrs);
    }
    self.round_up_further(rounded_up);
    self.pending_defs.push(PendingDef::Cluster(ClusterKind::F, i))
  }

  pub fn read_inference(&mut self, thesis: &Formula, it: &Inference) {
    if !self.g.cfg.checker_enabled {
      return
    }
    if !self.no_suppress_checker {
      stat("skipped by $V-");
      return
    }
    self.set_pos(it.pos);
    let refs = it.refs.iter().map(|r| match r.kind {
      ReferenceKind::Priv(lab) => &self.props[self.labels[lab].unwrap()],
      ReferenceKind::Thm(thm) => &self.libs.thm[&thm],
      ReferenceKind::Def(def) => &self.libs.def[&def],
    });
    let mut ck = Checker {
      g: &mut self.g,
      lc: &mut self.lc,
      expansions: &self.expansions,
      equals: &self.equals,
      identify: &self.identify,
      func_ids: &self.func_ids,
      reductions: &self.reductions,
      article: self.article,
      pos: it.pos,
    };
    match it.kind {
      InferenceKind::By { linked } => {
        if !self.treat_thm_as_axiom || linked || !it.refs.is_empty() {
          // eprintln!("thesis: {thesis:?}");
          let neg_thesis = thesis.clone().mk_neg();
          let mut premises = vec![&neg_thesis];
          if linked {
            premises.push(self.props.last().unwrap());
          }
          premises.extend(refs);
          ck.justify(premises);
        }
      }
      InferenceKind::From { sch } =>
        ck.justify_scheme(&self.libs.sch[&sch], refs.collect(), thesis),
    }
  }

  #[allow(clippy::blocks_in_if_conditions)]
  fn _dbg_scope_check(&self) {
    let ic = self.lc.infer_const.borrow();
    let infer_const = ic.len();

    struct Checker(u32, bool);
    impl Visit for Checker {
      fn abort(&self) -> bool { !self.1 }
      fn visit_term(&mut self, tm: &Term) {
        self.super_visit_term(tm);
        // Term::PrivFunc { nr, args, value } => self.2 &= nr.0 < self.0,
        if let Term::Infer(nr) = tm {
          self.1 &= nr.0 < self.0
        }
      }
    }
    fn with(ic: usize, f: impl FnOnce(&mut Checker)) -> bool {
      let mut ck = Checker(ic as u32, true);
      f(&mut ck);
      ck.1
    }
    for (i, c) in self.g.constrs.functor.0.iter().enumerate() {
      if !with(infer_const, |ck| ck.visit_type(&c.ty)) {
        panic!("bad functor: F{i:?}: {c:?}")
      }
    }
    for c in &self.lc.term_cache.borrow().terms {
      if !with(infer_const, |ck| {
        ck.visit_term(&c.0);
        ck.visit_type(&c.1)
      }) {
        panic!("bad term cache: {c:?}")
      }
    }
    for (i, c) in ic.enum_iter() {
      if !with(infer_const, |ck| {
        ck.visit_term(&c.def);
        ck.visit_type(&c.ty)
      }) {
        panic!("bad infer const: ?{i:?} := {c:?}")
      }
    }
  }
}
