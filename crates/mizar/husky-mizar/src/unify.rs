use crate::bignum::{Complex, Rational};
use crate::checker::{Atoms, Dnf, Open, OrUnsat, Overflow, Unsat};
use crate::equate::Equalizer;
use crate::types::*;
use crate::{
    vprintln, CheckLocus, EqCtx, Equate, ExpandPrivFunc, LocalContext, MizGlobal, Visit, VisitMut,
    WithGlobalLocal,
};
use enum_map::{Enum, EnumMap};
use itertools::Itertools;
use std::collections::{BTreeMap, HashMap};

const ENABLE_UNIFIER: bool = true;

#[derive(Default)]
struct MizEqTerm {
    ty_class: Vec<Type>,
    supercluster: Attrs,
    terms: EnumMap<ComplexTermKind, Vec<EqMarkId>>,
    number: Option<Complex>,
}

impl std::fmt::Debug for MizEqTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        LocalContext::with(|lc| {
            if let Some(lc) = lc {
                f.debug_list()
                    .entries(self.terms.values().flatten().map(|&m| &lc.marks[m].0))
                    .finish()
            } else {
                f.debug_list()
                    .entries(self.terms.values().flatten())
                    .finish()
            }
        })?;
        if let Some(n) = &self.number {
            write!(f, " = {n}")?
        }
        write!(f, ": {:?}{:?}", &self.supercluster, &self.ty_class)
    }
}

pub struct Unifier<'a> {
    g: &'a MizGlobal,
    lc: &'a mut LocalContext,
    infer: HashMap<InferId, EqClassId>,
    eq_class: MizIdxVec<EqClassId, MizEqTerm>,
    bas: &'a EnumMap<bool, Atoms>,
}

#[derive(Copy, Clone, Debug, Enum)]
enum ComplexTermKind {
    Functor,
    SchFunc,
    PrivFunc,
    Aggregate,
    Selector,
    Fraenkel,
    Choice,
}
use ComplexTermKind as CTK;

impl Term {
    fn complex_kind(&self) -> Option<ComplexTermKind> {
        match self {
            Term::Functor { .. } => Some(CTK::Functor),
            Term::SchFunc { .. } => Some(CTK::SchFunc),
            Term::PrivFunc { .. } => Some(CTK::PrivFunc),
            Term::Aggregate { .. } => Some(CTK::Aggregate),
            Term::Selector { .. } => Some(CTK::Selector),
            Term::Fraenkel { .. } => Some(CTK::Fraenkel),
            Term::The { .. } => Some(CTK::Choice),
            _ => None,
        }
    }
}

impl<'a> Unifier<'a> {
    /// InitUnifier
    pub fn new(eq: Equalizer<'a>, bas: &'a EnumMap<bool, Atoms>) -> Self {
        let mut u = Self {
            g: eq.g,
            lc: eq.lc,
            infer: Default::default(),
            eq_class: MizIdxVec::from_default(eq.next_eq_class.into_usize()),
            bas,
        };
        for etm in eq.terms.0 {
            let ec = &mut u.eq_class[etm.id];
            if !etm.eq_class.is_empty() {
                for m in etm.eq_class {
                    let t = &u.lc.marks[m].0;
                    match t.complex_kind() {
                        Some(k) => ec.terms[k].push(m),
                        None => match *t {
                            Term::Infer(i) => drop(u.infer.insert(i, etm.id)),
                            Term::Numeral(_) => {}
                            _ => unreachable!(),
                        },
                    }
                }
                ec.number = etm.number;
                ec.ty_class = etm.ty_class;
                ec.supercluster = etm.supercluster;
            }
        }
        if eq.g.cfg.unify_header && crate::verbose() {
            for (ec, etm) in u.eq_class.enum_iter() {
                vprintln!("e{ec:?} = {etm:?}");
            }
            for (i, j) in u.infer.iter().sorted() {
                vprintln!("{:?} = e{j:#?}", Term::Infer(*i));
            }
            for (pos, ats) in u.bas.iter() {
                for f in &ats.0 .0 {
                    vprintln!("{}: {f:?}", if pos { "assume" } else { "goal" });
                }
            }
        }
        u
    }

    /// Verify: Attempts to prove f |- false
    fn falsify(&mut self, mut f: Formula) -> Result<OrUnsat<()>, Overflow> {
        Standardize {
            g: self.g,
            lc: self.lc,
        }
        .visit_formula(&mut f);
        if self.g.cfg.unify_header {
            eprintln!("falsify: {f:?}");
        }
        let mut fvars = MizIdxVec::default();
        // Suppose f = ∀ xs, F(xs).
        // First, introduce metavariables ("free vars") to obtain a formula F(?v)
        OpenAsFreeVar(&mut fvars).open_quantifiers(&mut f, false);
        if self.g.cfg.unify_header {
            for (i, ty) in fvars.enum_iter() {
                vprintln!("v{i:?}: {ty:?}")
            }
        }
        let bas = self.bas;
        let mut u = self.unify(&fvars);

        // want to show: ∃ ?v. |- !F(?v)
        // Normalize !F(?v) into DNF: ∃ ?v. |- C_1(?v) \/ ... \/ C_n(?v)
        // If we get !F(?v) = true then we are done.
        let mut atoms = Atoms::default();
        let Dnf::Or(clauses) = atoms.normalize(u.g, u.lc, f, false)? else {
            return Ok(Err(Unsat));
        };
        if u.g.cfg.unify_header {
            vprintln!("atoms: {atoms:#?}");
        }

        // For the remainder we prove each clause separately.
        // Any of them being true will finish the goal.
        'next: for clause in clauses {
            if u.g.cfg.unify_header {
                vprintln!("clause: {clause:?}");
            }
            // We want to show: ∃ ?v. |- C(?v)
            assert!(!clause.0.is_empty()); // this would be a proof but is also not reachable

            // The strategy is to come up with an "assignment" P(?v) such that
            // ∃ ?v. P(?v) is true structurally and ∀ ?v. (P(?v) -> C(?v)) holds.
            // We write P(?v) in DNF, and ensure that each conjunct is satisfiable,
            // so it suffices to check that P(?v) is not identically false to ensure ∃ ?v. P(?v).

            let mut dnfs = vec![];
            // C(?v) is a conjunction A_1(?v) /\ ... /\ A_n(?v);
            // for each A_i(?v) we will construct P_i(?v) and AND them together
            for (a, val) in clause.0 {
                // Negate the conclusion to !A_i(?v) |- false to match the usual polarity,
                // and get an instantiation P_i(?v) such that P_i(?v), !A_i(?v) |- false.
                match u.compute_inst(bas, &atoms.0[a], !val)? {
                    // A_i(?v) is true without our help
                    Dnf::True => {}
                    // We failed to construct an instantiation,
                    // the strongest P_i(?v) we could come up with is 'false'
                    Dnf::Or(dnf) if dnf.is_empty() => continue 'next,
                    // Otherwise we push P_i(?v) on insts (we delay the join operation
                    // in case we can get one of the other two cases on some atoms)
                    Dnf::Or(dnf) => dnfs.push(dnf),
                }
            }
            // Unless /\_i P_i(?v) is the empty disjunction (false), it is satisfiable and we are done
            if !Dnf::and_many(dnfs)?.is_false() {
                return Ok(Err(Unsat));
            }
        }
        // falsification failed
        Ok(Ok(()))
    }

    /// Unifiable: Attempts to prove f_1, ..., f_n |- false
    fn resolution(&mut self, fs: &[&Formula]) -> Result<OrUnsat<()>, Overflow> {
        // CreateClauses
        let mut all_clauses = Dnf::FALSE;
        let mut atoms = Atoms::default();
        let mut fvars = MizIdxVec::default();
        // vprintln!("resolution: {fs:#?}");
        // We want to show |- !f_1 \/ ... \/ !f_n
        // Suppose f_i = ∀ xs, F_i(xs). Then !F_i(?v_i) implies !f_i,
        // so it suffices to show ∃ ?v_1 ... ?v_n. |- !F_1(?v_1) \/ ... \/ !F_n(?v_n)
        for f in fs {
            all_clauses.mk_or_else(|| {
                let mut f = f.visit_cloned(&mut Standardize {
                    g: self.g,
                    lc: self.lc,
                });
                OpenAsFreeVar(&mut fvars).open_quantifiers(&mut f, false);
                atoms.normalize(self.g, self.lc, f, false)
            })?;
        }
        // We normalized !F_1(?v_1) \/ ... \/ !F_n(?v_n) into DNF, as ∃ ?v. |- \/_i C_i(?v)
        // vprintln!("all_clauses = {all_clauses:#?}");
        let Dnf::Or(all_clauses) = all_clauses else {
            return Ok(Ok(()));
        };

        // This is not a complete procedure, we give up if there are not 2..=4 clauses C_i
        if all_clauses.len() < 2 || all_clauses.len() > 4 {
            return Ok(Ok(()));
        }

        // CollectComplementaryLiterals
        // vprintln!("atoms: {:#?}", atoms.0);
        let mut complementary = vec![];
        let bas = self.bas;
        let mut u = UnifyWithConst(self.unify(&fvars));
        // Each C_i is of the form /\_j A_ij, so we will look for "resolvents":
        // Suppose C and D are clauses such that C = C' /\ a and D = D' /\ !a;
        // then C \/ D = (C' /\ a) \/ (D' /\ !a) = if a { C' } else { D' }
        // so C' \/ D' implies C \/ D. So if we can prove C' \/ D' then we are done.
        //
        // In fact, we generalize this to the case where C has an atom a and D has !a'
        // and a and a' are unifiable, that is, the instantiation P(?v) implies a(?v) = a'(?v),
        // and also makes C'(?v) \/ D'(?v) true. Then as long as P(?v) is satisfiable we are done.
        for (cl1, cl2) in all_clauses.iter().tuple_combinations() {
            for (&a1, &val1) in &cl1.0 {
                for (&a2, &val2) in &cl2.0 {
                    if val1 != val2 && Similar.eq(u.0.g, u.0.lc, &atoms.0[a1], &atoms.0[a2]) {
                        if let Dnf::Or(dnf) = u.unify_basic_formula(&atoms.0[a1], &atoms.0[a2])? {
                            if !dnf.is_empty() {
                                // vprintln!("found resolvable clauses {cl1:?} <a{a1:?}!=a{a2:?}> {cl2:?} = {dnf:#?}");
                                complementary.push(([(cl1, a1), (cl2, a2)], dnf));
                            }
                        }
                    }
                }
            }
        }

        // This is a super sketchy test, since it is non-monotonic -
        // rejecting more possibilities can cause more proofs to check
        // if complementary.len() != 1 {
        //   return Ok(Ok(()))
        // }

        // This is heuristic but better than just failing when multiple
        // options are available. When there are several options but all of
        // them are equivalent then this method works
        complementary.truncate(1);

        // ResolventVerify
        'next: for (cls, dnf) in complementary {
            let mut dnfs = vec![dnf];
            for (cl, a1) in cls {
                for (&a2, &val) in &cl.0 {
                    if a2 != a1 {
                        match u.0.compute_inst(bas, &atoms.0[a2], !val)? {
                            Dnf::True => {}
                            Dnf::Or(dnf) if dnf.is_empty() => continue 'next,
                            Dnf::Or(dnf) => dnfs.push(dnf),
                        }
                    }
                }
            }
            if !Dnf::and_many(dnfs)?.is_false() {
                return Ok(Err(Unsat));
            }
        }

        Ok(Ok(()))
    }

    /// Unification
    pub fn run(&mut self) -> OrUnsat<()> {
        let univ = self.bas[true]
            .0
             .0
            .iter()
            .filter(|f| matches!(f, Formula::ForAll { .. }))
            .collect_vec();
        let mut overflow = false;
        let mut set_overflow = |Overflow| {
            overflow = true;
            Ok(())
        };
        for &f in &univ {
            match self.falsify(f.clone()) {
                Ok(or_unsat) => or_unsat?,
                Err(o) => {
                    set_overflow(o)?;
                    break;
                }
            }
        }
        if ENABLE_UNIFIER {
            for f in &univ {
                self.resolution(&[f]).unwrap_or_else(&mut set_overflow)?;
            }
            for (f1, f2) in univ.iter().tuple_combinations() {
                self.resolution(&[f1, f2])
                    .unwrap_or_else(&mut set_overflow)?;
            }
        }

        let mut fraenkel_fmlas = vec![];
        for (pos, bas) in self.bas.iter() {
            for f in &bas.0 .0 {
                if let Formula::Pred { nr, args } = f {
                    let (nr, args) = Formula::adjust_pred(*nr, args, Some(&self.g.constrs));
                    if self.g.reqs.belongs_to() == Some(nr) {
                        if let Some(ec) = args[1].unmark(self.lc).class() {
                            for &m in &self.eq_class[ec].terms[CTK::Fraenkel] {
                                if let Term::Fraenkel {
                                    args: tys,
                                    scope,
                                    compr,
                                } = &self.lc.marks[m].0
                                {
                                    let (tys, scope, compr) =
                                        (tys.clone(), (**scope).clone(), (**compr).clone());
                                    let mut fm = args[0].clone().not_in_fraenkel(
                                        tys,
                                        scope,
                                        compr,
                                        &self.g.reqs,
                                    );
                                    fm.distribute_quantifiers(&self.g.constrs, self.lc, 0);
                                    fraenkel_fmlas.push(fm.maybe_neg(!pos))
                                }
                            }
                        }
                        for f in fraenkel_fmlas.drain(..) {
                            self.falsify(f).unwrap_or_else(&mut set_overflow)?;
                        }
                    }
                }
            }
        }
        if overflow {
            vprintln!("note: at least one subproblem overflowed");
        }
        Ok(())
    }
}

struct OpenAsFreeVar<'a>(&'a mut MizIdxVec<FVarId, Type>);

impl Open for OpenAsFreeVar<'_> {
    fn mk_var(n: u32) -> Term {
        Term::FreeVar(FVarId(n))
    }
    fn base(&self) -> u32 {
        self.0.len() as u32
    }
    fn new_var(&mut self, ty: Type) {
        self.0.push(ty);
    }
}

impl Term {
    /// Given a fraenkel term `{ F(xs) where xs : P(xs) }` and a main term `self`,
    /// constructs the formula equivalent to `¬(self ∈ { F(xs) where xs : P(xs) })`,
    /// that is: `¬ ∃ xs, self = F(xs) /\ P(xs)`
    fn not_in_fraenkel(
        self,
        args: Box<[Type]>,
        scope: Term,
        compr: Formula,
        reqs: &RequirementIndexes,
    ) -> Formula {
        let mut conjs = vec![reqs.mk_eq(self, scope)];
        compr.append_conjuncts_to(&mut conjs);
        let mut f = Formula::Neg {
            f: Box::new(Formula::And { args: conjs }),
        };
        for ty in args.into_vec().into_iter().rev() {
            f = Formula::forall(ty, f)
        }
        f
    }
}

struct Standardize<'a> {
    g: &'a MizGlobal,
    lc: &'a mut LocalContext,
}

impl VisitMut for Standardize<'_> {
    fn visit_term(&mut self, _: &mut Term) {}
    fn visit_terms(&mut self, _: &mut [Term]) {}

    fn visit_type(&mut self, ty: &mut Type) {
        assert!(!CheckLocus::get(|cl| {
            cl.visit_attrs(&ty.attrs.0);
            cl.visit_attrs(&ty.attrs.1);
        }));
        self.visit_terms(&mut ty.args);
    }

    fn visit_formula(&mut self, f: &mut Formula) {
        self.standardize_formula(f, true)
    }
}

impl Standardize<'_> {
    /// * pos = true: PositivelyStandardized
    /// * pos = false: NegativelyStandardized
    fn standardize_formula(&mut self, f: &mut Formula, pos: bool) {
        loop {
            match f {
                Formula::Neg { f: f2 } => {
                    self.standardize_formula(f2, !pos);
                    if let Formula::Neg { f: f3 } = &mut **f2 {
                        *f = std::mem::take(f3)
                    }
                }
                Formula::And { args } => args
                    .iter_mut()
                    .for_each(|f| self.standardize_formula(f, pos)),
                Formula::ForAll { dom, scope } => {
                    if pos {
                        self.visit_type(dom);
                        self.lc.bound_var.push(std::mem::take(dom));
                        self.standardize_formula(scope, pos);
                        **dom = self.lc.bound_var.0.pop().unwrap();
                    }
                }
                Formula::SchPred { args, .. } | Formula::Pred { args, .. } => {
                    self.visit_terms(args)
                }
                Formula::Attr { nr, args } => {
                    let (main, rest) = args.split_last_mut().unwrap();
                    self.visit_term(main);
                    if !matches!(main.unmark(self.lc), Term::EqClass(_)) {
                        let attr = MizAttr {
                            nr: *nr,
                            pos: true,
                            args: rest.to_owned().into(),
                        };
                        *f = Box::new(std::mem::take(main)).mk_is(self.g, self.lc, attr);
                        continue;
                    }
                }
                Formula::PrivPred { args, value, .. } => {
                    self.visit_terms(args);
                    ExpandPrivFunc(&self.g.constrs, self.lc).visit_formula(value);
                }
                Formula::Is { term, ty } => {
                    self.visit_term(term);
                    self.visit_type(ty)
                }
                Formula::LegacyFlexAnd { .. } | Formula::FlexAnd { .. } | Formula::True => {}
            }
            break;
        }
    }
}

impl Term {
    /// ChReconQualFrm
    fn mk_is(self: Box<Term>, g: &MizGlobal, lc: &LocalContext, attr: MizAttr) -> Formula {
        let mut ty = self.get_type_uncached(g, lc);
        ty.attrs.0.insert(Some(&g.constrs), lc, attr);
        if matches!(ty.attrs.0, Attrs::Inconsistent) {
            Formula::Neg {
                f: Box::new(Formula::True),
            }
        } else {
            ty.attrs.1 = ty.attrs.0.clone();
            Formula::Is {
                term: self,
                ty: Box::new(ty),
            }
        }
    }
}

struct Unify<'a> {
    g: &'a MizGlobal,
    lc: &'a LocalContext,
    infer: &'a HashMap<InferId, EqClassId>,
    eq_class: &'a MizIdxVec<EqClassId, MizEqTerm>,
    fvars: &'a MizIdxVec<FVarId, Type>,
    cache: BTreeMap<(FVarId, EqClassId), Dnf<FVarId, EqClassId>>,
    base: u32,
    depth: u32,
}
impl WithGlobalLocal for Unify<'_> {
    fn global(&self) -> &MizGlobal {
        self.g
    }
    fn local(&self) -> &LocalContext {
        self.lc
    }
}

impl Unifier<'_> {
    fn unify<'a>(&'a mut self, fvars: &'a MizIdxVec<FVarId, Type>) -> Unify<'a> {
        Unify {
            g: self.g,
            lc: self.lc,
            infer: &self.infer,
            eq_class: &self.eq_class,
            fvars,
            cache: Default::default(),
            base: 0,
            depth: 0,
        }
    }
}

impl Unify<'_> {
    /// Constructs an instantiation P(?v) such that
    /// * pos = true: COMPInstAsTrue - P(?v) /\ F(?v) |- false
    /// * pos = false: COMPInstAsFalse - P(?v) /\ !F(?v) |- false
    fn compute_inst(
        &mut self,
        bas: &EnumMap<bool, Atoms>,
        f: &Formula,
        pos: bool,
    ) -> Result<Dnf<FVarId, EqClassId>, Overflow> {
        if self.g.cfg.unify_insts {
            vprintln!("compute_inst {pos}: {f:?}");
        }
        let mut inst = Dnf::FALSE;
        let mut skip = false;
        match *f {
            // We already DNF'd f so there should be no top-level propositional connectives remaining
            Formula::True | Formula::Neg { .. } | Formula::And { .. } => unreachable!(),
            Formula::Pred { nr, ref args } => {
                let props = self.g.constrs.predicate[nr].properties;
                if props.get(if pos {
                    PropertyKind::Irreflexivity
                } else {
                    PropertyKind::Reflexivity
                }) {
                    for (ec, _) in self.eq_class.enum_iter() {
                        let t = Term::EqClass(ec);
                        let mut inst1 = self.unify_term(&args[props.arg1 as usize], &t)?;
                        if !inst1.is_false() {
                            inst1.mk_and(self.unify_term(&args[props.arg2 as usize], &t)?)?;
                            inst.mk_or(inst1)?;
                        }
                    }
                }
                let (nr, args) = Formula::adjust_pred(nr, args, Some(&self.g.constrs));
                if self.g.reqs.belongs_to() == Some(nr) {
                    let [arg1, arg2] = args else { unreachable!() };
                    if let Some(empty) = self.g.reqs.empty() {
                        for (ec, etm) in self.eq_class.enum_iter() {
                            if etm.supercluster.find0(&self.g.constrs, empty, pos) {
                                let mut inst1 = self.unify_term(arg2, &Term::EqClass(ec))?;
                                if !inst1.is_false() {
                                    if pos {
                                        // x in A, A is empty |- false
                                        inst.mk_or(inst1)?;
                                    } else if let Some(element) = self.g.reqs.element() {
                                        let ty = Type {
                                            args: vec![Term::EqClass(ec)],
                                            ..Type::new(element.into())
                                        };
                                        let mut inst2 = Dnf::FALSE;
                                        for (ec2, etm2) in self.eq_class.enum_iter() {
                                            if etm2
                                                .ty_class
                                                .iter()
                                                .any(|ty2| self.eq_radices(ty2, &ty))
                                            {
                                                // !(x in A), A is not empty, x is Element of A |- false
                                                inst2.mk_or_else(|| {
                                                    self.unify_term(arg1, &Term::EqClass(ec2))
                                                })?;
                                            }
                                        }
                                        inst1.mk_and(inst2)?;
                                        inst.mk_or(inst1)?;
                                    }
                                }
                            }
                        }
                    }
                    if pos {
                        for f in &bas[false].0 .0 {
                            if let Formula::Is { term, ty } = f {
                                if let TypeKind::Mode(n) = ty.kind {
                                    let (n, args) = Type::adjust(n, &ty.args, &self.g.constrs);
                                    if self.g.reqs.element() == Some(n) {
                                        let mut inst1 = self.unify_term(arg1, term)?;
                                        if !inst1.is_false() {
                                            inst1.mk_and(self.unify_term(arg2, &args[0])?)?;
                                            inst.mk_or(inst1)?;
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else if self.g.reqs.inclusion() == Some(nr) {
                    if let Some(power) = self.g.reqs.power_set() {
                        let [arg1, arg2] = args else { unreachable!() };
                        if pos {
                            for f in &bas[false].0 .0 {
                                if let Formula::Is { term, ty } = f {
                                    if let TypeKind::Mode(n) = ty.kind {
                                        let (n, args) = Type::adjust(n, &ty.args, &self.g.constrs);
                                        if self.g.reqs.element() == Some(n) {
                                            let mut inst1 = self.unify_term(arg1, term)?;
                                            if !inst1.is_false() {
                                                if let Term::EqClass(ec) = *arg2 {
                                                    let Term::EqClass(ec2) = args[0] else {
                                                        unreachable!()
                                                    };
                                                    if self.get_eq_class(&Term::Functor {
                                                        nr: power,
                                                        args: Box::new([Term::EqClass(ec)]),
                                                    }) == Some(ec2)
                                                    {
                                                        inst.mk_or(inst1)?
                                                    }
                                                } else {
                                                    let t = Term::Functor {
                                                        nr: power,
                                                        args: Box::new([arg2.clone()]),
                                                    };
                                                    inst1.mk_and(self.unify_term(&t, &args[0])?)?;
                                                    inst.mk_or(inst1)?
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else if let Some(element) = self.g.reqs.element() {
                            for ec1 in (0..self.eq_class.len()).map(EqClassId::from_usize) {
                                let mut inst1 = self.unify_term(arg2, &Term::EqClass(ec1))?;
                                if !inst1.is_false() {
                                    let mut inst2 = Dnf::FALSE;
                                    if let Some(ec2) = self.get_eq_class(&Term::Functor {
                                        nr: power,
                                        args: Box::new([Term::EqClass(ec1)]),
                                    }) {
                                        let ty = Type {
                                            args: vec![Term::EqClass(ec2)],
                                            ..Type::new(element.into())
                                        };
                                        for (ec2, etm2) in self.eq_class.enum_iter() {
                                            if etm2
                                                .ty_class
                                                .iter()
                                                .any(|ty2| self.eq_radices(ty2, &ty))
                                            {
                                                inst2.mk_or_else(|| {
                                                    self.unify_term(arg1, &Term::EqClass(ec2))
                                                })?;
                                            }
                                        }
                                        inst1.mk_and(inst2)?;
                                        inst.mk_or(inst1)?
                                    }
                                }
                            }
                        }
                    }
                    // FIXME: the original control flow seems very haphazard/inconsistent here
                    if !pos {
                        skip = true
                    }
                } else if self.g.reqs.less_or_equal() == Some(nr) {
                    let [arg1, arg2] = args else { unreachable!() };
                    if !pos {
                        for f2 in &bas[true].0 .0 {
                            inst.mk_or_else(|| self.unify_formula(f, f2))?;
                        }
                    }
                    for (ec1, etm1) in self.eq_class.enum_iter() {
                        if let Some(n1) = &etm1.number {
                            let mut inst1 = self.unify_term(arg1, &Term::EqClass(ec1))?;
                            if !inst1.is_false() {
                                let mut inst2 = Dnf::FALSE;
                                for (ec2, etm2) in self.eq_class.enum_iter() {
                                    if let Some(n2) = &etm2.number {
                                        if n1.im == Rational::ZERO
                                            && n2.im == Rational::ZERO
                                            && (n1.re <= n2.re) != pos
                                        {
                                            inst2.mk_or(
                                                self.unify_term(arg2, &Term::EqClass(ec2))?,
                                            )?;
                                        }
                                    }
                                }
                                inst1.mk_and(inst2)?;
                                inst.mk_or(inst1)?;
                            }
                        }
                    }
                    if let (Some(positive), Some(negative)) =
                        (self.g.reqs.positive(), self.g.reqs.negative())
                    {
                        for (ec1, etm1) in self.eq_class.enum_iter() {
                            let mut inst1 = self.unify_term(arg1, &Term::EqClass(ec1))?;
                            if !inst1.is_false() {
                                let mut inst2 = Dnf::FALSE;
                                if pos {
                                    let pos1 =
                                        etm1.supercluster.find0(&self.g.constrs, positive, true);
                                    let nonneg1 =
                                        etm1.supercluster.find0(&self.g.constrs, negative, false);
                                    for (ec2, etm2) in self.eq_class.enum_iter() {
                                        if pos1
                                            && etm2.supercluster.find0(
                                                &self.g.constrs,
                                                positive,
                                                false,
                                            )
                                            || nonneg1
                                                && etm2.supercluster.find0(
                                                    &self.g.constrs,
                                                    negative,
                                                    true,
                                                )
                                        {
                                            inst2.mk_or_else(|| {
                                                self.unify_term(arg2, &Term::EqClass(ec2))
                                            })?;
                                        }
                                    }
                                } else {
                                    let nonpos1 =
                                        etm1.supercluster.find0(&self.g.constrs, positive, false);
                                    for (ec2, etm2) in self.eq_class.enum_iter() {
                                        if nonpos1
                                            && etm2.supercluster.find0(
                                                &self.g.constrs,
                                                negative,
                                                false,
                                            )
                                        {
                                            inst2.mk_or_else(|| {
                                                self.unify_term(arg2, &Term::EqClass(ec2))
                                            })?;
                                        }
                                    }
                                }
                                inst1.mk_and(inst2)?;
                                inst.mk_or(inst1)?;
                            }
                        }
                    }
                    if !pos {
                        skip = true
                    }
                } else if self.g.reqs.equals_to() == Some(nr) {
                    if pos {
                        let [arg1, arg2] = args else { unreachable!() };
                        for (ec1, etm1) in self.eq_class.enum_iter() {
                            if let Some(n1) = &etm1.number {
                                let mut inst1 = self.unify_term(arg1, &Term::EqClass(ec1))?;
                                if !inst1.is_false() {
                                    let mut inst2 = Dnf::FALSE;
                                    for (ec2, etm2) in self.eq_class.enum_iter() {
                                        if ec1 != ec2 {
                                            if let Some(n2) = &etm2.number {
                                                assert!(n1 != n2);
                                                inst2.mk_or(
                                                    self.unify_term(arg2, &Term::EqClass(ec2))?,
                                                )?;
                                            }
                                        }
                                    }
                                    inst1.mk_and(inst2)?;
                                    inst.mk_or(inst1)?
                                }
                            }
                        }
                    } else {
                        skip = true
                    }
                }
            }
            Formula::Attr { nr, ref args } => {
                // UniAttrFrm
                let (arg0, args) = args.split_last().unwrap();
                if let Some(ec) = arg0.unmark(self.lc).class() {
                    for attr in self.eq_class[ec].supercluster.attrs() {
                        if attr.nr == nr && attr.pos != pos {
                            inst.mk_or_else(|| self.unify_terms(args, &attr.args))?;
                        }
                    }
                }
                skip = true
            }
            Formula::Is { ref term, ref ty } => {
                if pos {
                    for f2 in &bas[!pos].0 .0 {
                        if let Formula::Is {
                            term: term2,
                            ty: ty2,
                        } = f2
                        {
                            let mut inst1 = self.unify_term(term, term2)?;
                            if !inst1.is_false() {
                                let mut inst2 = Dnf::FALSE;
                                match ty2.kind {
                                    TypeKind::Mode(n2) => {
                                        if let TypeKind::Mode(_) = ty.kind {
                                            let n2 = Type::adjust(n2, &ty2.args, &self.g.constrs).0;
                                            assert!(n2 != ModeId::ANY);
                                            let mut pty = CowBox::Borrowed(&**ty);
                                            while let TypeKind::Mode(pn) = pty.kind {
                                                if pn < n2 {
                                                    break;
                                                }
                                                inst2.mk_or_else(|| {
                                                    self.unify_radix_type(&pty, ty2)
                                                })?;
                                                pty = CowBox::Owned(
                                                    pty.widening(self.g, self.lc).unwrap(),
                                                );
                                            }
                                        }
                                    }
                                    TypeKind::Struct(_) => {
                                        if let Some(ty) = ty2.widening_of(self.g, self.lc, ty) {
                                            inst2 = self.unify_radix_type(&ty, ty2)?;
                                        }
                                    }
                                }
                                inst1.mk_and(inst2)?;
                                inst.mk_or(inst1)?;
                            }
                        }
                    }
                    for (ec, etm) in self.eq_class.enum_iter() {
                        let mut inst1 = self.unify_term(term, &Term::EqClass(ec))?;
                        if !inst1.is_false() {
                            let mut inst2 = Dnf::FALSE;
                            for attr in ty.attrs.1.attrs() {
                                for attr2 in etm.supercluster.attrs() {
                                    self.or_unify_attr(attr, attr2, false, &mut inst2)?
                                }
                            }
                            inst1.mk_and(inst2)?;
                            inst.mk_or(inst1)?;
                        }
                    }
                } else {
                    for (ec, etm) in self.eq_class.enum_iter() {
                        let mut inst1 = self.unify_term(term, &Term::EqClass(ec))?;
                        if !inst1.is_false() {
                            inst1.mk_and(self.unify_eq_class_types(etm, ty)?)?;
                            inst.mk_or(inst1)?;
                        }
                    }
                }
                skip = true
            }
            _ => {}
        }
        if !skip {
            for f2 in &bas[!pos].0 .0 {
                inst.mk_or_else(|| self.unify_formula(f, f2))?;
            }
        }
        if self.g.cfg.unify_insts {
            vprintln!("compute_inst (skip: {skip}) -> {inst:?}");
        }
        Ok(inst)
    }

    fn get_eq_class(&self, tm: &Term) -> Option<EqClassId> {
        self.equate_class().get(self.g, self.lc, tm)
    }

    /// InstCollection.UNIEqClassTyps
    fn unify_eq_class_types(
        &mut self,
        ec: &MizEqTerm,
        ty: &Type,
    ) -> Result<Dnf<FVarId, EqClassId>, Overflow> {
        // vprintln!("unify_eq_class_types {ec:?} as {ty:?}");
        let mut inst = Dnf::FALSE;
        for ty2 in &ec.ty_class {
            inst.mk_or_else(|| self.unify_radix_type(ty, ty2))?
        }
        inst.mk_and_then(|| self.unify_subset_attrs(&ty.attrs.0, &ec.supercluster, true))?;
        Ok(inst)
    }

    /// * pos = true: InstCollection.UNIAttr
    /// * pos = false: InstCollection.UNIAttr1
    fn or_unify_attr(
        &mut self,
        attr1: &MizAttr,
        attr2: &MizAttr,
        pos: bool,
        out: &mut Dnf<FVarId, EqClassId>,
    ) -> Result<(), Overflow> {
        // vprintln!("or_unify_attr {pos}: {attr1:?} <> {attr2:?} <- {out:?}");
        let (n1, args1) = attr1.adjust(Some(&self.g.constrs));
        let (n2, args2) = attr2.adjust(Some(&self.g.constrs));
        if n1 == n2 && (attr1.pos == attr2.pos) == pos {
            out.mk_or_else(|| self.unify_terms(args1, args2))?
        }
        // vprintln!("or_unify_attr {pos}: {attr1:?} <> {attr2:?} -> {out:?}");
        Ok(())
    }

    /// InstCollection.UNIRadices
    fn unify_radix_type(
        &mut self,
        ty1: &Type,
        ty2: &Type,
    ) -> Result<Dnf<FVarId, EqClassId>, Overflow> {
        match (ty1.kind, ty2.kind) {
            (TypeKind::Mode(n1), TypeKind::Mode(n2)) => {
                let (n1, args1) = Type::adjust(n1, &ty1.args, &self.g.constrs);
                let (n2, args2) = Type::adjust(n2, &ty2.args, &self.g.constrs);
                if n1 == n2 {
                    self.unify_terms(args1, args2)
                } else {
                    Ok(Dnf::FALSE)
                }
            }
            (TypeKind::Struct(n1), TypeKind::Struct(n2)) if n1 == n2 => {
                self.unify_terms(&ty1.args, &ty2.args)
            }
            _ => Ok(Dnf::FALSE),
        }
    }

    /// InstCollection.UNIInclClusters
    fn unify_subset_attrs(
        &mut self,
        attrs1: &Attrs,
        attrs2: &Attrs,
        fwd: bool,
    ) -> Result<Dnf<FVarId, EqClassId>, Overflow> {
        let Attrs::Consistent(attrs1) = attrs1 else {
            unreachable!()
        };
        let Attrs::Consistent(attrs2) = attrs2 else {
            unreachable!()
        };
        let mut inst = Dnf::True;
        let mut it2 = attrs2
            .iter()
            .map(|a| (a, a.adjusted_nr(&self.g.constrs)))
            .peekable();
        for attr1 in attrs1 {
            let n = attr1.adjusted_nr(&self.g.constrs);
            loop {
                let Some(a) = it2.peek() else {
                    return Ok(Dnf::FALSE);
                };
                if a.1 >= n {
                    break;
                }
                it2.next();
            }
            let mut inst1 = Dnf::FALSE;
            for (attr2, _) in it2.clone().take_while(|a| a.1 <= n) {
                if fwd {
                    self.or_unify_attr(attr1, attr2, true, &mut inst1)?;
                } else {
                    self.or_unify_attr(attr2, attr1, true, &mut inst1)?;
                }
            }
            inst.mk_and(inst1)?;
            if inst.is_false() {
                break;
            }
        }
        Ok(inst)
    }

    /// InstCollection.UNITyp
    fn unify_type(&mut self, ty1: &Type, ty2: &Type) -> Result<Dnf<FVarId, EqClassId>, Overflow> {
        match (ty1.kind, ty2.kind) {
            (TypeKind::Struct(_), TypeKind::Struct(_)) | (TypeKind::Mode(_), TypeKind::Mode(_)) => {
                let mut inst = self.unify_subset_attrs(&ty1.attrs.0, &ty2.attrs.1, true)?;
                inst.mk_and_then(|| self.unify_subset_attrs(&ty2.attrs.0, &ty1.attrs.1, false))?;
                inst.mk_and_then(|| self.unify_radix_type(ty1, ty2))?;
                Ok(inst)
            }
            _ => Ok(Dnf::FALSE),
        }
    }

    /// InstCollection.UNIFunc
    fn unify_func(
        &mut self,
        n1: FuncId,
        args1: &[Term],
        t2: &Term,
    ) -> Result<Dnf<FVarId, EqClassId>, Overflow> {
        let Term::Functor {
            nr: n2,
            args: ref args2,
        } = *t2
        else {
            return Ok(Dnf::FALSE);
        };
        // vprintln!("unify: {:?} =?= {:?}", args1, t2);
        let (n1, args1) = Term::adjust(n1, args1, Some(&self.g.constrs));
        let (n2, args2) = Term::adjust(n2, args2, Some(&self.g.constrs));
        if n1 == n2 {
            self.unify_terms(args1, args2)
        } else {
            Ok(Dnf::FALSE)
        }
    }

    /// InstCollection.UNIFraenkelTrm
    fn unify_fraenkel(
        &mut self,
        args1: &[Type],
        scope1: &Term,
        compr1: &Formula,
        args2: &[Type],
        scope2: &Term,
        compr2: &Formula,
    ) -> Result<Dnf<FVarId, EqClassId>, Overflow> {
        if args1.len() != args2.len() {
            return Ok(Dnf::FALSE);
        }
        let depth = self.depth;
        let mut inst = Dnf::True;
        for (ty1, ty2) in args1.iter().zip(args2) {
            inst.mk_and_then(|| self.unify_type(ty1, ty2))?;
            self.depth += 1;
        }
        inst.mk_and_then(|| self.unify_term(scope1, scope2))?;
        inst.mk_and_then(|| self.unify_formula(compr1, compr2))?;
        self.depth = depth;
        Ok(inst)
    }

    /// InstCollection.UNITrm
    fn unify_term(&mut self, t1: &Term, t2: &Term) -> Result<Dnf<FVarId, EqClassId>, Overflow> {
        macro_rules! function_like {
            ($tk:ident { $nr:expr, $args:expr }) => {{
                let mut inst = match t2 {
                    Term::$tk { nr, args, .. } if $nr == *nr => self.unify_terms($args, args)?,
                    _ => Dnf::FALSE,
                };
                if let Some(ec) = self.get_eq_class(t2) {
                    for &m in &self.eq_class[ec].terms[CTK::$tk] {
                        let Term::$tk { nr, args, .. } = &self.lc.marks[m].0 else {
                            unreachable!()
                        };
                        if $nr == *nr {
                            inst.mk_or_else(|| self.unify_terms($args, args))?
                        }
                    }
                }
                inst
            }};
        }
        // vprintln!("unify_term {t1:?} <> {t2:?}");
        let res = match *t1 {
            Term::FreeVar(n) => {
                if let Some(ec) = self.get_eq_class(t2) {
                    if let Some(inst) = self.cache.get(&(n, ec)) {
                        inst.clone()
                    } else {
                        let mut inst =
                            self.unify_eq_class_types(&self.eq_class[ec], &self.fvars[n].clone())?;
                        inst.mk_and_single(n, ec);
                        self.cache.insert((n, ec), inst.clone());
                        inst
                    }
                } else {
                    Dnf::FALSE
                }
            }
            Term::Bound(n1) => {
                Dnf::mk_bool(matches!(t2, Term::Bound(n2) if n1.0 - self.base == n2.0))
            }
            Term::Functor { nr, ref args } => {
                let mut inst = self.unify_func(nr, args, t2)?;
                if let Some(ec) = self.get_eq_class(t2) {
                    for &m in &self.eq_class[ec].terms[CTK::Functor] {
                        inst.mk_or_else(|| self.unify_func(nr, args, &self.lc.marks[m].0))?
                    }
                }
                inst
            }
            Term::Aggregate { nr, ref args } => function_like!(Aggregate { nr, args }),
            Term::SchFunc { nr, ref args } => function_like!(SchFunc { nr, args }),
            Term::PrivFunc { nr, ref args, .. } => function_like!(PrivFunc { nr, args }),
            Term::Selector { nr, ref args } => function_like!(Selector { nr, args }),
            Term::Fraenkel {
                args: ref a1,
                scope: ref s1,
                compr: ref c1,
            } => {
                let mut inst = if let Term::Fraenkel {
                    args: a2,
                    scope: s2,
                    compr: c2,
                } = t2
                {
                    self.unify_fraenkel(a1, s1, c1, a2, s2, c2)?
                } else {
                    Dnf::FALSE
                };
                let base = self.base;
                self.base = self.depth;
                if let Some(ec) = self.get_eq_class(t2) {
                    for &m in &self.eq_class[ec].terms[CTK::Fraenkel] {
                        let Term::Fraenkel {
                            args: a2,
                            scope: s2,
                            compr: c2,
                        } = &self.lc.marks[m].0
                        else {
                            unreachable!()
                        };
                        inst.mk_or_else(|| self.unify_fraenkel(a1, s1, c1, a2, s2, c2))?
                    }
                }
                self.base = base;
                inst
            }
            Term::The { ref ty } => {
                let mut inst = if let Term::The { ty: ty2 } = t2 {
                    self.unify_type(ty, ty2)?
                } else {
                    Dnf::FALSE
                };
                if let Some(ec) = self.get_eq_class(t2) {
                    for &m in &self.eq_class[ec].terms[CTK::Choice] {
                        let Term::The { ty: ty2 } = &self.lc.marks[m].0 else {
                            unreachable!()
                        };
                        inst.mk_or_else(|| self.unify_type(ty, ty2))?
                    }
                }
                inst
            }
            Term::EqClass(n) => Dnf::mk_bool(self.get_eq_class(t2) == Some(n)),
            Term::Numeral(_) | Term::Const(_) | Term::Infer(_) => {
                Dnf::mk_bool(Some(self.get_eq_class(t1).unwrap()) == self.get_eq_class(t2))
            }
            Term::EqMark(m) => self.unify_term(&self.lc.marks[m].0, t2)?,
            Term::Locus(_) | Term::Qua { .. } | Term::It => unreachable!(),
        };
        // vprintln!("unify_term {t1:?} <> {t2:?} -> {res:?}");
        Ok(res)
    }

    /// InstCollection.UNITrmList
    fn unify_terms(
        &mut self,
        tms1: &[Term],
        tms2: &[Term],
    ) -> Result<Dnf<FVarId, EqClassId>, Overflow> {
        assert!(tms1.len() == tms2.len());
        let mut res = Dnf::True;
        for (t1, t2) in tms1.iter().zip(tms2) {
            res.mk_and(self.unify_term(t1, t2)?)?;
            if res.is_false() {
                break;
            }
        }
        Ok(res)
    }

    /// InstCollection.UNIFrm
    fn unify_formula(
        &mut self,
        f1: &Formula,
        f2: &Formula,
    ) -> Result<Dnf<FVarId, EqClassId>, Overflow> {
        // vprintln!("unify_formula {f1:?} <> {f2:?}");
        let res = match (f1, f2) {
            (Formula::True, Formula::True) => Dnf::True,
            (Formula::Neg { f: f1 }, Formula::Neg { f: f2 }) => self.unify_formula(f1, f2)?,
            (Formula::And { args: args1 }, Formula::And { args: args2 })
                if args1.len() == args2.len() =>
            {
                let mut res = Dnf::True;
                for (f1, f2) in args1.iter().zip(args2) {
                    res.mk_and_then(|| self.unify_formula(f1, f2))?
                }
                res
            }
            (
                Formula::Pred {
                    nr: n1,
                    args: args1,
                },
                Formula::Pred {
                    nr: n2,
                    args: args2,
                },
            ) => {
                let (n1_adj, args1_adj) = Formula::adjust_pred(*n1, args1, Some(&self.g.constrs));
                let (n2_adj, args2_adj) = Formula::adjust_pred(*n2, args2, Some(&self.g.constrs));
                if n1_adj != n2_adj {
                    return Ok(Dnf::FALSE);
                }
                let mut inst = self.unify_terms(args1_adj, args2_adj)?;
                let c = &self.g.constrs.predicate[*n1];
                if c.properties.get(PropertyKind::Symmetry) {
                    inst.mk_or_else(|| {
                        let mut args1 = args1.to_vec();
                        args1.swap(c.properties.arg1 as usize, c.properties.arg2 as usize);
                        self.unify_terms(&args1[c.superfluous as usize..], args2_adj)
                    })?;
                }
                let c = &self.g.constrs.predicate[*n2];
                if c.properties.get(PropertyKind::Symmetry) {
                    inst.mk_or_else(|| {
                        let mut args2 = args2.to_vec();
                        args2.swap(c.properties.arg1 as usize, c.properties.arg2 as usize);
                        self.unify_terms(args1_adj, &args2[c.superfluous as usize..])
                    })?;
                }
                inst
            }
            (
                Formula::SchPred {
                    nr: SchPredId(n1),
                    args: args1,
                },
                Formula::SchPred {
                    nr: SchPredId(n2),
                    args: args2,
                },
            )
            | (
                Formula::PrivPred {
                    nr: PrivPredId(n1),
                    args: args1,
                    ..
                },
                Formula::PrivPred {
                    nr: PrivPredId(n2),
                    args: args2,
                    ..
                },
            ) if n1 == n2 => self.unify_terms(args1, args2)?,
            (
                Formula::Attr {
                    nr: n1,
                    args: args1,
                },
                Formula::Attr {
                    nr: n2,
                    args: args2,
                },
            ) => {
                let (n1, args1) = Formula::adjust_attr(*n1, args1, Some(&self.g.constrs));
                let (n2, args2) = Formula::adjust_attr(*n2, args2, Some(&self.g.constrs));
                if n1 == n2 {
                    self.unify_terms(args1, args2)?
                } else {
                    Dnf::FALSE
                }
            }
            (
                Formula::ForAll {
                    dom: dom1,
                    scope: sc1,
                },
                Formula::ForAll {
                    dom: dom2,
                    scope: sc2,
                },
            ) => {
                let mut inst = self.unify_type(dom1, dom2)?;
                self.depth += 1;
                inst.mk_and_then(|| self.unify_formula(sc1, sc2))?;
                self.depth -= 1;
                inst
            }
            (Formula::Is { term: tm1, ty: ty1 }, Formula::Is { term: tm2, ty: ty2 }) => {
                let mut inst = self.unify_term(tm1, tm2)?;
                inst.mk_and_then(|| self.unify_type(ty1, ty2))?;
                inst
            }
            (
                Formula::FlexAnd {
                    terms: t1,
                    scope: sc1,
                    ..
                },
                Formula::FlexAnd {
                    terms: t2,
                    scope: sc2,
                    ..
                },
            ) => {
                let mut inst = self.unify_term(&t1[0], &t2[0])?;
                inst.mk_and_then(|| self.unify_term(&t1[1], &t2[1]))?;
                self.depth += 1;
                inst.mk_and_then(|| self.unify_formula(sc1, sc2))?;
                self.depth -= 1;
                inst
            }
            (Formula::LegacyFlexAnd { orig: t1, .. }, Formula::LegacyFlexAnd { orig: t2, .. }) => {
                let mut inst = self.unify_formula(&t1[0], &t2[0])?;
                inst.mk_and_then(|| self.unify_formula(&t1[1], &t2[1]))?;
                inst
            }
            _ => Dnf::FALSE,
        };
        // vprintln!("unify_formula {f1:?} <> {f2:?} -> {res:?}");
        Ok(res)
    }
}

struct EquateClass<'a> {
    infer: &'a HashMap<InferId, EqClassId>,
    eq_class: &'a MizIdxVec<EqClassId, MizEqTerm>,
}

impl Unify<'_> {
    fn equate_class(&self) -> EquateClass<'_> {
        EquateClass {
            infer: self.infer,
            eq_class: self.eq_class,
        }
    }
}

struct UnifyWithConst<'a>(Unify<'a>);

impl UnifyWithConst<'_> {
    /// InstCollection.UnifyTrmsWithConsts
    fn unify_term(&mut self, t1: &Term, t2: &Term) -> Result<Dnf<FVarId, EqClassId>, Overflow> {
        let mut inst = Dnf::FALSE;
        for (ec, _) in self.0.eq_class.enum_iter() {
            let t = Term::EqClass(ec);
            let mut inst1 = self.0.unify_term(t1, &t)?;
            if !inst1.is_false() {
                inst1.mk_and(self.0.unify_term(t2, &t)?)?;
                inst.mk_or(inst1)?;
            }
        }
        Ok(inst)
    }

    /// InstCollection.UnifyTrmList
    fn unify_terms(
        &mut self,
        tms1: &[Term],
        tms2: &[Term],
    ) -> Result<Dnf<FVarId, EqClassId>, Overflow> {
        assert!(tms1.len() == tms2.len());
        let mut dnfs = Vec::with_capacity(tms1.len());
        for (t1, t2) in tms1.iter().zip(tms2) {
            match self.unify_term(t1, t2)? {
                Dnf::True => continue,
                Dnf::Or(dnf) if dnf.is_empty() => return Ok(Dnf::FALSE),
                Dnf::Or(dnf) => dnfs.push(dnf),
            }
        }
        Dnf::and_many(dnfs)
    }

    /// * pos = true: InstCollection.UNIAttr
    /// * pos = false: InstCollection.UNIAttr1
    fn or_unify_attr(
        &mut self,
        attr1: &MizAttr,
        attr2: &MizAttr,
        out: &mut Dnf<FVarId, EqClassId>,
    ) -> Result<(), Overflow> {
        let (n1, args1) = attr1.adjust(Some(&self.0.g.constrs));
        let (n2, args2) = attr2.adjust(Some(&self.0.g.constrs));
        if n1 == n2 && attr1.pos == attr2.pos {
            out.mk_or_else(|| self.unify_terms(args1, args2))?
        }
        Ok(())
    }

    /// InstCollection.UnifyInclClusters
    fn unify_subset_attrs(
        &mut self,
        attrs1: &Attrs,
        attrs2: &Attrs,
    ) -> Result<Dnf<FVarId, EqClassId>, Overflow> {
        let Attrs::Consistent(attrs1) = attrs1 else {
            unreachable!()
        };
        let Attrs::Consistent(attrs2) = attrs2 else {
            unreachable!()
        };
        if attrs1.len() > attrs2.len() {
            return Ok(Dnf::FALSE);
        }
        let mut inst = Dnf::True;
        let mut it2 = attrs2
            .iter()
            .map(|a| (a, a.adjusted_nr(&self.0.g.constrs)))
            .peekable();
        for attr1 in attrs1 {
            let n = attr1.adjusted_nr(&self.0.g.constrs);
            loop {
                let Some(a) = it2.peek() else {
                    return Ok(Dnf::FALSE);
                };
                if a.1 >= n {
                    break;
                }
                it2.next();
            }
            let mut inst1 = Dnf::FALSE;
            for (attr2, _) in it2.clone().take_while(|a| a.1 <= n) {
                self.or_unify_attr(attr1, attr2, &mut inst1)?;
            }
            inst.mk_and(inst1)?;
            if inst.is_false() {
                break;
            }
        }
        Ok(inst)
    }

    /// InstCollection.UnifyTypsWithConsts
    fn unify_type(&mut self, ty1: &Type, ty2: &Type) -> Result<Dnf<FVarId, EqClassId>, Overflow> {
        let mut inst = match (ty1.kind, ty2.kind) {
            (TypeKind::Struct(n1), TypeKind::Struct(n2)) if n1 == n2 => {
                self.unify_terms(&ty1.args, &ty2.args)?
            }
            (TypeKind::Mode(n1), TypeKind::Mode(n2)) => {
                let (n1, args1) = Type::adjust(n1, &ty1.args, &self.0.g.constrs);
                let (n2, args2) = Type::adjust(n2, &ty2.args, &self.0.g.constrs);
                if n1 == n2 {
                    self.unify_terms(args1, args2)?
                } else {
                    return Ok(Dnf::FALSE);
                }
            }
            _ => return Ok(Dnf::FALSE),
        };
        inst.mk_and_then(|| self.unify_subset_attrs(&ty1.attrs.0, &ty2.attrs.1))?;
        inst.mk_and_then(|| self.unify_subset_attrs(&ty2.attrs.0, &ty1.attrs.1))?;
        Ok(inst)
    }

    /// InstCollection.UnifyBasicFrm
    fn unify_basic_formula(
        &mut self,
        f1: &Formula,
        f2: &Formula,
    ) -> Result<Dnf<FVarId, EqClassId>, Overflow> {
        match (f1, f2) {
            (
                Formula::Pred {
                    nr: n1,
                    args: args1,
                },
                Formula::Pred {
                    nr: n2,
                    args: args2,
                },
            ) => {
                let (n1, args1) = Formula::adjust_pred(*n1, args1, Some(&self.0.g.constrs));
                let (n2, args2) = Formula::adjust_pred(*n2, args2, Some(&self.0.g.constrs));
                if n1 == n2 {
                    self.unify_terms(args1, args2)
                } else {
                    Ok(Dnf::FALSE)
                }
            }
            (
                Formula::SchPred {
                    nr: SchPredId(n1),
                    args: args1,
                },
                Formula::SchPred {
                    nr: SchPredId(n2),
                    args: args2,
                },
            )
            | (
                Formula::PrivPred {
                    nr: PrivPredId(n1),
                    args: args1,
                    ..
                },
                Formula::PrivPred {
                    nr: PrivPredId(n2),
                    args: args2,
                    ..
                },
            ) if n1 == n2 => self.unify_terms(args1, args2),
            (
                Formula::Attr {
                    nr: n1,
                    args: args1,
                },
                Formula::Attr {
                    nr: n2,
                    args: args2,
                },
            ) => {
                let (n1, args1) = Formula::adjust_attr(*n1, args1, Some(&self.0.g.constrs));
                let (n2, args2) = Formula::adjust_attr(*n2, args2, Some(&self.0.g.constrs));
                if n1 == n2 {
                    self.unify_terms(args1, args2)
                } else {
                    Ok(Dnf::FALSE)
                }
            }
            (Formula::Is { term: t1, ty: ty1 }, Formula::Is { term: t2, ty: ty2 }) => {
                let mut inst = self.unify_term(t1, t2)?;
                inst.mk_and_then(|| self.unify_type(ty1, ty2))?;
                Ok(inst)
            }
            (Formula::Neg { .. }, _)
            | (Formula::And { .. }, _)
            | (Formula::FlexAnd { .. }, _)
            | (Formula::LegacyFlexAnd { .. }, _)
            | (Formula::True, _) => unreachable!(),
            _ => Ok(Dnf::FALSE),
        }
    }
}

impl EquateClass<'_> {
    /// EqClassNr
    fn get(&mut self, g: &MizGlobal, lc: &LocalContext, tm: &Term) -> Option<EqClassId> {
        macro_rules! func_like {
            ($tk:ident { $nr:expr, $args:expr }) => {{
                let ecs = $args
                    .iter()
                    .map(|t| self.get(g, lc, t))
                    .collect::<Option<Vec<_>>>()?;
                for (ec, etm) in self.eq_class.enum_iter() {
                    for &m in &etm.terms[CTK::$tk] {
                        let Term::$tk { nr, ref args, .. } = lc.marks[m].0 else {
                            unreachable!()
                        };
                        if $nr == nr
                            && args
                                .iter()
                                .zip(&ecs)
                                .all(|(arg, &ec2)| arg.class() == Some(ec2))
                        {
                            return Some(ec);
                        }
                    }
                }
                None
            }};
        }
        match *tm {
            Term::EqClass(ec) => Some(ec),
            Term::Numeral(n) => {
                let c = Some(n.into());
                self.eq_class
                    .enum_iter()
                    .find(|p| p.1.number == c)
                    .map(|p| p.0)
            }
            Term::Infer(n) => self.infer.get(&n).copied(),
            Term::Functor { nr, ref args } => {
                let ecs = args
                    .iter()
                    .map(|t| self.get(g, lc, t))
                    .collect::<Option<Vec<_>>>()?;
                for (ec, etm) in self.eq_class.enum_iter() {
                    for &m in &etm.terms[CTK::Functor] {
                        let Term::Functor {
                            nr: nr2,
                            args: ref args2,
                        } = lc.marks[m].0
                        else {
                            unreachable!()
                        };
                        let (nr, adj) = Term::adjust(nr, args, Some(&g.constrs));
                        let (nr2, adj2) = Term::adjust(nr2, args2, Some(&g.constrs));
                        if nr != nr2 {
                            continue;
                        }
                        if (adj2.iter().zip(&ecs[args.len() - adj.len()..]))
                            .all(|(arg, &ec2)| arg.unmark(lc).class() == Some(ec2))
                        {
                            return Some(ec);
                        }
                    }
                }
                None
            }
            Term::Aggregate { nr, ref args } => func_like!(Aggregate { nr, args }),
            Term::SchFunc { nr, ref args } => func_like!(SchFunc { nr, args }),
            Term::PrivFunc { nr, ref args, .. } => func_like!(PrivFunc { nr, args }),
            Term::Selector { nr, ref args } => func_like!(Selector { nr, args }),
            Term::Locus(_) | Term::Bound(_) => None,
            Term::Fraenkel { .. } => (self.eq_class.enum_iter())
                .find(|p| {
                    p.1.terms[CTK::Fraenkel]
                        .iter()
                        .any(|&m| self.eq(g, lc, tm, &lc.marks[m].0))
                })
                .map(|p| p.0),
            Term::The { .. } => (self.eq_class.enum_iter())
                .find(|p| {
                    p.1.terms[CTK::Choice]
                        .iter()
                        .any(|&m| self.eq(g, lc, tm, &lc.marks[m].0))
                })
                .map(|p| p.0),
            Term::EqMark(m) => self.get(g, lc, &lc.marks[m].0),
            Term::Const(_) | Term::FreeVar(_) | Term::Qua { .. } | Term::It => unreachable!(),
        }
        // vprintln!("get_eq_class {tm:?} -> {res:?}");
    }
}

impl Equate for EquateClass<'_> {
    fn eq_class_right(&mut self, ctx: &mut EqCtx<'_>, t1: &Term, ec: EqClassId) -> bool {
        self.get(ctx.g, ctx.lc, t1) == Some(ec)
    }

    fn eq_pred(
        &mut self,
        ctx: &mut EqCtx<'_>,
        n1: PredId,
        n2: PredId,
        args1: &[Term],
        args2: &[Term],
    ) -> bool {
        let (n1_adj, args1_adj) = Formula::adjust_pred(n1, args1, Some(&ctx.g.constrs));
        let (n2_adj, args2_adj) = Formula::adjust_pred(n2, args2, Some(&ctx.g.constrs));
        if n1_adj != n2_adj {
            return false;
        }
        if self.eq_terms(ctx, args1_adj, args2_adj) {
            return true;
        }
        let c = &ctx.g.constrs.predicate[n1];
        if c.properties.get(PropertyKind::Symmetry) {
            let mut args1 = args1.iter().collect_vec();
            args1.swap(c.properties.arg1 as usize, c.properties.arg2 as usize);
            args1[c.superfluous as usize..]
                .iter()
                .zip(args2_adj)
                .all(|(t1, t2)| self.eq_term(ctx, t1, t2))
        } else {
            let c = &ctx.g.constrs.predicate[n2];
            c.properties.get(PropertyKind::Symmetry) && {
                let mut args2 = args2.iter().collect_vec();
                args2.swap(c.properties.arg1 as usize, c.properties.arg2 as usize);
                (args1_adj.iter().zip(&args2[c.superfluous as usize..]))
                    .all(|(t1, t2)| self.eq_term(ctx, t1, t2))
            }
        }
    }
}

struct Similar;
impl Equate for Similar {
    fn eq_terms(&mut self, _: &mut EqCtx<'_>, _: &[Term], _: &[Term]) -> bool {
        true
    }
    fn eq_term(&mut self, _: &mut EqCtx<'_>, _: &Term, _: &Term) -> bool {
        true
    }

    fn eq_type(&mut self, ctx: &mut EqCtx<'_>, ty1: &Type, ty2: &Type) -> bool {
        (match (&ty1.attrs.0, &ty1.attrs.1) {
            (Attrs::Inconsistent, Attrs::Inconsistent) => true,
            (Attrs::Consistent(attrs1), Attrs::Consistent(attrs2)) => {
                attrs1.len() == attrs2.len()
                    && attrs1
                        .iter()
                        .zip(attrs2)
                        .all(|(a1, a2)| self.eq_attr(ctx, a1, a2))
            }
            _ => false,
        }) && self.eq_radices(ctx, ty1, ty2)
    }

    fn eq_attr(&mut self, _: &mut EqCtx<'_>, a1: &MizAttr, a2: &MizAttr) -> bool {
        a1.nr == a2.nr && a1.pos == a2.pos
    }

    fn eq_forall(
        &mut self,
        _: &mut EqCtx<'_>,
        _: &Type,
        _: &Type,
        _: &Formula,
        _: &Formula,
    ) -> bool {
        false
    }
}
