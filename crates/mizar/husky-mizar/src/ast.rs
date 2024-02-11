use crate::mk_id;
use crate::types::*;
use std::rc::Rc;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Variable {
    pub pos: Position,
    /// 'varnr' attribute, MSLocusObj.nVarNr, MSVariableObj.nVarNr
    pub var: Option<ConstId>,
    pub spelling: Rc<str>,
}

impl Variable {
    #[track_caller]
    pub fn var(&self) -> ConstId {
        self.var.expect("variable is not resolved")
    }

    pub fn to_term(&self) -> Term {
        Term::Var {
            pos: self.pos,
            kind: self.var.map(VarKind::Const),
            spelling: (*self.spelling).into(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum VarKind {
    Bound(BoundId),
    Const(ConstId),
    Reserved(ReservedId),
}

#[derive(Copy, Clone, Debug)]
pub enum PrivFuncKind {
    PrivFunc(PrivFuncId),
    SchFunc(SchFuncId),
}

#[derive(Debug)]
pub enum Term {
    Placeholder {
        pos: Position,
        nr: LocusId,
    },
    Numeral {
        pos: Position,
        value: u32,
    },
    Var {
        pos: Position,
        kind: Option<VarKind>,
        spelling: String,
    },
    PrivFunc {
        pos: Position,
        kind: Option<PrivFuncKind>,
        spelling: Rc<str>,
        args: Vec<Term>,
    },
    Infix {
        pos: Position,
        sym: (FuncSymId, String),
        left: u8,
        args: Vec<Term>,
    },
    Bracket {
        pos: Position,
        lsym: (LeftBrkSymId, String),
        rsym: (RightBrkSymId, String),
        args: Vec<Term>,
    },
    Aggregate {
        pos: Position,
        sym: (StructSymId, String),
        args: Vec<Term>,
    },
    SubAggr {
        pos: Position,
        sym: (StructSymId, String),
        arg: Box<Term>,
    },
    Selector {
        pos: Position,
        sym: (SelSymId, String),
        arg: Box<Term>,
    },
    InternalSelector {
        pos: Position,
        sym: (SelSymId, String),
        id: Option<ConstId>,
    },
    Qua {
        pos: Position,
        term: Box<Term>,
        ty: Box<Type>,
    },
    The {
        pos: Position,
        ty: Box<Type>,
    },
    Fraenkel {
        pos: Position,
        vars: Vec<BinderGroup>,
        scope: Box<Term>,
        compr: Option<Box<Formula>>,
        nameck: Option<Box<crate::analyze::FraenkelNameckResult>>,
    },
    It {
        pos: Position,
    },
}
impl Term {
    pub fn pos(&self) -> Position {
        match self {
            Term::Placeholder { pos, .. }
            | Term::Numeral { pos, .. }
            | Term::Var { pos, .. }
            | Term::PrivFunc { pos, .. }
            | Term::Infix { pos, .. }
            | Term::Bracket { pos, .. }
            | Term::Aggregate { pos, .. }
            | Term::SubAggr { pos, .. }
            | Term::Selector { pos, .. }
            | Term::InternalSelector { pos, .. }
            | Term::Qua { pos, .. }
            | Term::The { pos, .. }
            | Term::Fraenkel { pos, .. }
            | Term::It { pos } => *pos,
        }
    }
}

mk_id! {
  ReservedId(u32),
  ResGroupId(u32),
}

#[derive(Debug)]
pub enum Type {
    Mode {
        pos: Position,
        sym: (ModeSymId, String),
        args: Vec<Term>,
    },
    Struct {
        pos: Position,
        sym: (StructSymId, String),
        args: Vec<Term>,
    },
    Cluster {
        pos: Position,
        attrs: Vec<Attr>,
        ty: Box<Type>,
    },
    Reservation {
        pos: Position,
        group: ResGroupId,
        subst: Vec<VarKind>,
    },
}
impl Type {
    pub fn pos(&self) -> Position {
        match self {
            Type::Mode { pos, .. }
            | Type::Struct { pos, .. }
            | Type::Cluster { pos, .. }
            | Type::Reservation { pos, .. } => *pos,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum PrivPredKind {
    PrivPred(PrivPredId),
    SchPred(SchPredId),
}

#[derive(Copy, Clone, Debug)]
pub enum FormulaBinop {
    And,
    Or,
    Imp,
    Iff,
    FlexAnd,
    FlexOr,
}

#[derive(Copy, Clone, Debug)]
pub enum FormulaBinder {
    ForAll,
    Exists,
}

#[derive(Debug)]
pub struct Pred {
    pub pos: Position,
    pub positive: bool,
    pub sym: (PredSymId, String),
    pub left: u8,
    pub args: Vec<Term>,
}

#[derive(Debug)]
pub struct PredRhs {
    pub pos: Position,
    pub positive: bool,
    pub sym: (PredSymId, String),
    pub right: Vec<Term>,
}

#[derive(Debug)]
pub enum Formula {
    Not {
        pos: Position,
        f: Box<Formula>,
    },
    Binop {
        kind: FormulaBinop,
        pos: Position,
        f1: Box<Formula>,
        f2: Box<Formula>,
    },
    Pred(Box<Pred>),
    ChainPred {
        pos: Position,
        first: Box<Pred>,
        rest: Vec<PredRhs>,
    },
    PrivPred {
        pos: Position,
        kind: Option<PrivPredKind>,
        spelling: Rc<str>,
        args: Vec<Term>,
    },
    Attr {
        pos: Position,
        positive: bool,
        term: Box<Term>,
        attrs: Vec<Attr>,
    },
    Is {
        pos: Position,
        positive: bool,
        term: Box<Term>,
        ty: Box<Type>,
    },
    Binder {
        kind: FormulaBinder,
        pos: Position,
        vars: Vec<BinderGroup>,
        st: Option<Box<Formula>>,
        scope: Box<Formula>,
    },
    False {
        pos: Position,
    },
    Thesis {
        pos: Position,
    },
}
impl Formula {
    pub fn pos(&self) -> Position {
        match self {
            Formula::Pred(p) => p.pos,
            Formula::Not { pos, .. }
            | Formula::Binop { pos, .. }
            | Formula::False { pos, .. }
            | Formula::ChainPred { pos, .. }
            | Formula::PrivPred { pos, .. }
            | Formula::Attr { pos, .. }
            | Formula::Is { pos, .. }
            | Formula::Binder { pos, .. }
            | Formula::Thesis { pos } => *pos,
        }
    }
}

#[derive(Debug)]
pub struct Proposition {
    pub label: Option<Box<Label>>,
    pub f: Formula,
}

#[derive(Debug)]
pub enum SchRef {
    Resolved(ArticleId, SchId),
    UnresolvedPriv(String),
}

#[derive(Debug)]
pub enum InferenceKind {
    By { link: Option<Position> },
    From { sch: SchRef },
}

#[derive(Debug, Clone)]
pub enum RefFragment {
    Thm { pos: Position, id: ThmId },
    Def { pos: Position, id: DefId },
}
#[derive(Debug, Clone)]
pub enum ReferenceKind {
    Priv(Option<LabelId>),
    UnresolvedPriv(String),
    Global(ArticleId, Vec<RefFragment>),
}

#[derive(Debug, Clone)]
pub struct Reference {
    pub pos: Position,
    pub kind: ReferenceKind,
}

#[derive(Debug)]
pub enum Justification {
    Inference {
        pos: Position,
        kind: InferenceKind,
        refs: Vec<Reference>,
    },
    Block {
        pos: (Position, Position),
        items: Vec<Item>,
    },
}

#[derive(Debug)]
pub enum SchemeBinderGroup {
    Pred {
        pos: Position,
        vars: Vec<Variable>,
        tys: Vec<Type>,
    },
    Func {
        pos: Position,
        vars: Vec<Variable>,
        tys: Vec<Type>,
        ret: Type,
    },
}

#[derive(Debug)]
pub struct BinderGroup {
    pub vars: Vec<Variable>,
    // The vars list must be length 1 when this is None
    pub ty: Option<Box<Type>>,
}

#[derive(Debug)]
pub enum ReconsiderVar {
    /// Only occurs in wsm
    Var(Variable),
    Equality {
        var: Variable,
        tm: Term,
    },
}

#[derive(Debug)]
pub struct Item {
    pub pos: Position,
    pub kind: ItemKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CaseKind {
    Case,
    Suppose,
}

#[derive(Debug)]
pub struct Field {
    pub pos: Position,
    pub sym: (SelSymId, String),
}

#[derive(Debug)]
pub struct FieldGroup {
    pub pos: Position,
    pub vars: Vec<Field>,
    pub ty: Type,
}

#[derive(Debug)]
pub struct PatternStruct {
    pub sym: (StructSymId, String),
    pub args: Vec<Variable>,
}

impl PatternStruct {
    pub fn to_mode_format(&self) -> FormatStruct {
        FormatStruct {
            sym: self.sym.0,
            args: self.args.len() as u8,
        }
    }
    pub fn to_aggr_format(&self, n: usize) -> FormatAggr {
        FormatAggr {
            sym: self.sym.0,
            args: n as u8,
        }
    }
    pub fn to_subaggr_format(&self) -> StructSymId {
        self.sym.0
    }
}

#[derive(Debug)]
pub enum PatternFunc {
    Func {
        pos: Position,
        sym: (FuncSymId, String),
        left: u8,
        args: Vec<Variable>,
    },
    Bracket {
        pos: Position,
        lsym: (LeftBrkSymId, String),
        rsym: (RightBrkSymId, String),
        args: Vec<Variable>,
    },
}

impl PatternFunc {
    pub fn pos(&self) -> Position {
        let (PatternFunc::Func { pos, .. } | PatternFunc::Bracket { pos, .. }) = *self;
        pos
    }
    pub fn args(&self) -> &[Variable] {
        let (PatternFunc::Func { args, .. } | PatternFunc::Bracket { args, .. }) = self;
        args
    }
    pub fn args_mut(&mut self) -> &mut [Variable] {
        let (PatternFunc::Func { args, .. } | PatternFunc::Bracket { args, .. }) = self;
        args
    }
    pub fn to_format(&self) -> FormatFunc {
        match *self {
            PatternFunc::Func {
                ref sym,
                left,
                ref args,
                ..
            } => FormatFunc::Func {
                sym: sym.0,
                left,
                right: args.len() as u8 - left,
            },
            PatternFunc::Bracket {
                ref lsym,
                ref rsym,
                ref args,
                ..
            } => FormatFunc::Bracket {
                lsym: lsym.0,
                rsym: rsym.0,
                args: args.len() as u8,
            },
        }
    }
}

#[derive(Debug)]
pub struct PatternPred {
    pub pos: Position,
    pub sym: (PredSymId, String),
    pub left: u8,
    pub args: Vec<Variable>,
}
impl PatternPred {
    pub fn to_format(&self) -> FormatPred {
        FormatPred {
            sym: self.sym.0,
            left: self.left,
            right: self.args.len() as u8 - self.left,
        }
    }
}

#[derive(Debug)]
pub struct PatternMode {
    pub pos: Position,
    pub sym: (ModeSymId, String),
    pub args: Vec<Variable>,
}
impl PatternMode {
    pub fn to_format(&self) -> FormatMode {
        FormatMode {
            sym: self.sym.0,
            args: self.args.len() as u8,
        }
    }
}

#[derive(Debug)]
pub struct PatternAttr {
    pub pos: Position,
    pub sym: (AttrSymId, String),
    pub args: Vec<Variable>,
}
impl PatternAttr {
    pub fn to_format(&self) -> FormatAttr {
        FormatAttr {
            sym: self.sym.0,
            args: self.args.len() as u8,
        }
    }
}

#[derive(Debug)]
pub enum Pattern {
    Pred(Box<PatternPred>),
    Func(Box<PatternFunc>),
    Mode(Box<PatternMode>),
    Attr(Box<PatternAttr>),
}

impl Pattern {
    pub fn pos(&self) -> Position {
        match self {
            Pattern::Pred(p) => p.pos,
            Pattern::Func(p) => p.pos(),
            Pattern::Mode(p) => p.pos,
            Pattern::Attr(p) => p.pos,
        }
    }
}

#[derive(Debug)]
pub struct DefCase<T> {
    pub case: Box<T>,
    pub guard: Box<Formula>,
}

#[derive(Debug)]
pub struct DefBody<T> {
    /// nPartialDefinientia
    pub cases: Vec<DefCase<T>>,
    pub otherwise: Option<Box<T>>,
}

#[derive(Debug)]
pub enum DefValue {
    Term(DefBody<Term>),
    Formula(DefBody<Formula>),
}

#[derive(Debug)]
pub struct Definiens {
    pub pos: Position,
    pub label: Option<Box<Label>>,
    pub kind: DefValue,
}

#[derive(Debug)]
pub enum DefModeKind {
    Expandable {
        expansion: Box<Type>,
    },
    Standard {
        spec: Option<Box<Type>>,
        def: Option<Box<Definiens>>,
    },
}

#[derive(Debug)]
pub enum PatternRedef {
    Pred {
        new: Box<PatternPred>,
        orig: Box<PatternPred>,
        pos: bool,
    },
    Func {
        new: Box<PatternFunc>,
        orig: Box<PatternFunc>,
    },
    Mode {
        new: Box<PatternMode>,
        orig: Box<PatternMode>,
    },
    Attr {
        new: Box<PatternAttr>,
        orig: Box<PatternAttr>,
        pos: bool,
    },
}

#[derive(Debug)]
pub enum Attr {
    Attr {
        pos: Position,
        sym: (AttrSymId, String),
        args: Vec<Term>,
    },
    Non {
        pos: Position,
        attr: Box<Attr>,
    },
}

impl Attr {
    pub fn pos(&self) -> Position {
        match *self {
            Attr::Attr { pos, .. } | Attr::Non { pos, .. } => pos,
        }
    }
}

#[derive(Debug)]
pub enum ClusterDeclKind {
    Exist {
        concl: Vec<Attr>,
        ty: Box<Type>,
    },
    Func {
        term: Box<Term>,
        concl: Vec<Attr>,
        ty: Option<Box<Type>>,
    },
    Cond {
        antecedent: Vec<Attr>,
        concl: Vec<Attr>,
        ty: Box<Type>,
    },
}

#[derive(Debug)]
pub struct Label {
    pub pos: Position,
    pub id: (Option<LabelId>, Rc<str>),
}

#[derive(Debug)]
pub enum Assumption {
    Single {
        pos: Position,
        prop: Box<Proposition>,
    },
    Collective {
        pos: Position,
        conds: Vec<Proposition>,
    },
}
impl Assumption {
    pub fn conds(&mut self) -> &mut [Proposition] {
        match self {
            Assumption::Single { prop, .. } => std::slice::from_mut(prop),
            Assumption::Collective { conds, .. } => conds,
        }
    }
}

#[derive(Debug)]
pub struct SetDecl {
    pub var: Box<Variable>,
    pub value: Box<Term>,
}

#[derive(Debug)]
pub struct TakeDecl {
    pub var: Option<Box<Variable>>,
    pub term: Box<Term>,
}

#[derive(Debug)]
pub struct IterStep {
    pub pos: Position,
    pub rhs: Term,
    pub just: Justification,
}

#[derive(Debug)]
pub enum Statement {
    Proposition {
        prop: Box<Proposition>,
        just: Box<Justification>,
    },
    IterEquality {
        /// lhs = first rhs
        prop: Box<Proposition>,
        just: Box<Justification>,
        /// subsequent rhs's
        steps: Vec<IterStep>,
    },
    Now {
        end: Position,
        label: Option<Box<Label>>,
        items: Vec<Item>,
    },
}

#[derive(Debug)]
pub enum DefinitionKind {
    Func {
        pat: Box<PatternFunc>,
        spec: Option<Box<Type>>,
        def: Option<Box<Definiens>>,
    },
    Pred {
        pat: Box<PatternPred>,
        def: Option<Box<Definiens>>,
    },
    Mode {
        pat: Box<PatternMode>,
        kind: DefModeKind,
    },
    Attr {
        pat: Box<PatternAttr>,
        def: Option<Box<Definiens>>,
    },
}

impl DefinitionKind {
    pub fn pos(&self) -> Position {
        match self {
            DefinitionKind::Func { pat, .. } => match **pat {
                PatternFunc::Func { pos, .. } | PatternFunc::Bracket { pos, .. } => pos,
            },
            DefinitionKind::Pred { pat, .. } => pat.pos,
            DefinitionKind::Mode { pat, .. } => pat.pos,
            DefinitionKind::Attr { pat, .. } => pat.pos,
        }
    }
}

#[derive(Debug)]
pub struct CorrCond {
    pub pos: Position,
    pub kind: CorrCondKind,
    pub just: Justification,
}

#[derive(Debug)]
pub struct Correctness {
    pub pos: Position,
    pub just: Justification,
}

#[derive(Debug)]
pub struct Property {
    pub kind: PropertyKind,
    pub just: Box<Justification>,
}

#[derive(Debug)]
pub struct SchemeHead {
    pub sym: Option<Rc<str>>,
    pub nr: Option<SchId>,
    pub groups: Vec<SchemeBinderGroup>,
    pub concl: Formula,
    pub prems: Vec<Proposition>,
}

#[derive(Debug)]
pub struct SchemeBlock {
    pub end: Position,
    pub head: SchemeHead,
    pub items: Vec<Item>,
}

#[derive(Debug)]
pub struct Reservation {
    pub vars: Vec<Variable>,
    pub tys: Option<Vec<Type>>,
    pub fvars: Option<MizIdxVec<BoundId, ReservedId>>,
    pub ty: Box<Type>,
}

#[derive(Debug)]
pub struct Definition {
    pub kind: DefinitionKind,
    pub body: DefinitionBody,
}
#[derive(Debug)]
pub struct DefinitionBody {
    pub redef: bool,
    pub conds: Vec<CorrCond>,
    pub corr: Option<Correctness>,
    pub props: Vec<Property>,
}

#[derive(Debug)]
pub struct DefStruct {
    pub parents: Vec<Type>,
    pub fields: Vec<FieldGroup>,
    pub pat: PatternStruct,
}

#[derive(Debug)]
pub struct Cluster {
    pub kind: ClusterDeclKind,
    pub conds: Vec<CorrCond>,
    pub corr: Option<Correctness>,
}

#[derive(Debug)]
pub struct IdentifyFunc {
    pub lhs: Box<PatternFunc>,
    pub rhs: Box<PatternFunc>,
    pub eqs: Vec<(Variable, Variable)>,
    pub conds: Vec<CorrCond>,
    pub corr: Option<Correctness>,
}

#[derive(Debug)]
pub struct Reduction {
    pub from: Box<Term>,
    pub to: Box<Term>,
    pub conds: Vec<CorrCond>,
    pub corr: Option<Correctness>,
}

#[derive(Debug)]
pub struct CaseBlock {
    pub end: Position,
    pub hyp: Box<Assumption>,
    pub items: Vec<Item>,
}

#[derive(Debug)]
pub enum Pragma {
    /// $CD, $CT, $CS
    Canceled(CancelKind, u32),
    /// $N
    ThmDesc(String),
    /// $INSERT
    Insert(String),
    /// $V-, $V+
    SetVerify(bool),
    Other(String),
}

impl FromStr for Pragma {
    type Err = std::convert::Infallible;
    fn from_str(spelling: &str) -> Result<Self, Self::Err> {
        let parse_num = |s: &str| {
            if s.is_empty() {
                1
            } else {
                s.trim().parse::<u32>().unwrap()
            }
        };
        Ok(if let Some(s) = spelling.strip_prefix("$CD") {
            Pragma::Canceled(CancelKind::Def, parse_num(s))
        } else if let Some(s) = spelling.strip_prefix("$CT") {
            Pragma::Canceled(CancelKind::Thm, parse_num(s))
        } else if let Some(s) = spelling.strip_prefix("$CS") {
            Pragma::Canceled(CancelKind::Sch, parse_num(s))
        } else if let Some(s) = spelling.strip_prefix("$N") {
            Pragma::ThmDesc(s.trim_start().to_owned())
        } else if let Some(s) = spelling.strip_prefix("$INSERT") {
            Pragma::Insert(s.trim_start().to_owned())
        } else {
            match spelling {
                "$V-" => Pragma::SetVerify(false),
                "$V+" => Pragma::SetVerify(true),
                _ => Pragma::Other(spelling.to_owned()),
            }
        })
    }
}

#[derive(Debug)]
pub enum ItemKind {
    Section,
    Block {
        end: Position,
        kind: BlockKind,
        items: Vec<Item>,
    },
    SchemeBlock(Box<SchemeBlock>),
    Theorem {
        prop: Box<Proposition>,
        just: Box<Justification>,
    },
    Reservation(Vec<Reservation>),
    /// itConclusion
    Thus(Statement),
    Statement(Statement),
    /// itChoice
    Consider {
        vars: Vec<BinderGroup>,
        conds: Vec<Proposition>,
        just: Box<Justification>,
    },
    Reconsider {
        vars: Vec<ReconsiderVar>,
        ty: Box<Type>,
        just: Box<Justification>,
    },
    /// itPrivFuncDefinition
    DefFunc {
        var: Box<Variable>,
        tys: Vec<Type>,
        value: Box<Term>,
    },
    /// itPrivPredDefinition
    DefPred {
        var: Box<Variable>,
        tys: Vec<Type>,
        value: Box<Formula>,
    },
    /// itConstantDefinition
    Set(Vec<SetDecl>),
    /// itGeneralization, itLociDeclaration
    Let {
        vars: Vec<BinderGroup>,
        conds: Vec<Proposition>,
    },
    /// itExistentialAssumption
    Given {
        vars: Vec<BinderGroup>,
        conds: Vec<Proposition>,
    },
    /// itExemplification
    Take(Vec<TakeDecl>),
    PerCases {
        just: Box<Justification>,
        kind: CaseKind,
        blocks: Vec<CaseBlock>,
    },
    Assume(Assumption),
    Unfold(Vec<Reference>),
    Definition(Box<Definition>),
    DefStruct(Box<DefStruct>),
    PatternRedef(PatternRedef),
    Cluster(Box<Cluster>),
    IdentifyFunc(Box<IdentifyFunc>),
    Reduction(Box<Reduction>),
    SethoodRegistration {
        ty: Box<Type>,
        just: Box<Justification>,
    },
    Pragma(Pragma),
    /// parser internal use only
    SchemeHead(Box<SchemeHead>),
    /// parser internal use only
    CaseHead(CaseKind, Assumption),
    /// parser internal use only
    PerCasesHead(Box<Justification>),
}

impl Default for ItemKind {
    fn default() -> Self {
        Self::Section
    }
}
