use crate::equate::Equalizer;
use crate::types::*;
use crate::unify::Unifier;
use crate::util::RetainMutFrom;
#[allow(unused)]
use crate::vprintln;
use crate::{
    set_verbose, stat, Assignment, CheckBound, ExpandPrivFunc, FixedVar, Inst, Inst0, InternConst,
    LocalContext, MizGlobal, OnVarMut, Visit, VisitMut, WithGlobalLocal,
};
use idx::{helpers::IdxOrBool, vec::IdxVec, Idx};
use itertools::Itertools;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::marker::PhantomData;

pub struct Checker<'a> {
    pub g: &'a MizGlobal,
    pub lc: &'a mut LocalContext,
    pub expansions: &'a [Definiens],
    pub equals: &'a BTreeMap<ConstrKind, Vec<EqualsDef>>,
    pub identify: &'a [IdentifyFunc],
    pub func_ids: &'a BTreeMap<ConstrKind, Vec<usize>>,
    pub reductions: &'a [Reduction],
    pub article: Article,
    pub pos: Position,
}

impl<'a> Checker<'a> {
    fn intern_const(&self) -> InternConst<'_> {
        InternConst::new(self.g, self.lc, self.equals, self.identify, self.func_ids)
    }

    pub fn justify(&mut self, premises: Vec<&'a Formula>) {
        if let Some(n) = self.g.cfg.first_verbose_line {
            set_verbose(self.pos.line >= n);
        }
        if self.g.cfg.skip_to_verbose && !crate::verbose() {
            return;
        }
        self.lc.term_cache.get_mut().open_scope();
        let infer_const = self.lc.infer_const.get_mut().len();
        let fixed_var = self.lc.fixed_var.len();

        if self.g.cfg.checker_inputs {
            eprintln!();
        }
        let mut check_f = Formula::mk_and_with(|conjs| {
            for f in premises {
                if self.g.cfg.checker_inputs {
                    eprintln!("input: {f:?}");
                }
                let mut f = f.clone();
                Expand {
                    g: self.g,
                    lc: self.lc,
                    expansions: self.expansions,
                }
                .expand(&mut f, true);
                if self.g.cfg.legacy_flex_handling {
                    ExpandLegacyFlex { depth: 0 }.visit_formula(&mut f);
                }
                // vprintln!("expand: {f:?}");
                f.distribute_quantifiers(&self.g.constrs, self.lc, 0);
                // vprintln!("distributed: {f:?}");
                f.append_conjuncts_to(conjs);
            }
        });
        if self.g.cfg.checker_header {
            eprintln!("refuting {:?}:{:?}:\n  {check_f:?}", self.article, self.pos);
        }

        OpenAsConst(self).open_quantifiers(&mut check_f, true);
        // vprintln!("opened {:?}:{:?}:\n  {check_f:?}", self.article, self.pos);

        check_f.visit(&mut self.intern_const());
        // vprintln!("interned {:?}:{:?}:\n  {check_f:?}", self.article, self.pos);

        let mut atoms = Atoms::default();
        let Dnf::Or(mut normal_form) = atoms.normalize(self.g, self.lc, check_f, true).unwrap()
        else {
            panic!("it is not true")
        };
        // vprintln!("normalized {:?}:{:?}:\n  {normal_form:?}", self.article, self.pos);

        self.process_is(&mut atoms, &mut normal_form).unwrap();
        // vprintln!("process_is {:?}:{:?}:\n  {normal_form:?}", self.article, self.pos);

        let mut err = false;
        for (i, f) in normal_form.into_iter().enumerate() {
            if self.g.cfg.checker_conjuncts {
                eprintln!(
                    "falsifying {:?}:{:?}.{i}: {:#?}",
                    self.article,
                    self.pos,
                    f.0.iter()
                        .map(|(&a, &val)| atoms.0[a].clone().maybe_neg(val))
                        .collect_vec()
                );
            }
            let sat = (|| {
                let mut eq = Equalizer::new(self);
                let res = eq.run(&atoms, &f)?;
                Unifier::new(eq, &res).run()
            })();
            // assert!(sat.is_err(), "failed to justify");
            if sat.is_err() {
                if self.g.cfg.checker_result {
                    eprintln!(
                        "proved {:?}:{:?}.{i}! {:#?}",
                        self.article,
                        self.pos,
                        f.0.iter()
                            .map(|(&a, &val)| atoms.0[a].clone().maybe_neg(val))
                            .collect_vec()
                    );
                }
            } else {
                err = true;
                if self.g.cfg.checker_result {
                    eprintln!(
                        "FAILED TO JUSTIFY {:?}:{:?}.{i}: {:#?}",
                        self.article,
                        self.pos,
                        f.0.iter()
                            .map(|(&a, &val)| atoms.0[a].clone().maybe_neg(val))
                            .collect_vec()
                    );
                }
                stat("failure");
                println!("failed to justify {:?}:{:?}.{i}", self.article, self.pos);
                if self.g.cfg.panic_on_fail {
                    panic!("failed to justify {:?}:{:?}.{i}", self.article, self.pos);
                }
                break;
            }
        }
        if !err {
            stat("success");
        }
        self.lc.fixed_var.0.truncate(fixed_var);
        self.lc.infer_const.get_mut().truncate(infer_const);
        self.lc.term_cache.get_mut().close_scope();
    }

    fn process_is(
        &self,
        atoms: &mut Atoms,
        normal_form: &mut Vec<Conjunct<AtomId, bool>>,
    ) -> Result<(), Overflow> {
        let (mut i, mut len) = (0, normal_form.len());
        while i < len {
            let conj = &normal_form[i];
            let mut is_ats = vec![];
            for (a, f) in atoms.0.enum_iter() {
                if let (Some(false), Formula::Is { .. }) = (conj.0.get(&a), f) {
                    is_ats.push(a)
                }
            }
            if is_ats.is_empty() {
                i += 1;
                continue;
            }
            let mut conj1 = conj.clone();
            for a in &is_ats {
                conj1.0.remove(a);
            }
            let mut inst = Dnf::single(conj1);
            for a in is_ats {
                let Formula::Is { term, ty } = &atoms.0[a].clone() else {
                    unreachable!()
                };
                let Attrs::Consistent(attrs) = ty.attrs.0.clone() else {
                    unreachable!()
                };
                let ty2 = Type {
                    kind: ty.kind,
                    attrs: Default::default(),
                    args: ty.args.clone(),
                };
                let f2 = Formula::Is {
                    term: term.clone(),
                    ty: Box::new(ty2),
                };
                let a2 = atoms.insert(self.g, self.lc, Cow::Owned(f2));
                let mut inst1 = vec![];
                for attr in attrs {
                    let c = &self.g.constrs.attribute[attr.nr];
                    let mut inst = Inst::new(&self.g.constrs, self.lc, &attr.args, 0);
                    let mut attrs = c.ty.attrs.1.visit_cloned(&mut inst);
                    let attr = MizAttr {
                        nr: attr.nr,
                        pos: !attr.pos,
                        args: attr.args.clone(),
                    };
                    attrs.insert(Some(&self.g.constrs), self.lc, attr);
                    attrs.visit(&mut self.intern_const());
                    let ty3 = Type {
                        kind: ty.kind,
                        attrs: (attrs.clone(), attrs),
                        args: ty.args.clone(),
                    };
                    let f3 = Formula::Is {
                        term: term.clone(),
                        ty: Box::new(ty3),
                    };
                    let a3 = atoms.insert(self.g, self.lc, Cow::Owned(f3));
                    Dnf::insert_and_absorb(&mut inst1, Conjunct::single(a3, true))?;
                }
                let mut inst2 = vec![Conjunct::single(a2, false)];
                if let TypeKind::Struct(n) = ty.kind {
                    let orig = term.get_type_uncached(self.g, self.lc);
                    if let Some(w) = ty.widening_of(self.g, self.lc, &orig).as_deref() {
                        let c = &self.g.constrs.struct_mode[n];
                        let mut inst3 = vec![];
                        for &sel in &*c.fields {
                            let tm2 = Term::mk_select(self.g, self.lc, sel, term, ty);
                            let f3 = Formula::Is {
                                term: Box::new(Term::mk_select(self.g, self.lc, sel, term, w)),
                                ty: Box::new(tm2.get_type_uncached(self.g, self.lc)),
                            };
                            let a3 = atoms.insert(self.g, self.lc, Cow::Owned(f3));
                            Dnf::insert_and_absorb(&mut inst3, Conjunct::single(a3, false))?;
                        }
                        Dnf::mk_and_core(&mut inst2, &inst3)?
                    }
                }
                let mut inst1 = Dnf::Or(inst1);
                inst1.mk_or(Dnf::Or(inst2))?;
                inst.mk_and(inst1)?;
            }
            let Dnf::Or(inst) = &mut inst else {
                unreachable!()
            };
            normal_form.remove(i);
            len -= 1;
            normal_form.append(inst);
        }
        Ok(())
    }

    pub fn justify_scheme(
        &mut self,
        sch: &Scheme,
        premises: Vec<&'a Formula>,
        thesis: &'a Formula,
    ) {
        if let Some(n) = self.g.cfg.first_verbose_line {
            set_verbose(self.pos.line >= n);
        }
        if self.g.cfg.skip_to_verbose && !crate::verbose() {
            return;
        }
        self.lc.term_cache.get_mut().open_scope();

        if self.g.cfg.checker_inputs {
            eprintln!();
            for f in &premises {
                eprintln!("input: {f:?}");
            }
            eprintln!("thesis: {thesis:?}");
            eprintln!("scheme: {sch:#?}");
        }
        assert!(premises.len() == sch.prems.len());
        let mut ctx = SchemeCtx {
            primary: &sch.sch_funcs,
            g: self.g,
            lc: self.lc,
            subst: Default::default(),
        };
        if ctx.eq_formula(&sch.thesis, thesis, true)
            && (sch.prems.iter().zip(premises.iter())).all(|(f1, f2)| ctx.eq_formula(f1, f2, true))
        {
            stat("success");
            if self.g.cfg.checker_result {
                eprintln!("proved sch {:?}:{:?}!", self.article, self.pos);
            }
        } else {
            stat("failure");
            if self.g.cfg.checker_result {
                eprintln!("FAILED TO JUSTIFY sch {:?}:{:?}", self.article, self.pos);
            }
            println!("failed to justify sch {:?}:{:?}", self.article, self.pos);
            if self.g.cfg.panic_on_fail {
                panic!("failed to justify sch {:?}:{:?}", self.article, self.pos);
            }
        }
        self.lc.term_cache.get_mut().close_scope();
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Unsat;
pub type OrUnsat<T> = Result<T, Unsat>;

struct Expand<'a> {
    g: &'a MizGlobal,
    lc: &'a mut LocalContext,
    expansions: &'a [Definiens],
}

impl Expand<'_> {
    fn expand(&mut self, f: &mut Formula, pos: bool) {
        match f {
            Formula::Neg { f: arg } => {
                self.expand(arg, !pos);
                if let Formula::Neg { f: f2 } = &mut **arg {
                    *f = std::mem::take(&mut **f2)
                }
            }
            Formula::And { args } => {
                *f = Formula::mk_and_with(|new_args| {
                    for mut f in std::mem::take(args) {
                        self.expand(&mut f, pos);
                        f.append_conjuncts_to(new_args);
                    }
                })
            }
            Formula::ForAll { dom, scope } if !pos => {
                self.lc.bound_var.push((**dom).clone());
                self.expand(scope, pos);
                self.lc.bound_var.0.pop().unwrap();
            }
            Formula::Pred { nr, args } => {
                let (n2, args2) = Formula::adjust_pred(*nr, args, Some(&self.g.constrs));
                let expansions = self.well_matched_expansions(ConstrKind::Pred(n2), args2);
                f.conjdisj_many(pos, expansions);
            }
            Formula::Attr { nr, args } => {
                let n2 = Formula::adjust_attr(*nr, args, Some(&self.g.constrs)).0;
                let expansions = self.well_matched_expansions(ConstrKind::Attr(n2), args);
                f.conjdisj_many(pos, expansions);
            }
            Formula::FlexAnd {
                nat,
                le,
                terms,
                scope,
            } => {
                if self.lc.bound_var.is_empty() {
                    *f = Formula::mk_and_with(move |conjs| {
                        {
                            let mut epf = ExpandPrivFunc(&self.g.constrs, self.lc);
                            let terms2 = (*terms).visit_cloned(&mut epf);
                            let scope2 = (*scope).visit_cloned(&mut epf);
                            let scope3 = if self.g.cfg.flex_expansion_bug {
                                let mut f = (*scope2).clone().mk_neg();
                                if f.conjuncts().len() > 1 {
                                    stat("flex expansion bug");
                                    f = f.conjuncts()[0].clone()
                                }
                                Box::new(f.mk_neg())
                            } else {
                                scope2.clone()
                            };
                            let nat = std::mem::take(nat);
                            let f2 = MizGlobal::expand_flex_and(
                                nat.clone(),
                                *le,
                                (*terms2).clone(),
                                scope3,
                                0,
                            );
                            let f1 = Formula::FlexAnd {
                                nat,
                                le: *le,
                                terms: terms2,
                                scope: scope2,
                            };
                            conjs.push(f1.maybe_neg(pos));
                            f2.maybe_neg(pos).append_conjuncts_to(conjs);
                        }
                        if pos {
                            self.expand_flex(terms, scope, conjs);
                        } else {
                            let f = Formula::mk_and_with(|conjs2| {
                                self.expand_flex(terms, scope, conjs2)
                            });
                            f.mk_neg().append_conjuncts_to(conjs);
                        }
                    })
                    .maybe_neg(pos);
                }
            }
            Formula::LegacyFlexAnd { .. }
            | Formula::SchPred { .. }
            | Formula::PrivPred { .. }
            | Formula::Is { .. }
            | Formula::ForAll { .. }
            | Formula::True => {}
        }
    }

    /// ExpandFlex
    fn expand_flex(&mut self, terms: &[Term; 2], scope: &Formula, conjs: &mut Vec<Formula>) {
        fn get_number<'a>(
            g: &MizGlobal,
            ic: &'a IdxVec<InferId, Assignment>,
            mut tm: &'a Term,
            zero: &mut Option<Term>,
        ) -> Option<u32> {
            loop {
                match tm {
                    Term::Functor { nr, args }
                        if Some(Term::adjust(*nr, args, Some(&g.constrs)).0)
                            == g.reqs.zero_number() =>
                    {
                        *zero = Some(tm.clone());
                        return Some(0);
                    }
                    &Term::Numeral(nr) => return Some(nr),
                    &Term::Infer(nr) => {
                        tm = &ic[nr].def;
                        continue;
                    }
                    _ => return None,
                }
            }
        }
        assert!(self.lc.bound_var.is_empty());
        let mut zero = None;
        let ic = self.lc.infer_const.borrow();
        let Some(left) = get_number(self.g, &ic, &terms[0], &mut zero) else {
            return;
        };
        let Some(right) = get_number(self.g, &ic, &terms[1], &mut zero) else {
            return;
        };
        if right.saturating_sub(left) <= 100 {
            for i in left..=right {
                let i = if i == 0 {
                    zero.take().unwrap()
                } else {
                    Term::Numeral(i)
                };
                scope
                    .visit_cloned(&mut Inst0(0, &i))
                    .append_conjuncts_to(conjs);
            }
        }
    }

    fn well_matched_expansions(&self, kind: ConstrKind, args: &[Term]) -> Vec<Formula> {
        let mut expansions = vec![];
        for exp in self.expansions.iter().rev() {
            let Formula::True = exp.assumptions else {
                continue;
            };
            let DefValue::Formula(body) = &exp.value else {
                continue;
            };
            let [] = *body.cases else { continue };
            let Some(subst) = exp.matches(self.g, self.lc, kind, args) else {
                continue;
            };
            let base = self.lc.bound_var.len() as u32;
            let mut result = body
                .otherwise
                .as_ref()
                .expect("no cases and no otherwise?")
                .clone();
            subst.inst_formula_mut(&self.g.constrs, self.lc, &mut result, base);
            expansions.push(result)
        }
        expansions
    }
}

struct ExpandLegacyFlex {
    depth: u32,
}
impl VisitMut for ExpandLegacyFlex {
    fn push_bound(&mut self, _: &mut Type) {
        self.depth += 1
    }
    fn pop_bound(&mut self, n: u32) {
        self.depth -= n
    }
    fn visit_formula(&mut self, f: &mut Formula) {
        if let Formula::FlexAnd {
            nat,
            le,
            terms,
            scope,
        } = f
        {
            *f = MizGlobal::into_legacy_flex_and(nat, *le, terms, scope, self.depth)
        }
        self.super_visit_formula(f)
    }
}

impl Formula {
    pub fn distribute_quantifiers(&mut self, ctx: &Constructors, lc: &LocalContext, depth: u32) {
        loop {
            match self {
                Formula::Neg { f: arg } => {
                    arg.distribute_quantifiers(ctx, lc, depth);
                    if let Formula::Neg { f: f2 } = &mut **arg {
                        *self = std::mem::take(&mut **f2)
                    }
                }
                Formula::And { args } => {
                    *self = Formula::mk_and_with(|conjs| {
                        for mut f in std::mem::take(args) {
                            f.distribute_quantifiers(ctx, lc, depth);
                            f.append_conjuncts_to(conjs)
                        }
                    })
                }
                Formula::ForAll { dom, scope } => {
                    ExpandPrivFunc(ctx, lc).visit_type(dom);
                    scope.distribute_quantifiers(ctx, lc, depth + 1);
                    if let Formula::And { args } = &mut **scope {
                        for f in args {
                            let mut nontrivial = false;
                            f.visit(&mut OnVarMut(|nr| nontrivial |= *nr == depth));
                            if nontrivial {
                                *f = Formula::ForAll {
                                    dom: dom.clone(),
                                    scope: Box::new(std::mem::take(f)),
                                }
                            } else {
                                f.visit(&mut OnVarMut(|nr| {
                                    if *nr > depth {
                                        *nr -= 1
                                    }
                                }))
                            }
                        }
                        *self = std::mem::take(scope);
                    }
                }
                Formula::PrivPred { value, .. } => {
                    *self = std::mem::take(value);
                    continue;
                }
                Formula::LegacyFlexAnd { .. }
                | Formula::SchPred { .. }
                | Formula::Pred { .. }
                | Formula::Attr { .. }
                | Formula::Is { .. }
                | Formula::FlexAnd { .. }
                | Formula::True => ExpandPrivFunc(ctx, lc).visit_formula(self),
            }
            break;
        }
    }
}

pub trait Open {
    fn mk_var(n: u32) -> Term;
    fn base(&self) -> u32;
    fn new_var(&mut self, ty: Type);

    /// * pos = true: RemoveIntQuantifier
    /// * pos = false: RemoveExtQuantifier
    fn open_quantifiers(&mut self, fmla: &mut Formula, pos: bool) {
        loop {
            match fmla {
                Formula::Neg { f } => {
                    self.open_quantifiers(f, !pos);
                    if let Formula::Neg { f } = &mut **f {
                        *fmla = std::mem::take(f);
                    }
                }
                Formula::And { args } => {
                    *fmla = Formula::mk_and_with(|conjs| {
                        for mut f in std::mem::take(args) {
                            self.open_quantifiers(&mut f, pos);
                            f.append_conjuncts_to(conjs)
                        }
                    })
                }
                Formula::ForAll { dom, scope } => {
                    if !pos {
                        let mut set_var = SetVar::new(self, 1);
                        self.new_var(std::mem::take(&mut **dom));
                        let mut f = std::mem::take(scope);
                        while let Formula::ForAll { mut dom, scope } = *f {
                            set_var.visit_type(&mut dom);
                            self.new_var(*dom);
                            set_var.depth += 1;
                            f = scope
                        }
                        set_var.visit_formula(&mut f);
                        *fmla = *f;
                        continue;
                    }
                }
                Formula::LegacyFlexAnd { .. }
                | Formula::SchPred { .. }
                | Formula::Pred { .. }
                | Formula::Attr { .. }
                | Formula::PrivPred { .. }
                | Formula::Is { .. }
                | Formula::FlexAnd { .. }
                | Formula::True => {}
            }
            return;
        }
    }
}

struct OpenAsConst<'a, 'b>(&'b mut Checker<'a>);

impl Open for OpenAsConst<'_, '_> {
    fn mk_var(n: u32) -> Term {
        Term::Const(ConstId(n))
    }
    fn base(&self) -> u32 {
        self.0.lc.fixed_var.len() as u32
    }
    fn new_var(&mut self, mut ty: Type) {
        ty.visit(&mut self.0.intern_const());
        self.0.lc.fixed_var.push(FixedVar { ty, def: None });
    }
}

struct SetVar<O: ?Sized> {
    depth: u32,
    base: u32,
    open: PhantomData<O>,
}

impl<O: Open + ?Sized> SetVar<O> {
    fn new(open: &O, depth: u32) -> SetVar<O> {
        SetVar {
            depth,
            base: open.base(),
            open: PhantomData,
        }
    }
}

impl<O: Open + ?Sized> VisitMut for SetVar<O> {
    fn visit_term(&mut self, tm: &mut Term) {
        match tm {
            Term::Bound(nr) => {
                if nr.0 >= self.depth {
                    nr.0 -= self.depth
                } else {
                    *tm = O::mk_var(self.base + nr.0)
                }
            }
            _ => self.super_visit_term(tm),
        }
    }
}

#[derive(Default, Debug)]
pub struct Atoms(pub IdxVec<AtomId, Formula>);

impl Atoms {
    pub fn find(&self, g: &MizGlobal, lc: &LocalContext, f: &Formula) -> Option<AtomId> {
        self.0
            .enum_iter()
            .find(|(_, atom)| g.eq(lc, f, atom))
            .map(|p| p.0)
    }

    pub fn insert(&mut self, g: &MizGlobal, lc: &LocalContext, f: Cow<'_, Formula>) -> AtomId {
        match self.find(g, lc, &f) {
            Some(i) => i,
            None => self.0.push(f.into_owned()),
        }
    }
}

/// A conjunction is a map from atoms to true or false, so
/// `{a: true, b: false, c: true}` represents `a /\ ~b /\ c`.
/// Invariant: the map is not empty when in a `DNF`.
#[derive(Clone, Default)]
pub struct Conjunct<K, V>(pub BTreeMap<K, V>);

impl<K, V> Conjunct<K, V> {
    pub const TRUE: Self = Self(BTreeMap::new());
}

impl<K: std::fmt::Debug, V: IdxOrBool> std::fmt::Debug for Conjunct<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if V::is_bool_ty() {
            let mut it = self.0.iter().map(|(a, &b)| (a, b.into_bool()));
            if let Some((a, b)) = it.next() {
                let b = b.into_bool();
                write!(f, "{}a{:?}", if b { "" } else { "¬" }, a)?;
                for (a, b) in it {
                    write!(f, " ∧ {}a{:?}", if b { "" } else { "¬" }, a)?;
                }
                Ok(())
            } else {
                write!(f, "false")
            }
        } else {
            let mut it = self.0.iter();
            if let Some((a, b)) = it.next() {
                write!(f, "v{:?} := e{:?}", a, b.into_usize())?;
                for (a, &b) in it {
                    write!(f, " ∧ v{:?} := e{:?}", a, b.into_usize())?;
                }
                Ok(())
            } else {
                write!(f, "false")
            }
        }
    }
}

impl<K: Ord + Clone, V: PartialEq + Clone> Conjunct<K, V>
where
    Self: std::fmt::Debug,
{
    /// InitSingle
    pub fn single(k: K, v: V) -> Self {
        Self(std::iter::once((k, v)).collect())
    }

    /// NatFunc.WeakerThan
    /// True if every atom in self is present in other with the same polarity.
    fn weaker_than(&self, other: &Self) -> bool {
        self.0.len() <= other.0.len() && self.0.iter().all(|(a, val2)| other.0.get(a) == Some(val2))
    }

    /// NatFunc.JoinAtom
    /// If it returns Err, then the conjunction is unsatisfiable
    /// and `self` is left in indeterminate state.
    fn mk_and(&mut self, other: &Self) -> Result<(), ()> {
        for (k, v) in &other.0 {
            if let Some(v1) = self.0.insert(k.clone(), v.clone()) {
                if v1 != *v {
                    return Err(());
                }
            }
        }
        Ok(())
    }
}

#[derive(Clone)]
pub enum Dnf<K, V> {
    /// The constant true is represented specially, although we could use `Or([[]])`
    /// to represent it (that is, the singleton of the empty map).
    True,
    /// A collection of conjunctions connected by OR.
    Or(Vec<Conjunct<K, V>>),
}

// We can handle a few orders of magnitude more than this before
// things really start to chug, but Mizar has this as a hard limit
// so if we go past it using MML inputs then something must have gone wrong
const MAX_DISJUNCTS: usize = 6000;

impl<K, V> std::fmt::Debug for Dnf<K, V>
where
    Conjunct<K, V>: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::True => write!(f, "True"),
            Self::Or(arg0) => f.debug_tuple("Or").field(arg0).finish(),
        }
    }
}

#[derive(Debug)]
pub struct Overflow;

impl<K: Ord + Clone, V: PartialEq + Clone> Dnf<K, V>
where
    Conjunct<K, V>: std::fmt::Debug,
{
    pub const FALSE: Dnf<K, V> = Dnf::Or(vec![]);

    pub fn is_false(&self) -> bool {
        matches!(self, Dnf::Or(conjs) if conjs.is_empty())
    }

    /// * pos = true: PreInstCollection.InitTop
    /// * pos = false: PreInstCollection.InitBottom
    pub fn mk_bool(pos: bool) -> Self {
        if pos {
            Self::True
        } else {
            Self::FALSE
        }
    }

    pub fn single(conj: Conjunct<K, V>) -> Self {
        if conj.0.is_empty() {
            Self::True
        } else {
            Self::Or(vec![conj])
        }
    }

    /// PreInstCollection.InsertAndAbsorb
    pub fn insert_and_absorb(
        this: &mut Vec<Conjunct<K, V>>,
        conj: Conjunct<K, V>,
    ) -> Result<bool, Overflow> {
        for (i, conj1) in this.iter_mut().enumerate() {
            if conj1.weaker_than(&conj) {
                return Ok(false);
            }
            if conj.weaker_than(conj1) {
                this.retain_mut_from(i + 1, |conj2| !conj.weaker_than(conj2));
                this[i] = conj;
                return Ok(true);
            }
        }
        this.push(conj);
        if this.len() > MAX_DISJUNCTS {
            return Err(Overflow);
        }
        Ok(true)
    }

    /// PreInstCollection.UnionWith
    pub fn mk_or(&mut self, other: Self) -> Result<(), Overflow> {
        let Dnf::Or(this) = self else { return Ok(()) };
        let Dnf::Or(other) = other else {
            *self = Dnf::True;
            return Ok(());
        };
        other.into_iter().try_for_each(|conj| {
            Self::insert_and_absorb(this, conj)?;
            Ok(())
        })
    }

    /// PreInstCollection.UnionWith
    #[inline]
    pub fn mk_or_else(
        &mut self,
        other: impl FnOnce() -> Result<Self, Overflow>,
    ) -> Result<(), Overflow> {
        if matches!(self, Dnf::Or(_)) {
            self.mk_or(other()?)?
        }
        Ok(())
    }

    pub fn mk_and_single(&mut self, k: K, v: V) {
        match self {
            Dnf::True => *self = Self::Or(vec![Conjunct::single(k, v)]),
            Dnf::Or(conjs) => conjs.iter_mut().for_each(|conj| {
                conj.0.insert(k.clone(), v.clone());
            }),
        }
    }

    fn mk_and_core(
        this: &mut Vec<Conjunct<K, V>>,
        other: &[Conjunct<K, V>],
    ) -> Result<(), Overflow> {
        if let [conj2] = other {
            this.retain_mut(|conj1| conj1.mk_and(conj2).is_ok())
        } else {
            let this1 = std::mem::take(this);
            for conj2 in other {
                for conj1 in &this1 {
                    let mut conj = conj1.clone();
                    if let Ok(()) = conj.mk_and(conj2) {
                        Self::insert_and_absorb(this, conj)?;
                        if this.len() > MAX_DISJUNCTS {
                            return Err(Overflow);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// PreInstCollection.JoinWith
    pub fn mk_and(&mut self, other: Self) -> Result<(), Overflow> {
        match self {
            Dnf::True => *self = other,
            Dnf::Or(this) => match other {
                Dnf::True => {}
                _ if this.is_empty() => {}
                Dnf::Or(other) if other.is_empty() => this.clear(),
                Dnf::Or(other) => Self::mk_and_core(this, &other)?,
            },
        }
        Ok(())
    }

    pub fn mk_and_then(
        &mut self,
        other: impl FnOnce() -> Result<Self, Overflow>,
    ) -> Result<(), Overflow> {
        if !self.is_false() {
            self.mk_and(other()?)?
        }
        Ok(())
    }

    /// PreInstCollection.JoinInstList
    /// Constructs the AND of a set of (nontrivial) DNF expressions.
    pub fn and_many(mut dnfs: Vec<Vec<Conjunct<K, V>>>) -> Result<Self, Overflow> {
        'restart: loop {
            // We sort the DNFs by length to prioritize a small accumulator
            dnfs.sort_unstable_by_key(|dnf| !dnf.len());
            let Some(mut this) = dnfs.pop() else {
                return Ok(Dnf::True);
            };
            while !dnfs.is_empty() {
                if let [conj1] = &*this {
                    // If 'this' is a single conjunct, we use it to reduce all future DNFs
                    for dnf in &mut dnfs {
                        dnf.retain_mut(|conj2| conj2.mk_and(conj1).is_ok())
                    }
                    continue 'restart;
                } else {
                    Self::mk_and_core(&mut this, &dnfs.pop().unwrap())?
                }
            }
            return Ok(Dnf::Or(this));
        }
    }
}

impl Atoms {
    /// * pos = true: PreInstCollection.NormalizeAsTrue
    /// * pos = false: PreInstCollection.NormalizeAsFalse
    pub fn normalize(
        &mut self,
        g: &MizGlobal,
        lc: &LocalContext,
        f: Formula,
        pos: bool,
    ) -> Result<Dnf<AtomId, bool>, Overflow> {
        match f {
            Formula::Neg { f } => self.normalize(g, lc, *f, !pos),
            Formula::And { args } => {
                let mut res = Dnf::mk_bool(pos);
                if pos {
                    args.into_iter()
                        .try_for_each(|f| res.mk_and_then(|| self.normalize(g, lc, f, pos)))?
                } else {
                    args.into_iter()
                        .try_for_each(|f| res.mk_or_else(|| self.normalize(g, lc, f, pos)))?;
                }
                Ok(res)
            }
            Formula::True => Ok(Dnf::mk_bool(pos)),
            _ => {
                let a = self.insert(g, lc, Cow::Owned(f));
                Ok(Dnf::Or(vec![Conjunct::single(a, pos)]))
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum PredKind {
    Pred(PredId),
    SchPred(SchPredId),
    PrivPred(PrivPredId),
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum FuncKind {
    Func(FuncId),
    SchFunc(SchFuncId),
    PrivFunc(PrivFuncId),
}

#[derive(Clone, Default)]
struct SchemeSubst {
    cnst: IdxVec<SchFuncId, Option<Term>>,
    func: IdxVec<SchFuncId, Option<FuncKind>>,
    pred: IdxVec<SchPredId, Option<(bool, PredKind)>>,
}

struct SchemeCtx<'a> {
    g: &'a MizGlobal,
    lc: &'a mut LocalContext,
    primary: &'a [Type],
    subst: SchemeSubst,
}
impl WithGlobalLocal for SchemeCtx<'_> {
    fn global(&self) -> &MizGlobal {
        self.g
    }
    fn local(&self) -> &LocalContext {
        self.lc
    }
}

impl<'a> SchemeCtx<'a> {
    fn observing(&mut self, f: impl FnOnce(&mut Self) -> bool) -> bool {
        let orig = self.subst.clone();
        f(self) || {
            self.subst = orig;
            false
        }
    }

    fn eq_formula(&mut self, f1: &Formula, f2: &Formula, pos: bool) -> bool {
        use Formula::*;
        // vprintln!("sch {pos}. {f1:?} <> {f2:?}");
        let res = match (f1, f2) {
            (Neg { f: f1 }, _) => self.eq_formula(f1, f2, !pos),
            (_, Neg { f: f2 }) => self.eq_formula(f1, f2, !pos),
            (
                SchPred {
                    nr: n1,
                    args: args1,
                },
                _,
            ) => {
                let (kind, args2) = match f2 {
                    Pred { nr, args } => (PredKind::Pred(*nr), args),
                    SchPred { nr, args } => (PredKind::SchPred(*nr), args),
                    PrivPred { nr, args, .. } => (PredKind::PrivPred(*nr), args),
                    _ => return false,
                };
                match self.subst.pred.get_mut_extending(*n1) {
                    Some(asgn) if *asgn == (pos, kind) => {}
                    Some(_) => return false,
                    x @ None => *x = Some((pos, kind)),
                }
                self.eq_terms(args1, args2)
            }
            (True, True) if pos => true,
            (Is { term: t1, ty: ty1 }, Is { term: t2, ty: ty2 }) if pos => {
                self.eq_term(t1, t2) && self.eq_type(ty1, ty2)
            }
            (And { args: args1 }, And { args: args2 }) if pos => self.eq_formulas(args1, args2),
            (
                PrivPred {
                    nr: PrivPredId(n1),
                    args: args1,
                    ..
                },
                PrivPred {
                    nr: PrivPredId(n2),
                    args: args2,
                    ..
                },
            ) if pos => n1 == n2 && self.eq_terms(args1, args2),
            (
                Attr {
                    nr: n1,
                    args: args1,
                },
                Attr {
                    nr: n2,
                    args: args2,
                },
            ) if pos => {
                let (n1, args1) = Formula::adjust_attr(*n1, args1, Some(&self.g.constrs));
                let (n2, args2) = Formula::adjust_attr(*n2, args2, Some(&self.g.constrs));
                n1 == n2 && self.eq_terms(args1, args2)
            }
            (
                Pred {
                    nr: n1,
                    args: args1,
                },
                Pred {
                    nr: n2,
                    args: args2,
                },
            ) if pos => {
                let (n1, args1) = Formula::adjust_pred(*n1, args1, Some(&self.g.constrs));
                let (n2, args2) = Formula::adjust_pred(*n2, args2, Some(&self.g.constrs));
                n1 == n2 && self.eq_terms(args1, args2)
            }
            (
                ForAll {
                    dom: dom1,
                    scope: sc1,
                },
                ForAll {
                    dom: dom2,
                    scope: sc2,
                },
            ) if pos => {
                self.eq_type(dom1, dom2) && {
                    self.lc.term_cache.get_mut().open_scope();
                    self.lc.bound_var.0.push((**dom2).clone());
                    let r = self.eq_formula(sc1, sc2, true);
                    self.lc.bound_var.0.pop();
                    self.lc.term_cache.get_mut().close_scope();
                    r
                }
            }
            #[allow(clippy::explicit_auto_deref)]
            (
                FlexAnd {
                    nat,
                    terms: t1,
                    scope: sc1,
                    ..
                },
                FlexAnd {
                    terms: t2,
                    scope: sc2,
                    ..
                },
            ) if pos => {
                self.eq_terms(&**t1, &**t2) && {
                    self.lc.term_cache.get_mut().open_scope();
                    self.lc.bound_var.0.push((**nat).clone());
                    let r = self.eq_formula(sc1, sc2, true);
                    self.lc.bound_var.0.pop();
                    self.lc.term_cache.get_mut().close_scope();
                    r
                }
            }
            (_, PrivPred { value, .. }) => self.eq_formula(f1, value, pos),
            _ => false,
        };
        // vprintln!("sch {pos}. {f1:?} <> {f2:?} -> {res}");
        res
    }

    fn eq_terms(&mut self, t1: &[Term], t2: &[Term]) -> bool {
        t1.len() == t2.len() && t1.iter().zip(t2).all(|(t1, t2)| self.eq_term(t1, t2))
    }

    fn wider(&mut self, tgt: &Type, src: &Type) -> bool {
        // vprintln!("sch {tgt:?}\n  wider than {src:?}");
        self.is_subset_of(&tgt.attrs.0, &src.attrs.1, |this, a1, a2| {
            this.eq_attr(a1, a2)
        }) && (self.observing(|this| this.eq_radices(tgt, src))
            || tgt.kind != src.kind
                && if let TypeKind::Mode(n1) = tgt.kind {
                    let mut src = CowBox::Borrowed(src);
                    loop {
                        let Some(w) = src.widening(self.g, self.lc) else {
                            return false;
                        };
                        let Some(w) = tgt.widening_of(self.g, self.lc, &w) else {
                            return false;
                        };
                        if self.observing(|this| this.eq_radices(tgt, &w)) {
                            return true;
                        }
                        if self.g.constrs.mode[n1].redefines.is_some() {
                            return false;
                        }
                        src = CowBox::Owned(w.to_owned());
                    }
                } else {
                    let Some(src) = tgt.widening_of(self.g, self.lc, src) else {
                        return false;
                    };
                    tgt.kind == src.kind && self.eq_radices(tgt, &src)
                })
    }

    fn eq_term(&mut self, t1: &Term, t2: &Term) -> bool {
        use Term::*;
        // vprintln!("sch {t1:?} <> {t2:?}");
        let res = match (t1, t2) {
            (
                SchFunc {
                    nr: n1,
                    args: args1,
                },
                _,
            ) => {
                if args1.is_empty() {
                    let depth = self.lc.bound_var.len() as u32;
                    if CheckBound::get(0..depth, |cb| cb.visit_term(t2)) {
                        return false;
                    }
                    let t2 = t2.visit_cloned(&mut OnVarMut(|n| *n -= depth));
                    if let Some(tm) = self.subst.cnst.get_mut_extending(*n1) {
                        self.g.eq(self.lc, &t2, tm)
                    } else if self.wider(
                        &self.primary[Idx::into_usize(*n1)],
                        &t2.get_type(self.g, self.lc, false),
                    ) {
                        // vprintln!("assign S{n1:?}() := {t2:?}");
                        self.subst.cnst[*n1] = Some(t2);
                        true
                    } else {
                        false
                    }
                } else {
                    let (kind, args2) = match t2 {
                        Functor { nr, args } => (FuncKind::Func(*nr), args),
                        SchFunc { nr, args } => (FuncKind::SchFunc(*nr), args),
                        PrivFunc { nr, args, .. } => (FuncKind::PrivFunc(*nr), args),
                        Infer(n2) => {
                            let tm = (self.lc.infer_const.get_mut()[*n2].def).visit_cloned(
                                &mut OnVarMut(|v| *v += self.lc.bound_var.len() as u32),
                            );
                            return self.eq_term(t1, &tm);
                        }
                        _ => return false,
                    };
                    match self.subst.func.get_mut_extending(*n1) {
                        Some(asgn) if *asgn == kind => {}
                        Some(_) => return false,
                        x @ None => *x = Some(kind),
                    }
                    self.eq_terms(args1, args2)
                }
            }
            (Locus(LocusId(n1)), Locus(LocusId(n2))) => n1 == n2,
            (Bound(BoundId(n1)), Bound(BoundId(n2)))
            | (Const(ConstId(n1)), Const(ConstId(n2)))
            | (FreeVar(FVarId(n1)), FreeVar(FVarId(n2)))
            | (Numeral(n1), Numeral(n2)) => n1 == n2,
            (Infer(InferId(n1)), Infer(InferId(n2))) if n1 == n2 => true,
            (
                Functor {
                    nr: n1,
                    args: args1,
                },
                Functor {
                    nr: n2,
                    args: args2,
                },
            ) => {
                let (n1, args1) = Term::adjust(*n1, args1, Some(&self.g.constrs));
                let (n2, args2) = Term::adjust(*n2, args2, Some(&self.g.constrs));
                n1 == n2 && self.eq_terms(args1, args2)
            }
            (
                Aggregate {
                    nr: AggrId(n1),
                    args: args1,
                },
                Aggregate {
                    nr: AggrId(n2),
                    args: args2,
                },
            )
            | (
                Selector {
                    nr: SelId(n1),
                    args: args1,
                },
                Selector {
                    nr: SelId(n2),
                    args: args2,
                },
            ) => n1 == n2 && self.eq_terms(args1, args2),
            (The { ty: ty1 }, The { ty: ty2 }) => self.eq_type(ty1, ty2),
            (
                Fraenkel {
                    args: args1,
                    scope: sc1,
                    compr: c1,
                },
                Fraenkel {
                    args: args2,
                    scope: sc2,
                    compr: c2,
                },
            ) => {
                args1.len() == args2.len() && {
                    self.lc.term_cache.get_mut().open_scope();
                    let n = self.lc.bound_var.len();
                    let r = args1.iter().zip(&**args2).all(|(ty1, ty2)| {
                        let r = self.eq_type(ty1, ty2);
                        self.lc.bound_var.0.push(ty2.clone());
                        r
                    }) && self.eq_term(sc1, sc2)
                        && self.eq_formula(c1, c2, true);
                    self.lc.bound_var.0.truncate(n);
                    self.lc.term_cache.get_mut().close_scope();
                    r
                }
            }
            (_, &Infer(nr)) => {
                let t = (self.lc.infer_const.get_mut()[nr].def)
                    .visit_cloned(&mut OnVarMut(|v| *v += self.lc.bound_var.len() as u32));
                self.eq_term(t1, &t)
            }
            (&Infer(nr), _) => {
                let t = (self.lc.infer_const.get_mut()[nr].def)
                    .visit_cloned(&mut OnVarMut(|v| *v += self.lc.bound_var.len() as u32));
                self.eq_term(&t, t2)
            }
            _ => false,
        };
        // vprintln!("sch {t1:?} <> {t2:?} -> {res}");
        res
    }

    fn eq_radices(&mut self, ty1: &Type, ty2: &Type) -> bool {
        match (ty1.kind, ty2.kind) {
            (TypeKind::Mode(n1), TypeKind::Mode(n2)) => {
                let (n1, args1) = Type::adjust(n1, &ty1.args, &self.g.constrs);
                let (n2, args2) = Type::adjust(n2, &ty2.args, &self.g.constrs);
                n1 == n2 && self.eq_terms(args1, args2)
            }
            (TypeKind::Struct(n1), TypeKind::Struct(n2)) => {
                n1 == n2 && self.eq_terms(&ty1.args, &ty2.args)
            }
            _ => false,
        }
    }

    fn is_subset_of(
        &mut self,
        attrs1: &Attrs,
        attrs2: &Attrs,
        mut eq: impl FnMut(&mut Self, &MizAttr, &MizAttr) -> bool,
    ) -> bool {
        let res = match (attrs1, attrs2) {
            (Attrs::Inconsistent, Attrs::Consistent(_)) => false,
            (Attrs::Consistent(this), Attrs::Consistent(other)) => {
                other.len() >= this.len()
                    && this
                        .iter()
                        .all(|i| other.iter().any(|j| self.observing(|this| eq(this, i, j))))
            }
            (_, Attrs::Inconsistent) => true,
        };
        // vprintln!("sch {attrs1:?} c= {attrs2:?} -> {res}");
        res
    }

    fn eq_type(&mut self, ty1: &Type, ty2: &Type) -> bool {
        self.is_subset_of(&ty1.attrs.0, &ty2.attrs.1, |this, a1, a2| {
            this.eq_attr(a1, a2)
        }) && self.is_subset_of(&ty2.attrs.0, &ty1.attrs.1, |this, a2, a1| {
            this.eq_attr(a1, a2)
        }) && self.eq_radices(ty1, ty2)
    }

    fn eq_attr(&mut self, a1: &MizAttr, a2: &MizAttr) -> bool {
        let (n1, args1) = a1.adjust(Some(&self.g.constrs));
        let (n2, args2) = a2.adjust(Some(&self.g.constrs));
        n1 == n2 && a1.pos == a2.pos && self.eq_terms(args1, args2)
    }

    fn eq_formulas(&mut self, args1: &[Formula], args2: &[Formula]) -> bool {
        args1.len() == args2.len()
            && args1
                .iter()
                .zip(args2)
                .all(|(f1, f2)| self.eq_formula(f1, f2, true))
    }
}
