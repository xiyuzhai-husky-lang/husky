use crate::accom::SigBuilder;
use crate::*;
use enum_map::{Enum, EnumMap};
use idx::vec::sorted::SortedIdxVec;
use paste::paste;
use std::collections::{BTreeMap, BTreeSet, HashSet};

#[derive(Clone)]
pub enum CowBox<'a, A: ?Sized> {
    Borrowed(&'a A),
    Owned(Box<A>),
}

impl<'a, A: ?Sized + std::fmt::Debug> std::fmt::Debug for CowBox<'a, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (**self).fmt(f)
    }
}

impl<A: ?Sized> std::ops::Deref for CowBox<'_, A> {
    type Target = A;
    fn deref(&self) -> &Self::Target {
        match self {
            CowBox::Borrowed(x) => x,
            CowBox::Owned(x) => x,
        }
    }
}

impl<A: ?Sized + Clone> CowBox<'_, A> {
    #[allow(clippy::wrong_self_convention)]
    pub fn to_owned(self) -> Box<A> {
        match self {
            CowBox::Borrowed(x) => Box::new(x.clone()),
            CowBox::Owned(x) => x,
        }
    }
}

mk_id! {
  ModeId(u32) + Visit(visit_mode_id),
  StructId(u32) + Visit(visit_struct_id),
  AttrId(u32) + Visit(visit_attr_id),
  PredId(u32) + Visit(visit_pred_id),
  SchPredId(u32),
  PrivPredId(u32),
  FuncId(u32) + Visit(visit_func_id),
  SelId(u32) + Visit(visit_sel_id),
  AggrId(u32) + Visit(visit_aggr_id),
  BoundId(u32),
  ConstId(u32),
  FVarId(u32),
  InferId(u32),
  EqClassId(u32),
  EqTermId(u32),
  EqMarkId(u32),
  SchFuncId(u32),
  PrivFuncId(u32),
  LocusId(u8),
  LabelId(u32),
  ArticleId(u32),
  DefId(u32),
  ThmId(u32),
  SchId(u32),
  AtomId(u32),
}
impl ArticleId {
    pub const SELF: ArticleId = ArticleId(0);
}

/// "Requirements" are schemes which are built into the system reasoning itself,
/// which hence act as axioms, although they are used in a way that should be a
/// conservative extension.
///
/// * `HIDDEN`: this introduces `object`, `set`, `=`, `in`.
///   This is not a proper mizar file, and every mizar file implicitly depends on it.
/// * `BOOLE` (introduced after XBOOLE_0): this introduces `x is empty`, `{}`, as well as
///   set operators: `A \/ B`, `A /\ B`, `A \ B`, `A \+\ B`, `A meets B`.
///   This is not used for much, just a few extra properties in the equalizer.
/// * `SUBSET` (introduced after SUBSET_1):
///   introduces `Element of A`, `bool A`, `A c= B`, `Subset of A`.
///   The equalizer knows about how subsets are elements of the powerset and so on.
/// * `NUMERALS` (introduced after ORDINAL1):
///   introduces `succ x`, `x is natural`, `omega`, `0`, `x is zero`.
///   After this point the system knows about positive numerals like 37: they have type
///   `Element of omega` (they already existed before but were uninterpeted sets),
///   and it knows how to evaluate `succ (succ 0) = 2` and `1 <> 2`.
///   These can be metatheoretically justified by the theorems `succ x = succ y -> x = y`
///   and `succ x is non zero`.
/// * `REAL` (introduced after XXREAL_0): introduces `x <= y`, `x is positive`, `x is negative`.
///   The equalizer knows some basic things about transitivity of less and less-equal
///   and its relation to the `positive` and `negative` attributes.
///   * `NUMERALS` + `REAL` also enables the support for flex conjunctions P(1) /\ ... /\ P(n).
///     The main new thing that is needed is the expansion in the concrete `n` case,
///     which amounts to the theorem `0 <= x <= n -> x = 0 \/ ... \/ x = n`.
///     I didn't see anything immediately relevant to proving this in XXREAL_0
///     but I'm sure it's possible.
/// * `ARITHM` (introduced after XCMPLX_0):
///   introduces `x + y`, `x * y`, `-x`, `x"`, `x - y`, `x / y`, `<i>`, `x is complex`.
///   Also enables bignum arithmetic for complex rational numeric expressions.
///   Also enables polynomial evaluation of ring expressions in terms of these operators;
///   this last one seems like a bit of a cheat since there isn't anything like a proof of
///   `-(x + y) = -x + -y` and the like in XCMPLX_0. Many of the theorems in XCMPLX_1
///   reference earlier theorems, maybe to justify why they can now be taken as axioms,
///   but some references like JGRAPH_6 also use ARITHM so this isn't very convincing.
///
pub struct RequirementIndexes {
    pub fwd: EnumMap<Requirement, u32>,
    pub rev: IdxVec<FuncId, Option<Requirement>>,
}

impl Default for RequirementIndexes {
    fn default() -> Self {
        Self {
            fwd: Default::default(),
            rev: IdxVec::new(),
        }
    }
}

impl std::fmt::Debug for RequirementIndexes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fwd.fmt(f)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum RequirementKind {
    Func(FuncId),
    Mode(ModeId),
    Pred(PredId),
    Attr(AttrId),
}
impl From<AttrId> for RequirementKind {
    fn from(v: AttrId) -> Self {
        Self::Attr(v)
    }
}
impl From<PredId> for RequirementKind {
    fn from(v: PredId) -> Self {
        Self::Pred(v)
    }
}
impl From<ModeId> for RequirementKind {
    fn from(v: ModeId) -> Self {
        Self::Mode(v)
    }
}
impl From<FuncId> for RequirementKind {
    fn from(v: FuncId) -> Self {
        Self::Func(v)
    }
}

macro_rules! mk_requirements {
  (@to_kind FuncId $e:tt) => { ConstrKind::Func($e) };
  (@to_kind AttrId $e:tt) => { ConstrKind::Attr($e) };
  (@to_kind PredId $e:tt) => { ConstrKind::Pred($e) };
  (@to_kind ModeId $e:tt) => { ConstrKind::Mode($e) };
  (@is_func FuncId) => { true };
  (@is_func $_:tt) => { false };
  ($($(#[$attr:meta])* $id:ident: $ty:tt,)*) => {
    #[derive(Copy, Clone, Debug, Enum)]
    pub enum Requirement {
      $($(#[$attr])* $id,)*
    }
    impl RequirementIndexes {
      paste! {
        $(
          $(#[$attr])*
          pub fn [<$id:snake>](&self) -> Option<$ty> { self.get_raw(Requirement::$id).map($ty) }
        )*
      }
      pub fn on_func_ids(mut f: impl FnMut(Requirement)) {
        $(if mk_requirements!(@is_func $ty) { f(Requirement::$id) })*
      }

      pub fn get(&self, req: Requirement) -> Option<RequirementKind> {
        match req {
          $(Requirement::$id => self.get_raw(req).map(|x| $ty(x).into()),)*
        }
      }

      pub fn set(&mut self, req: Requirement, value: ConstrKind) {
        match (req, value) {
          $((Requirement::$id, mk_requirements!(@to_kind $ty n)) => self.fwd[req] = n.0 + 1,)*
          _ => panic!("incorrect type for requirement"),
        }
      }
    }
  }
}

mk_requirements! {
  /// mode `object`; from HIDDEN
  Any: ModeId,
  /// mode `set`; from HIDDEN
  SetMode: ModeId,
  /// predicate `a = b`; from HIDDEN
  EqualsTo: PredId,
  /// predicate `a in b`; from HIDDEN
  BelongsTo: PredId,
  /// attribute `x is empty`; from BOOLE [XBOOLE_0]
  Empty: AttrId,
  /// functor `{}`; from BOOLE [XBOOLE_0]
  EmptySet: FuncId,
  /// mode `Element of A`; from SUBSET [SUBSET_1]
  Element: ModeId,
  /// functor `bool A`; from SUBSET [SUBSET_1]
  PowerSet: FuncId,
  /// predicate `A c= B`; from SUBSET [SUBSET_1]
  Inclusion: PredId,
  /// mode `Subset of A`; from SUBSET [SUBSET_1]
  SubDomElem: ModeId,
  /// functor `REAL` (unused)
  RealDom: FuncId,
  /// functor `NAT` (unused)
  NatDom: FuncId,
  /// functor `x + y`; from ARITHM [XCMPLX_0]
  RealAdd: FuncId,
  /// functor `x * y`; from ARITHM [XCMPLX_0]
  RealMult: FuncId,
  /// predicate `x <= y`; from REAL [XXREAL_0]
  LessOrEqual: PredId,
  /// functor `succ x`; from NUMERALS [ORDINAL1]
  Succ: FuncId,
  /// functor `A \/ B`; from BOOLE [XBOOLE_0]
  Union: FuncId,
  /// functor `A /\ B`; from BOOLE [XBOOLE_0]
  Intersection: FuncId,
  /// functor `A \ B`; from BOOLE [XBOOLE_0]
  Subtraction: FuncId,
  /// functor `A \+\ B`; from BOOLE [XBOOLE_0]
  SymmetricDifference: FuncId,
  /// predicate `A meets B`; from BOOLE [XBOOLE_0]
  Meets: PredId,
  /// functor `-x`; from ARITHM [XCMPLX_0]
  RealNeg: FuncId,
  /// functor `x"`; from ARITHM [XCMPLX_0]
  RealInv: FuncId,
  /// functor `x - y`; from ARITHM [XCMPLX_0]
  RealDiff: FuncId,
  /// functor `x / y`; from ARITHM [XCMPLX_0]
  RealDiv: FuncId,
  /// attribute `x is real` (unused)
  Real: AttrId,
  /// attribute `x is positive`; from REAL [XXREAL_0]
  Positive: AttrId,
  /// attribute `x is negative`; from REAL [XXREAL_0]
  Negative: AttrId,
  /// attribute `x is natural`; from NUMERALS [ORDINAL1]
  Natural: AttrId,
  /// functor `<i>`; from ARITHM [XCMPLX_0]
  ImaginaryUnit: FuncId,
  /// attribute `x is complex`; from ARITHM [XCMPLX_0]
  Complex: AttrId,
  /// functor `omega`; from NUMERALS [ORDINAL1]
  Omega: FuncId,
  /// functor `0`; from NUMERALS [ORDINAL1]
  ZeroNumber: FuncId,
  /// attribute `x is zero`; from NUMERALS [ORDINAL1]
  Zero: AttrId,
}

impl ModeId {
    // Every mizar file needs this one and it needs to be mode 0
    pub const ANY: ModeId = ModeId(0);
    // Every mizar file needs this one and it needs to be mode 1
    pub const SET: ModeId = ModeId(1);
}

impl RequirementIndexes {
    pub fn init_rev(&mut self) {
        assert_eq!(self.fwd[Requirement::Any], ModeId::ANY.0 + 1);
        assert_eq!(self.fwd[Requirement::SetMode], ModeId::SET.0 + 1);
        assert_eq!(self.fwd[Requirement::RealDom], 0);
        assert_eq!(self.fwd[Requirement::NatDom], 0);
        assert_eq!(self.fwd[Requirement::Real], 0);
        Self::on_func_ids(|req| {
            if let Some(r) = self.get_raw(req) {
                *self.rev.get_mut_extending(FuncId(r)) = Some(req);
            }
        })
    }

    pub fn get_raw(&self, req: Requirement) -> Option<u32> {
        self.fwd[req].checked_sub(1)
    }

    pub fn mk_eq(&self, t1: Term, t2: Term) -> Formula {
        Formula::Pred {
            nr: self.equals_to().unwrap(),
            args: Box::new([t1, t2]),
        }
    }
}

pub trait Visitable<V> {
    fn visit(&mut self, v: &mut V);
    fn visit_cloned(&self, v: &mut V) -> Self
    where
        Self: Clone,
    {
        let mut t = self.clone();
        t.visit(v);
        t
    }
}

impl<V> Visitable<V> for () {
    fn visit(&mut self, _: &mut V) {}
}
impl<V, T: Visitable<V> + ?Sized> Visitable<V> for &mut T {
    fn visit(&mut self, v: &mut V) {
        (**self).visit(v)
    }
}
impl<V, T: Visitable<V> + ?Sized> Visitable<V> for Box<T> {
    fn visit(&mut self, v: &mut V) {
        (**self).visit(v)
    }
}
impl<V, T: Visitable<V>> Visitable<V> for [T] {
    fn visit(&mut self, v: &mut V) {
        self.iter_mut().for_each(|t| t.visit(v))
    }
}
impl<V, T: Visitable<V>, const N: usize> Visitable<V> for [T; N] {
    fn visit(&mut self, v: &mut V) {
        <[T]>::visit(self, v)
    }
}
impl<V, T: Visitable<V>> Visitable<V> for Option<T> {
    fn visit(&mut self, v: &mut V) {
        self.iter_mut().for_each(|t| t.visit(v))
    }
}
impl<V, T: Visitable<V>> Visitable<V> for Vec<T> {
    fn visit(&mut self, v: &mut V) {
        (**self).visit(v)
    }
}
impl<V, T: Visitable<V>> Visitable<V> for ExtVec<T> {
    fn visit(&mut self, v: &mut V) {
        self.vec.visit(v)
    }
}
impl<I, V, T: Visitable<V>> Visitable<V> for IdxVec<I, T> {
    fn visit(&mut self, v: &mut V) {
        self.0.visit(v)
    }
}
impl<V, A: Visitable<V>, B: Visitable<V>> Visitable<V> for (A, B) {
    fn visit(&mut self, v: &mut V) {
        self.0.visit(v);
        self.1.visit(v)
    }
}

/// This type alias is used to indicate that the term might have a Qua at the top level.
pub type TermQua = Term;

#[derive(Clone, PartialEq, Eq)]
pub enum Term {
    /// Invariant: nr != 0. Zero is not a numeral (!),
    /// it is a `Functor` using Requirement::ZeroNumber
    Numeral(u32),
    /// Locus numbers are shifted from mizar to start at 0
    Locus(LocusId),
    /// Bound var numbers are shifted from mizar to start at 0
    Bound(BoundId),
    /// Constant numbers are shifted from mizar to start at 0
    Const(ConstId),
    /// ikEqConst: This is used by the equalizer, it is not read in
    EqClass(EqClassId),
    /// Not in mizar. Used for term sharing in the equalizer
    EqMark(EqMarkId),
    /// Used for term sharing in the verifier, but not used in mizar imports
    Infer(InferId),
    SchFunc {
        nr: SchFuncId,
        args: Box<[Term]>,
    },
    Aggregate {
        nr: AggrId,
        args: Box<[Term]>,
    },
    PrivFunc {
        nr: PrivFuncId,
        args: Box<[Term]>,
        value: Box<Term>,
    },
    Functor {
        nr: FuncId,
        args: Box<[Term]>,
    },
    Selector {
        nr: SelId,
        args: Box<[Term]>,
    },
    FreeVar(FVarId),
    The {
        ty: Box<Type>,
    },
    Fraenkel {
        args: Box<[Type]>,
        scope: Box<Term>,
        compr: Box<Formula>,
    },
    /// Only used/valid in the analyzer, at the top level of a term expression.
    /// Indicates that the term should be treated as having type `ty` instead
    /// of looking at `term`'s type.
    Qua {
        value: Box<Term>,
        ty: Box<Type>,
    },
    /// Only used/valid in the analyzer. Used in definiens that are predicates,
    /// to refer to the object being defined.
    It,
}

impl Default for Term {
    fn default() -> Self {
        Self::Numeral(0)
    }
}

impl Term {
    pub fn args(&self) -> Option<&[Term]> {
        match self {
            Term::SchFunc { args, .. }
            | Term::Aggregate { args, .. }
            | Term::PrivFunc { args, .. }
            | Term::Functor { args, .. }
            | Term::Selector { args, .. } => Some(args),
            _ => None,
        }
    }

    pub fn strip_qua_mut(&mut self) {
        if let Term::Qua { value, .. } = self {
            *self = std::mem::take(&mut **value)
        }
    }

    pub fn strip_qua(self: TermQua) -> Term {
        match self {
            Term::Qua { value, .. } => *value,
            _ => self,
        }
    }
    pub fn unqua<'a>(self: &'a TermQua) -> &'a Term {
        match self {
            Term::Qua { value, .. } => value,
            _ => self,
        }
    }
}

impl<V: VisitMut> Visitable<V> for Term {
    fn visit(&mut self, v: &mut V) {
        v.visit_term(self)
    }
}

impl Term {
    pub fn discr(&self) -> u8 {
        match self {
            Term::Locus(_) => b'A',
            Term::Bound(_) => b'B',
            Term::Const(_) => b'C',
            Term::Infer(_) => b'D',
            Term::EqClass(..) => b'E',
            Term::EqMark(_) => b'M', // NEW
            Term::SchFunc { .. } => b'F',
            Term::Aggregate { .. } => b'G',
            Term::PrivFunc { .. } => b'H',
            Term::Functor { .. } => b'K',
            Term::Numeral(_) => b'N',
            Term::Selector { .. } => b'U',
            Term::FreeVar(_) => b'X',
            Term::The { .. } => 216,
            Term::Fraenkel { .. } => 232,
            Term::Qua { .. } => 213,
            Term::It { .. } => 234,
        }
    }
}

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Type {
    /// The kind of type (either Mode or Struct), and the id
    pub kind: TypeKind,
    /// The first is the attributes written by the user ("lower cluster"),
    /// the second is the attributes calculated by the system ("upper cluster")
    pub attrs: (Attrs, Attrs),
    /// The mode arguments (ModArgs)
    pub args: Vec<Term>,
}

impl Type {
    pub const fn new(kind: TypeKind) -> Self {
        Self {
            kind,
            attrs: (Attrs::EMPTY, Attrs::EMPTY),
            args: vec![],
        }
    }
    pub const ANY: Type = Type::new(TypeKind::Mode(ModeId::ANY));
    pub const SET: Type = Type::new(TypeKind::Mode(ModeId::SET));

    /// precondition: the type has kind Struct
    pub fn struct_id(&self) -> StructId {
        match self.kind {
            TypeKind::Mode(_) => panic!("not a struct"),
            TypeKind::Struct(n) => n,
        }
    }
}

impl<V: VisitMut> Visitable<V> for Type {
    fn visit(&mut self, v: &mut V) {
        v.visit_type(self)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TypeKind {
    Struct(StructId),
    Mode(ModeId),
}

impl Default for TypeKind {
    fn default() -> Self {
        Self::Mode(ModeId(0))
    }
}

impl std::fmt::Debug for TypeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Struct(n) => write!(f, "S{n:?}"),
            Self::Mode(n) => write!(f, "M{n:?}"),
        }
    }
}

impl From<StructId> for TypeKind {
    fn from(v: StructId) -> Self {
        Self::Struct(v)
    }
}

impl From<ModeId> for TypeKind {
    fn from(v: ModeId) -> Self {
        Self::Mode(v)
    }
}

impl TypeKind {
    pub fn discr(&self) -> u8 {
        match self {
            TypeKind::Mode(_) => b'M',
            TypeKind::Struct(_) => b'G',
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum Formula {
    SchPred {
        nr: SchPredId,
        args: Box<[Term]>,
    },
    Pred {
        nr: PredId,
        args: Box<[Term]>,
    },
    Attr {
        nr: AttrId,
        /// Invariant: args is not empty
        args: Box<[Term]>,
    },
    PrivPred {
        nr: PrivPredId,
        args: Box<[Term]>,
        value: Box<Formula>,
    },
    /// ikFrmQual
    Is {
        term: Box<Term>,
        ty: Box<Type>,
    },
    Neg {
        /// Invariant: the formula is not Neg
        f: Box<Formula>,
    },
    /// ikFrmConj
    And {
        /// Invariant: args.len() > 1 and does not contain And expressions
        args: Vec<Formula>,
    },
    /// ikFrmUniv
    ForAll {
        dom: Box<Type>,
        scope: Box<Formula>,
    },
    /// ikFrmFlexConj
    FlexAnd {
        nat: Box<Type>,
        le: PredId,
        terms: Box<[Term; 2]>,
        scope: Box<Formula>,
    },
    LegacyFlexAnd {
        orig: Box<[Formula; 2]>,
        terms: Box<[Term; 2]>,
        expansion: Box<Formula>,
    },
    /// ikFrmVerum
    True,
}

impl Default for Formula {
    fn default() -> Self {
        Self::True
    }
}

impl<V: VisitMut> Visitable<V> for Formula {
    fn visit(&mut self, v: &mut V) {
        v.visit_formula(self)
    }
}

impl Formula {
    pub fn discr(&self) -> u8 {
        match self {
            Formula::SchPred { .. } => b'P',
            Formula::Pred { .. } => b'R',
            Formula::Attr { .. } => b'V',
            Formula::PrivPred { .. } => b'S',
            Formula::Is { .. } => 144,
            Formula::Neg { .. } => 170,
            Formula::And { .. } => b'&',
            Formula::ForAll { .. } => 157,
            Formula::FlexAnd { .. } | Formula::LegacyFlexAnd { .. } => b'b',
            Formula::True => b'%',
            // Formula::Thesis => b'$',
        }
    }

    pub fn mk_neg(self) -> Self {
        match self {
            Formula::Neg { f } => *f,
            _ => Formula::Neg { f: Box::new(self) },
        }
    }

    /// This calculates `self == pos`.
    /// That is, it negates `self` if `pos == false` and leaves it unchanged otherwise.
    pub fn maybe_neg(self, pos: bool) -> Self {
        if pos {
            self
        } else {
            self.mk_neg()
        }
    }

    #[inline]
    pub fn forall(dom: Type, scope: Self) -> Self {
        Self::ForAll {
            dom: Box::new(dom),
            scope: Box::new(scope),
        }
    }

    pub fn conjuncts(&self) -> &[Formula] {
        match self {
            Formula::True => &[],
            Formula::And { args } => args,
            f => std::slice::from_ref(f),
        }
    }

    pub fn into_conjuncts(self) -> Vec<Formula> {
        match self {
            Formula::True => vec![],
            Formula::And { args } => args,
            f => vec![f],
        }
    }

    // postcondition: the things pushed to vec are not And expressions
    pub fn append_conjuncts_to(self, vec: &mut Vec<Formula>) {
        match self {
            Formula::True => {}
            Formula::And { mut args } => vec.append(&mut args),
            f => vec.push(f),
        }
    }

    // Precondition: the args are not And expressions
    pub fn mk_and(args: Vec<Formula>) -> Formula {
        match args.len() {
            0 => Formula::True,
            1 => { args }.pop().unwrap(),
            _ => Formula::And { args },
        }
    }

    #[inline]
    pub fn mk_and_with(f: impl FnOnce(&mut Vec<Formula>)) -> Formula {
        let mut args = vec![];
        f(&mut args);
        Self::mk_and(args)
    }

    /// * pos = true: constructs self && vec[0] && ... && vec[n-1]
    /// * pos = false: constructs self || vec[0] || ... || vec[n-1]
    pub fn conjdisj_many(&mut self, pos: bool, vec: Vec<Formula>) {
        if !vec.is_empty() {
            *self = Formula::mk_and_with(|conjs| {
                std::mem::take(self)
                    .maybe_neg(pos)
                    .append_conjuncts_to(conjs);
                vec.into_iter()
                    .for_each(|f| f.maybe_neg(pos).append_conjuncts_to(conjs));
            })
            .maybe_neg(pos);
        }
    }

    pub fn mk_iff(self, other: Formula) -> Formula {
        Formula::mk_and_with(|conjs| {
            let f1 = Formula::mk_and_with(|conjs1| {
                self.clone().append_conjuncts_to(conjs1);
                other.clone().mk_neg().append_conjuncts_to(conjs1);
            });
            f1.mk_neg().append_conjuncts_to(conjs);
            let f2 = Formula::mk_and_with(|conjs2| {
                other.append_conjuncts_to(conjs2);
                self.mk_neg().append_conjuncts_to(conjs2);
            });
            f2.mk_neg().append_conjuncts_to(conjs);
        })
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum Attrs {
    Inconsistent,
    Consistent(Vec<MizAttr>),
}

impl Attrs {
    pub const EMPTY: Attrs = Self::Consistent(vec![]);

    pub fn attrs(&self) -> &[MizAttr] {
        match self {
            Attrs::Inconsistent => &[],
            Attrs::Consistent(attrs) => attrs,
        }
    }
}
impl Default for Attrs {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl<V: VisitMut> Visitable<V> for Attrs {
    fn visit(&mut self, v: &mut V) {
        v.visit_attrs(self)
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct MizAttr {
    pub nr: AttrId,
    pub pos: bool,
    pub args: Box<[Term]>,
}

impl MizAttr {
    pub fn new0(nr: AttrId, pos: bool) -> Self {
        Self {
            nr,
            pos,
            args: Box::new([]),
        }
    }
}

impl<V: VisitMut> Visitable<V> for MizAttr {
    fn visit(&mut self, v: &mut V) {
        self.args.visit(v)
    }
}

pub const MAX_ARTICLE_LEN: usize = 8;
#[derive(Hash, Copy, Clone, Default, PartialEq, Eq)]
pub struct Article([u8; MAX_ARTICLE_LEN]);

impl std::fmt::Debug for Article {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl std::fmt::Display for Article {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

#[derive(Debug)]
pub enum ToArticleError {
    TooLong,
    NotAscii,
}
impl std::fmt::Display for ToArticleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToArticleError::NotAscii => write!(f, "non-ASCII characters in article name"),
            ToArticleError::TooLong => {
                write!(f, "article names can only be {MAX_ARTICLE_LEN} characters")
            }
        }
    }
}

impl Article {
    pub const HIDDEN: Article = Article(*b"hidden\0\0");
    pub fn from_lower(s: &[u8]) -> Result<Article, ToArticleError> {
        if s.len() > MAX_ARTICLE_LEN {
            return Err(ToArticleError::TooLong);
        }
        let mut arr = [0; MAX_ARTICLE_LEN];
        arr[..s.len()].copy_from_slice(s);
        if !arr.iter().all(u8::is_ascii) {
            return Err(ToArticleError::NotAscii);
        }
        Ok(Article(arr))
    }
    pub fn from_upper(s: &[u8]) -> Result<Article, ToArticleError> {
        if s.len() > MAX_ARTICLE_LEN {
            return Err(ToArticleError::TooLong);
        }
        let mut arr = [0; MAX_ARTICLE_LEN];
        arr[..s.len()].copy_from_slice(s);
        if !arr.iter().all(u8::is_ascii) {
            return Err(ToArticleError::NotAscii);
        }
        std::str::from_utf8_mut(&mut arr[..s.len()])
            .map_err(|_| ToArticleError::NotAscii)?
            .make_ascii_lowercase();
        Ok(Article(arr))
    }
    pub fn as_bytes(&self) -> &[u8] {
        &self.0[..self.0.iter().position(|&x| x == 0).unwrap_or(8)]
    }
    pub fn as_str(&self) -> &str {
        std::str::from_utf8(self.as_bytes()).unwrap()
    }
}

macro_rules! mk_property_kind {
  (enum $ty:ident { $($(#[$attr:meta])* $id:ident = $upper:literal | $lower:literal,)* }) => {
    #[derive(Copy, Clone, Debug, Enum, PartialEq, Eq)]
    pub enum $ty {
      $($(#[$attr])* $id,)*
    }
    impl $ty {
      pub fn to_upper(self) -> &'static str {
        match self {
          $(Self::$id => stringify!($id),)*
        }
      }
      pub fn to_lower(self) -> &'static [u8] {
        match self {
          $(Self::$id => $lower,)*
        }
      }
    }
    impl TryFrom<&[u8]> for $ty {
      type Error = ();
      fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        match value {
          $($lower | $upper => Ok($ty::$id),)*
          _ => Err(()),
        }
      }
    }
  }
}
mk_property_kind! {
  enum PropertyKind {
    /// Applicable to PredId. Means: `∀ x y, P[x, y] -> P[y, x]`
    Symmetry = b"Symmetry" | b"symmetry",
    /// Applicable to PredId. Means: `∀ x, P[x, x]`
    Reflexivity = b"Reflexivity" | b"reflexivity",
    /// Applicable to PredId. Means: `∀ x, !P[x, x]`
    Irreflexivity = b"Irreflexivity" | b"irreflexivity",
    /// unused
    Associativity = b"Associativity" | b"associativity",
    /// unused
    Transitivity = b"Transitivity" | b"transitivity",
    /// Applicable to FuncId. Means: `∀ x y, F(x, y) = F(y, x)`
    Commutativity = b"Commutativity" | b"commutativity",
    /// Applicable to PredId. Means: `∀ x y, !P[x, y] -> P[y, x]`
    Connectedness = b"Connectedness" | b"connectedness",
    /// Applicable to PredId. Means: `∀ x y, P[x, y] -> !P[y, x]`
    Asymmetry = b"Asymmetry" | b"asymmetry",
    /// Applicable to FuncId. Means: `∀ x, F(x, x) = x`
    Idempotence = b"Idempotence" | b"idempotence",
    /// Applicable to FuncId. Means: `∀ x, F(F(x)) = x`
    Involutiveness = b"Involutiveness" | b"involutiveness",
    /// Applicable to FuncId. Means: `∀ x, F(F(x)) = F(x)`
    Projectivity = b"Projectivity" | b"projectivity",
    /// Applicable to ModeId. Means: `∃ S:set, ∀ x:M, x ∈ S`
    Sethood = b"Sethood" | b"sethood",
    /// Special property for AttrId, not in source text.
    /// Means "not strict", where "strict" is an adjective on structure types
    /// meaning no additional fields are present.
    Abstractness = b"Abstractness" | b"abstractness",
  }
}

impl PropertyKind {
    pub fn flip(self) -> Self {
        match self {
            Self::Reflexivity => Self::Irreflexivity,
            Self::Irreflexivity => Self::Reflexivity,
            Self::Connectedness => Self::Asymmetry,
            Self::Asymmetry => Self::Connectedness,
            _ => self,
        }
    }
}

#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct PropertySet(u16);

impl PropertySet {
    pub const EMPTY: Self = Self(0);
    const USES_ARG2: Self = Self(
        1 << PropertyKind::Symmetry as u16
            | 1 << PropertyKind::Reflexivity as u16
            | 1 << PropertyKind::Irreflexivity as u16
            | 1 << PropertyKind::Connectedness as u16
            | 1 << PropertyKind::Asymmetry as u16
            | 1 << PropertyKind::Commutativity as u16
            | 1 << PropertyKind::Idempotence as u16
            | 1 << PropertyKind::Associativity as u16
            | 1 << PropertyKind::Transitivity as u16,
    );
    const USES_ARG1: Self = Self(
        Self::USES_ARG2.0
            | 1 << PropertyKind::Involutiveness as u16
            | 1 << PropertyKind::Projectivity as u16,
    );
    pub const fn uses_arg1(self) -> bool {
        self.0 & Self::USES_ARG1.0 != 0
    }
    pub const fn uses_arg2(self) -> bool {
        self.0 & Self::USES_ARG2.0 != 0
    }
}

impl std::fmt::Debug for PropertySet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_set()
            .entries(
                (0..PropertyKind::LENGTH)
                    .map(PropertyKind::from_usize)
                    .filter(|&p| self.get(p)),
            )
            .finish()
    }
}

impl PropertySet {
    pub fn get(&self, prop: PropertyKind) -> bool {
        self.0 & (1 << prop as u16) != 0
    }
    pub fn set(&mut self, prop: PropertyKind) {
        self.0 |= 1 << prop as u16
    }
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Properties {
    pub properties: PropertySet,
    pub arg1: u8,
    pub arg2: u8,
}
impl std::ops::DerefMut for Properties {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.properties
    }
}
impl std::ops::Deref for Properties {
    type Target = PropertySet;

    fn deref(&self) -> &Self::Target {
        &self.properties
    }
}

impl Properties {
    pub const EMPTY: Self = Self {
        properties: PropertySet::EMPTY,
        arg1: 0,
        arg2: 0,
    };
    pub const fn offset(mut self, n: u8) -> Self {
        if self.properties.uses_arg1() {
            self.arg1 += n
        }
        if self.properties.uses_arg2() {
            self.arg2 += n
        }
        self
    }
    pub fn trim(&mut self) {
        if !self.properties.uses_arg1() {
            self.arg1 = 0
        }
        if !self.properties.uses_arg2() {
            self.arg2 = 0
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct Constructor<I> {
    // pub article: Article,
    // /// number of constructor in article
    // pub abs_nr: u32,
    pub primary: Box<[Type]>,
    pub redefines: Option<I>,
    pub superfluous: u8,
    pub properties: Properties,
}

impl<I> Constructor<I> {
    pub const fn new(primary: Box<[Type]>) -> Self {
        Self {
            primary,
            redefines: None,
            superfluous: 0,
            properties: Properties::EMPTY,
        }
    }
}
impl<V: VisitMut, I: Visitable<V>> Visitable<V> for Constructor<I> {
    fn visit(&mut self, v: &mut V) {
        v.with_locus_tys(&mut self.primary, |v| self.redefines.visit(v))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TyConstructor<I> {
    pub c: Constructor<I>,
    pub ty: Type,
}

impl<I> std::ops::Deref for TyConstructor<I> {
    type Target = Constructor<I>;
    fn deref(&self) -> &Self::Target {
        &self.c
    }
}
impl<V: VisitMut, I: Visitable<V>> Visitable<V> for TyConstructor<I> {
    fn visit(&mut self, v: &mut V) {
        v.with_locus_tys(&mut self.c.primary, |v| {
            self.c.redefines.visit(v);
            self.ty.visit(v)
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StructMode {
    pub c: Constructor<StructId>,
    /// These are guaranteed to be struct types
    pub parents: Box<[Type]>,
    pub aggr: AggrId,
    /// sorted by id
    pub fields: Box<[SelId]>,
}

impl std::ops::Deref for StructMode {
    type Target = Constructor<StructId>;
    fn deref(&self) -> &Self::Target {
        &self.c
    }
}
impl<V: VisitMut> Visitable<V> for StructMode {
    fn visit(&mut self, v: &mut V) {
        v.with_locus_tys(&mut self.c.primary, |v| {
            self.c.redefines.visit(v);
            self.parents.visit(v);
        });
        self.aggr.visit(v);
        self.fields.visit(v);
        if V::MODIFY_IDS {
            self.fields.sort_unstable();
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Aggregate {
    pub c: TyConstructor<AggrId>,
    pub base: u8,
    /// ordered the same as the constructor arguments
    pub fields: Box<[SelId]>,
}
impl<V: VisitMut> Visitable<V> for Aggregate {
    fn visit(&mut self, v: &mut V) {
        self.c.visit(v);
        self.fields.visit(v);
    }
}

impl std::ops::Deref for Aggregate {
    type Target = TyConstructor<AggrId>;
    fn deref(&self) -> &Self::Target {
        &self.c
    }
}
impl std::ops::DerefMut for Aggregate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.c
    }
}

macro_rules! impl_constructors {
  (struct Constructors {
    $($(#[$attr:meta])* $variant:ident($field:ident): IdxVec<$id:ty, $ty:ty> = $lit:expr,)*
  }) => {
    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    pub struct Constructors { $($(#[$attr])* pub $field: IdxVec<$id, $ty>),* }

    impl<V: VisitMut> Visitable<V> for Constructors {
      fn visit(&mut self, v: &mut V) { $(self.$field.visit(v));* }
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct ConstructorsBase { $(pub $field: u32),* }

    impl std::ops::AddAssign for ConstructorsBase {
      fn add_assign(&mut self, rhs: Self) { $(self.$field += rhs.$field);* }
    }
    impl std::ops::Sub for ConstructorsBase {
      type Output = Self;
      fn sub(self, rhs: Self) -> Self { Self { $($field: self.$field - rhs.$field),* } }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ConstructorsRef<'a> { $(pub $field: &'a [$ty]),* }

    #[derive(Clone, Debug)]
    pub enum ConstructorDef { $($variant($ty),)* }
    impl<V: VisitMut> Visitable<V> for ConstructorDef {
      fn visit(&mut self, v: &mut V) { match self { $(Self::$variant(c) => c.visit(v)),* } }
    }

    impl Constructors {
      pub fn push(&mut self, c: ConstructorDef) -> ConstrKind {
        match c { $(ConstructorDef::$variant(c) => ConstrKind::$variant(self.$field.push(c))),* }
      }

      pub fn append(&mut self, other: &mut Constructors) {
        $(self.$field.0.append(&mut other.$field.0));*
      }

      pub fn visit_at<V: VisitMut>(&mut self, v: &mut V, k: ConstrKind) {
        match k { $(ConstrKind::$variant(k) => self.$field[k].visit(v)),* }
      }

      pub fn visit_range<V: VisitMut>(&mut self, v: &mut V, r: std::ops::Range<&ConstructorsBase>) {
        $(self.$field.0[r.start.$field as usize..r.end.$field as usize].visit(v));*
      }

      pub fn since(&self, base: &ConstructorsBase) -> ConstructorsRef<'_> {
        ConstructorsRef { $($field: &self.$field.0[base.$field as usize..]),* }
      }

      pub fn upto(&self, base: &ConstructorsBase) -> ConstructorsRef<'_> {
        ConstructorsRef { $($field: &self.$field.0[..base.$field as usize]),* }
      }

      pub fn extend(&mut self, other: &ConstructorsRef<'_>) {
        $(self.$field.0.extend_from_slice(other.$field));*
      }

      pub fn len(&self) -> ConstructorsBase {
        ConstructorsBase { $($field: self.$field.len() as u32),* }
      }

      pub fn as_ref(&self) -> ConstructorsRef<'_> {
        ConstructorsRef { $($field: &self.$field.0),* }
      }
    }

    impl ConstructorsRef<'_> {
      pub fn is_empty(&self) -> bool { $(self.$field.is_empty())&&* }

      pub fn len(&self) -> ConstructorsBase {
        ConstructorsBase { $($field: self.$field.len() as u32),* }
      }

      pub fn to_owned(self) -> Constructors {
        Constructors { $($field: IdxVec::from(self.$field.to_vec())),* }
      }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum ConstrKind { $($variant($id),)* }
    impl<V: VisitMut> Visitable<V> for ConstrKind {
      fn visit(&mut self, v: &mut V) { match self { $(Self::$variant(c) => c.visit(v)),* } }
    }

    impl ConstrKind {
      pub fn discr_nr(&self) -> (u8, u32) {
        match *self { $(Self::$variant(n) => ($lit, n.0),)* }
      }

      pub fn lt(self, base: &ConstructorsBase) -> bool {
        match self { $(Self::$variant(c) => c.0 < base.$field),* }
      }
    }
  };
}

impl_constructors! {
  struct Constructors {
    Mode(mode): IdxVec<ModeId, TyConstructor<ModeId>> = b'M',
    Struct(struct_mode): IdxVec<StructId, StructMode> = b'S',
    /// Invariant: The `ty` field is always equal to `primary.last()`
    Attr(attribute): IdxVec<AttrId, TyConstructor<AttrId>> = b'V',
    Pred(predicate): IdxVec<PredId, Constructor<PredId>> = b'R',
    Func(functor): IdxVec<FuncId, TyConstructor<FuncId>> = b'K',
    Sel(selector): IdxVec<SelId, TyConstructor<SelId>> = b'U',
    Aggr(aggregate): IdxVec<AggrId, Aggregate> = b'G',
  }
}

#[derive(Clone, Debug, Default)]
pub struct Clusters {
    pub registered: Vec<RegisteredCluster>,
    /// sorted by |a, b| FunctorCluster::cmp_term(&a.term, ctx, &b.term)
    pub functor: SortedIdxVec<usize, FunctorCluster>,
    pub conditional: ConditionalClusters,
}
impl<V: VisitMut> Visitable<V> for Clusters {
    fn visit(&mut self, v: &mut V) {
        self.registered.visit(v);
        self.functor.visit(v);
        self.conditional.vec.visit(v);
    }
}

impl Clusters {
    pub fn len(&self) -> ClustersBase {
        ClustersBase {
            registered: self.registered.len() as u32,
            functor: self.functor.len() as u32,
            conditional: self.conditional.len() as u32,
        }
    }

    pub fn since(&self, base: &ClustersBase) -> ClustersRef<'_> {
        ClustersRef {
            registered: &self.registered[base.registered as usize..],
            functor: &self.functor.0[base.functor as usize..],
            conditional: &self.conditional.vec[base.conditional as usize..],
        }
    }

    pub fn append(&mut self, ctx: &Constructors, other: &mut ClustersRaw) {
        self.registered.append(&mut other.registered);
        self.functor.0.append(&mut other.functor);
        for cc in &other.conditional {
            self.conditional.update_attr_clusters(ctx, &cc.antecedent);
        }
        self.conditional.vec.append(&mut other.conditional)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ClustersBase {
    pub registered: u32,
    pub functor: u32,
    pub conditional: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClustersRef<'a> {
    pub registered: &'a [RegisteredCluster],
    pub functor: &'a [FunctorCluster],
    pub conditional: &'a [ConditionalCluster],
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ClustersRaw {
    pub registered: Vec<RegisteredCluster>,
    pub functor: Vec<FunctorCluster>,
    pub conditional: Vec<ConditionalCluster>,
}
impl<V: VisitMut> Visitable<V> for ClustersRaw {
    fn visit(&mut self, v: &mut V) {
        self.registered.visit(v);
        self.functor.visit(v);
        self.conditional.visit(v);
    }
}

impl ClustersRef<'_> {
    pub fn is_empty(&self) -> bool {
        self.registered.is_empty() && self.functor.is_empty() && self.conditional.is_empty()
    }

    pub fn to_owned(self) -> ClustersRaw {
        ClustersRaw {
            registered: self.registered.to_owned(),
            functor: self.functor.to_owned(),
            conditional: self.conditional.to_owned(),
        }
    }
}

impl ClustersRaw {
    pub fn as_ref(&self) -> ClustersRef<'_> {
        ClustersRef {
            registered: &self.registered,
            functor: &self.functor,
            conditional: &self.conditional,
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Cluster {
    /// nPrimaryList
    pub primary: Box<[Type]>,
    /// nConsequent.(Lower, Upper)
    pub consequent: (Attrs, Attrs),
    // /// nArticle
    // pub article: Article,
    // /// nAbsNr
    // pub abs_nr: u32,
}
impl<V: VisitMut> Visitable<V> for Cluster {
    fn visit(&mut self, v: &mut V) {
        v.with_locus_tys(&mut self.primary, |v| {
            v.visit_attr_pair(&mut self.consequent)
        });
    }
}

impl std::fmt::Debug for Cluster {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cluster")
            .field("primary", &self.primary)
            .field("consequent.lower", &self.consequent.0)
            .field("consequent.upper", &self.consequent.1)
            // .field("article", &self.article)
            // .field("abs_nr", &self.abs_nr)
            .finish()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RegisteredCluster {
    pub cl: Cluster,
    pub ty: Box<Type>,
}

impl std::ops::Deref for RegisteredCluster {
    type Target = Cluster;
    fn deref(&self) -> &Self::Target {
        &self.cl
    }
}
impl std::ops::DerefMut for RegisteredCluster {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cl
    }
}
impl<V: VisitMut> Visitable<V> for RegisteredCluster {
    fn visit(&mut self, v: &mut V) {
        v.with_locus_tys(&mut self.cl.primary, |v| {
            v.visit_attr_pair(&mut self.cl.consequent);
            self.ty.visit(v);
        });
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConditionalCluster {
    pub cl: Cluster,
    pub ty: Box<Type>,
    pub antecedent: Attrs,
}
impl std::ops::Deref for ConditionalCluster {
    type Target = Cluster;
    fn deref(&self) -> &Self::Target {
        &self.cl
    }
}
impl std::ops::DerefMut for ConditionalCluster {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cl
    }
}
impl<V: VisitMut> Visitable<V> for ConditionalCluster {
    fn visit(&mut self, v: &mut V) {
        v.with_locus_tys(&mut self.cl.primary, |v| {
            v.visit_attr_pair(&mut self.cl.consequent);
            self.ty.visit(v);
            self.antecedent.visit(v);
        });
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FunctorCluster {
    pub cl: Cluster,
    pub ty: Option<Box<Type>>,
    pub term: Box<Term>,
}

impl std::ops::Deref for FunctorCluster {
    type Target = Cluster;
    fn deref(&self) -> &Self::Target {
        &self.cl
    }
}
impl std::ops::DerefMut for FunctorCluster {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cl
    }
}
impl<V: VisitMut> Visitable<V> for FunctorCluster {
    fn visit(&mut self, v: &mut V) {
        v.with_locus_tys(&mut self.cl.primary, |v| {
            v.visit_attr_pair(&mut self.cl.consequent);
            self.ty.visit(v);
            self.term.visit(v);
        });
    }
}

impl FunctorCluster {
    pub fn cmp_term(this: &Term, ctx: &Constructors, other: &Term) -> std::cmp::Ordering {
        match (this, other) {
            (&Term::Functor { nr: n1, .. }, &Term::Functor { nr: n2, .. }) => {
                let n1 = ctx.functor[n1].redefines.unwrap_or(n1);
                let n2 = ctx.functor[n2].redefines.unwrap_or(n2);
                n1.cmp(&n2)
            }
            (Term::Functor { .. }, _) => std::cmp::Ordering::Greater,
            (_, Term::Functor { .. }) => std::cmp::Ordering::Less,
            (_, _) => std::cmp::Ordering::Equal,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct ConditionalClusters {
    pub vec: Vec<ConditionalCluster>,
    pub attr_clusters: EnumMap<bool, BTreeMap<AttrId, BTreeSet<u32>>>,
}
impl std::ops::Deref for ConditionalClusters {
    type Target = [ConditionalCluster];
    fn deref(&self) -> &Self::Target {
        &self.vec
    }
}
impl std::ops::DerefMut for ConditionalClusters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.vec
    }
}
impl<V: VisitMut> Visitable<V> for ConditionalClusters {
    fn visit(&mut self, v: &mut V) {
        self.vec.visit(v);
    }
}

impl ConditionalClusters {
    pub fn update_attr_clusters(&mut self, ctx: &Constructors, attrs: &Attrs) {
        if let Attrs::Consistent(attrs) = attrs {
            for attr in attrs {
                self.attr_clusters[attr.pos]
                    .entry(attr.adjusted_nr(ctx))
                    .or_default()
                    .insert(self.vec.len() as u32);
            }
        }
    }
    pub fn push(&mut self, ctx: &Constructors, cc: ConditionalCluster) {
        self.update_attr_clusters(ctx, &cc.antecedent);
        self.vec.push(cc)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConstrDef {
    pub def_nr: DefId,
    pub article: Article,
    pub constr: ConstrKind,
    pub primary: Box<[Type]>,
}
impl<V: VisitMut> Visitable<V> for ConstrDef {
    fn visit(&mut self, v: &mut V) {
        self.constr.visit(v);
        v.with_locus_tys(&mut self.primary, |_| {})
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DefCase<T> {
    pub case: T,
    pub guard: Formula,
}
impl<V: VisitMut, T: Visitable<V>> Visitable<V> for DefCase<T> {
    fn visit(&mut self, v: &mut V) {
        self.case.visit(v);
        self.guard.visit(v)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DefBody<T> {
    /// nPartialDefinientia
    pub cases: Box<[DefCase<T>]>,
    pub otherwise: Option<T>,
}
impl<V: VisitMut, T: Visitable<V>> Visitable<V> for DefBody<T> {
    fn visit(&mut self, v: &mut V) {
        self.cases.visit(v);
        self.otherwise.visit(v)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DefValue {
    Term(DefBody<Term>),
    Formula(DefBody<Formula>),
}
impl<V: VisitMut> Visitable<V> for DefValue {
    fn visit(&mut self, v: &mut V) {
        match self {
            DefValue::Term(body) => body.visit(v),
            DefValue::Formula(body) => body.visit(v),
        }
    }
}

impl DefValue {
    pub fn discr(&self) -> u8 {
        match self {
            DefValue::Term(_) => b'e',
            DefValue::Formula(_) => b'm',
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Definiens {
    pub c: ConstrDef,
    // pub lab_id: Option<LabelId>,
    pub essential: Box<[LocusId]>,
    pub assumptions: Formula,
    pub value: DefValue,
}

impl std::ops::Deref for Definiens {
    type Target = ConstrDef;
    fn deref(&self) -> &Self::Target {
        &self.c
    }
}

impl<V: VisitMut> Visitable<V> for Definiens {
    fn visit(&mut self, v: &mut V) {
        self.c.constr.visit(v);
        v.with_locus_tys(&mut self.c.primary, |v| {
            self.assumptions.visit(v);
            self.value.visit(v)
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Property {
    // pub article: Article,
    // pub abs_nr: u32,
    pub primary: Box<[Type]>,
    pub ty: Type,
    pub kind: PropertyKind,
}
impl<V: VisitMut> Visitable<V> for Property {
    fn visit(&mut self, v: &mut V) {
        v.with_locus_tys(&mut self.primary, |v| self.ty.visit(v))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentifyFunc {
    // pub article: Article,
    // pub abs_nr: u32,
    pub primary: Box<[Type]>,
    /// lhs must be Term::Functor
    pub lhs: Term,
    pub rhs: Term,
    pub eq_args: Box<[(LocusId, LocusId)]>,
}
impl<V: VisitMut> Visitable<V> for IdentifyFunc {
    fn visit(&mut self, v: &mut V) {
        v.with_locus_tys(&mut self.primary, |v| {
            self.lhs.visit(v);
            self.rhs.visit(v)
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reduction {
    // pub article: Article,
    // pub abs_nr: u32,
    pub primary: Box<[Type]>,
    pub terms: [Term; 2],
}
impl<V: VisitMut> Visitable<V> for Reduction {
    fn visit(&mut self, v: &mut V) {
        v.with_locus_tys(&mut self.primary, |v| {
            self.terms.iter_mut().for_each(|t| t.visit(v))
        });
    }
}

#[derive(Debug)]
pub struct EqualsDef {
    pub primary: Box<[Type]>,
    pub expansion: Term,
    pub pattern: (FuncId, Box<[Term]>),
    pub essential: Box<[LocusId]>,
}
impl<V: VisitMut> Visitable<V> for EqualsDef {
    fn visit(&mut self, v: &mut V) {
        v.with_locus_tys(&mut self.primary, |v| {
            self.expansion.visit(v);
            self.pattern.1.visit(v)
        })
    }
}

pub type ThmRef = (ArticleId, ThmId);
pub type DefRef = (ArticleId, DefId);
pub type SchRef = (ArticleId, SchId);

#[derive(Default, Debug)]
pub struct References {
    pub thm: HashSet<ThmRef>,
    pub def: HashSet<DefRef>,
    pub sch: HashSet<SchRef>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Scheme {
    pub sch_funcs: Box<[Type]>,
    pub prems: Box<[Formula]>,
    pub thesis: Formula,
}
impl<V: VisitMut> Visitable<V> for Scheme {
    fn visit(&mut self, v: &mut V) {
        v.with_sch_func_tys(&mut self.sch_funcs, |v| {
            self.prems.visit(v);
            self.thesis.visit(v);
        })
    }
}

#[derive(Default, Debug)]
pub struct Libraries {
    pub thm: BTreeMap<ThmRef, Formula>,
    pub def: BTreeMap<DefRef, Formula>,
    pub sch: BTreeMap<SchRef, Scheme>,
}
impl<V: VisitMut> Visitable<V> for Libraries {
    fn visit(&mut self, v: &mut V) {
        self.thm.values_mut().for_each(|f| f.visit(v));
        self.def.values_mut().for_each(|f| f.visit(v));
        self.sch.values_mut().for_each(|f| f.visit(v));
    }
}

#[derive(Copy, Clone, Default, Eq)]
pub struct Position {
    pub line: u32,
    pub col: u32,
}

impl PartialEq for Position {
    /// When used in export verification, we don't compare positions
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl std::fmt::Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.col)
    }
}

#[derive(Clone, Debug)]
pub enum SchemeDecl {
    Func { args: Box<[Type]>, ty: Type },
    Pred { args: Box<[Type]> },
}
impl<V: VisitMut> Visitable<V> for SchemeDecl {
    fn visit(&mut self, v: &mut V) {
        match self {
            SchemeDecl::Func { args, ty } => (args, ty).visit(v),
            SchemeDecl::Pred { args } => args.visit(v),
        }
    }
}

#[derive(Debug)]
pub enum InferenceKind {
    By { linked: bool },
    From { sch: SchRef },
}

#[derive(Debug, Clone, Copy)]
pub enum ReferenceKind {
    Priv(LabelId),
    Thm(ThmRef),
    Def(DefRef),
}

#[derive(Debug, Clone, Copy)]
pub struct Reference {
    pub pos: Position,
    pub kind: ReferenceKind,
}

#[derive(Debug)]
pub struct Inference {
    pub kind: InferenceKind,
    pub pos: Position,
    pub refs: Vec<Reference>,
}

#[derive(Debug)]
pub struct Thesis {
    pub f: Formula,
    pub exps: Vec<(u32, u32)>,
}

#[derive(Debug)]
pub enum Justification {
    Simple(Inference),
    Proof {
        pos: (Position, Position),
        label: Option<LabelId>,
        thesis: Formula,
        items: Vec<(Item, Option<Thesis>)>,
    },
    SkippedProof,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DefinitionKind {
    PrAttr,
    Mode,
    Pred,
    Func,
    ExpandMode,
}

#[derive(Debug)]
pub enum ClusterKind {
    R,
    F,
    C,
}

#[derive(Debug)]
pub enum ClusterDeclKind {
    R(RegisteredCluster),
    F(FunctorCluster),
    C(ConditionalCluster),
}
impl<V: VisitMut> Visitable<V> for ClusterDeclKind {
    fn visit(&mut self, v: &mut V) {
        match self {
            ClusterDeclKind::R(c) => c.visit(v),
            ClusterDeclKind::F(c) => c.visit(v),
            ClusterDeclKind::C(c) => c.visit(v),
        }
    }
}

#[derive(Debug)]
pub struct ClusterDecl {
    pub kind: ClusterDeclKind,
    pub conds: Vec<CorrCond>,
    pub corr: Option<Correctness>,
}

#[derive(Debug)]
pub struct JustifiedProperty {
    pub kind: PropertyKind,
    pub prop: Proposition,
    pub just: Justification,
}

#[derive(Debug)]
pub struct Definition {
    pub pos: Position,
    pub label: Option<LabelId>,
    pub redef: bool,
    pub kind: DefinitionKind,
    pub conds: Vec<CorrCond>,
    pub corr: Option<Correctness>,
    pub props: Vec<JustifiedProperty>,
    pub constr: Option<ConstructorDef>,
    pub patts: Vec<Pattern>,
}

#[derive(Debug)]
pub struct DefStruct {
    pub pos: Position,
    pub label: Option<LabelId>,
    pub constrs: Vec<ConstructorDef>,
    pub cl: ClusterDecl,
    pub conds: Vec<CorrCond>,
    pub corr: Option<Correctness>,
    pub patts: Vec<Pattern>,
}

#[derive(Clone)]
pub struct Proposition {
    pub pos: Position,
    pub label: Option<LabelId>,
    pub f: Formula,
}

impl std::fmt::Debug for Proposition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?}] ", self.pos)?;
        if let Some(id) = self.label {
            write!(f, "L{id:?}: ")?;
        }
        write!(f, "{:?}", self.f)
    }
}
impl<V: VisitMut> Visitable<V> for Proposition {
    fn visit(&mut self, v: &mut V) {
        self.f.visit(v)
    }
}
#[derive(Debug)]
pub enum Statement {
    Proposition {
        prop: Proposition,
        just: Justification,
    },
    IterEquality {
        start: Position,
        label: Option<LabelId>,
        lhs: Term,
        steps: Vec<(Term, Inference)>,
    },
    Now {
        pos: (Position, Position),
        label: Option<LabelId>,
        thesis: Formula,
        items: Box<[Item]>,
    },
}
impl Statement {
    pub fn pos(&self) -> Position {
        match self {
            Statement::Proposition { prop, .. } => prop.pos,
            Statement::IterEquality { start, .. } => *start,
            Statement::Now { pos, .. } => pos.0,
        }
    }
}

#[derive(Debug)]
pub struct GivenItem {
    pub prop: Proposition,
    pub fixed: Vec<Type>,
    pub intro: Vec<Proposition>,
}

#[derive(Debug)]
pub enum AuxiliaryItem {
    Statement(Statement),
    /// itChoice
    Consider {
        prop: Proposition,
        just: Justification,
        fixed: Vec<Type>,
        intro: Vec<Proposition>,
    },
    /// itConstantDefinition
    Set {
        term: Term,
        ty: Type,
    },
    Reconsider {
        terms: Vec<(Type, Term)>,
        prop: Proposition,
        just: Justification,
    },
    /// itPrivFuncDefinition
    DefFunc {
        args: Box<[Type]>,
        ty: Type,
        value: Term,
    },
    /// itPrivPredDefinition
    DefPred {
        args: Box<[Type]>,
        value: Formula,
    },
}
impl AuxiliaryItem {
    pub fn pos(&self) -> Option<Position> {
        match self {
            AuxiliaryItem::Statement(stmt) => Some(stmt.pos()),
            AuxiliaryItem::Consider { prop, .. } => Some(prop.pos),
            AuxiliaryItem::Set { .. }
            | AuxiliaryItem::Reconsider { .. }
            | AuxiliaryItem::DefFunc { .. }
            | AuxiliaryItem::DefPred { .. } => None,
        }
    }
}

#[derive(Debug)]
pub enum Registration {
    Cluster(ClusterDecl),
    Identify {
        kind: IdentifyFunc,
        conds: Vec<CorrCond>,
        corr: Option<Correctness>,
    },
    Reduction {
        kind: Reduction,
        conds: Vec<CorrCond>,
        corr: Option<Correctness>,
    },
    Property {
        kind: Property,
        prop: Proposition,
        just: Justification,
    },
}

#[derive(Clone, Copy, Debug, Enum, PartialEq, Eq)]
pub enum CorrCondKind {
    Coherence,
    Compatibility,
    Consistency,
    Existence,
    Uniqueness,
    Reducibility,
}

impl CorrCondKind {
    pub fn name(self) -> &'static [u8] {
        match self {
            CorrCondKind::Coherence => b"coherence",
            CorrCondKind::Compatibility => b"compatibility",
            CorrCondKind::Consistency => b"consistency",
            CorrCondKind::Existence => b"existence",
            CorrCondKind::Uniqueness => b"uniqueness",
            CorrCondKind::Reducibility => b"reducibility",
        }
    }
}

impl TryFrom<&[u8]> for CorrCondKind {
    type Error = ();
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        match value {
            b"coherence" => Ok(CorrCondKind::Coherence),
            b"compatibility" => Ok(CorrCondKind::Compatibility),
            b"consistency" => Ok(CorrCondKind::Consistency),
            b"existence" => Ok(CorrCondKind::Existence),
            b"uniqueness" => Ok(CorrCondKind::Uniqueness),
            b"reducibility" => Ok(CorrCondKind::Reducibility),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct SimpleCorrCond {
    pub kind: CorrCondKind,
    pub f: Formula,
}

#[derive(Debug)]
pub struct CorrCond {
    pub kind: CorrCondKind,
    pub prop: Proposition,
    pub just: Justification,
}

#[derive(Debug)]
pub struct Correctness {
    pub conds: Vec<SimpleCorrCond>,
    pub prop: Proposition,
    pub just: Justification,
}

#[derive(Debug)]
pub struct SchemeBlock {
    pub pos: (Position, Position),
    pub nr: SchId,
    pub decls: Vec<SchemeDecl>,
    pub prems: Vec<Proposition>,
    pub thesis: Proposition,
    pub just: Justification,
}

#[derive(Copy, Clone, Debug)]
pub enum CancelKind {
    Def,
    Thm,
    Sch,
}

#[derive(Debug)]
pub enum CaseKind {
    Case(Vec<Proposition>),
    Suppose(Vec<Proposition>),
}

#[derive(Debug)]
pub struct CaseBlock {
    pub pos: (Position, Position),
    pub block_thesis: Formula,
    pub cs: CaseKind,
    pub items: Vec<(Item, Option<Thesis>)>,
    pub thesis: Option<Thesis>,
}

#[derive(Debug)]
pub struct PerCases {
    pub pos: (Position, Position),
    pub label: Option<LabelId>,
    pub block_thesis: Formula,
    pub cases: Vec<CaseBlock>,
    pub prop: Proposition,
    pub just: Justification,
    pub thesis: Option<Thesis>,
}

#[derive(Copy, Clone, Debug)]
pub enum BlockKind {
    Definition,
    Registration,
    Notation,
}

#[derive(Debug)]
pub enum Item {
    /// itGeneralization
    Let(Vec<Type>),
    /// itExistentialAssumption
    Given(GivenItem),
    /// itConclusion
    Thus(Statement),
    /// itAssumption
    /// invariant: not empty
    Assume(Vec<Proposition>),
    /// itSimpleExemplification
    Take(Term),
    /// itExemplificationWithEquality
    TakeAsVar(Type, Term),
    PerCases(PerCases),
    AuxiliaryItem(AuxiliaryItem),
    Registration(Registration),
    Scheme(SchemeBlock),
    Theorem {
        prop: Proposition,
        just: Justification,
    },
    DefTheorem {
        kind: Option<ConstrKind>,
        prop: Proposition,
    },
    Reservation {
        ids: Vec<u32>,
        ty: Box<Type>,
    },
    Canceled(CancelKind),
    Definition(Definition),
    DefStruct(DefStruct),
    Definiens(Definiens),
    Pattern(Pattern),
    Block {
        kind: BlockKind,
        pos: (Position, Position),
        label: Option<LabelId>,
        items: Vec<Item>,
    },
}

impl Item {
    pub fn pos(&self) -> Option<Position> {
        match self {
            Item::Given(it) => Some(it.prop.pos),
            Item::Thus(stmt) => Some(stmt.pos()),
            Item::Assume(prop) => Some(prop[0].pos),
            Item::PerCases(it) => Some(it.pos.0),
            Item::AuxiliaryItem(it) => it.pos(),
            Item::Scheme(it) => Some(it.pos.0),
            Item::Theorem { prop, .. } | Item::DefTheorem { prop, .. } => Some(prop.pos),
            Item::Definition(it) => Some(it.pos),
            Item::DefStruct(it) => Some(it.pos),
            Item::Block { pos, .. } => Some(pos.0),
            Item::Let(_)
            | Item::Take(_)
            | Item::TakeAsVar(..)
            | Item::Registration(_)
            | Item::Reservation { .. }
            | Item::Canceled(_)
            | Item::Definiens(_)
            | Item::Pattern(_) => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Enum, PartialEq, Eq, PartialOrd, Ord)]
pub enum SymbolKindClass {
    Struct,
    LeftBrk,
    RightBrk,
    Mode,
    Func,
    Pred,
    Sel,
    Attr,
}

impl SymbolKindClass {
    pub fn discr(&self) -> u8 {
        match self {
            SymbolKindClass::Struct => b'G',
            SymbolKindClass::LeftBrk => b'K',
            SymbolKindClass::RightBrk => b'L',
            SymbolKindClass::Mode => b'M',
            SymbolKindClass::Func => b'O',
            SymbolKindClass::Pred => b'R',
            SymbolKindClass::Sel => b'U',
            SymbolKindClass::Attr => b'V',
        }
    }
    pub fn parse(c: u8) -> Self {
        match c {
            b'G' => SymbolKindClass::Struct,
            b'K' => SymbolKindClass::LeftBrk,
            b'L' => SymbolKindClass::RightBrk,
            b'M' => SymbolKindClass::Mode,
            b'O' => SymbolKindClass::Func,
            b'R' => SymbolKindClass::Pred,
            b'U' => SymbolKindClass::Sel,
            b'V' => SymbolKindClass::Attr,
            _ => panic!("unexpected symbol kind {:?}", c as char),
        }
    }
}

mk_id! {
  FuncSymId(u32),
  LeftBrkSymId(u32),
  RightBrkSymId(u32),
  PredSymId(u32),
  ModeSymId(u32),
  AttrSymId(u32),
  StructSymId(u32),
  SelSymId(u32),
  IdentId(u32),
}

impl ModeSymId {
    pub const SET: Self = Self(0); // set
}
impl PredSymId {
    pub const EQUAL: Self = Self(0); // =
}
impl LeftBrkSymId {
    pub const LBRACK: Self = Self(0); // [
    pub const LBRACE: Self = Self(1); // {
    pub const LPAREN: Self = Self(2); // (
}
impl RightBrkSymId {
    pub const RBRACK: Self = Self(0); // ]
    pub const RBRACE: Self = Self(1); // }
    pub const RPAREN: Self = Self(2); // )
}
impl AttrSymId {
    // The "strict" (a.k.a "not abstract") builtin attribute
    pub const STRICT: Self = Self(0);
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SymbolKind {
    Func(FuncSymId),
    LeftBrk(LeftBrkSymId),
    RightBrk(RightBrkSymId),
    Pred(PredSymId),
    Mode(ModeSymId),
    Attr(AttrSymId),
    Struct(StructSymId),
    Sel(SelSymId),
}
impl From<FuncSymId> for SymbolKind {
    fn from(v: FuncSymId) -> Self {
        Self::Func(v)
    }
}
impl From<LeftBrkSymId> for SymbolKind {
    fn from(v: LeftBrkSymId) -> Self {
        Self::LeftBrk(v)
    }
}
impl From<RightBrkSymId> for SymbolKind {
    fn from(v: RightBrkSymId) -> Self {
        Self::RightBrk(v)
    }
}
impl From<PredSymId> for SymbolKind {
    fn from(v: PredSymId) -> Self {
        Self::Pred(v)
    }
}
impl From<ModeSymId> for SymbolKind {
    fn from(v: ModeSymId) -> Self {
        Self::Mode(v)
    }
}
impl From<AttrSymId> for SymbolKind {
    fn from(v: AttrSymId) -> Self {
        Self::Attr(v)
    }
}
impl From<StructSymId> for SymbolKind {
    fn from(v: StructSymId) -> Self {
        Self::Struct(v)
    }
}
impl From<SelSymId> for SymbolKind {
    fn from(v: SelSymId) -> Self {
        Self::Sel(v)
    }
}
impl From<(SymbolKindClass, u32)> for SymbolKind {
    fn from((kind, n): (SymbolKindClass, u32)) -> Self {
        match kind {
            SymbolKindClass::Struct => Self::Struct(StructSymId(n)),
            SymbolKindClass::LeftBrk => Self::LeftBrk(LeftBrkSymId(n)),
            SymbolKindClass::RightBrk => Self::RightBrk(RightBrkSymId(n)),
            SymbolKindClass::Mode => Self::Mode(ModeSymId(n)),
            SymbolKindClass::Func => Self::Func(FuncSymId(n)),
            SymbolKindClass::Pred => Self::Pred(PredSymId(n)),
            SymbolKindClass::Sel => Self::Sel(SelSymId(n)),
            SymbolKindClass::Attr => Self::Attr(AttrSymId(n)),
        }
    }
}

impl From<SymbolKind> for (SymbolKindClass, u32) {
    fn from(kind: SymbolKind) -> Self {
        match kind {
            SymbolKind::Struct(StructSymId(n)) => (SymbolKindClass::Struct, n),
            SymbolKind::LeftBrk(LeftBrkSymId(n)) => (SymbolKindClass::LeftBrk, n),
            SymbolKind::RightBrk(RightBrkSymId(n)) => (SymbolKindClass::RightBrk, n),
            SymbolKind::Mode(ModeSymId(n)) => (SymbolKindClass::Mode, n),
            SymbolKind::Func(FuncSymId(n)) => (SymbolKindClass::Func, n),
            SymbolKind::Pred(PredSymId(n)) => (SymbolKindClass::Pred, n),
            SymbolKind::Sel(SelSymId(n)) => (SymbolKindClass::Sel, n),
            SymbolKind::Attr(AttrSymId(n)) => (SymbolKindClass::Attr, n),
        }
    }
}

pub type Symbols = Vec<(SymbolKind, String)>;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct FormatAggr {
    pub sym: StructSymId,
    pub args: u8,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct FormatStruct {
    pub sym: StructSymId,
    pub args: u8,
}
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct FormatMode {
    pub sym: ModeSymId,
    pub args: u8,
}
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct FormatAttr {
    pub sym: AttrSymId,
    pub args: u8,
}
impl FormatAttr {
    // The "strict" (a.k.a "not abstract") builtin attribute
    pub const STRICT: Self = Self {
        sym: AttrSymId::STRICT,
        args: 1,
    };
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum FormatFunc {
    Func {
        sym: FuncSymId,
        left: u8,
        right: u8,
    },
    Bracket {
        lsym: LeftBrkSymId,
        rsym: RightBrkSymId,
        args: u8,
    },
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct FormatPred {
    pub sym: PredSymId,
    pub left: u8,
    pub right: u8,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Format {
    Aggr(FormatAggr),
    SubAggr(StructSymId),
    Struct(FormatStruct),
    Mode(FormatMode),
    Sel(SelSymId),
    Attr(FormatAttr),
    Func(FormatFunc),
    Pred(FormatPred),
}

macro_rules! impl_format_visit {
  ($visit:ident$(, $mutbl:tt)?) => {
    pub fn $visit(&$($mutbl)? self, mut f: impl FnMut(SymbolKindClass, $(&$mutbl)? u32)) {
      match self {
        Format::Mode(fmt) => f(SymbolKindClass::Mode, $(&$mutbl)? fmt.sym.0),
        Format::Struct(FormatStruct { sym, .. })
        | Format::Aggr(FormatAggr { sym, .. })
        | Format::SubAggr(sym) => f(SymbolKindClass::Struct, $(&$mutbl)? sym.0),
        Format::Sel(sym) => f(SymbolKindClass::Sel, $(&$mutbl)? sym.0),
        Format::Attr(fmt) => f(SymbolKindClass::Attr, $(&$mutbl)? fmt.sym.0),
        Format::Func(FormatFunc::Func { sym, .. }) => f(SymbolKindClass::Func, $(&$mutbl)? sym.0),
        Format::Func(FormatFunc::Bracket { lsym, rsym, .. }) => {
          f(SymbolKindClass::LeftBrk, $(&$mutbl)? lsym.0);
          f(SymbolKindClass::RightBrk, $(&$mutbl)? rsym.0)
        }
        Format::Pred(fmt) => f(SymbolKindClass::Pred, $(&$mutbl)? fmt.sym.0),
      }
    }
  }
}

impl Format {
    impl_format_visit!(visit);
    impl_format_visit!(visit_mut, mut);

    pub fn discr(&self) -> u8 {
        match self {
            Format::Aggr(_) => b'G',
            Format::SubAggr(_) => b'J',
            Format::Struct(_) => b'L',
            Format::Mode(_) => b'M',
            Format::Sel(_) => b'U',
            Format::Attr(_) => b'V',
            Format::Func(FormatFunc::Func { .. }) => b'O',
            Format::Func(FormatFunc::Bracket { .. }) => b'K',
            Format::Pred(_) => b'R',
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum PriorityKind {
    Functor(FuncSymId),
    LeftBrk(LeftBrkSymId),
    RightBrk(RightBrkSymId),
}

mk_id! {
  FormatId(u32),
}
impl FormatId {
    // The first format is always the "strict" (a.k.a "not abstract") builtin attribute
    pub const STRICT: Self = Self(0);
}

#[derive(Debug, Default)]
pub struct Formats {
    pub formats: IdxVec<FormatId, Format>,
    // pub priority: Vec<(PriorityKind, u32)>,
}

#[derive(Clone, Copy, Debug, Enum)]
pub enum PatternKindClass {
    Mode,
    Struct,
    Attr,
    Pred,
    Func,
    Sel,
    Aggr,
    SubAggr,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PatternKind {
    Mode(ModeId),
    ExpandableMode { expansion: Box<Type> },
    Struct(StructId),
    Attr(AttrId),
    Pred(PredId),
    Func(FuncId),
    Sel(SelId),
    Aggr(AggrId),
    SubAggr(StructId),
}
impl<V: VisitMut> Visitable<V> for PatternKind {
    fn visit(&mut self, v: &mut V) {
        match self {
            Self::Mode(nr) => nr.visit(v),
            Self::ExpandableMode { expansion } => expansion.visit(v),
            Self::Struct(nr) => nr.visit(v),
            Self::Attr(nr) => nr.visit(v),
            Self::Pred(nr) => nr.visit(v),
            Self::Func(nr) => nr.visit(v),
            Self::Sel(nr) => nr.visit(v),
            Self::Aggr(nr) => nr.visit(v),
            Self::SubAggr(nr) => nr.visit(v),
        }
    }
}

impl PatternKind {
    pub fn class(&self) -> PatternKindClass {
        match self {
            Self::Mode(_) | Self::ExpandableMode { .. } => PatternKindClass::Mode,
            Self::Struct(_) => PatternKindClass::Struct,
            Self::Attr(_) => PatternKindClass::Attr,
            Self::Pred(_) => PatternKindClass::Pred,
            Self::Func(_) => PatternKindClass::Func,
            Self::Sel(_) => PatternKindClass::Sel,
            Self::Aggr(_) => PatternKindClass::Aggr,
            Self::SubAggr(_) => PatternKindClass::SubAggr,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pattern<F = FormatId> {
    pub kind: PatternKind,
    // pub pid: u32,
    pub article: Article,
    pub abs_nr: u32,
    pub fmt: F,
    // pub redefines: Option<u32>,
    pub primary: Box<[Type]>,
    pub visible: Box<[LocusId]>,
    pub pos: bool,
}
impl<V: VisitMut, F> Visitable<V> for Pattern<F> {
    fn visit(&mut self, v: &mut V) {
        self.kind.visit(v);
        self.primary.visit(v);
    }
}

#[derive(Debug, Default)]
pub struct Notations<F>(pub Vec<Pattern<F>>);

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct SymbolsBase(pub EnumMap<SymbolKindClass, u32>);

impl std::ops::AddAssign<&Self> for SymbolsBase {
    fn add_assign(&mut self, rhs: &Self) {
        for (i, val) in &rhs.0 {
            self.0[i] += val
        }
    }
}
impl std::ops::Sub<&Self> for SymbolsBase {
    type Output = Self;
    fn sub(mut self, rhs: &Self) -> Self::Output {
        for (i, val) in rhs.0 {
            self.0[i] -= val
        }
        self
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Vocabularies(pub Vec<(Article, SymbolsBase)>);

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DepNotation {
    pub sig: Vec<Article>,
    pub vocs: Vocabularies,
    pub pats: Vec<Pattern<Format>>,
}

#[derive(Debug, Default)]
pub struct AccumConstructors {
    pub sig: SigBuilder,
    pub constrs: Constructors,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct DepConstructors {
    pub sig: Vec<Article>,
    pub counts: ConstructorsBase,
    // The indexes here are offset by `counts`
    pub constrs: Constructors,
}

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct DepClusters {
    pub sig: Vec<Article>,
    pub cl: ClustersRaw,
}

#[derive(Clone, Default, PartialEq, Eq)]
pub struct DepIdentify {
    pub sig: Vec<Article>,
    pub defs: Vec<Definiens>,
}

#[derive(Clone, Default, PartialEq, Eq)]
pub struct DepReductions {
    pub sig: Vec<Article>,
    pub defs: Vec<Definiens>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TheoremKind {
    CanceledThm,
    CanceledDef,
    Def(ConstrKind),
    Thm,
}
impl<V: VisitMut> Visitable<V> for TheoremKind {
    fn visit(&mut self, v: &mut V) {
        if let Self::Def(k) = self {
            k.visit(v)
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Theorem {
    pub pos: Position,
    pub kind: TheoremKind,
    pub stmt: Formula,
}
impl<V: VisitMut> Visitable<V> for Theorem {
    fn visit(&mut self, v: &mut V) {
        self.kind.visit(v);
        self.stmt.visit(v)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DepTheorems {
    pub sig: Vec<Article>,
    pub thm: Vec<Theorem>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DepSchemes {
    pub sig: Vec<Article>,
    pub sch: Vec<Option<Scheme>>,
}

#[derive(Clone, Copy, Debug, Enum, PartialEq, Eq)]
pub enum DirectiveKind {
    Vocabularies,
    Notations,
    Definitions,
    Theorems,
    Schemes,
    Registrations,
    Constructors,
    Requirements,
    Equalities,
    Expansions,
}

impl DirectiveKind {
    pub fn name(self) -> &'static str {
        match self {
            Self::Vocabularies => "vocabularies",
            Self::Notations => "notations",
            Self::Definitions => "definitions",
            Self::Theorems => "theorems",
            Self::Schemes => "schemes",
            Self::Registrations => "registrations",
            Self::Constructors => "constructors",
            Self::Requirements => "requirements",
            Self::Equalities => "equalities",
            Self::Expansions => "expansions",
        }
    }
}

#[derive(Debug, Default)]
pub struct Directives(pub EnumMap<DirectiveKind, Vec<(Position, Article)>>);

#[derive(Clone, Debug)]
pub struct DepRequirement {
    pub req: Requirement,
    pub kind: ConstrKind,
}

#[derive(Clone, Debug, Default)]
pub struct DepRequirements {
    pub sig: Vec<Article>,
    pub reqs: Vec<DepRequirement>,
}

pub const DEFAULT_PRIO: u32 = 64;
pub enum SymbolDataKind<'a> {
    Struct,
    LeftBrk,
    RightBrk,
    Mode,
    Func { prio: u32 },
    Pred { infinitive: Option<&'a str> },
    Sel,
    Attr,
}
impl<'a> SymbolDataKind<'a> {
    pub fn class(&self) -> SymbolKindClass {
        match *self {
            SymbolDataKind::Struct => SymbolKindClass::Struct,
            SymbolDataKind::LeftBrk => SymbolKindClass::LeftBrk,
            SymbolDataKind::RightBrk => SymbolKindClass::RightBrk,
            SymbolDataKind::Mode => SymbolKindClass::Mode,
            SymbolDataKind::Func { .. } => SymbolKindClass::Func,
            SymbolDataKind::Pred { .. } => SymbolKindClass::Pred,
            SymbolDataKind::Sel => SymbolKindClass::Sel,
            SymbolDataKind::Attr => SymbolKindClass::Attr,
        }
    }
}

pub struct SymbolData<'a> {
    pub kind: SymbolDataKind<'a>,
    pub token: &'a str,
}

impl SymbolData<'static> {
    pub const BUILTIN_SYMBOLS: &'static [(SymbolKindClass, &'static str)] = &[
        (SymbolKindClass::Mode, "set"),
        (SymbolKindClass::Pred, "="),
        (SymbolKindClass::LeftBrk, "["),
        (SymbolKindClass::RightBrk, "]"),
        (SymbolKindClass::LeftBrk, "{"),
        (SymbolKindClass::RightBrk, "}"),
        (SymbolKindClass::LeftBrk, "("),
        (SymbolKindClass::RightBrk, ")"),
    ];
}

#[derive(Default)]
pub struct Vocabulary<'a> {
    pub base: SymbolsBase,
    pub symbols: Vec<SymbolData<'a>>,
}
