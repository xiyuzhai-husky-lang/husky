use crate::bignum::Complex;
use crate::format::Formatter;
use crate::types::*;
use crate::*;
use enum_map::EnumMap;
use idx::vec::sorted::SortedIdxVec;
use itertools::EitherOrBoth;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fs::File;
use std::io;
use std::ops::Range;
use std::path::PathBuf;

#[derive(Debug)]
pub struct MizGlobal {
    pub cfg: Config,
    pub reqs: RequirementIndexes,
    pub constrs: Constructors,
    pub clusters: Clusters,
    /// This is the type that nonzero numerals have.
    /// It is `set` until the NUMERALS requirement is read,
    /// and then it changes to `Element of omega`
    pub numeral_type: Type,
}

impl MizGlobal {
    /// TypReachable(fWider = wider, fNarrower = narrower)
    fn type_reachable(&self, wider: &Type, narrower: &Type) -> bool {
        // vprintln!("TypReachable {wider:?} -> {narrower:?}");
        if let (TypeKind::Mode(_), TypeKind::Mode(w_mode)) = (narrower.kind, wider.kind) {
            if w_mode == ModeId::ANY {
                return true;
            }
            let mode = self.constrs.mode[w_mode].redefines.unwrap_or(w_mode);
            let mut narrower = narrower;
            while let TypeKind::Mode(n_mode) = narrower.kind {
                if n_mode < mode {
                    return false;
                }
                if n_mode == mode {
                    return true;
                }
                let cnst = &self.constrs.mode[n_mode];
                if cnst.redefines == Some(mode) {
                    return true;
                }
                narrower = &cnst.ty;
            }
            false
        } else {
            true
        }
    }

    fn matching_func_clusters(&self, t: &Term) -> &[usize] {
        let fcs =
            &self.clusters.functor.sorted[self.clusters.functor.sorted.partition_point(|&i| {
                FunctorCluster::cmp_term(&self.clusters.functor.vec[i].term, &self.constrs, t)
                    == Ordering::Less
            })..];
        &fcs[..fcs.partition_point(|&i| {
            FunctorCluster::cmp_term(&self.clusters.functor.vec[i].term, &self.constrs, t)
                != Ordering::Greater
        })]
    }

    pub fn round_up_clusters(&mut self, lc: &mut LocalContext) {
        for k in 0..self.clusters.registered.len() {
            let mut cl = &self.clusters.registered[k];
            for l in 0..cl.primary.len() {
                lc.load_locus_tys(&cl.primary);
                let mut attrs = cl.primary[l].attrs.1.clone();
                attrs.round_up_with(self, lc, &cl.primary[l], false);
                self.clusters.registered[k].primary[l].attrs.1 = attrs;
                cl = &self.clusters.registered[k];
                lc.unload_locus_tys();
            }
            lc.load_locus_tys(&cl.primary);
            let mut attrs = cl.consequent.1.clone();
            attrs.round_up_with(self, lc, &cl.ty, false);
            self.clusters.registered[k].consequent.1 = attrs;
            lc.unload_locus_tys();
        }

        for k in 0..self.clusters.functor.len() {
            let cl = &self.clusters.functor[k];
            lc.load_locus_tys(&cl.primary);
            let mut attrs = cl.consequent.1.clone();
            let ty = match &cl.ty {
                None => cl.term.round_up_type(self, lc, false),
                Some(ty) => CowBox::Borrowed(&**ty),
            };
            attrs.enlarge_by(&self.constrs, lc, &ty.attrs.1);
            attrs.round_up_with(self, lc, &ty, false);
            self.clusters.functor[k].consequent.1 = attrs;
            lc.unload_locus_tys();
        }

        macro_rules! round_up_constrs {
      ($($x:ident),*) => {$(
        for k in 0..self.constrs.$x.0.len() {
          self.constrs.$x.0[k].ty.attrs.1 = self.constrs.$x.0[k].round_up(self, lc);
        }
      )*};
    }
        // TODO: why not round up aggregate too? (copied from original source)
        round_up_constrs! { mode, functor, selector };
        lc.term_cache.get_mut().terms = vec![];
    }

    // RoundUpItem
    pub fn round_up_term_cache(&self, lc: &mut LocalContext) {
        let mut n = 0;
        while let Some((tm, ty, _)) = lc.term_cache.get_mut().terms.get(n) {
            let fcs = self.matching_func_clusters(tm);
            if !fcs.is_empty() {
                let (tm, ty) = (tm.clone(), ty.clone());
                let mut attrs = ty.attrs.1.clone();
                for &i in fcs {
                    let fc = &self.clusters.functor.vec[i];
                    fc.round_up_with(self, lc, &tm, &ty, &mut attrs, false);
                }
                lc.term_cache.get_mut().terms[n].1.attrs.1 = attrs;
            }
            n += 1;
        }
    }

    /// MotherStructNr
    fn parent_struct(&self, sel: SelId) -> StructId {
        self.constrs
            .struct_mode
            .enum_iter()
            .find(|c| c.1.fields.contains(&sel))
            .unwrap()
            .0
    }

    pub fn expand_flex_and(
        nat: Box<Type>,
        le: PredId,
        [t1, t2]: [Term; 2],
        scope: Box<Formula>,
        depth: u32,
    ) -> Formula {
        let f = Formula::mk_and_with(|conjs| {
            conjs.push(Formula::Pred {
                nr: le,
                args: Box::new([t1, Term::Bound(BoundId(depth))]),
            });
            conjs.push(Formula::Pred {
                nr: le,
                args: Box::new([Term::Bound(BoundId(depth)), t2]),
            });
            scope.mk_neg().append_conjuncts_to(conjs);
        });
        Formula::ForAll {
            dom: nat,
            scope: Box::new(f.mk_neg()),
        }
    }

    pub fn into_legacy_flex_and(
        nat: &mut Box<Type>,
        le: PredId,
        terms: &mut Box<[Term; 2]>,
        scope: &mut Box<Formula>,
        depth: u32,
    ) -> Formula {
        let orig1 = (**scope).visit_cloned(&mut Inst0(depth, &terms[0]));
        let orig2 = (**scope).visit_cloned(&mut Inst0(depth, &terms[1]));
        let expansion = Box::new(Self::expand_flex_and(
            std::mem::take(nat),
            le,
            (**terms).clone(),
            std::mem::take(scope),
            depth,
        ));
        Formula::LegacyFlexAnd {
            orig: Box::new([orig1, orig2]),
            terms: std::mem::take(terms),
            expansion,
        }
    }
}

pub struct MizRoundUpTypes<'a> {
    g: &'a MizGlobal,
    lc: &'a mut LocalContext,
}

impl MizRoundUpTypes<'_> {
    pub fn with(g: &MizGlobal, lc: &mut LocalContext, f: impl FnOnce(&mut MizRoundUpTypes<'_>)) {
        lc.term_cache.get_mut().open_scope();
        f(&mut MizRoundUpTypes { g, lc });
        lc.term_cache.get_mut().close_scope();
    }
}

impl VisitMut for MizRoundUpTypes<'_> {
    fn push_bound(&mut self, ty: &mut Type) {
        self.lc.bound_var.push(ty.clone());
    }
    fn pop_bound(&mut self, n: u32) {
        self.lc
            .bound_var
            .0
            .truncate(self.lc.bound_var.0.len() - n as usize);
        self.lc.clear_term_cache()
    }
    fn visit_type(&mut self, ty: &mut Type) {
        self.visit_attrs(&mut ty.attrs.0);
        self.visit_terms(&mut ty.args);
        ty.attrs.1 = ty.attrs.0.clone();
        if let TypeKind::Mode(n) = ty.kind {
            let mut inst = Inst::new(&self.g.constrs, self.lc, &ty.args, 0);
            let new = self.g.constrs.mode[n].ty.attrs.1.visit_cloned(&mut inst);
            ty.attrs.1.enlarge_by(&self.g.constrs, self.lc, &new);
        }
        ty.round_up_with_self(self.g, self.lc, false)
    }

    fn visit_flex_and(
        &mut self,
        nat: &mut Type,
        _le: &mut PredId,
        [tm_l, tm_r]: &mut [Term; 2],
        scope: &mut Formula,
    ) {
        self.visit_type(nat);
        self.visit_term(tm_l);
        self.visit_term(tm_r);
        self.push_bound(nat);
        self.visit_formula(scope);
        self.pop_bound(1)
    }

    fn visit_push_locus_tys(&mut self, tys: &mut [Type]) {
        for ty in tys {
            self.visit_type(ty);
            self.lc.locus_ty.push(ty.clone());
        }
    }

    fn pop_locus_tys(&mut self, n: usize) {
        assert!(self.lc.locus_ty.len() == n);
        self.lc.locus_ty.0.clear();
        self.lc.clear_term_cache()
    }

    fn visit_push_sch_func_tys(&mut self, tys: &mut [Type]) {
        for ty in tys {
            self.visit_type(ty);
            self.lc.sch_func_ty.push(ty.clone());
        }
    }
    fn pop_sch_func_tys(&mut self, n: usize) {
        assert!(self.lc.sch_func_ty.len() == n);
        self.lc.sch_func_ty.0.clear()
    }
}

fn sorted_subset<T: Ord>(a: &[T], b: &[T]) -> bool {
    if b.len() < a.len() {
        return false;
    }
    let mut it = b.iter();
    'a: for i in a {
        for j in it.by_ref() {
            match j.cmp(i) {
                Ordering::Less => {}
                Ordering::Equal => continue 'a,
                Ordering::Greater => break,
            }
        }
        return false;
    }
    true
}

impl MizAttr {
    pub fn adjusted_nr(&self, ctx: &Constructors) -> AttrId {
        ctx.attribute[self.nr].c.redefines.unwrap_or(self.nr)
    }

    pub fn adjust(&self, ctx: Option<&Constructors>) -> (AttrId, &[Term]) {
        let Some(ctx) = ctx else {
            return (self.nr, &self.args);
        };
        let c = &ctx.attribute[self.nr].c;
        let Some(nr) = c.redefines else {
            return (self.nr, &self.args);
        };
        (nr, &self.args[c.superfluous as usize..])
    }

    pub fn cmp_abs(
        &self,
        ctx: Option<&Constructors>,
        lc: Option<&LocalContext>,
        other: &Self,
        style: CmpStyle,
    ) -> Ordering {
        let (n1, args1) = self.adjust(ctx);
        let (n2, args2) = other.adjust(ctx);
        n1.cmp(&n2)
            .then_with(|| Term::cmp_list(ctx, lc, args1, args2, style))
    }

    pub fn cmp(
        &self,
        ctx: Option<&Constructors>,
        lc: Option<&LocalContext>,
        other: &Self,
        style: CmpStyle,
    ) -> Ordering {
        self.cmp_abs(ctx, lc, other, style)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

#[derive(Copy, Clone)]
pub enum CmpStyle {
    Strict,
    Attr,
    Red,
    Alt,
}

impl Term {
    fn locus_list(n: usize) -> Box<[Term]> {
        (0..n).map(|i| Term::Locus(Idx::from_usize(i))).collect()
    }

    pub fn adjust<'a>(
        n: FuncId,
        args: &'a [Term],
        ctx: Option<&Constructors>,
    ) -> (FuncId, &'a [Term]) {
        let Some(ctx) = ctx else { return (n, args) };
        let c = &ctx.functor[n].c;
        let Some(nr) = c.redefines else {
            return (n, args);
        };
        (nr, &args[c.superfluous as usize..])
    }

    pub fn adjusted_nr(nr: FuncId, ctx: &Constructors) -> FuncId {
        ctx.functor[nr].c.redefines.unwrap_or(nr)
    }

    fn skip_infer<R>(&self, lc: Option<&LocalContext>, f: impl FnOnce(&Self) -> R) -> R {
        match *self {
            Term::Infer(n) => f(&lc.unwrap().infer_const.borrow()[n].def),
            _ => f(self),
        }
    }

    pub fn skip_priv_func<'a>(&'a self, lc: Option<&'a LocalContext>) -> &'a Self {
        let mut t = self;
        loop {
            match t {
                Term::PrivFunc { value, .. } => t = value,
                &Term::EqMark(m) if lc.is_some() => t = &lc.unwrap().marks[m].0,
                _ => return t,
            }
        }
    }

    /// SizeOfTrm(fTrm:TrmPtr)
    fn size(&self) -> u32 {
        match self {
            Term::SchFunc { args, .. }
            | Term::Aggregate { args, .. }
            | Term::Functor { args, .. }
            | Term::Selector { args, .. } => args.iter().map(|t| t.size()).sum::<u32>() + 1,
            Term::PrivFunc { value, .. } => value.size(),
            Term::The { ty } => ty.size(),
            // Term::Fraenkel { .. } => {} // FIXME?
            _ => 1,
        }
    }

    /// * CmpStyle::Strict: CompTrms(fTrm1 = self, fTrm2 = other)
    /// * CmpStyle::Red: CompRdTrms(fTrm1 = self, fTrm2 = other)
    /// * CmpStyle::Alt: CompareTrms(fTrm1 = self, fTrm2 = other)
    pub fn cmp(
        &self,
        ctx: Option<&Constructors>,
        lc: Option<&LocalContext>,
        other: &Term,
        style: CmpStyle,
    ) -> Ordering {
        use Term::*;
        fn cmp(
            ctx: Option<&Constructors>,
            lc: Option<&LocalContext>,
            this: &Term,
            other: &Term,
            style: CmpStyle,
        ) -> Ordering {
            Term::discr(this)
                .cmp(&Term::discr(other))
                .then_with(|| match (this, other) {
                    (Locus(LocusId(n1)), Locus(LocusId(n2))) => n1.cmp(n2),
                    (Bound(BoundId(n1)), Bound(BoundId(n2)))
                    | (Const(ConstId(n1)), Const(ConstId(n2)))
                    | (Infer(InferId(n1)), Infer(InferId(n2)))
                    | (FreeVar(FVarId(n1)), FreeVar(FVarId(n2)))
                    | (EqClass(EqClassId(n1)), EqClass(EqClassId(n2)))
                    | (EqMark(EqMarkId(n1)), EqMark(EqMarkId(n2)))
                    | (Numeral(n1), Numeral(n2)) => n1.cmp(n2),
                    (
                        Functor {
                            nr: n1,
                            args: args1,
                        },
                        Functor {
                            nr: n2,
                            args: args2,
                        },
                    ) => match style {
                        CmpStyle::Strict | CmpStyle::Attr | CmpStyle::Alt => n1
                            .cmp(n2)
                            .then_with(|| Term::cmp_list(ctx, lc, args1, args2, style)),
                        CmpStyle::Red => {
                            let (n1, args1) = Term::adjust(*n1, args1, ctx);
                            let (n2, args2) = Term::adjust(*n2, args2, ctx);
                            n1.cmp(&n2)
                                .then_with(|| Term::cmp_list(ctx, lc, args1, args2, style))
                        }
                    },
                    (
                        SchFunc {
                            nr: SchFuncId(n1),
                            args: args1,
                        },
                        SchFunc {
                            nr: SchFuncId(n2),
                            args: args2,
                        },
                    )
                    | (
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
                        PrivFunc {
                            nr: PrivFuncId(n1),
                            args: args1,
                            ..
                        },
                        PrivFunc {
                            nr: PrivFuncId(n2),
                            args: args2,
                            ..
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
                    ) => n1
                        .cmp(n2)
                        .then_with(|| Term::cmp_list(ctx, lc, args1, args2, style)),
                    (The { ty: ty1 }, The { ty: ty2 }) => ty1.cmp(ctx, lc, ty2, style),
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
                    ) => sc1.cmp(ctx, lc, sc2, style).then_with(|| {
                        c1.cmp(ctx, lc, c2, style).then_with(|| {
                            cmp_list(args1, args2, |ty1, ty2| ty1.cmp(ctx, lc, ty2, style))
                        })
                    }),
                    (It, It) => Ordering::Equal,
                    (
                        Qua {
                            value: val1,
                            ty: ty1,
                        },
                        Qua {
                            value: val2,
                            ty: ty2,
                        },
                    ) => val1
                        .cmp(ctx, lc, val2, style)
                        .then_with(|| ty1.cmp(ctx, lc, ty2, style)),
                    _ => unreachable!(),
                })
        }
        match style {
            CmpStyle::Strict => cmp(ctx, lc, self, other, style),
            CmpStyle::Attr => self.skip_infer(lc, |this| {
                other.skip_infer(lc, |other| cmp(ctx, lc, this, other, style))
            }),
            CmpStyle::Red => cmp(
                ctx,
                lc,
                self.skip_priv_func(lc),
                other.skip_priv_func(lc),
                style,
            ),
            CmpStyle::Alt => {
                match self.size().cmp(&other.size()) {
                    Ordering::Equal => {}
                    o => return o,
                }
                cmp(
                    ctx,
                    lc,
                    self.skip_priv_func(lc),
                    other.skip_priv_func(lc),
                    style,
                )
            }
        }
    }

    fn cmp_list(
        ctx: Option<&Constructors>,
        lc: Option<&LocalContext>,
        tms1: &[Term],
        tms2: &[Term],
        style: CmpStyle,
    ) -> Ordering {
        cmp_list(tms1, tms2, |tm1, tm2| tm1.cmp(ctx, lc, tm2, style))
    }

    /// ReconSelectTrm
    pub fn mk_select(g: &MizGlobal, lc: &LocalContext, nr: SelId, arg: &Term, ty: &Type) -> Term {
        assert!(matches!(ty.kind, TypeKind::Struct(_)));
        let mut args = Type::new(g.parent_struct(nr).into())
            .widening_of(g, lc, ty)
            .unwrap()
            .to_owned()
            .args;
        args.push(arg.clone());
        Self::Selector {
            nr,
            args: args.into(),
        }
    }

    /// ReconAggregTrm
    /// performs eta expansion of aggregates: `foo` ~> `(# foo.1 , foo.2 #)`
    pub fn mk_aggr(g: &MizGlobal, lc: &LocalContext, s: StructId, arg: &Term, ty: &Type) -> Term {
        assert!(!g.constrs.struct_mode[s].fields.is_empty());
        let nr = g.constrs.struct_mode[s].aggr;
        let ty = &*Type::new(s.into()).widening_of(g, lc, ty).unwrap();
        let mut args = ty.args.clone();
        for &sel in &*g.constrs.aggregate[nr].fields {
            args.push(Self::mk_select(g, lc, sel, arg, ty));
        }
        Term::Aggregate {
            nr,
            args: args.into(),
        }
    }
}

struct WideningStruct<'a> {
    g: &'a MizGlobal,
    stack: Vec<Option<&'a Type>>,
    tgt: StructId,
}

impl<'a> WideningStruct<'a> {
    fn new(g: &'a MizGlobal, tgt: StructId) -> Self {
        Self {
            g,
            stack: vec![],
            tgt,
        }
    }

    fn widening_path(&mut self, n: StructId) -> bool {
        let c = &self.g.constrs.struct_mode[n];
        let pos = self.stack.len();
        self.stack.push(None);
        for ty in &*c.parents {
            let n = ty.struct_id();
            if n == self.tgt {
                self.stack[pos] = Some(ty);
                return true;
            }
            if self.widening_path(n) {
                self.stack[pos] = Some(ty);
                return true;
            }
        }
        self.stack.pop();
        false
    }
}

fn cmp_list<T>(a: &[T], b: &[T], mut cmp: impl FnMut(&T, &T) -> Ordering) -> Ordering {
    assert!(a.len() == b.len());
    for (a, b) in a.iter().zip(b) {
        match cmp(a, b) {
            Ordering::Equal => {}
            o => return o,
        }
    }
    Ordering::Equal
}

impl Type {
    pub fn adjust<'a>(n: ModeId, args: &'a [Term], ctx: &Constructors) -> (ModeId, &'a [Term]) {
        let c = &ctx.mode[n].c;
        match c.redefines {
            Some(mode) => (mode, &args[c.superfluous as usize..]),
            None => (n, args),
        }
    }

    pub fn adjust_mut(&mut self, ctx: &Constructors) {
        if let TypeKind::Mode(n) = &mut self.kind {
            let (n2, args) = Type::adjust(*n, &self.args, ctx);
            *n = n2;
            self.args = args.to_vec()
        }
    }

    fn cmp(
        &self,
        ctx: Option<&Constructors>,
        lc: Option<&LocalContext>,
        other: &Type,
        style: CmpStyle,
    ) -> Ordering {
        self.kind.cmp(&other.kind).then_with(|| {
            let o = self.attrs.0.cmp(ctx, lc, &other.attrs.0, style);
            o.then_with(|| Term::cmp_list(ctx, lc, &self.args, &other.args, style))
        })
    }

    /// SizeOfTyp(fTyp:TypPtr)
    fn size(&self) -> u32 {
        self.args.iter().map(|t| t.size()).sum::<u32>() + 1
    }

    /// TypObj.DecreasingAttrs but with f flipped
    pub fn decreasing_attrs(
        &self,
        other: &Self,
        f: impl FnMut(&MizAttr, &MizAttr) -> bool,
    ) -> bool {
        matches!(&other.attrs.0, Attrs::Consistent(attrs) if attrs.is_empty())
            || other.attrs.0.is_subset_of(&self.attrs.1, f)
    }

    /// TypObj.Widening
    pub fn widening(&self, g: &MizGlobal, lc: &LocalContext) -> Option<Box<Self>> {
        match self.kind {
            TypeKind::Mode(n) => {
                if n == ModeId::ANY {
                    return None;
                }
                let cnst = &g.constrs.mode[n];
                let mut ty = cnst
                    .ty
                    .visit_cloned(&mut Inst::new(&g.constrs, lc, &self.args, 0));
                ty.attrs.1 = self.attrs.1.clone_allowed(&g.constrs, n, &self.args);
                Some(Box::new(ty))
            }
            TypeKind::Struct(_) => Some(Box::new(Type::SET)), // should be ANY? (comment from original)
        }
    }

    /// TypObj.WidenToStruct
    /// postcondition: the returned type has kind Struct
    fn widen_to_struct(&self, g: &MizGlobal, lc: &LocalContext) -> Option<Box<Self>> {
        let mut ty = Box::new(self.clone());
        while let TypeKind::Mode(_) = ty.kind {
            ty = ty.widening(g, lc)?
        }
        Some(ty)
    }

    /// TypObj.WideningOf
    pub fn widening_of<'a>(
        &self,
        g: &MizGlobal,
        lc: &LocalContext,
        from: &'a Type,
    ) -> Option<CowBox<'a, Self>> {
        match self.kind {
            TypeKind::Mode(n) => {
                let (n, _) = Type::adjust(n, &self.args, &g.constrs);
                let mut ty = CowBox::Borrowed(from);
                loop {
                    match ty.kind {
                        TypeKind::Mode(n2) if n2 >= n => {
                            if n2 == n {
                                return Some(ty);
                            }
                            let cnst = &g.constrs.mode[n2];
                            if cnst.redefines == Some(n) {
                                return Some(ty);
                            }
                            ty = CowBox::Owned(ty.widening(g, lc)?);
                        }
                        TypeKind::Struct(_) if n == ModeId::SET || n == ModeId::ANY => {
                            return Some(CowBox::Owned(Box::new(Type::new(n.into()))))
                        }
                        _ => return None,
                    }
                }
            }
            TypeKind::Struct(tgt) => {
                let mut ty = from.widen_to_struct(g, lc)?;
                let n = ty.struct_id();
                if tgt == n {
                    return Some(CowBox::Owned(ty));
                }
                let c = &g.constrs.struct_mode[tgt];
                if c.fields.is_empty()
                    || !sorted_subset(&c.fields, &g.constrs.struct_mode[n].fields)
                {
                    return None;
                }
                let mut widening = WideningStruct::new(g, tgt);
                if !widening.widening_path(n) {
                    return None;
                }
                for ty2 in widening.stack {
                    ty = Box::new(
                        ty2.unwrap()
                            .visit_cloned(&mut Inst::new(&g.constrs, lc, &ty.args, 0)),
                    );
                }
                Some(CowBox::Owned(ty))
            }
        }
    }

    /// TypObj.IsWiderThan
    pub fn is_wider_than(&self, g: &MizGlobal, lc: &LocalContext, other: &Self) -> bool {
        if !other.decreasing_attrs(self, |a1, a2| g.eq(lc, a1, a2)) {
            return false;
        }
        match self.kind {
            TypeKind::Mode(n) => {
                let n = Type::adjust(n, &self.args, &g.constrs).0;
                let mut w = CowBox::Borrowed(other);
                loop {
                    let TypeKind::Mode(n2) = w.kind else { break };
                    let true = n2 >= n else { break };
                    if g.eq_radices(lc, self, &w) {
                        return true;
                    }
                    let Some(w2) = w.widening(g, lc) else { break };
                    w = CowBox::Owned(w2)
                }
                matches!(w.kind, TypeKind::Struct(_)) && (n == ModeId::SET || n == ModeId::ANY)
            }
            TypeKind::Struct(_) => {
                let Some(w) = self.widening_of(g, lc, other) else {
                    return false;
                };
                g.eq_radices(lc, self, &w)
            }
        }
    }

    /// TypObj.RoundUp
    pub fn round_up_with_self(&mut self, g: &MizGlobal, lc: &LocalContext, recursive: bool) {
        // vprintln!("[{:?}] round_up_with_self {:?}", lc.infer_const.borrow().len(), self);
        let mut attrs = self.attrs.1.clone();
        attrs.round_up_with(g, lc, self, recursive);
        self.attrs.1 = attrs;
        // vprintln!("[{:?}] round_up_with_self -> {:?}", lc.infer_const.borrow().len(), self);
    }

    /// FrOpVarTypeOK
    pub fn is_set(&self, g: &MizGlobal, lc: &LocalContext, props: &[Property]) -> bool {
        if let TypeKind::Mode(n) = self.kind {
            if g.constrs.mode[n].properties.get(PropertyKind::Sethood) {
                return true;
            }
        }
        // is_SethoodType
        for prop in props {
            if !matches!(prop.kind, PropertyKind::Sethood) || !g.type_reachable(&prop.ty, self) {
                continue;
            }
            let mut subst = Subst::new(prop.primary.len());
            let Some(ty) = prop.ty.widening_of(g, lc, self) else {
                continue;
            };
            if subst.eq_radices(&mut subst.eq_ctx(g, lc), &prop.ty, &ty)
                && prop
                    .ty
                    .attrs
                    .0
                    .is_subset_of(&ty.attrs.1, |a1, a2| subst.eq(g, lc, a1, a2))
                && subst.check_loci_types::<false>(g, lc, &prop.primary, false)
            {
                return true;
            }
        }
        false
    }
}

impl Formula {
    pub fn adjust_pred<'a>(
        n: PredId,
        args: &'a [Term],
        ctx: Option<&Constructors>,
    ) -> (PredId, &'a [Term]) {
        let Some(ctx) = ctx else { return (n, args) };
        let c = &ctx.predicate[n];
        let Some(nr) = c.redefines else {
            return (n, args);
        };
        (nr, &args[c.superfluous as usize..])
    }

    pub fn adjust_attr<'a>(
        n: AttrId,
        args: &'a [Term],
        ctx: Option<&Constructors>,
    ) -> (AttrId, &'a [Term]) {
        let Some(ctx) = ctx else { return (n, args) };
        let c = &ctx.attribute[n].c;
        let Some(nr) = c.redefines else {
            return (n, args);
        };
        (nr, &args[c.superfluous as usize..])
    }

    fn cmp(
        &self,
        ctx: Option<&Constructors>,
        lc: Option<&LocalContext>,
        other: &Formula,
        style: CmpStyle,
    ) -> Ordering {
        // vprintln!("{self:?} <?> {other:?}");
        self.discr().cmp(&other.discr()).then_with(|| {
            use Formula::*;
            match (self, other) {
                (True, True) => Ordering::Equal,
                (Neg { f: f1 }, Neg { f: f2 }) => f1.cmp(ctx, lc, f2, style),
                (Is { term: t1, ty: ty1 }, Is { term: t2, ty: ty2 }) => t1
                    .cmp(ctx, lc, t2, style)
                    .then_with(|| ty1.cmp(ctx, lc, ty2, style)),
                (And { args: args1 }, And { args: args2 }) => args1
                    .len()
                    .cmp(&args2.len())
                    .then_with(|| Formula::cmp_list(ctx, lc, args1, args2, style)),
                (
                    SchPred {
                        nr: SchPredId(n1),
                        args: args1,
                    },
                    SchPred {
                        nr: SchPredId(n2),
                        args: args2,
                    },
                )
                | (
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
                ) => n1
                    .cmp(n2)
                    .then_with(|| Term::cmp_list(ctx, lc, args1, args2, style)),
                (
                    Attr {
                        nr: n1,
                        args: args1,
                    },
                    Attr {
                        nr: n2,
                        args: args2,
                    },
                ) => match style {
                    CmpStyle::Red => {
                        let (n1, args1) = Formula::adjust_attr(*n1, args1, ctx);
                        let (n2, args2) = Formula::adjust_attr(*n2, args2, ctx);
                        n1.cmp(&n2)
                            .then_with(|| Term::cmp_list(ctx, lc, args1, args2, style))
                    }
                    _ => n1
                        .cmp(n2)
                        .then_with(|| Term::cmp_list(ctx, lc, args1, args2, style)),
                },
                (
                    Pred {
                        nr: n1,
                        args: args1,
                    },
                    Pred {
                        nr: n2,
                        args: args2,
                    },
                ) => match style {
                    CmpStyle::Red => {
                        let (n1, args1) = Formula::adjust_pred(*n1, args1, ctx);
                        let (n2, args2) = Formula::adjust_pred(*n2, args2, ctx);
                        n1.cmp(&n2)
                            .then_with(|| Term::cmp_list(ctx, lc, args1, args2, style))
                    }
                    _ => n1
                        .cmp(n2)
                        .then_with(|| Term::cmp_list(ctx, lc, args1, args2, style)),
                },
                (
                    ForAll {
                        dom: dom1,
                        scope: sc1,
                    },
                    ForAll {
                        dom: dom2,
                        scope: sc2,
                    },
                ) => dom1
                    .cmp(ctx, lc, dom2, style)
                    .then_with(|| sc1.cmp(ctx, lc, sc2, style)),
                #[allow(clippy::explicit_auto_deref)]
                (
                    FlexAnd {
                        terms: t1,
                        scope: sc1,
                        ..
                    },
                    FlexAnd {
                        terms: t2,
                        scope: sc2,
                        ..
                    },
                ) => Term::cmp_list(ctx, lc, &**t1, &**t2, style)
                    .then_with(|| sc1.cmp(ctx, lc, sc2, style)),
                (LegacyFlexAnd { orig: args1, .. }, LegacyFlexAnd { orig: args2, .. }) => {
                    Formula::cmp_list(ctx, lc, &**args1, &**args2, style)
                }
                _ => unreachable!(),
            }
        })
    }

    fn cmp_list(
        ctx: Option<&Constructors>,
        lc: Option<&LocalContext>,
        fs1: &[Formula],
        fs2: &[Formula],
        style: CmpStyle,
    ) -> Ordering {
        // vprintln!("{fs1:?} <?> {fs2:?}");
        cmp_list(fs1, fs2, |f1, f2| f1.cmp(ctx, lc, f2, style))
    }
}

pub struct EqCtx<'a> {
    pub g: &'a MizGlobal,
    pub lc: &'a LocalContext,
    pub depth1: u32,
    pub depth2: u32,
    lift1: u32,
    lift2: u32,
}

impl<'a> EqCtx<'a> {
    pub fn new(g: &'a MizGlobal, lc: &'a LocalContext) -> Self {
        let depth = lc.bound_var.len() as u32;
        Self {
            g,
            lc,
            depth1: depth,
            depth2: depth,
            lift1: 0,
            lift2: 0,
        }
    }

    pub fn enter<R>(&mut self, n: u32, f: impl FnOnce(&mut Self) -> R) -> R {
        self.depth1 += n;
        self.depth2 += n;
        let r = f(self);
        self.depth1 -= n;
        self.depth2 -= n;
        r
    }

    pub fn lift1<R>(&mut self, f: impl FnOnce(&mut Self) -> R) -> R {
        let depth = std::mem::take(&mut self.depth1);
        self.lift1 += depth;
        let r = f(self);
        self.lift1 -= depth;
        self.depth1 = depth;
        r
    }

    pub fn lift2<R>(&mut self, f: impl FnOnce(&mut Self) -> R) -> R {
        let depth = std::mem::take(&mut self.depth2);
        self.lift2 += depth;
        let r = f(self);
        self.lift2 -= depth;
        self.depth2 = depth;
        r
    }
}

pub trait Equate {
    const ADJUST_LEFT: bool = true;
    const IGNORE_MARKS: bool = true;

    fn locus_var_left(&mut self, _: &mut EqCtx<'_>, _nr: LocusId, _t2: &Term) -> bool {
        false
    }
    fn eq_locus_var(&mut self, _: &mut EqCtx<'_>, n1: LocusId, n2: LocusId) -> bool {
        n1 == n2
    }

    fn eq_terms(&mut self, ctx: &mut EqCtx<'_>, t1: &[Term], t2: &[Term]) -> bool {
        t1.len() == t2.len() && t1.iter().zip(t2).all(|(t1, t2)| self.eq_term(ctx, t1, t2))
    }

    fn eq_class_right(&mut self, _: &mut EqCtx<'_>, _t1: &Term, _ec: EqClassId) -> bool {
        false
    }

    /// on (): EqTrm(fTrm1 = t1, fTrm2 = t2)
    /// on Subst: EsTrm(fTrm = t1, aTrm = t2)
    fn eq_term(&mut self, ctx: &mut EqCtx<'_>, t1: &Term, t2: &Term) -> bool {
        use Term::*;
        // vprintln!("{t1:?} [{}] =? {t2:?} [{}]", ctx.depth1, ctx.depth2);
        match (t1, t2) {
            (&EqMark(nr), _) if !Self::IGNORE_MARKS => {
                matches!(*t2, Term::EqMark(nr2) if ctx.lc.marks[nr].1 == ctx.lc.marks[nr2].1)
            }
            (&Locus(nr), _) if self.locus_var_left(ctx, nr, t2) => true,
            (&Locus(n1), &Locus(n2)) if self.eq_locus_var(ctx, n1, n2) => true,
            (Bound(n1), Bound(n2)) => n1.0 + ctx.lift1 == n2.0 + ctx.lift2,
            (Const(ConstId(n1)), Const(ConstId(n2)))
            | (FreeVar(FVarId(n1)), FreeVar(FVarId(n2)))
            | (Numeral(n1), Numeral(n2)) => n1 == n2,
            (EqClass(EqClassId(n1)), EqClass(EqClassId(n2)))
            | (EqMark(EqMarkId(n1)), EqMark(EqMarkId(n2)))
            | (Infer(InferId(n1)), Infer(InferId(n2)))
                if n1 == n2 =>
            {
                true
            }
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
                let (n1, args1) = Term::adjust(*n1, args1, Some(&ctx.g.constrs));
                let (n2, args2) = Term::adjust(*n2, args2, Some(&ctx.g.constrs));
                n1 == n2 && self.eq_terms(ctx, args1, args2)
            }
            (
                SchFunc {
                    nr: SchFuncId(n1),
                    args: args1,
                },
                SchFunc {
                    nr: SchFuncId(n2),
                    args: args2,
                },
            )
            | (
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
                PrivFunc {
                    nr: PrivFuncId(n1),
                    args: args1,
                    ..
                },
                PrivFunc {
                    nr: PrivFuncId(n2),
                    args: args2,
                    ..
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
            ) => n1 == n2 && self.eq_terms(ctx, args1, args2),
            (The { ty: ty1 }, The { ty: ty2 }) => self.eq_type(ctx, ty1, ty2),
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
                    let (depth1, depth2) = (ctx.depth1, ctx.depth2);
                    let res = args1.iter().zip(&**args2).all(|(ty1, ty2)| {
                        let r = self.eq_type(ctx, ty1, ty2);
                        ctx.depth1 += 1;
                        ctx.depth2 += 1;
                        r
                    }) && self.eq_term(ctx, sc1, sc2)
                        && self.eq_formula(ctx, c1, c2);
                    (ctx.depth1, ctx.depth2) = (depth1, depth2);
                    res
                }
            }
            (It, It) => true,
            (Qua { .. }, _) | (_, Qua { .. }) => panic!("invalid qua"),
            (_, &Infer(nr)) => {
                ctx.lift2(|ctx| self.eq_term(ctx, t1, &ctx.lc.infer_const.borrow()[nr].def))
            }
            (&Infer(nr), _) => {
                ctx.lift1(|ctx| self.eq_term(ctx, &ctx.lc.infer_const.borrow()[nr].def, t2))
            }
            (_, &EqMark(nr)) => ctx.lift2(|ctx| self.eq_term(ctx, t1, &ctx.lc.marks[nr].0)),
            (&EqMark(nr), _) => ctx.lift1(|ctx| self.eq_term(ctx, &ctx.lc.marks[nr].0, t2)),
            (_, &EqClass(nr)) => self.eq_class_right(ctx, t1, nr),
            (PrivFunc { .. }, _) | (_, PrivFunc { .. }) => self.eq_term(
                ctx,
                t1.skip_priv_func(Some(ctx.lc)),
                t2.skip_priv_func(Some(ctx.lc)),
            ),
            _ => false,
        }
    }

    /// for (): TypObj.EqRadices
    /// for Subst: CompEsTyp (with fExactly = false)
    fn eq_radices(&mut self, ctx: &mut EqCtx<'_>, ty1: &Type, ty2: &Type) -> bool {
        // vprintln!("{ty1:?} [{}] =r? {ty2:?} [{}]", ctx.depth1, ctx.depth2);
        match (ty1.kind, ty2.kind) {
            (TypeKind::Mode(n1), TypeKind::Mode(n2)) if !Self::ADJUST_LEFT && n1 == n2 => {
                self.eq_terms(ctx, &ty1.args, &ty2.args)
            }
            (TypeKind::Mode(n1), TypeKind::Mode(n2)) => {
                let (n1, args1) = if Self::ADJUST_LEFT {
                    Type::adjust(n1, &ty1.args, &ctx.g.constrs)
                } else {
                    (n1, &*ty1.args)
                };
                let (n2, args2) = Type::adjust(n2, &ty2.args, &ctx.g.constrs);
                n1 == n2 && self.eq_terms(ctx, args1, args2)
            }
            (TypeKind::Struct(n1), TypeKind::Struct(n2)) => {
                n1 == n2 && self.eq_terms(ctx, &ty1.args, &ty2.args)
            }
            _ => false,
        }
    }

    fn eq_attrs(&mut self, ctx: &mut EqCtx<'_>, ty1: &Type, ty2: &Type) -> bool {
        ty1.attrs
            .0
            .is_subset_of(&ty2.attrs.1, |a1, a2| self.eq_attr(ctx, a1, a2))
            && ty2
                .attrs
                .0
                .is_subset_of(&ty1.attrs.1, |a2, a1| self.eq_attr(ctx, a1, a2))
    }

    fn eq_type(&mut self, ctx: &mut EqCtx<'_>, ty1: &Type, ty2: &Type) -> bool {
        self.eq_attrs(ctx, ty1, ty2) && self.eq_radices(ctx, ty1, ty2)
    }

    /// on (): EqAttr
    /// on Subst: EsAttr
    fn eq_attr(&mut self, ctx: &mut EqCtx<'_>, a1: &MizAttr, a2: &MizAttr) -> bool {
        // vprintln!("{a1:?} =? {a2:?}");
        let (n1, args1) = a1.adjust(Some(&ctx.g.constrs));
        let (n2, args2) = a2.adjust(Some(&ctx.g.constrs));
        n1 == n2 && a1.pos == a2.pos && self.eq_terms(ctx, args1, args2)
    }

    fn eq_formulas(&mut self, ctx: &mut EqCtx<'_>, args1: &[Formula], args2: &[Formula]) -> bool {
        args1.len() == args2.len()
            && args1
                .iter()
                .zip(args2)
                .all(|(f1, f2)| self.eq_formula(ctx, f1, f2))
    }

    fn eq_and(&mut self, ctx: &mut EqCtx<'_>, args1: &[Formula], args2: &[Formula]) -> bool {
        self.eq_formulas(ctx, args1, args2)
    }

    fn eq_pred(
        &mut self,
        ctx: &mut EqCtx<'_>,
        n1: PredId,
        n2: PredId,
        args1: &[Term],
        args2: &[Term],
    ) -> bool {
        let (n1, args1) = if Self::ADJUST_LEFT {
            Formula::adjust_pred(n1, args1, Some(&ctx.g.constrs))
        } else {
            (n1, args1)
        };
        let (n2, args2) = Formula::adjust_pred(n2, args2, Some(&ctx.g.constrs));
        n1 == n2 && self.eq_terms(ctx, args1, args2)
    }

    fn eq_forall(
        &mut self,
        ctx: &mut EqCtx<'_>,
        dom1: &Type,
        dom2: &Type,
        sc1: &Formula,
        sc2: &Formula,
    ) -> bool {
        self.eq_type(ctx, dom1, dom2) && ctx.enter(1, |ctx| self.eq_formula(ctx, sc1, sc2))
    }

    fn eq_formula(&mut self, ctx: &mut EqCtx<'_>, f1: &Formula, f2: &Formula) -> bool {
        use Formula::*;
        match (f1.skip_priv_pred(), f2.skip_priv_pred()) {
            (True, True) => true,
            (Neg { f: f1 }, Neg { f: f2 }) => self.eq_formula(ctx, f1, f2),
            (Is { term: t1, ty: ty1 }, Is { term: t2, ty: ty2 }) => {
                self.eq_term(ctx, t1, t2) && self.eq_type(ctx, ty1, ty2)
            }
            (And { args: args1 }, And { args: args2 }) => self.eq_and(ctx, args1, args2),
            (
                SchPred {
                    nr: SchPredId(n1),
                    args: args1,
                },
                SchPred {
                    nr: SchPredId(n2),
                    args: args2,
                },
            )
            | (
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
            ) => n1 == n2 && self.eq_terms(ctx, args1, args2),
            (
                Attr {
                    nr: n1,
                    args: args1,
                },
                Attr {
                    nr: n2,
                    args: args2,
                },
            ) => {
                let (n1, args1) = Formula::adjust_attr(*n1, args1, Some(&ctx.g.constrs));
                let (n2, args2) = Formula::adjust_attr(*n2, args2, Some(&ctx.g.constrs));
                n1 == n2 && self.eq_terms(ctx, args1, args2)
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
            ) => self.eq_pred(ctx, *n1, *n2, args1, args2),
            (
                ForAll {
                    dom: dom1,
                    scope: sc1,
                },
                ForAll {
                    dom: dom2,
                    scope: sc2,
                },
            ) => self.eq_forall(ctx, dom1, dom2, sc1, sc2),
            (
                FlexAnd {
                    terms: t1,
                    scope: sc1,
                    ..
                },
                FlexAnd {
                    terms: t2,
                    scope: sc2,
                    ..
                },
            ) => self.eq_terms(ctx, &**t1, &**t2) && self.eq_formula(ctx, sc1, sc2),
            (LegacyFlexAnd { orig: args1, .. }, LegacyFlexAnd { orig: args2, .. }) => {
                self.eq_formulas(ctx, &**args1, &**args2)
            }
            _ => false,
        }
        // vprintln!("eq_formula {f1:?} <> {f2:?} -> {res}");
    }

    fn eq_ctx<'a>(&self, g: &'a MizGlobal, lc: &'a LocalContext) -> EqCtx<'a> {
        EqCtx::new(g, lc)
    }

    fn eq<T: Equatable + ?Sized>(
        &mut self,
        g: &MizGlobal,
        lc: &LocalContext,
        a: &T,
        b: &T,
    ) -> bool {
        let mut ctx = self.eq_ctx(g, lc);
        a.equate(self, &mut ctx, b)
    }
}

impl Equate for () {
    fn eq_and(&mut self, ctx: &mut EqCtx<'_>, args1: &[Formula], args2: &[Formula]) -> bool {
        // vprintln!("eq_and {args1:?} <> {args2:?}");
        if args1.len() == args2.len() {
            args1
                .iter()
                .zip(args2)
                .all(|(f1, f2)| self.eq_formula(ctx, f1, f2))
        } else {
            let mut conjs1 = ConjIter(args1.iter(), None);
            let mut conjs2 = ConjIter(args2.iter(), None);
            loop {
                match (conjs1.next(), conjs2.next()) {
                    (None, None) => break true,
                    (Some(f1), Some(f2)) if self.eq_formula(ctx, f1, f2) => {}
                    _ => break false,
                }
            }
        }
    }

    fn eq_pred(
        &mut self,
        ctx: &mut EqCtx<'_>,
        n1: PredId,
        n2: PredId,
        args1: &[Term],
        args2: &[Term],
    ) -> bool {
        let (n1, args1) = Formula::adjust_pred(n1, args1, Some(&ctx.g.constrs));
        let (n2, args2) = Formula::adjust_pred(n2, args2, Some(&ctx.g.constrs));
        n1 == n2
            && (self.eq_terms(ctx, args1, args2)
                || Some(n1) == ctx.g.reqs.equals_to()
                    && if let ([l1, r1], [l2, r2]) = (args1, args2) {
                        self.eq_term(ctx, r1, l2) && self.eq_term(ctx, l1, r2)
                    } else {
                        unreachable!()
                    })
    }
}

pub trait Equatable {
    fn equate<T: Equate + ?Sized>(&self, t: &mut T, ctx: &mut EqCtx<'_>, other: &Self) -> bool;
}
impl<A: Equatable + ?Sized> Equatable for Box<A> {
    fn equate<T: Equate + ?Sized>(&self, t: &mut T, ctx: &mut EqCtx<'_>, other: &Self) -> bool {
        (**self).equate(t, ctx, other)
    }
}
macro_rules! impl_equatable {
  ($($name:ident($ty:ty),)*) => {$(
    impl Equatable for $ty {
      fn equate<T: Equate + ?Sized>(&self, t: &mut T, ctx: &mut EqCtx<'_>, other: &Self) -> bool {
        t.$name(ctx, self, other)
      }
    }
  )*}
}
impl_equatable! {
  eq_term(Term),
  eq_terms([Term]),
  eq_type(Type),
  eq_attr(MizAttr),
  eq_formula(Formula),
  eq_formulas([Formula]),
}

impl MizGlobal {
    pub fn with_eq<'a, R>(
        &'a self,
        lc: &'a LocalContext,
        f: impl FnOnce(&mut EqCtx<'a>) -> R,
    ) -> R {
        f(&mut EqCtx::new(self, lc))
    }
    pub fn eq<'a, T: Equatable + ?Sized>(&'a self, lc: &'a LocalContext, a: &T, b: &T) -> bool {
        self.with_eq(lc, |ctx| a.equate(&mut (), ctx, b))
    }
    pub fn eq_radices<'a>(&'a self, lc: &'a LocalContext, a: &Type, b: &Type) -> bool {
        self.with_eq(lc, |ctx| ().eq_radices(ctx, a, b))
    }
    pub fn eq_attrs<'a>(&'a self, lc: &'a LocalContext, a: &Type, b: &Type) -> bool {
        self.with_eq(lc, |ctx| ().eq_attrs(ctx, a, b))
    }
}
pub trait WithGlobalLocal {
    fn global(&self) -> &MizGlobal;
    fn local(&self) -> &LocalContext;
    fn with_eq<'a, R>(&'a self, f: impl FnOnce(&mut EqCtx<'a>) -> R) -> R {
        f(&mut EqCtx::new(self.global(), self.local()))
    }
    fn eq<T: Equatable + ?Sized>(&self, a: &T, b: &T) -> bool {
        self.with_eq(|ctx| a.equate(&mut (), ctx, b))
    }
    fn eq_radices(&self, a: &Type, b: &Type) -> bool {
        self.with_eq(|ctx| ().eq_radices(ctx, a, b))
    }
}
impl WithGlobalLocal for (&MizGlobal, &LocalContext) {
    fn global(&self) -> &MizGlobal {
        self.0
    }
    fn local(&self) -> &LocalContext {
        self.1
    }
}

#[derive(Clone, Debug, Default)]
pub struct Subst<'a> {
    // subst_ty: Vec<Option<Box<Term>>>,
    /// gSubstTrm
    /// `IdxVec<LocusId, Option<Box<Term>>>` but fixed length
    pub subst_term: Box<[Option<CowBox<'a, TermQua>>]>,
}

macro_rules! mk_visit {
  ($visit:ident$(, $mutbl:tt)?) => {
    pub trait $visit {
      const MODIFY_IDS: bool = false;
      #[inline] fn abort(&self) -> bool { false }
      fn push_bound(&mut self, _ty: &$($mutbl)? Type) {}
      fn pop_bound(&mut self, _n: u32) {}

      fn visit_mode_id(&mut self, _n: $(&$mutbl)? ModeId) {}
      fn visit_struct_id(&mut self, _n: $(&$mutbl)? StructId) {}
      fn visit_attr_id(&mut self, _n: $(&$mutbl)? AttrId) {}
      fn visit_pred_id(&mut self, _n: $(&$mutbl)? PredId) {}
      fn visit_func_id(&mut self, _n: $(&$mutbl)? FuncId) {}
      fn visit_sel_id(&mut self, _n: $(&$mutbl)? SelId) {}
      fn visit_aggr_id(&mut self, _n: $(&$mutbl)? AggrId) {}

      fn super_visit_term(&mut self, tm: &$($mutbl)? Term) {
        if self.abort() { return }
        match tm {
          Term::Locus(_)
          | Term::Bound(_)
          | Term::Const(_)
          | Term::EqClass(_)
          | Term::EqMark(_)
          | Term::Infer(_)
          | Term::FreeVar(_)
          | Term::It => {}
          Term::SchFunc { args, .. } => self.visit_terms(args),
          Term::Aggregate { nr, args } => {
            self.visit_aggr_id($(&$mutbl)? *nr);
            self.visit_terms(args)
          }
          Term::Functor { nr, args } => {
            self.visit_func_id($(&$mutbl)? *nr);
            self.visit_terms(args)
          }
          Term::Selector { nr, args } =>  {
            self.visit_sel_id($(&$mutbl)? *nr);
            self.visit_terms(args)
          }
          Term::PrivFunc { args, value, .. } => {
            self.visit_terms(args);
            self.visit_term(value)
          }
          Term::Numeral { .. } => {}
          Term::The { ty } => self.visit_type(ty),
          Term::Fraenkel { args, scope, compr } => {
            for ty in &$($mutbl)? **args {
              self.visit_type(ty);
              self.push_bound(ty);
            }
            self.visit_term(scope);
            self.visit_formula(compr);
            self.pop_bound(args.len() as u32);
          }
          Term::Qua { value, ty } => {
            self.visit_term(value);
            self.visit_type(ty);
          }
        }
      }

      fn visit_term(&mut self, tm: &$($mutbl)? Term) {
        self.super_visit_term(tm)
      }

      fn visit_terms(&mut self, tms: &$($mutbl)? [Term]) {
        for tm in tms {
          if self.abort() { return }
          self.visit_term(tm)
        }
      }

      fn visit_type(&mut self, ty: &$($mutbl)? Type) {
        if self.abort() { return }
        self.visit_attr_pair(&$($mutbl)? ty.attrs);
        match &$($mutbl)? ty.kind {
          TypeKind::Mode(nr) => self.visit_mode_id($(&$mutbl)? *nr),
          TypeKind::Struct(nr) => self.visit_struct_id($(&$mutbl)? *nr),
        }
        self.visit_terms(&$($mutbl)? ty.args);
      }

      fn visit_types(&mut self, tys: &$($mutbl)? [Type]) {
        for ty in tys {
          if self.abort() { return }
          self.visit_type(ty)
        }
      }

      fn visit_attrs(&mut self, attrs: &$($mutbl)? Attrs) {
        if let Attrs::Consistent(attrs) = attrs {
          for attr in attrs {
            if self.abort() { return }
            self.visit_attr_id($(&$mutbl)? attr.nr);
            self.visit_terms(&$($mutbl)? attr.args)
          }
        }
      }

      fn visit_attr_pair(&mut self, attrs: &$($mutbl)? (Attrs, Attrs)) {
        self.visit_attrs(&$($mutbl)? attrs.0);
        self.visit_attrs(&$($mutbl)? attrs.1);
      }

      fn visit_flex_and(
        &mut self, nat: &$($mutbl)? Type, le: $(&$mutbl)? PredId,
        [tm_l, tm_r]: &$($mutbl)? [Term; 2], scope: &$($mutbl)? Formula,
      ) {
        self.visit_type(nat);
        self.visit_pred_id(le);
        self.visit_term(tm_l);
        self.visit_term(tm_r);
        self.push_bound(nat);
        self.visit_formula(scope);
        self.pop_bound(1)
      }

      fn super_visit_formula(&mut self, f: &$($mutbl)? Formula) {
        if self.abort() { return }
        match f {
          Formula::SchPred { args, .. } => self.visit_terms(args),
          Formula::Pred { nr, args } =>  {
            self.visit_pred_id($(&$mutbl)? *nr);
            self.visit_terms(args)
          }
          Formula::Attr { nr, args } =>  {
            self.visit_attr_id($(&$mutbl)? *nr);
            self.visit_terms(args)
          }
          Formula::PrivPred { args, value, .. } => {
            self.visit_terms(args);
            self.visit_formula(value)
          }
          Formula::Is { term, ty } => {
            self.visit_term(term);
            self.visit_type(ty)
          }
          Formula::Neg { f } => self.visit_formula(f),
          Formula::And { args } =>
            for f in args {
              self.visit_formula(f)
            },
          Formula::ForAll { dom, scope } => {
            self.visit_type(dom);
            self.push_bound(dom);
            self.visit_formula(scope);
            self.pop_bound(1)
          }
          Formula::FlexAnd { nat, le, terms, scope } =>
            self.visit_flex_and(nat, $(&$mutbl)? *le, terms, scope),
          Formula::LegacyFlexAnd { orig, terms, expansion } => {
            for f in &$($mutbl)? **orig {
              self.visit_formula(f)
            }
            for t in &$($mutbl)? **terms {
              self.visit_term(t)
            }
            self.visit_formula(expansion);
          },
          Formula::True => {}
        }
      }

      fn visit_formula(&mut self, f: &$($mutbl)? Formula) {
        self.super_visit_formula(f)
      }

      fn visit_push_locus_tys(&mut self, tys: &$($mutbl)? [Type]) {
        for ty in tys {
          self.visit_type(ty)
        }
      }
      fn pop_locus_tys(&mut self, _n: usize) {}

      fn with_locus_tys(&mut self, tys: &$($mutbl)? [Type], f: impl FnOnce(&mut Self)) {
        self.visit_push_locus_tys(tys);
        f(self);
        self.pop_locus_tys(tys.len());
      }

      fn visit_push_sch_func_tys(&mut self, tys: &$($mutbl)? [Type]) {
        for ty in tys {
          self.visit_type(ty)
        }
      }
      fn pop_sch_func_tys(&mut self, _n: usize) {}

      fn with_sch_func_tys(&mut self, tys: &$($mutbl)? [Type], f: impl FnOnce(&mut Self)) {
        self.visit_push_sch_func_tys(tys);
        f(self);
        self.pop_sch_func_tys(tys.len());
      }
    }
  };
}
mk_visit!(Visit);
mk_visit!(VisitMut, mut);

pub struct OnVarMut<F: FnMut(&mut u32)>(pub F);
impl<F: FnMut(&mut u32)> VisitMut for OnVarMut<F> {
    fn visit_term(&mut self, tm: &mut Term) {
        self.super_visit_term(tm);
        if let Term::Bound(BoundId(nr)) = tm {
            self.0(nr)
        }
    }
}

pub struct CheckBound {
    range: Range<u32>,
    found: bool,
}
impl CheckBound {
    pub fn get(range: Range<u32>, f: impl FnOnce(&mut Self)) -> bool {
        let mut cb = Self {
            range,
            found: false,
        };
        f(&mut cb);
        cb.found
    }
}
impl Visit for CheckBound {
    fn abort(&self) -> bool {
        self.found
    }
    fn visit_term(&mut self, tm: &Term) {
        self.super_visit_term(tm);
        match *tm {
            Term::Bound(BoundId(nr)) => self.found |= self.range.contains(&nr),
            Term::Locus(_) => self.found = true,
            _ => (),
        }
    }
}

pub struct CheckLocus(pub bool);
impl CheckLocus {
    pub fn get(f: impl FnOnce(&mut Self)) -> bool {
        let mut cb = Self(false);
        f(&mut cb);
        cb.0
    }
}
impl Visit for CheckLocus {
    fn abort(&self) -> bool {
        self.0
    }
    fn visit_term(&mut self, tm: &Term) {
        self.super_visit_term(tm);
        if let Term::Locus(_) = *tm {
            self.0 = true
        }
    }
}

struct OnFunc<F: FnMut(FuncId, &[Term])>(F);
impl<F: FnMut(FuncId, &[Term])> Visit for OnFunc<F> {
    fn visit_term(&mut self, tm: &Term) {
        self.super_visit_term(tm);
        if let Term::Functor { nr, args } = tm {
            (self.0)(*nr, args)
        }
    }
    fn visit_type(&mut self, _: &Type) {}
    fn visit_formula(&mut self, _: &Formula) {}
}

fn has_func<'a>(ctx: &'a Constructors, nr: FuncId, found: &'a mut bool) -> impl Visit + 'a {
    OnFunc(move |n, args| *found |= n == nr || Term::adjust(n, args, Some(ctx)).0 == nr)
}

pub struct Inst<'a> {
    ctx: &'a Constructors,
    lc: &'a LocalContext,
    subst: &'a [Term],
    base: u32,
    depth: u32,
}

impl<'a> Inst<'a> {
    pub fn new(ctx: &'a Constructors, lc: &'a LocalContext, subst: &'a [Term], base: u32) -> Self {
        Self {
            ctx,
            lc,
            subst,
            base,
            depth: 0,
        }
    }
}

impl VisitMut for Inst<'_> {
    fn push_bound(&mut self, _: &mut Type) {
        self.depth += 1
    }
    fn pop_bound(&mut self, n: u32) {
        self.depth -= n
    }
    fn visit_term(&mut self, tm: &mut Term) {
        match *tm {
            Term::Bound(ref mut nr) => nr.0 += self.base,
            Term::Locus(nr) => {
                *tm = self.subst[nr.0 as usize].clone();
                if self.depth != 0 {
                    OnVarMut(|nr| {
                        if *nr >= self.base {
                            *nr += self.depth
                        }
                    })
                    .visit_term(tm);
                }
            }
            _ => self.super_visit_term(tm),
        }
    }

    fn visit_attrs(&mut self, attrs: &mut Attrs) {
        attrs.reinsert_all(Some(self.ctx), self.lc, !self.lc.attr_sort_bug, |attr| {
            self.visit_terms(&mut attr.args)
        })
    }
}

pub struct Inst0<'a>(pub u32, pub &'a Term);
impl VisitMut for Inst0<'_> {
    /// ReplacePlaceHolderByConjunctNumber
    fn visit_term(&mut self, tm: &mut Term) {
        match tm {
            Term::Bound(nr) => match nr.0.cmp(&self.0) {
                Ordering::Less => {}
                Ordering::Equal => *tm = self.1.clone(),
                Ordering::Greater => nr.0 -= 1,
            },
            _ => self.super_visit_term(tm),
        }
    }
}

impl Term {
    pub fn has_func(&self, ctx: &Constructors, nr: FuncId) -> bool {
        let mut found = false;
        has_func(ctx, nr, &mut found).visit_term(self);
        found
    }

    /// RoundUpTrmType(fTrm = self)
    pub fn round_up_type<'a>(
        &self,
        g: &MizGlobal,
        lc: &'a LocalContext,
        recursive: bool,
    ) -> CowBox<'a, Type> {
        let tm = self.skip_priv_func(Some(lc));
        let ty = Box::new(tm.get_type_uncached(g, lc));
        tm.round_up_type_from(g, lc, CowBox::Owned(ty), recursive)
    }

    /// RoundUpTrmTypeWithType(lTyp = ty, fTrm = self)
    fn round_up_type_from<'a>(
        &self,
        g: &MizGlobal,
        lc: &'a LocalContext,
        mut ty: CowBox<'a, Type>,
        recursive: bool,
    ) -> CowBox<'a, Type> {
        // vprintln!("RoundUpTrmTypeWithType {self:?}, {ty:?}");
        if let Term::Functor { .. } | Term::Selector { .. } | Term::Aggregate { .. } = self {
            let mut attrs = ty.attrs.1.clone();
            // if verbose() {
            //   eprintln!("compare: {self:?}");
            //   for &i in &g.clusters.functor.sorted {
            //     eprintln!(
            //       "{:?} <- {:?}",
            //       FunctorCluster::cmp_term(&g.clusters.functor.vec[i].term, &g.constrs, self),
            //       g.clusters.functor.vec[i].term
            //     )
            //   }
            // }
            let fcs = g.matching_func_clusters(self);
            if !fcs.is_empty() {
                let mut used = vec![false; fcs.len()];
                'main: loop {
                    for (&i, used) in fcs.iter().zip(&mut used) {
                        if *used {
                            continue;
                        }
                        let fc = &g.clusters.functor.vec[i];
                        if fc.round_up_with(g, lc, self, &ty, &mut attrs, recursive) {
                            attrs.round_up_with(g, lc, &ty, recursive);
                            let mut ty2 = ty.to_owned();
                            ty2.attrs.1 = attrs.clone();
                            ty = CowBox::Owned(ty2);
                            if let Attrs::Inconsistent = ty.attrs.1 {
                                break 'main;
                            }
                            *used = true;
                            if fc.ty.is_some() {
                                continue 'main;
                            }
                        } else if fc.ty.is_none() {
                            *used = true
                        }
                    }
                    break;
                }
            }
        }
        ty
    }

    /// GetTrmType(self = fTrm)
    pub fn get_type(&self, g: &MizGlobal, lc: &LocalContext, round_up: bool) -> Type {
        // vprintln!("GetTrmType {self:?}");
        match self {
            Term::Functor { .. } | Term::Selector { .. } | Term::Aggregate { .. } => {
                let cache = lc.term_cache.borrow();
                if let Ok(i) = cache.find(&g.constrs, self) {
                    return cache.terms[i].1.clone();
                }
                drop(cache);
                let i = TermCollection::insert(g, lc, self, round_up);
                lc.term_cache.borrow().terms[i].1.clone()
            }
            &Term::EqMark(m) => lc.marks[m].0.get_type(g, lc, round_up),
            _ => self.get_type_uncached(g, lc),
        }
    }

    /// CopyTrmType(self = fTrm)
    pub fn get_type_uncached(&self, g: &MizGlobal, lc: &LocalContext) -> Type {
        let ty = match *self {
            Term::Bound(nr) => lc.bound_var[nr].clone(),
            Term::Const(nr) => {
                let base = lc.bound_var.len() as u32;
                lc.fixed_var[nr]
                    .ty
                    .visit_cloned(&mut OnVarMut(|nr| *nr += base))
            }
            Term::Infer(nr) => lc.infer_const.borrow()[nr].ty.clone(),
            Term::Numeral(_) => g.numeral_type.clone(),
            Term::Locus(nr) => lc.locus_ty[nr].clone(),
            Term::SchFunc { nr, .. } => lc.sch_func_ty[nr].clone(),
            Term::PrivFunc { nr, ref args, .. } => {
                (*lc.priv_func[nr].ty).visit_cloned(&mut Inst::new(&g.constrs, lc, args, 0))
            }
            Term::Functor { nr, ref args } => g.constrs.functor[nr]
                .ty
                .visit_cloned(&mut Inst::new(&g.constrs, lc, args, 0)),
            Term::Selector { nr, ref args } => g.constrs.selector[nr]
                .ty
                .visit_cloned(&mut Inst::new(&g.constrs, lc, args, 0)),
            Term::Aggregate { nr, ref args } => g.constrs.aggregate[nr]
                .ty
                .visit_cloned(&mut Inst::new(&g.constrs, lc, args, 0)),
            Term::Fraenkel { .. } => Type::SET,
            Term::It => lc.it_type.as_deref().expect("unexpected 'it'").clone(),
            Term::The { ref ty } | Term::Qua { ref ty, .. } => (**ty).clone(),
            Term::EqMark(m) => lc.marks[m].0.get_type_uncached(g, lc),
            Term::EqClass(_) | Term::FreeVar(_) => {
                unreachable!("get_type_uncached(EqClass | FreeVar)")
            }
        };
        // vprintln!("[{:?}] get_type {:?} -> {:?}", lc.infer_const.borrow().len(), self, ty);
        ty
    }
}

#[derive(Default)]
pub struct TermCollection {
    scope: u32,
    pub terms: Vec<(Term, Type, u32)>,
}

impl TermCollection {
    /// MarkTermsInTTColl
    pub fn open_scope(&mut self) {
        // vprintln!("[{}] open scope", self.scope);
        if self.scope == 0 && self.terms.len() > crate::GC_THRESHOLD {
            stat("gc");
            self.terms.clear()
        }
        self.scope += 1
    }

    /// RemoveTermsFromTTColl
    pub fn close_scope(&mut self) {
        self.scope -= 1;
        // vprintln!("[{}] close scope", self.scope);
        self.terms.retain(|a| a.2 <= self.scope);
    }

    /// MSortedCollection.Search(self = self, Key = tm, out Index)
    fn find(&self, ctx: &Constructors, tm: &Term) -> Result<usize, usize> {
        self.terms
            .binary_search_by(|a| a.0.cmp(Some(ctx), None, tm, CmpStyle::Alt))
    }

    fn insert_raw(&mut self, ctx: &Constructors, base: u32, tm: Term, ty: Type) -> usize {
        let i = self.find(ctx, &tm).unwrap_err();
        // vprintln!("[{}] insert {tm:?}", self.scope);
        debug_assert!(self.scope != 0 || !CheckBound::get(0..base, |cb| cb.visit_term(&tm)));
        self.terms.insert(i, (tm, ty, self.scope));
        i
    }

    fn get_mut(&mut self, ctx: &Constructors, tm: &Term) -> &mut (Term, Type, u32) {
        let i = self.find(ctx, tm).unwrap();
        &mut self.terms[i]
    }

    /// InsertTermInTTColl(fTrm = tm)
    pub fn insert(g: &MizGlobal, lc: &LocalContext, tm: &Term, round_up: bool) -> usize {
        // eprintln!("[{}] InsertTermInTTColl {tm:?}", lc.term_cache.borrow().scope);
        if let Term::Functor { args, .. }
        | Term::Selector { args, .. }
        | Term::Aggregate { args, .. } = tm
        {
            for tm in &**args {
                let tm = tm.skip_priv_func(Some(lc));
                if let Term::Functor { .. } | Term::Selector { .. } | Term::Aggregate { .. } = tm {
                    Self::insert(g, lc, tm, round_up);
                }
            }
        }
        if let Ok(i) = lc.term_cache.borrow().find(&g.constrs, tm) {
            return i;
        }

        // There are some horrible race conditions here. round_up_type_from() and round_up_with_self()
        // are mutually recursive with this function, so we can end up trying to insert a term
        // while things are changing under us. We have to clone the type several times,
        // and we also have to search anew for the term every time
        // because it might have been shuffled about.

        // Get the type of the term.
        let ty = tm.get_type_uncached(g, lc);

        // 1. Insert the term with its type provisionally into the cache
        let base = lc.bound_var.len() as u32;
        let i = lc
            .term_cache
            .borrow_mut()
            .insert_raw(&g.constrs, base, tm.clone(), ty);

        // clone the type so that we don't hold on to the cache for the next bit
        let ty = Box::new(lc.term_cache.borrow().terms[i].1.clone());
        // Round up the type using the term we inserted
        let mut ty = tm
            .round_up_type_from(g, lc, CowBox::Owned(ty), round_up)
            .to_owned();
        // 2. Put the new type into the cache.
        // (Yes, stuff between (1) and (2) can see the term with the unrounded type...)
        // also clone the type *again*
        lc.term_cache.borrow_mut().get_mut(&g.constrs, tm).1 = (*ty).clone();

        // Round up the type using its own attributes
        ty.round_up_with_self(g, lc, round_up);
        // eprintln!("[{}] caching {tm:?} : {ty:?}", lc.term_cache.borrow().scope);
        let cache = &mut *lc.term_cache.borrow_mut();
        // search for the term one last time and return the index.
        // This index has a very limited shelf life
        let i = cache.find(&g.constrs, tm).unwrap();
        // 3. Put the newer type into the cache.
        cache.terms[i].1 = *ty;

        // Note: the original source doesn't do the two clones above,
        // but that's definitely a segfault hazard.
        i
    }

    fn clear(&mut self) {
        self.terms.clear()
    }
}

// static _CALLS: AtomicU32 = AtomicU32::new(0);

impl<'a> Subst<'a> {
    pub fn new(size: usize) -> Self {
        Self {
            subst_term: vec![None; size].into(),
        }
    }

    fn clear(&mut self) {
        for t in &mut *self.subst_term {
            *t = None;
        }
    }

    /// InitEssentialsArgs
    pub fn from_essential(len: usize, essential: &[LocusId], args: &'a [Term]) -> Self {
        // eprintln!("from_essential {essential:?}");
        assert_eq!(args.len(), essential.len());
        let mut subst = Self::new(len);
        for (&n, t) in essential.iter().zip(args) {
            subst.subst_term[Idx::into_usize(n)] = Some(CowBox::Borrowed(t))
        }
        subst
    }

    /// InitInst
    pub fn finish(self) -> Box<[Term]> {
        self.subst_term
            .into_vec()
            .into_iter()
            .map(|t| *t.unwrap().to_owned())
            .collect()
    }

    pub fn trim_to(self, len: usize) -> Box<[TermQua]> {
        let n = self.subst_term.len().checked_sub(len).unwrap();
        self.subst_term
            .into_vec()
            .into_iter()
            .skip(n)
            .map(|t| *t.unwrap().to_owned())
            .collect()
    }

    pub fn inst_term_mut(self, ctx: &Constructors, lc: &LocalContext, tm: &mut Term, base: u32) {
        Inst::new(ctx, lc, &self.finish(), base).visit_term(tm)
    }

    /// InstSubstTrm
    pub fn inst_term(self, ctx: &Constructors, lc: &LocalContext, tm: &Term, base: u32) -> Term {
        let mut tm = tm.clone();
        self.inst_term_mut(ctx, lc, &mut tm, base);
        tm
    }

    /// InstSubstFrm
    pub fn inst_formula_mut(
        self,
        ctx: &Constructors,
        lc: &LocalContext,
        f: &mut Formula,
        base: u32,
    ) {
        Inst::new(ctx, lc, &self.finish(), base).visit_formula(f)
    }

    fn snapshot(&self) -> Box<[bool]> {
        self.subst_term.iter().map(Option::is_some).collect()
    }

    fn rollback(&mut self, snapshot: &[bool], upto: usize) {
        #[allow(clippy::needless_range_loop)]
        for j in 0..=upto {
            match &mut self.subst_term[j] {
                x @ Some(_) if !snapshot[j] => *x = None,
                _ => {}
            }
        }
    }

    /// NEW = false: CheckLociTypes
    /// NEW = true: CheckLociTypesN
    /// round_up: ItIsChecker
    pub fn check_loci_types<const NEW: bool>(
        &mut self,
        g: &MizGlobal,
        lc: &LocalContext,
        tys: &[Type],
        round_up: bool,
    ) -> bool {
        let mut i = tys.len();
        assert!(self.subst_term.len() == i);
        let mut subst_ty = vec![None; i];
        // self.subst_term, tys, and subst_ty are all parallel arrays.
        // * subst_term[i] is either missing (unassigned), or it should have type tys[i].
        // * subst_ty[i] is the actual type of subst_term[i], which should be a subtype of tys[i].
        //
        // At the start of the algorithm, subst_ty is empty, and subst_term is partially filled.
        // The index i is where we are currently working; we progress from right to left.
        // We maintain the invariant that if subst_ty[i] is set, then we have checked that
        //
        //   subst_term[i] : subst_ty[i]   and   subst_ty[i] <: subst(tys[i]).
        //
        // let n = _CALLS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        // vprintln!("\n[{n}] CheckLociTypes {NEW}: subst = {:?}, tys = {tys:?}", self.subst_term);
        'main: loop {
            // vprintln!("[{n}] main {i:?}, subst = {:?} : {subst_ty:?}", self.subst_term);
            // Decrease i to skip past `None`s in subst_term, and then `let Some(tm) = subst_term[i]`
            let tm = loop {
                if i == 0 {
                    // vprintln!("[{n}] -> true");
                    return true;
                }
                i -= 1;
                if let Some(t) = &self.subst_term[i] {
                    break &**t;
                }
                assert!(!NEW)
            };
            // vprintln!("[{n}] main {i:?}, subst = {:?} : {subst_ty:?}, tm = {tm:?}", self.subst_term);
            let mut snap = self.snapshot();
            // let wty be the type of subst_term[i]
            let wty = if round_up {
                tm.round_up_type(g, lc, round_up)
            } else {
                CowBox::Owned(Box::new(tm.get_type(g, lc, round_up)))
            };
            // vprintln!("[{n}] main {i:?}, subst = {:?} : {subst_ty:?}, wty = {wty:?}", self.subst_term);
            // Are the attributes of tys[i] all contained in wty's?
            // This is a necessary condition for wty <: tys[i].
            let mut ok = if wty.decreasing_attrs(&tys[i], |a1, a2| self.eq(g, lc, a1, a2)) {
                if NEW {
                    snap = self.snapshot();
                }
                Some(wty)
            } else {
                None
            };
            // This loop { match ... } is a workaround for the lack of goto in rust
            loop {
                // vprintln!("[{n}] loop {i:?}, subst = {:?} : {subst_ty:?}, ok = {ok:?}", self.subst_term);
                if let Some(wty) = ok {
                    // We have a candidate type `wty` which is the type of `subst_term[i]`.

                    // Try widening wty to make it a candidate for unification with tys[i].
                    // If this fails, we have to backtrack
                    let Some(wty) = tys[i].widening_of(g, lc, &wty) else {
                        ok = None;
                        continue;
                    };

                    // Unify subst(tys[i]) and wty, which can assign some entries in subst_term.
                    let comp = self.eq_radices(&mut self.eq_ctx(g, lc), &tys[i], &wty);
                    // Record that subst_ty[i] := wty
                    subst_ty[i] = Some(wty.to_owned());
                    if comp {
                        // We were successful, so we can continue the main loop
                        continue 'main;
                    }
                    // Unset anything that was set as a result of the unification
                    self.rollback(&snap, i)
                } else {
                    // we get here when we want to backtrack because we can't satisfy
                    // the current substitution
                    loop {
                        i += 1;
                        if NEW {
                            if i >= self.subst_term.len() {
                                // vprintln!("[{n}] -> false");
                                return false;
                            }
                        } else {
                            // Increase i to the beginning of the last run of Nones in subst_term,
                            // by checking that subst_term[i] is set
                            loop {
                                match self.subst_term.get(i) {
                                    None => {
                                        // vprintln!("[{n}] -> false");
                                        return false;
                                    }
                                    Some(Some(_)) => break,
                                    _ => {}
                                }
                                i += 1;
                            }
                        }
                        // vprintln!("[{n}] bad {i:?}, subst = {:?} : {subst_ty:?}", self.subst_term);
                        let ty = subst_ty[i].as_deref().unwrap();
                        // vprintln!("[{n}] bad {i:?}, subst = {:?} : {subst_ty:?}, ty = {ty:?}", self.subst_term);

                        // I don't know what this check is doing. I guess StructId(0) is special?
                        // In tests it is always STRUCT_0:1 which is "1-sorted".
                        // Maybe it is a superclass of all structs?
                        if ty.kind != TypeKind::Struct(StructId(0))
                            && matches!(tys[i].kind, TypeKind::Mode(n) if g.constrs.mode[n].redefines.is_none())
                        {
                            break;
                        }
                    }
                }
                // subst_ty[i] is necessarily filled at this point,
                // and the substitution didn't work out.
                // So we unset it and widen it:
                // * if the widening fails, then we continue to backtrack
                // * If we get wty we pass it back to the unification check
                ok = subst_ty[i]
                    .take()
                    .unwrap()
                    .widening(g, lc)
                    .map(CowBox::Owned)
            }
        }
    }

    /// CheckArgType
    /// this seems to be essentially the same as CheckLociTypes,
    /// but written as a recursive backtracking algorithm which seems a lot more sensible.
    pub fn check_types(&mut self, g: &MizGlobal, lc: &LocalContext, tys: &[Type]) -> bool {
        let [tys @ .., ty] = tys else { return true };
        let i = tys.len();
        let mut wty = self.subst_term[i].as_ref().unwrap().get_type(g, lc, false);
        wty.round_up_with_self(g, lc, false);
        // vprintln!("check_types {i}: {:?} : {wty:?} <:? {ty:?}", self.subst_term[i]);
        if !wty.decreasing_attrs(ty, |a1, a2| self.eq(g, lc, a1, a2)) {
            return false;
        }
        let Some(mut wty) = ty.widening_of(g, lc, &wty) else {
            return false;
        };
        let snap = self.snapshot();
        if self.eq_radices(&mut self.eq_ctx(g, lc), ty, &wty) && self.check_types(g, lc, tys) {
            return true;
        }
        self.rollback(&snap, i);
        if matches!(ty.kind, TypeKind::Mode(n) if g.constrs.mode[n].redefines.is_none()) {
            while let Some(sty) = wty.widening(g, lc) {
                let Some(wty2) = ty.widening_of(g, lc, &sty) else {
                    break;
                };
                if self.eq_radices(&mut self.eq_ctx(g, lc), ty, &wty2)
                    && self.check_types(g, lc, tys)
                {
                    return true;
                }
                self.rollback(&snap, i);
                wty = CowBox::Owned(wty2.to_owned());
            }
        }
        false
    }
}

impl Equate for Subst<'_> {
    const ADJUST_LEFT: bool = false;

    fn eq_locus_var(&mut self, _: &mut EqCtx<'_>, _: LocusId, _: LocusId) -> bool {
        false
    }
    fn locus_var_left(&mut self, ctx: &mut EqCtx<'_>, nr: LocusId, t2: &Term) -> bool {
        // vprintln!("{self:?} @ v{nr:?} =? {t2:?}");
        match &mut self.subst_term[Idx::into_usize(nr)] {
            x @ None => {
                ctx.depth1 == 0 && {
                    *x = Some(CowBox::Owned(Box::new(t2.clone())));
                    true
                }
            }
            Some(tm) => ().eq_term(ctx, t2, tm.unqua()),
        }
    }

    fn eq_ctx<'a>(&self, g: &'a MizGlobal, lc: &'a LocalContext) -> EqCtx<'a> {
        EqCtx {
            depth1: 0,
            ..EqCtx::new(g, lc)
        }
    }
}

pub struct ConjIter<'a>(
    pub std::slice::Iter<'a, Formula>,
    pub Option<Box<ConjIter<'a>>>,
);

impl<'a> Default for ConjIter<'a> {
    fn default() -> Self {
        Self([].iter(), None)
    }
}

impl<'a> Iterator for ConjIter<'a> {
    type Item = &'a Formula;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.0.next() {
                Some(f) => match f.skip_priv_pred() {
                    Formula::And { args } => {
                        *self = ConjIter(args.iter(), Some(Box::new(std::mem::take(self))))
                    }
                    f => return Some(f),
                },
                None => *self = *self.1.take()?,
            }
        }
    }
}

impl Formula {
    pub fn skip_priv_pred(&self) -> &Self {
        let mut ty = self;
        loop {
            while let Formula::PrivPred { value, .. } = ty {
                ty = value
            }
            if let Formula::Neg { f } = ty {
                if let Formula::PrivPred { value, .. } = &**f {
                    let mut l = &**value;
                    while let Formula::PrivPred { value, .. } = l {
                        l = value
                    }
                    if let Formula::Neg { f } = l {
                        ty = f;
                        continue;
                    }
                }
            }
            // vprintln!("skip_priv_pred {self:?} -> {ty:?}");
            return ty;
        }
    }
}
impl Attrs {
    pub fn push(&mut self, attr: MizAttr) {
        if let Self::Consistent(attrs) = self {
            attrs.push(attr)
        }
    }

    /// MAttrCollection.IsSubsetOf(self = self, aClu = other, aEqAttr(x, y) = eq(y, x))
    pub fn is_subset_of(
        &self,
        other: &Self,
        mut eq: impl FnMut(&MizAttr, &MizAttr) -> bool,
    ) -> bool {
        // let n = CALLS2.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        // vprintln!("{n:?}: {self:?} <:? {other:?}");
        match (self, other) {
            (Attrs::Inconsistent, Attrs::Consistent(_)) => false,
            (Attrs::Consistent(this), Attrs::Consistent(other)) => {
                other.len() >= this.len() && this.iter().all(|i| other.iter().any(|j| eq(i, j)))
            }
            (_, Attrs::Inconsistent) => {
                // You would think this case is just "true", but we use this function to
                // construct substitutions by unification, and so we have to report "false"
                // if a variable that would have been unified is left unbound.
                struct ContainsLocusVar(bool);
                impl Visit for ContainsLocusVar {
                    fn abort(&self) -> bool {
                        self.0
                    }
                    fn visit_term(&mut self, tm: &Term) {
                        match tm {
                            Term::Locus(_) => self.0 = true,
                            _ => self.super_visit_term(tm),
                        }
                    }
                }
                let mut v = ContainsLocusVar(false);
                v.visit_attrs(self);
                !v.0
            }
        }
    }

    pub fn cmp(
        &self,
        ctx: Option<&Constructors>,
        lc: Option<&LocalContext>,
        other: &Attrs,
        style: CmpStyle,
    ) -> Ordering {
        let (this, other) = (self.attrs(), other.attrs());
        this.len()
            .cmp(&other.len())
            .then_with(|| cmp_list(this, other, |a, b| a.cmp(ctx, lc, b, style)))
    }

    /// MAttrCollection.CopyAllowed(aTyp = (n, args), aOrigin = self)
    pub fn clone_allowed(&self, ctx: &Constructors, n: ModeId, args: &[Term]) -> Self {
        match self {
            Attrs::Inconsistent => Attrs::Inconsistent,
            Attrs::Consistent(attrs) => {
                let mut out = Self::default();
                let (n, _) = Type::adjust(n, args, ctx);
                for attr in attrs {
                    if ctx.attribute[attr.adjusted_nr(ctx)].ty.kind != TypeKind::Mode(n) {
                        out.push(attr.clone());
                    }
                }
                out
            }
        }
    }

    /// MAttrCollection.Insert(self = self, aItem = item)
    /// returns true if self changed
    pub fn insert(&mut self, ctx: Option<&Constructors>, lc: &LocalContext, item: MizAttr) -> bool {
        let Self::Consistent(this) = self else {
            return false;
        };
        match this.binary_search_by(|attr| attr.cmp_abs(ctx, Some(lc), &item, CmpStyle::Attr)) {
            Ok(i) => {
                if this[i].pos != item.pos {
                    *self = Self::Inconsistent;
                    true
                } else {
                    false
                }
            }
            Err(i) => {
                this.insert(i, item);
                true
            }
        }
    }

    /// MAttrCollection.GetAttr(self = self, aAttrNr = item.nr, aAttrArgs = item.args)
    pub fn find(&self, ctx: &Constructors, lc: &LocalContext, item: &MizAttr) -> Option<&MizAttr> {
        let Self::Consistent(this) = self else {
            return None;
        };
        let n = this
            .binary_search_by(|attr| attr.cmp_abs(Some(ctx), Some(lc), item, CmpStyle::Attr))
            .ok()?;
        Some(&this[n])
    }

    pub fn find0_abs(&self, ctx: &Constructors, nr: AttrId) -> Option<&MizAttr> {
        let Self::Consistent(this) = self else {
            return None;
        };
        assert!(&ctx.attribute[nr].c.redefines.is_none());
        let n = this
            .binary_search_by(|attr| attr.adjust(Some(ctx)).0.cmp(&nr))
            .ok()?;
        Some(&this[n])
    }

    pub fn find0(&self, ctx: &Constructors, nr: AttrId, pos: bool) -> bool {
        self.find0_abs(ctx, nr)
            .map_or(false, |attr| attr.pos == pos)
    }

    pub fn reinsert_all(
        &mut self,
        ctx: Option<&Constructors>,
        lc: &LocalContext,
        yes: bool,
        mut f: impl FnMut(&mut MizAttr),
    ) {
        if let Attrs::Consistent(attrs1) = self {
            if yes {
                for mut attr in std::mem::take(attrs1) {
                    f(&mut attr);
                    self.insert(ctx, lc, attr);
                }
            } else {
                attrs1.iter_mut().for_each(f)
            }
        }
    }

    pub fn visit_enlarge_by(
        &mut self,
        ctx: &Constructors,
        lc: &LocalContext,
        other: &Self,
        v: &mut impl VisitMut,
    ) {
        if lc.attr_sort_bug {
            self.enlarge_by_map(ctx, lc, other, |a| a.visit_cloned(v))
        } else if let Self::Consistent(_) = self {
            if let Self::Consistent(other) = other {
                for attr in other {
                    self.insert(Some(ctx), lc, attr.visit_cloned(v));
                }
            } else {
                *self = Self::Inconsistent
            }
        }
    }

    /// MAttrCollection.EnlargeBy(self = self, aAnother = other, CopyAttribute = map)
    pub fn enlarge_by_map(
        &mut self,
        ctx: &Constructors,
        lc: &LocalContext,
        other: &Self,
        map: impl FnMut(&MizAttr) -> MizAttr,
    ) {
        if let Self::Consistent(this) = self {
            if let Self::Consistent(other) = other {
                if other.is_empty() {
                    return;
                }
                for item in
                    itertools::merge_join_by(std::mem::take(this), other.iter().map(map), |a, b| {
                        a.cmp_abs(Some(ctx), Some(lc), b, CmpStyle::Attr)
                    })
                {
                    match item {
                        EitherOrBoth::Left(attr) | EitherOrBoth::Right(attr) => this.push(attr),
                        EitherOrBoth::Both(attr1, attr2) => {
                            if attr1.pos != attr2.pos {
                                *self = Self::Inconsistent;
                                return;
                            }
                            this.push(attr1)
                        }
                    }
                }
            } else {
                *self = Self::Inconsistent
            }
        }
    }

    pub fn enlarge_by(&mut self, ctx: &Constructors, lc: &LocalContext, other: &Self) {
        self.enlarge_by_map(ctx, lc, other, MizAttr::clone)
    }

    /// ContradictoryAttrs(aClu1 = self, aClu2 = other)
    pub fn contradicts(&self, ctx: &Constructors, lc: &LocalContext, other: &Self) -> bool {
        let (Self::Consistent(this), Self::Consistent(other)) = (self, other) else {
            return true;
        };
        itertools::merge_join_by(this, other, |a, b| {
            a.cmp_abs(Some(ctx), Some(lc), b, CmpStyle::Attr)
        })
        .any(|item| matches!(item, EitherOrBoth::Both(attr1, attr2) if attr1.pos != attr2.pos))
    }

    /// MCondClList.RoundUpCluster(aCluster = self, aTyp = ty)
    /// MAttrCollection.RoundUpWith(self = self, aTyp = ty)
    pub fn round_up_with(&mut self, g: &MizGlobal, lc: &LocalContext, ty: &Type, round_up: bool) {
        struct State<'a> {
            cl_fire: Vec<u32>,
            jobs: Vec<u32>,
            old_attr_nums: &'a mut EnumMap<bool, BTreeMap<AttrId, u32>>,
            new_attr_nums: &'a mut EnumMap<bool, BTreeMap<AttrId, u32>>,
        }
        fn sorted_insert(jobs: &mut Vec<u32>, value: u32) {
            if let Err(i) = jobs.binary_search(&value) {
                jobs.insert(i, value)
            }
        }

        impl State<'_> {
            /// HandleUsageAndFire
            fn handle_usage_and_fire(&mut self, g: &MizGlobal, attrs: &Attrs) {
                if let Attrs::Consistent(attrs) = attrs {
                    for (_, map) in &mut *self.new_attr_nums {
                        map.clear()
                    }
                    for attr in attrs {
                        *self.new_attr_nums[attr.pos]
                            .entry(attr.adjusted_nr(&g.constrs))
                            .or_default() += 1;
                    }
                    for (pos, map) in &*self.new_attr_nums {
                        for (&adj_nr, &val) in map {
                            if self.old_attr_nums[pos]
                                .get(&adj_nr)
                                .map_or(true, |&v| v < val)
                            {
                                if let Some(set) =
                                    g.clusters.conditional.attr_clusters[pos].get(&adj_nr)
                                {
                                    for &nr in set {
                                        let x = &mut self.cl_fire[nr as usize];
                                        *x = x.saturating_sub(1);
                                        if *x == 0 {
                                            sorted_insert(&mut self.jobs, nr);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    std::mem::swap(self.old_attr_nums, self.new_attr_nums)
                }
            }
        }

        // vprintln!("round_up_with {self:?} in {ty:?}");
        let mut state = State {
            cl_fire: Default::default(),
            jobs: Default::default(),
            old_attr_nums: &mut Default::default(),
            new_attr_nums: &mut Default::default(),
        };
        state.cl_fire.extend(
            g.clusters
                .conditional
                .vec
                .iter()
                .map(|cl| match cl.antecedent {
                    Attrs::Inconsistent => 0,
                    Attrs::Consistent(ref attrs) => attrs.len() as u32,
                }),
        );
        for (j, &c) in state.cl_fire.iter().enumerate() {
            if c == 0 {
                sorted_insert(&mut state.jobs, j as u32);
            }
        }
        state.handle_usage_and_fire(g, self);
        while let Self::Consistent(_) = self {
            let last = if let Some(last) = state.jobs.pop() {
                last
            } else {
                break;
            };
            // vprintln!("job {last}");
            let cl = &g.clusters.conditional.vec[last as usize];
            if let Some(subst) = cl.try_apply(g, lc, self, ty, round_up) {
                // vprintln!("enlarge {:?} by {:?}\n  {cl:?}", self, cl.consequent.1);
                let mut inst = Inst::new(&g.constrs, lc, &subst, 0);
                self.visit_enlarge_by(&g.constrs, lc, &cl.consequent.1, &mut inst);
                state.handle_usage_and_fire(g, self)
            }
        }
    }
}

impl ConditionalCluster {
    pub fn try_apply(
        &self,
        g: &MizGlobal,
        lc: &LocalContext,
        attrs: &Attrs,
        ty: &Type,
        round_up: bool,
    ) -> Option<Box<[Term]>> {
        if !g.type_reachable(&self.ty, ty) {
            return None;
        }
        let mut subst = Subst::new(self.primary.len());
        // TryRounding()
        if !self
            .antecedent
            .is_subset_of(attrs, |a1, a2| subst.eq(g, lc, a1, a2))
            || self.consequent.1.is_subset_of(attrs, |a1, a2| {
                (a1.adjusted_nr(&g.constrs), a1.pos) == (a2.adjusted_nr(&g.constrs), a2.pos)
            })
        {
            return None;
        }
        // vprintln!("try rounding: {self:?} in {ty:?}");
        let ty = self.ty.widening_of(g, lc, ty)?;
        if subst.eq_radices(&mut subst.eq_ctx(g, lc), &self.ty, &ty)
            && self
                .ty
                .attrs
                .0
                .is_subset_of(&ty.attrs.1, |a1, a2| subst.eq(g, lc, a1, a2))
            && subst.check_loci_types::<false>(g, lc, &self.primary, round_up)
        {
            Some(subst.finish())
        } else {
            None
        }
    }
}

impl<I> TyConstructor<I> {
    fn round_up(&self, g: &MizGlobal, lc: &mut LocalContext) -> Attrs {
        let mut attrs = self.ty.attrs.0.clone();
        if let TypeKind::Mode(nr) = self.ty.kind {
            let mut inst = Inst::new(&g.constrs, lc, &self.ty.args, 0);
            let cl = g.constrs.mode[nr].ty.attrs.1.visit_cloned(&mut inst);
            attrs.enlarge_by(&g.constrs, lc, &cl)
        }
        lc.load_locus_tys(&self.primary);
        attrs.round_up_with(g, lc, &self.ty, false);
        lc.unload_locus_tys();
        attrs
    }
}

impl FunctorCluster {
    /// RoundUpWith(fCluster = self, fTrm = term, fTyp = ty, fClusterPtr = attrs)
    pub fn round_up_with(
        &self,
        g: &MizGlobal,
        lc: &LocalContext,
        term: &Term,
        ty: &Type,
        attrs: &mut Attrs,
        recursive: bool,
    ) -> bool {
        // vprintln!("RoundUpWith {term:?}, {ty:?} <- {attrs:?} in {self:#?}");
        let mut subst = Subst::new(self.primary.len());
        let mut eq = subst.eq(g, lc, &*self.term, term)
            && subst.check_loci_types::<false>(g, lc, &self.primary, recursive);
        if !eq {
            if let Term::Functor { nr, ref args } = *term {
                let props = g.constrs.functor[nr].properties;
                if props.get(PropertyKind::Commutativity) {
                    let mut args = args.clone();
                    args.swap(props.arg1 as usize, props.arg2 as usize);
                    let term = Term::Functor { nr, args };
                    subst.clear();
                    eq = subst.eq(g, lc, &*self.term, &term)
                        && subst.check_loci_types::<false>(g, lc, &self.primary, recursive);
                }
            }
        }
        if eq {
            if let Some(cluster_ty) = &self.ty {
                match cluster_ty.widening_of(g, lc, ty) {
                    Some(ty)
                        if subst.eq_radices(&mut subst.eq_ctx(g, lc), cluster_ty, &ty)
                            && cluster_ty
                                .attrs
                                .0
                                .is_subset_of(&ty.attrs.1, |a1, a2| subst.eq(g, lc, a1, a2))
                            && subst.check_loci_types::<false>(g, lc, &self.primary, recursive) => {
                    }
                    _ => return false,
                }
            }
            let subst = subst.finish();
            let mut inst = Inst::new(&g.constrs, lc, &subst, 0);
            attrs.visit_enlarge_by(&g.constrs, lc, &self.consequent.1, &mut inst);
            // vprintln!("enlarged -> {attrs:?}");
        }
        eq
    }
}

impl Definiens {
    /// EqualsExpansion
    pub fn equals_expansion(&self) -> Option<EqualsDef> {
        let ConstrKind::Func(nr) = self.constr else {
            return None;
        };
        let Formula::True = self.assumptions else {
            return None;
        };
        let DefValue::Term(DefBody {
            cases,
            otherwise: Some(ow),
        }) = &self.value
        else {
            return None;
        };
        let [] = **cases else { return None };
        let primary = self.primary.split_last().unwrap().1.to_vec().into(); // TODO: is this an unwrap?
        let expansion = ow.clone();
        let essential = self.essential.split_last().unwrap().1.to_vec().into(); // TODO: is this an unwrap?
        let args = self.essential.iter().map(|&nr| Term::Locus(nr)).collect();
        Some(EqualsDef {
            primary,
            expansion,
            pattern: (nr, args),
            essential,
        })
    }

    /// Matches (in identify)
    pub fn matches<'a>(
        &self,
        g: &MizGlobal,
        lc: &LocalContext,
        kind: ConstrKind,
        args: &'a [Term],
    ) -> Option<Subst<'a>> {
        if self.constr != kind {
            return None;
        }
        let mut subst = Subst::from_essential(self.primary.len(), &self.essential, args);
        if !subst.check_loci_types::<true>(g, lc, &self.primary, false) {
            return None;
        }
        Some(subst)
    }
}

impl EqualsDef {
    /// ExpandTrmIfEqual
    pub fn expand_if_equal(
        &self,
        g: &MizGlobal,
        lc: &LocalContext,
        args: &[Term],
        depth: u32,
    ) -> Option<Term> {
        let mut subst = Subst::from_essential(self.primary.len(), &self.essential, args);
        let true = subst.check_loci_types::<true>(g, lc, &self.primary, false) else {
            return None;
        };
        Some(subst.inst_term(&g.constrs, lc, &self.expansion, depth))
    }
}

impl IdentifyFunc {
    pub fn try_apply_lhs(
        &self,
        g: &MizGlobal,
        lc: &LocalContext,
        lhs: &Term,
        tm: &Term,
    ) -> Option<Subst<'static>> {
        let mut subst = Subst::new(self.primary.len());
        subst.eq(g, lc, lhs, tm).then_some(())?;
        subst
            .check_loci_types::<false>(g, lc, &self.primary, false)
            .then_some(())?;
        for &(x, y) in &*self.eq_args {
            let (ux, uy) = (Idx::into_usize(x), Idx::into_usize(y));
            assert!(subst.subst_term[uy].is_none());
            self.primary[uy]
                .is_wider_than(g, lc, &self.primary[ux])
                .then_some(())?;
            subst.subst_term[uy] = subst.subst_term[ux].clone();
        }
        Some(subst)
    }
}
pub struct ExpandPrivFunc<'a>(pub &'a Constructors, pub &'a LocalContext);

impl VisitMut for ExpandPrivFunc<'_> {
    /// CopyExpTrm
    fn visit_term(&mut self, tm: &mut Term) {
        if let Term::PrivFunc { value, .. } = tm {
            *tm = std::mem::take(value);
            self.visit_term(tm)
        } else {
            self.super_visit_term(tm)
        }
    }

    fn visit_attrs(&mut self, attrs: &mut Attrs) {
        attrs.reinsert_all(Some(self.0), self.1, true, |attr| {
            self.visit_terms(&mut attr.args)
        })
    }

    /// CopyExpFrm
    fn visit_formula(&mut self, f: &mut Formula) {
        match f {
            Formula::Neg { f: f2 } => {
                self.visit_formula(f2);
                if let Formula::Neg { f: f3 } = &mut **f2 {
                    *f = std::mem::take(f3)
                }
            }
            Formula::And { args } => {
                for mut f in std::mem::take(args) {
                    self.visit_formula(&mut f);
                    match f {
                        Formula::And { args: fs } => args.extend(fs),
                        _ => args.push(f),
                    }
                }
            }
            Formula::PrivPred { value, .. } => {
                *f = std::mem::take(value);
                self.visit_formula(f)
            }
            _ => self.super_visit_formula(f),
        }
    }
}

impl Term {
    fn try_to_number(&self, g: &MizGlobal, lc: &LocalContext) -> Option<Complex> {
        match *self {
            Term::Numeral(n) => Some(n.into()),
            Term::Functor { nr, ref args } => {
                let (nr, args) = Term::adjust(nr, args, Some(&g.constrs));
                macro_rules! op {
                    (|$x:ident, $y:ident| $e:expr) => {{
                        let [arg1, arg2] = args else { unreachable!() };
                        let Term::Infer(i1) = *arg1.skip_priv_func(Some(lc)) else {
                            unreachable!()
                        };
                        let Term::Infer(i2) = *arg2.skip_priv_func(Some(lc)) else {
                            unreachable!()
                        };
                        let ic = lc.infer_const.borrow();
                        let ($x, $y) = (ic[i1].number.clone()?, ic[i2].number.clone()?);
                        $e
                    }};
                    (|$x:ident| $e:expr) => {{
                        let [arg1] = args else { unreachable!() };
                        let Term::Infer(i1) = *arg1.skip_priv_func(Some(lc)) else {
                            unreachable!()
                        };
                        let ic = lc.infer_const.borrow();
                        let $x = ic[i1].number.clone()?;
                        $e
                    }};
                }
                match g.reqs.rev.get(nr) {
                    // Some(Some(Requirement::ZeroNumber)) => Some(Complex::ZERO),
                    // Some(Some(Requirement::Succ)) => op!(|x| x + Complex::ONE),
                    Some(Some(Requirement::ImaginaryUnit)) => Some(Complex::I),
                    Some(Some(Requirement::RealAdd)) => op!(|x, y| Some(x + y)),
                    Some(Some(Requirement::RealMult)) => op!(|x, y| Some(x * y)),
                    Some(Some(Requirement::RealDiff)) => op!(|x, y| Some(x - y)),
                    Some(Some(Requirement::RealNeg)) => op!(|x| Some(-x)),
                    Some(Some(Requirement::RealInv)) => op!(|x| if x == Complex::ZERO {
                        None
                    } else {
                        Some(x.inv())
                    }),
                    Some(Some(Requirement::RealDiv)) => op!(|x, y| if y == Complex::ZERO {
                        None
                    } else {
                        Some(x / y)
                    }),
                    _ => None,
                }
            }
            _ => None,
        }
    }
}

pub struct InternConst<'a> {
    g: &'a MizGlobal,
    lc: &'a LocalContext,
    equals: &'a BTreeMap<ConstrKind, Vec<EqualsDef>>,
    identify: &'a [IdentifyFunc],
    func_ids: &'a BTreeMap<ConstrKind, Vec<usize>>,
    only_constants: bool,
    equals_expansion_level: u32,
    depth: u32,
    /// InferConsts
    infer_consts: BTreeSet<FuncId>,
}

impl<'a> InternConst<'a> {
    pub fn new(
        g: &'a MizGlobal,
        lc: &'a LocalContext,
        equals: &'a BTreeMap<ConstrKind, Vec<EqualsDef>>,
        identify: &'a [IdentifyFunc],
        func_ids: &'a BTreeMap<ConstrKind, Vec<usize>>,
    ) -> Self {
        Self {
            g,
            lc,
            equals,
            func_ids,
            identify,
            only_constants: true,
            equals_expansion_level: 0,
            depth: 0,
            infer_consts: Default::default(),
        }
    }

    /// CollectInferConst
    /// * postcondition: if self.only_constants, then tm will be Term::Infer after
    fn collect_infer_const(&mut self, tm: &mut Term) {
        if self.only_constants {
            let mut ic = self.lc.infer_const.borrow_mut();
            let nr = match ic
                .find_index(|a| a.def.cmp(Some(&self.g.constrs), None, tm, CmpStyle::Strict))
            {
                Ok(nr) => nr,
                Err(i) => {
                    drop(ic);
                    let mut ty = *tm.round_up_type(self.g, self.lc, false).to_owned();
                    // vprintln!("collect_infer_const {tm:?} : {ty:?}");
                    ty.round_up_with_self(self.g, self.lc, false);
                    let def = std::mem::take(tm);
                    let number = def.try_to_number(self.g, self.lc);
                    ic = self.lc.infer_const.borrow_mut();
                    let nr = ic.insert_at(
                        i,
                        Assignment {
                            def,
                            ty,
                            number,
                            eq_const: vec![],
                        },
                    );
                    // vprintln!("insert ?{nr:?} : {:?} := {:?}", ic[nr].ty, ic[nr].def);
                    let mut ty = ic[nr].ty.clone();
                    drop(ic);
                    self.visit_type(&mut ty);
                    self.lc.infer_const.borrow_mut()[nr].ty = ty;
                    nr
                }
            };
            *tm = Term::Infer(nr);
        }
    }

    /// CollectEqualsConst
    /// precondition: tm must be Term::Functor
    fn collect_equals_const(&mut self, tm: &Term) -> Vec<InferId> {
        // vprintln!("collect_equals_const {tm:?}");
        let mut eq = vec![];
        if self.equals_expansion_level >= 3 {
            return eq;
        }
        let (nr, args) = {
            let Term::Functor { nr, ref args } = *tm else {
                unreachable!()
            };
            Term::adjust(nr, args, Some(&self.g.constrs))
        };
        if self.infer_consts.contains(&nr) {
            return eq;
        }
        let mut insert_one = |this: &mut Self, mut tm| {
            ExpandPrivFunc(&this.g.constrs, this.lc).visit_term(&mut tm);
            this.equals_expansion_level += 1;
            this.infer_consts.insert(nr);
            let depth = std::mem::take(&mut this.depth);
            this.visit_term(&mut tm);
            this.depth = depth;
            this.equals_expansion_level -= 1;
            this.infer_consts.remove(&nr);
            let Term::Infer(nr) = tm else {
                unreachable!("{:?}", tm)
            };
            eq.push(nr);
        };
        if let Some(eq_defs) = self.equals.get(&ConstrKind::Func(nr)) {
            for eq_def in eq_defs {
                if let Some(tm2) = eq_def.expand_if_equal(self.g, self.lc, args, 0) {
                    // vprintln!("{tm:?} -> {tm2:?} using {eq_def:?}");
                    insert_one(self, tm2);
                }
            }
        }
        if let Some(ids) = self.func_ids.get(&ConstrKind::Func(nr)) {
            for &id in ids {
                let id = &self.identify[id];
                // vprintln!("applying {tm:?} <- {id:?}");
                if let Some(subst) = id.try_apply_lhs(self.g, self.lc, &id.lhs, tm) {
                    let tm = subst.inst_term(&self.g.constrs, self.lc, &id.rhs, 0);
                    insert_one(self, tm);
                }
            }
        }
        eq
    }
}

impl VisitMut for InternConst<'_> {
    fn push_bound(&mut self, _: &mut Type) {
        self.depth += 1
    }
    fn pop_bound(&mut self, n: u32) {
        self.depth -= n
    }

    /// CollectConstInTrm
    fn visit_term(&mut self, tm: &mut Term) {
        let only_constants = std::mem::replace(&mut self.only_constants, true);
        let equals_expansion_level = std::mem::replace(&mut self.equals_expansion_level, 0);
        // static FOO: AtomicU32 = AtomicU32::new(0);
        // let foo = FOO.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        // vprintln!("InternConst {foo} @ {} <- {tm:?}", self.depth);
        match tm {
            Term::Locus(_) | Term::Bound(_) => self.only_constants = false,
            &mut Term::Const(nr) => {
                let mut eq = None;
                if let Some((ref fv, _)) = self.lc.fixed_var[nr].def {
                    let mut fv = (**fv).visit_cloned(&mut ExpandPrivFunc(&self.g.constrs, self.lc));
                    self.visit_term(&mut fv);
                    if self.only_constants {
                        let Term::Infer(nr) = fv else { unreachable!() };
                        eq = Some(nr);
                    }
                    self.only_constants = true;
                }
                self.collect_infer_const(tm);
                let Term::Infer(nr) = *tm else { unreachable!() };
                if let Some(n) = eq {
                    self.lc.infer_const.borrow_mut()[nr].insert_eq_const(n)
                }
            }
            Term::Infer(_) => {}
            Term::SchFunc { args, .. }
            | Term::Aggregate { args, .. }
            | Term::Selector { args, .. } => {
                self.visit_terms(args);
                self.collect_infer_const(tm)
            }
            Term::PrivFunc { args, value, .. } => {
                self.visit_terms(args);
                self.visit_term(value)
            }
            Term::Functor { args, .. } => {
                self.visit_terms(args);
                if self.only_constants {
                    let ic = self.lc.infer_const.borrow();
                    match ic.find_index(|a| {
                        a.def.cmp(Some(&self.g.constrs), None, tm, CmpStyle::Strict)
                    }) {
                        Ok(nr) => *tm = Term::Infer(nr),
                        _ => {
                            // vprintln!("search for {tm:?} failed");
                            drop(ic);
                            self.collect_infer_const(tm);
                            let Term::Infer(nr) = *tm else { unreachable!() };
                            self.equals_expansion_level = equals_expansion_level;
                            let tm = self.lc.infer_const.borrow()[nr].def.clone();
                            let eq = self.collect_equals_const(&tm);
                            let asgn = &mut self.lc.infer_const.borrow_mut()[nr];
                            eq.into_iter().for_each(|n| asgn.insert_eq_const(n));
                        }
                    }
                }
            }
            Term::Numeral(_) => self.collect_infer_const(tm),
            Term::The { ty } => {
                self.visit_type(ty);
                self.collect_infer_const(tm)
            }
            Term::Fraenkel { args, scope, compr } => {
                for ty in &mut **args {
                    self.visit_type(ty);
                    self.push_bound(ty);
                }
                self.visit_term(scope);
                self.visit_formula(compr);
                self.pop_bound(args.len() as u32);
                self.only_constants = !CheckBound::get(0..self.depth, |cb| cb.visit_term(tm));
                if self.only_constants {
                    OnVarMut(|n| *n -= self.depth).visit_term(tm);
                    self.collect_infer_const(tm)
                }
            }
            Term::FreeVar(_) | Term::EqClass(_) | Term::EqMark(_) | Term::It | Term::Qua { .. } => {
                unreachable!("CollectConst::visit_term(FreeVar | EqConst | EqMark | It | Qua)")
            }
        }
        // vprintln!("InternConst {foo} @ {} -> {tm:?}", self.depth);
        self.only_constants &= only_constants;
        self.equals_expansion_level = equals_expansion_level;
    }

    fn visit_attrs(&mut self, attrs: &mut Attrs) {
        attrs.reinsert_all(
            Some(&self.g.constrs),
            self.lc,
            !self.lc.attr_sort_bug,
            |attr| self.visit_terms(&mut attr.args),
        )
    }

    // locus types are not interned
    fn visit_push_locus_tys(&mut self, _: &mut [Type]) {}
}

pub struct ExpandConsts<'a> {
    ctx: &'a Constructors,
    lc: &'a LocalContext,
    ic: &'a IdxVec<InferId, Assignment>,
    depth: u32,
}
impl LocalContext {
    pub fn expand_consts(&self, ctx: &Constructors, f: impl FnOnce(&mut ExpandConsts<'_>)) {
        f(&mut ExpandConsts {
            ctx,
            lc: self,
            ic: &self.infer_const.borrow().vec,
            depth: 0,
        })
    }
}

impl VisitMut for ExpandConsts<'_> {
    fn push_bound(&mut self, _: &mut Type) {
        self.depth += 1
    }
    fn pop_bound(&mut self, n: u32) {
        self.depth -= n
    }

    /// ExpandInferConsts
    fn visit_term(&mut self, tm: &mut Term) {
        if let Term::Infer(nr) = *tm {
            *tm = self.ic[nr]
                .def
                .visit_cloned(&mut OnVarMut(|v| *v += self.depth));
        }
        self.super_visit_term(tm);
    }

    fn visit_attrs(&mut self, attrs: &mut Attrs) {
        attrs.reinsert_all(Some(self.ctx), self.lc, !self.lc.attr_sort_bug, |attr| {
            self.visit_terms(&mut attr.args)
        })
    }
}

#[derive(Debug)]
pub struct Descope {
    num_consts: u32,
    infer_const: u32,
    pub remap: HashMap<InferId, InferId>,
    // The entries in remap are Assignment::default() (which is invalid)
    old: Vec<Assignment>,
}

impl VisitMut for Descope {
    fn visit_term(&mut self, tm: &mut Term) {
        loop {
            match *tm {
                Term::Const(c) => assert!(c.0 < self.num_consts, "nongeneralizable variable"),
                Term::Infer(ref mut nr) => {
                    if let Some(i) = nr.0.checked_sub(self.infer_const) {
                        if let Some(nr2) = self.remap.get(nr) {
                            *nr = *nr2
                        } else {
                            *tm = self.old[i as usize].def.clone();
                            continue;
                        }
                    }
                }
                _ => self.super_visit_term(tm),
            }
            break;
        }
    }
}

#[derive(Debug)]
pub struct FixedVar {
    // ident: u32,
    pub ty: Type,
    /// if true, it will unfold eagerly
    pub def: Option<(Box<Term>, bool)>,
}

#[derive(Default, Debug)]
pub struct Assignment {
    /// Must be Term::Functor
    pub def: Term,
    pub ty: Type,
    pub number: Option<Complex>,
    pub eq_const: Vec<InferId>,
    // numeric_value: Option<Complex>,
}

impl Assignment {
    pub fn insert_eq_const(&mut self, n: InferId) {
        if !self.eq_const.contains(&n) {
            self.eq_const.push(n)
        }
    }
}

impl<V: VisitMut> Visitable<V> for Assignment {
    fn visit(&mut self, v: &mut V) {
        self.def.visit(v);
        self.ty.visit(v);
    }
}

#[derive(Debug, Default)]
pub struct FuncDef {
    pub primary: Box<[Type]>,
    pub ty: Box<Type>,
    pub value: Box<Term>,
}

#[derive(Default)]
pub struct LocalContext {
    // here for easy printing
    pub formatter: Formatter,
    /// LocArgTyp
    // FIXME: this is non-owning in mizar
    pub locus_ty: IdxVec<LocusId, Type>,
    /// BoundVarNbr, BoundVar
    pub bound_var: IdxVec<BoundId, Type>,
    /// FixedVar
    pub fixed_var: IdxVec<ConstId, FixedVar>,
    /// InferConstDef
    /// sorted by Assignment::def (by CmpStyle::Strict)
    pub infer_const: RefCell<SortedIdxVec<InferId, Assignment>>,
    pub sch_func_ty: IdxVec<SchFuncId, Type>,
    /// LocFuncDef
    pub priv_func: IdxVec<PrivFuncId, FuncDef>,
    /// gTermCollection
    pub term_cache: RefCell<TermCollection>,
    /// ItTyp
    pub it_type: Option<Box<Type>>,
    /// Not in mizar, used in equalizer for TrmInfo marks
    pub marks: IdxVec<EqMarkId, (Term, EqTermId)>,
    pub attr_sort_bug: bool,
}

impl LocalContext {
    /// gTermCollection.FreeAll
    pub fn clear_term_cache(&self) {
        self.term_cache.borrow_mut().clear()
    }

    pub fn load_locus_tys(&mut self, tys: &[Type]) {
        self.term_cache.get_mut().open_scope();
        self.locus_ty.0.extend_from_slice(tys)
    }

    pub fn unload_locus_tys(&mut self) {
        self.locus_ty.0.clear();
        self.clear_term_cache();
        self.term_cache.get_mut().close_scope()
    }

    pub fn with_locus_tys<R>(&mut self, tys: &[Type], f: impl FnOnce(&mut Self) -> R) -> R {
        self.load_locus_tys(tys);
        let r = f(self);
        self.unload_locus_tys();
        r
    }

    /// FreeConstDef
    pub fn truncate_infer_const(
        &mut self,
        ctx: &Constructors,
        check_for_local_const: bool,
        len: usize,
    ) -> Descope {
        let ic = self.infer_const.get_mut();
        let mut descope = Descope {
            remap: HashMap::new(),
            num_consts: self.fixed_var.len() as u32,
            infer_const: len as u32,
            old: vec![],
        };
        if len >= ic.0.len() {
            return descope;
        }
        descope.old = ic.vec.0.split_off(len);
        ic.sorted.retain(|t| Idx::into_usize(*t) < len);
        assert!(ic.sorted.len() == len);
        if check_for_local_const {
            let mut has_local_const = HashSet::<InferId>::new();
            // vprintln!("start loop {} .. {}", len, len + descope.old.len());
            'retry: loop {
                for (i, asgn) in descope.old.iter().enumerate() {
                    let i = Idx::from_usize(len + i);
                    if has_local_const.contains(&i) {
                        continue;
                    }
                    struct CheckForLocalConst<'a> {
                        has_local_const: &'a HashSet<InferId>,
                        num_consts: u32,
                        found: bool,
                    }
                    impl Visit for CheckForLocalConst<'_> {
                        fn abort(&self) -> bool {
                            self.found
                        }
                        fn visit_term(&mut self, tm: &Term) {
                            self.super_visit_term(tm);
                            match tm {
                                Term::Const(nr) => self.found |= nr.0 >= self.num_consts,
                                Term::Infer(nr) => self.found |= self.has_local_const.contains(nr),
                                _ => {}
                            }
                        }
                    }
                    let mut cc = CheckForLocalConst {
                        has_local_const: &has_local_const,
                        num_consts: descope.num_consts,
                        found: false,
                    };
                    cc.visit_term(&asgn.def);
                    cc.visit_type(&asgn.ty);
                    if cc.found {
                        has_local_const.insert(i);
                        continue 'retry;
                    }
                }
                break;
            }
            // vprintln!("done loop {} -> {}", len, old.len());
            let mut i = Idx::from_usize(len);
            for asgn in &mut descope.old {
                if !has_local_const.contains(&i) {
                    match ic.find_index(|a| a.def.cmp(Some(ctx), None, &asgn.def, CmpStyle::Strict))
                    {
                        Ok(nr) => descope.remap.insert(i, nr),
                        Err(idx) => {
                            let j = ic.insert_at(idx, std::mem::take(asgn));
                            // vprintln!("reinsert ?{i:?} => ?{j:?} : {:?} := {:?}", ic[j].ty, ic[j].def);
                            descope.remap.insert(i, j)
                        }
                    };
                }
                i.0 += 1;
            }
            ic.visit(&mut descope)
        }
        for asgn in &mut ic.0 {
            if asgn.eq_const.iter().any(|n| n.0 >= descope.infer_const) {
                asgn.eq_const.retain_mut(|n| {
                    if let Some(&n2) = descope.remap.get(n) {
                        *n = n2;
                        true
                    } else {
                        n.0 < descope.infer_const
                    }
                });
            }
        }
        descope
    }

    pub fn mk_forall(&mut self, range: Range<usize>, istart: u32, pop: bool, f: &mut Formula) {
        // vprintln!("mk_forall {range:?} (pop = {pop}) <- {f:?}");
        if pop {
            self.fixed_var.0.truncate(range.end);
        }
        let mut abst = Abstract {
            base: range.start as u32,
            lift: (range.end - range.start) as u32,
            ic: self.infer_const.get_mut(),
            istart,
            depth: 0,
        };
        abst.visit_formula(f);
        let mut process = |mut ty| {
            abst.lift -= 1;
            if abst.lift != 0 {
                abst.visit_type(&mut ty);
            }
            *f = Formula::forall(ty, std::mem::take(f));
        };
        if pop {
            self.fixed_var
                .0
                .drain(range)
                .rev()
                .for_each(|var| process(var.ty))
        } else {
            self.fixed_var.0[range]
                .iter()
                .rev()
                .for_each(|var| process(var.ty.clone()))
        }
        // vprintln!("mk_forall -> {f:?}");
    }
}

#[derive(Debug)]
struct Abstract<'a> {
    base: u32,
    lift: u32,
    ic: &'a IdxVec<InferId, Assignment>,
    istart: u32,
    depth: u32,
}

impl VisitMut for Abstract<'_> {
    fn push_bound(&mut self, _: &mut Type) {
        self.depth += 1
    }
    fn pop_bound(&mut self, n: u32) {
        self.depth -= n
    }
    fn visit_term(&mut self, tm: &mut Term) {
        match tm {
            Term::Bound(nr) => nr.0 += self.lift,
            Term::Const(nr) if nr.0 >= self.base => {
                let i = nr.0 - self.base;
                assert!(i < self.lift, "invalid local constant in thesis");
                *tm = Term::Bound(BoundId(i))
            }
            Term::Infer(nr) => {
                if nr.0 >= self.istart {
                    let depth = self.depth;
                    self.depth = 0;
                    self.lift += depth;
                    *tm = self.ic[*nr].def.visit_cloned(self);
                    self.depth = depth;
                    self.lift -= depth;
                }
            }
            _ => self.super_visit_term(tm),
        }
    }
}

impl Constructors {
    fn dump_mode(&self, nr: ModeId) {
        let c = &self.mode[nr];
        let args = Term::locus_list(c.primary.len()).into();
        eprintln!(
            "mode {:?} = {:?}",
            Type {
                args,
                ..Type::new(nr.into())
            },
            c
        );
    }
    fn dump_struct(&self, nr: StructId) {
        let c = &self.struct_mode[nr];
        let args = Term::locus_list(c.primary.len()).into();
        eprintln!(
            "struct {:?} = {:?}",
            Type {
                args,
                ..Type::new(nr.into())
            },
            c
        );
    }
    fn dump_attr(&self, nr: AttrId) {
        let c = &self.attribute[nr];
        let args = Term::locus_list(c.primary.len());
        eprintln!("attr {:?} = {:?}", Formula::Attr { nr, args }, c);
    }
    fn dump_pred(&self, nr: PredId) {
        let c = &self.predicate[nr];
        let args = Term::locus_list(c.primary.len());
        eprintln!("pred {:?} = {:?}", Formula::Pred { nr, args }, c);
    }
    fn dump_func(&self, nr: FuncId) {
        let c = &self.functor[nr];
        let args = Term::locus_list(c.primary.len());
        eprintln!("func {:?} = {:?}", Term::Functor { nr, args }, c);
    }
    fn dump_sel(&self, nr: SelId) {
        let c = &self.selector[nr];
        let args = Term::locus_list(c.primary.len());
        eprintln!("sel {:?} = {:?}", Term::Selector { nr, args }, c);
    }
    fn dump_aggr(&self, nr: AggrId) {
        let c = &self.aggregate[nr];
        let args = Term::locus_list(c.primary.len());
        eprintln!("aggr {:?} = {:?}", Term::Aggregate { nr, args }, c);
    }

    pub fn dump(&self) {
        self.mode.enum_iter().for_each(|p| self.dump_mode(p.0));
        self.struct_mode
            .enum_iter()
            .for_each(|p| self.dump_struct(p.0));
        self.attribute.enum_iter().for_each(|p| self.dump_attr(p.0));
        self.predicate.enum_iter().for_each(|p| self.dump_pred(p.0));
        self.functor.enum_iter().for_each(|p| self.dump_func(p.0));
        self.selector.enum_iter().for_each(|p| self.dump_sel(p.0));
        self.aggregate.enum_iter().for_each(|p| self.dump_aggr(p.0));
    }
}

impl RequirementIndexes {
    pub fn dump(&self, ctx: &Constructors) {
        for (req, _) in &self.fwd {
            if let Some(val) = self.get(req) {
                eprint!("req[{req:?}[{}]] = ", req as u8);
                match val {
                    RequirementKind::Func(nr) => ctx.dump_func(nr),
                    RequirementKind::Mode(nr) => ctx.dump_mode(nr),
                    RequirementKind::Pred(nr) => ctx.dump_pred(nr),
                    RequirementKind::Attr(nr) => ctx.dump_attr(nr),
                }
            }
        }
    }
}

impl Clusters {
    pub fn dump(&self) {
        self.registered.iter().for_each(|cl| eprintln!("{cl:?}"));
        self.functor.vec.0.iter().for_each(|cl| eprintln!("{cl:?}"));
        self.conditional
            .vec
            .iter()
            .for_each(|cl| eprintln!("{cl:?}"));
    }
}

pub struct MizPath {
    pub art: Article,
}

impl MizPath {
    pub fn new(s: &str) -> Result<Self, ToArticleError> {
        Ok(Self {
            art: Article::from_lower(s.as_bytes())?,
        })
    }

    pub fn mml(&self) -> PathBuf {
        let s = self.art.as_str();
        format!("miz/mizshare/mml/{s}").into()
    }

    fn prel(&self, new_prel: bool) -> PathBuf {
        let s = self.art.as_str();
        if new_prel {
            format!("miz/prel/{}/{s}", &s[..1]).into()
        } else {
            format!("miz/mizshare/prel/{}/{s}", &s[..1]).into()
        }
    }

    pub fn to_path(&self, mml: bool, new_prel: bool, ext: &str) -> PathBuf {
        let mut path = if mml { self.mml() } else { self.prel(new_prel) };
        path.set_extension(ext);
        path
    }

    pub fn open(&self, mml: bool, new_prel: bool, ext: &str) -> io::Result<File> {
        let path = self.to_path(mml, new_prel, ext);
        // eprintln!("opening {}", path.to_str().unwrap());
        File::open(path)
    }

    pub fn read_miz(&self) -> io::Result<Vec<u8>> {
        let path = self.to_path(true, false, "miz");
        // eprintln!("opening {}", path.to_str().unwrap());
        std::fs::read(path)
    }

    #[allow(clippy::unwrap_used)]
    pub fn create(&self, mml: bool, new_prel: bool, ext: &str) -> io::Result<File> {
        let path = self.to_path(mml, new_prel, ext);
        std::fs::create_dir_all(path.parent().unwrap())?;
        // eprintln!("writing {}", path.to_str().unwrap());
        File::create(path)
    }
}
