#![warn(unused)]
use super::{MizParseError, PathResult, Result, XmlReader};
use crate::ast::{SchRef, *};
use crate::types::{
    ArticleId, AttrSymId, BoundId, ConstId, DefId, FuncSymId, LabelId, LeftBrkSymId, LocusId,
    ModeSymId, Position, PredSymId, PrivFuncId, PrivPredId, PropertyKind, RightBrkSymId, SchFuncId,
    SchId, SchPredId, SelSymId, StructSymId, ThmId,
};
use crate::{types, MizPath};
use enum_map::Enum;
use idx::{from_str_pos::FromStrPosParseError, vec::IdxVec};
use quick_xml::events::{BytesStart, Event};
use std::borrow::Cow;
use std::fs::File;
use std::path::PathBuf;

// Parser for WSM and MSM formats
pub struct MsmParser {
    r: XmlReader,
    pub path: PathBuf,
    buf: Vec<u8>,
    // true for MSM, false for WSM
    msm: bool,
}

impl MizPath {
    pub fn open_msx(&self) -> PathResult<MsmParser> {
        MsmParser::new(self.to_path(true, false, "msx"), true)
    }

    pub fn open_wsx(&self) -> PathResult<MsmParser> {
        MsmParser::new(self.to_path(true, false, "wsx"), false)
    }
}

#[derive(Copy, Clone, Debug, Enum)]
pub enum IdentKind {
    Free,
    Reserved,
    Bound,
    Const,
    DefConst,
    SchFunc,
    PrivFunc,
    SchPred,
    PrivPred,
}
impl Default for IdentKind {
    fn default() -> Self {
        Self::Free
    }
}

impl std::str::FromStr for IdentKind {
    type Err = String;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "Free" => Ok(Self::Free),
            "Reserved" => Ok(Self::Reserved),
            "Bound" => Ok(Self::Bound),
            "Constant" => Ok(Self::Const),
            "DefConstant" => Ok(Self::DefConst),
            "SchemataFunc" => Ok(Self::SchFunc),
            "PrivateFunc" => Ok(Self::PrivFunc),
            "SchemataPred" => Ok(Self::SchPred),
            "PrivatePred" => Ok(Self::PrivPred),
            _ => Err(s.to_string()),
        }
    }
}
impl super::FromStrPos for IdentKind {
    fn to_err(s: String, pos: usize) -> FromStrPosParseError {
        FromStrPosParseError::UnexpectedElement {
            pos,
            expected: "identifier kind",
            found: Some(s.into()),
        }
    }
}

impl TryFrom<&[u8]> for Shape {
    type Error = ();
    fn try_from(value: &[u8]) -> std::result::Result<Self, Self::Error> {
        match value {
            b"Diffuse-Statement" => Ok(Self::DiffuseStatement),
            b"Compact-Statement" => Ok(Self::CompactStatement),
            b"Iterative-Equality" => Ok(Self::IterativeEquality),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum BlockKind {
    Def(types::BlockKind),
    Now,
    Hereby,
    Proof,
    Case(CaseKind),
    Scheme,
}

#[derive(Debug)]
struct Block {
    kind: BlockKind,
    pos: (Position, Position),
    items: Vec<Item>,
}

impl ItemKind {
    fn corr_mut(&mut self) -> Option<(&mut Vec<CorrCond>, &mut Option<Correctness>)> {
        match self {
            ItemKind::Definition(it) => Some((&mut it.body.conds, &mut it.body.corr)),
            ItemKind::Cluster(it) => Some((&mut it.conds, &mut it.corr)),
            ItemKind::IdentifyFunc(it) => Some((&mut it.conds, &mut it.corr)),
            ItemKind::Reduction(it) => Some((&mut it.conds, &mut it.corr)),
            _ => None,
        }
    }

    fn property_mut(&mut self) -> Option<&mut Vec<Property>> {
        match self {
            ItemKind::Definition(it) => Some(&mut it.body.props),
            _ => None,
        }
    }
}

pub enum Shape {
    DiffuseStatement,
    CompactStatement,
    IterativeEquality,
}

#[derive(Debug)]
enum Elem {
    Block(Box<Block>),
    Inference(Position, InferenceKind, Vec<Reference>),
    Binder(Box<BinderGroup>),
    Conditions(Vec<Proposition>),
    Variable(Box<Variable>),
    Equality(Box<Variable>, Box<Term>),
    Type(Box<Type>),
    Term(Box<Term>),
    Formula(Box<Formula>),
    PredRhs(Box<PredRhs>),
    Redefine,
    Pattern(Pattern),
    TypeSpecification(Box<Type>),
    Definiens(Box<Definiens>),
    LociEquality(Box<Variable>, Box<Variable>),
    Label(Option<Box<Label>>),
    Link(Position),
    Reference(Reference),
    DefCaseTerm(DefCase<Term>),
    DefCaseFormula(DefCase<Formula>),
    Selector(Field),
    Other,
    End,
}

impl XmlReader {
    fn parse_variable_attrs(&mut self, e: &BytesStart<'_>) -> Result<Box<Variable>> {
        let (mut pos, (mut var, mut spelling)) = <(Position, _)>::default();
        for attr in e.attributes() {
            let attr = attr?;
            match attr.key.0 {
                b"line" => pos.line = self.get_attr(&attr.value)?,
                b"col" => pos.col = self.get_attr(&attr.value)?,
                b"varnr" => {
                    var = self
                        .get_attr::<u32>(&attr.value)?
                        .checked_sub(1)
                        .map(ConstId)
                }
                b"spelling" => spelling = attr.unescape_value().unwrap(),
                // omitted: origin, kind, serialnr, idnr
                _ => {}
            }
        }
        Ok(Box::new(Variable {
            pos,
            var,
            spelling: spelling.into(),
        }))
    }
}

impl MsmParser {
    fn new(path: PathBuf, msm: bool) -> PathResult<MsmParser> {
        let mut buf = vec![];
        match (|| {
            let file = File::open(&path)?;
            let mut r = XmlReader::new(file, &mut buf)?;
            assert!(matches!(r.read_event(&mut buf)?,
        Event::Start(e) if e.local_name().as_ref() == b"Text-Proper"));
            Ok(r)
        })() {
            Ok(r) => Ok(MsmParser { r, path, buf, msm }),
            Err(e) => Err((path, e)),
        }
    }

    fn parse_variable(&mut self) -> Result<Option<Box<Variable>>> {
        Ok(match self.parse_elem()? {
            Elem::Variable(v) => Some(v),
            Elem::End => None,
            _ => panic!("expected <Variable>"),
        })
    }

    fn parse_variables(&mut self) -> Result<Vec<Variable>> {
        self.r.read_start(&mut self.buf, Some("Variables"))?;
        let mut out = vec![];
        while let Some(v) = self.parse_variable()? {
            out.push(*v)
        }
        Ok(out)
    }

    fn parse_substitution(&mut self) -> Result<Vec<VarKind>> {
        let mut vars = vec![];
        while let Ok(e) = self.r.try_read_start(&mut self.buf, Some("Substitution"))? {
            let (mut y1, mut y2) = <_>::default();
            for attr in e.attributes() {
                let attr = attr?;
                match attr.key.0 {
                    b"y1" => y1 = IdentKind::from_usize(self.r.get_attr::<usize>(&attr.value)?),
                    b"y2" => y2 = self.r.get_attr::<u32>(&attr.value)? - 1,
                    _ => {}
                }
            }
            self.r.end_tag(&mut self.buf)?;
            vars.push(match y1 {
                IdentKind::Bound => VarKind::Bound(BoundId(y2)),
                IdentKind::Const | IdentKind::DefConst => VarKind::Const(ConstId(y2)),
                _ => unreachable!(),
            })
        }
        Ok(vars)
    }

    fn parse_formula(&mut self) -> Result<Box<Formula>> {
        Ok(match self.parse_elem()? {
            Elem::Formula(f) => f,
            _ => panic!("expected formula"),
        })
    }

    fn parse_term(&mut self) -> Result<Option<Box<Term>>> {
        Ok(match self.parse_elem()? {
            Elem::Term(t) => Some(t),
            Elem::End => None,
            _ => panic!("expected term"),
        })
    }

    fn parse_terms(&mut self) -> Result<Vec<Term>> {
        let mut terms = vec![];
        while let Some(t) = self.parse_term()? {
            terms.push(*t)
        }
        Ok(terms)
    }

    fn parse_type(&mut self) -> Result<Option<Box<Type>>> {
        Ok(match self.parse_elem()? {
            Elem::Type(ty) => Some(ty),
            Elem::End => None,
            _ => panic!("expected type"),
        })
    }

    fn parse_types(&mut self) -> Result<Vec<Type>> {
        self.r.read_start(&mut self.buf, Some("Type-List"))?;
        let mut tys = vec![];
        while let Some(t) = self.parse_type()? {
            tys.push(*t)
        }
        Ok(tys)
    }

    fn parse_proposition(&mut self) -> Result<Option<Box<Proposition>>> {
        let Ok(_) = self.r.try_read_start(&mut self.buf, Some("Proposition"))? else {
            return Ok(None);
        };
        let (label, f) = match self.parse_elem()? {
            Elem::Label(lab) => (lab, self.parse_formula()?),
            Elem::Formula(f) => (None, f),
            _ => panic!("expected formula"),
        };
        self.r.end_tag(&mut self.buf)?;
        Ok(Some(Box::new(Proposition { label, f: *f })))
    }

    fn parse_stmt(&mut self, shape: Shape) -> Result<Statement> {
        Ok(match shape {
            Shape::DiffuseStatement => {
                let (label, bl) = match self.parse_elem()? {
                    Elem::Label(lab) => (lab, self.parse_block()?.unwrap()),
                    Elem::Block(bl) => (None, bl),
                    _ => panic!("expected block"),
                };
                self.r.end_tag(&mut self.buf)?;
                assert!(matches!(bl.kind, BlockKind::Now | BlockKind::Hereby));
                Statement::Now {
                    end: bl.pos.1,
                    label,
                    items: bl.items,
                }
            }
            Shape::CompactStatement => {
                let prop = self.parse_proposition()?.unwrap();
                let just = self.parse_justification()?;
                self.r.end_tag(&mut self.buf)?;
                Statement::Proposition { prop, just }
            }
            Shape::IterativeEquality => {
                let prop = self.parse_proposition()?.unwrap();
                let just = self.parse_justification()?;
                let mut steps = vec![];
                while let Ok(e) = self
                    .r
                    .try_read_start(&mut self.buf, Some("Iterative-Step"))?
                {
                    steps.push(IterStep {
                        pos: self.r.get_pos(&e)?,
                        rhs: *self.parse_term()?.unwrap(),
                        just: *self.parse_justification()?,
                    });
                    self.r.end_tag(&mut self.buf)?;
                }
                Statement::IterEquality { prop, just, steps }
            }
        })
    }

    fn parse_justification(&mut self) -> Result<Box<Justification>> {
        Ok(Box::new(match self.parse_elem()? {
            Elem::Block(bl) if matches!(bl.kind, BlockKind::Proof) => Justification::Block {
                pos: bl.pos,
                items: bl.items,
            },
            Elem::Inference(pos, kind, refs) => Justification::Inference { pos, kind, refs },
            _ => panic!("expected justification"),
        }))
    }

    fn parse_block(&mut self) -> Result<Option<Box<Block>>> {
        Ok(match self.parse_elem()? {
            Elem::Block(bl) => Some(bl),
            Elem::End => None,
            _ => panic!("expected <Block>"),
        })
    }

    fn parse_binder(&mut self) -> Result<Box<BinderGroup>> {
        Ok(match self.parse_elem()? {
            Elem::Binder(var) => var,
            _ => panic!("expected binder group"),
        })
    }

    fn parse_pattern(&mut self) -> Result<Pattern> {
        Ok(match self.parse_elem()? {
            Elem::Pattern(pat) => pat,
            _ => panic!("expected pattern"),
        })
    }

    fn parse_attr(&mut self) -> Result<Option<Attr>> {
        let Ok(e) = self.r.try_read_start(&mut self.buf, None)? else {
            return Ok(None);
        };
        Ok(Some(match e.local_name().as_ref() {
            b"Adjective" => {
                let (mut pos, (mut sym, mut spelling)) = <(Position, _)>::default();
                for attr in e.attributes() {
                    let attr = attr?;
                    match attr.key.0 {
                        b"line" => pos.line = self.r.get_attr(&attr.value)?,
                        b"col" => pos.col = self.r.get_attr(&attr.value)?,
                        b"nr" => sym = AttrSymId(self.r.get_attr::<u32>(&attr.value)? - 1),
                        b"spelling" => spelling = attr.unescape_value().unwrap().to_string(),
                        _ => {}
                    }
                }
                let args = self.parse_terms()?;
                Attr::Attr {
                    pos,
                    sym: (sym, spelling),
                    args,
                }
            }
            b"NegatedAdjective" => {
                let attr = Attr::Non {
                    pos: self.r.get_pos(&e)?,
                    attr: Box::new(self.parse_attr()?.unwrap()),
                };
                self.r.end_tag(&mut self.buf)?;
                attr
            }
            _ => panic!("expected an adjective"),
        }))
    }

    fn parse_attrs(&mut self) -> Result<Vec<Attr>> {
        self.r
            .read_start(&mut self.buf, Some("Adjective-Cluster"))?;
        let mut attrs = vec![];
        while let Some(attr) = self.parse_attr()? {
            attrs.push(attr)
        }
        Ok(attrs)
    }

    fn parse_definiens(&mut self) -> Result<Option<Box<Definiens>>> {
        Ok(match self.parse_elem()? {
            Elem::Definiens(d) => Some(d),
            Elem::End => None,
            _ => panic!("expected <Definiens>"),
        })
    }

    fn parse_locus(&mut self) -> Result<Option<Box<Variable>>> {
        let Ok(e) = self.r.try_read_start(&mut self.buf, Some("Locus"))? else {
            return Ok(None);
        };
        let v = self.r.parse_variable_attrs(&e)?;
        self.r.end_tag(&mut self.buf)?;
        Ok(Some(v))
    }

    fn parse_loci(&mut self, out: &mut Vec<Variable>) -> Result<()> {
        self.r.read_start(&mut self.buf, Some("Loci"))?;
        while let Some(v) = self.parse_locus()? {
            out.push(*v)
        }
        Ok(())
    }

    fn parse_right_pattern(&mut self) -> Result<(RightBrkSymId, String)> {
        let (mut rsym, mut rsp) = Default::default();
        let e = self
            .r
            .read_start(&mut self.buf, Some("Right-Circumflex-Symbol"))?;
        for attr in e.attributes() {
            let attr = attr?;
            match attr.key.0 {
                b"nr" => rsym = RightBrkSymId(self.r.get_attr::<u32>(&attr.value)? - 1),
                b"spelling" => rsp = attr.unescape_value().unwrap().to_string(),
                _ => {}
            }
        }
        self.r.end_tag(&mut self.buf)?;
        Ok((rsym, rsp))
    }

    fn parse_assumption(&mut self) -> Result<Assumption> {
        let e = self.r.read_start(&mut self.buf, None)?;
        let pos = self.r.get_pos(&e)?;
        let out = match e.local_name().as_ref() {
            b"Single-Assumption" => Assumption::Single {
                pos,
                prop: self.parse_proposition()?.unwrap(),
            },
            b"Collective-Assumption" => {
                let Elem::Conditions(conds) = self.parse_elem()? else {
                    panic!("expected <Conditions>")
                };
                Assumption::Collective { pos, conds }
            }
            _ => panic!("expected assumption"),
        };
        self.r.end_tag(&mut self.buf)?;
        Ok(out)
    }

    fn parse_references(&mut self) -> Result<Vec<Reference>> {
        let mut refs = vec![];
        loop {
            match self.parse_elem()? {
                Elem::Reference(r) => refs.push(r),
                Elem::End => break,
                _ => panic!("unexpected element"),
            }
        }
        Ok(refs)
    }

    pub fn push_parse_item(&mut self, items: &mut Vec<Item>) -> Result<bool> {
        let Ok(e) = self.r.try_read_start(&mut self.buf, Some("Item"))? else {
            return Ok(false);
        };
        let (mut pos, (mut kind, mut property, mut shape, mut spelling, mut condition)) =
            <(Position, _)>::default();
        for attr in e.attributes() {
            let attr = attr?;
            match attr.key.0 {
                b"line" => pos.line = self.r.get_attr(&attr.value)?,
                b"col" => pos.col = self.r.get_attr(&attr.value)?,
                b"kind" => kind = attr.value,
                b"property" => property = Some((*attr.value).try_into().unwrap()),
                b"shape" => shape = attr.value,
                b"spelling" => spelling = Some(attr.unescape_value().unwrap()),
                b"condition" => condition = attr.value,
                // Some((*attr.value).try_into().unwrap()),
                _ => {}
            }
        }
        let mut end_tag = false;
        let kind = match &*kind {
            b"Definition-Item" => {
                let Block {
                    pos,
                    kind: BlockKind::Def(kind),
                    items,
                } = *self.parse_block()?.unwrap()
                else {
                    panic!("expected a definition block")
                };
                ItemKind::Block {
                    end: pos.1,
                    kind,
                    items,
                }
            }
            b"Scheme-Block-Item" => {
                let Block {
                    pos,
                    kind: BlockKind::Scheme,
                    mut items,
                } = *self.parse_block()?.unwrap()
                else {
                    panic!("expected a scheme block")
                };
                let ItemKind::SchemeHead(head) = items.remove(0).kind else {
                    panic!()
                };
                ItemKind::SchemeBlock(Box::new(SchemeBlock {
                    end: pos.1,
                    head: *head,
                    items,
                }))
            }
            b"Scheme-Head" => {
                let e = self.r.read_start(&mut self.buf, Some("Scheme"))?;
                let (mut spelling, mut nr) = <_>::default();
                for attr in e.attributes() {
                    let attr = attr?;
                    match attr.key.0 {
                        b"nr" => nr = Some(SchId(self.r.get_attr::<u32>(&attr.value)? - 1)),
                        b"spelling" => spelling = attr.unescape_value().unwrap().to_string(),
                        _ => {}
                    }
                }
                self.r.end_tag(&mut self.buf)?;
                self.r
                    .read_start(&mut self.buf, Some("Schematic-Variables"))?;
                let mut groups = vec![];
                while let Event::Start(e) = self.r.read_event(&mut self.buf)? {
                    let pos = self.r.get_pos(&e)?;
                    match e.local_name().as_ref() {
                        b"Predicate-Segment" => groups.push(SchemeBinderGroup::Pred {
                            pos,
                            vars: self.parse_variables()?,
                            tys: self.parse_types()?,
                        }),
                        b"Functor-Segment" => groups.push(SchemeBinderGroup::Func {
                            pos,
                            vars: self.parse_variables()?,
                            tys: self.parse_types()?,
                            ret: match self.parse_elem()? {
                                Elem::TypeSpecification(ty) => *ty,
                                _ => panic!("expected <Type-Specification>"),
                            },
                        }),
                        _ => panic!("unexpected scheme variable group"),
                    }
                    self.r.end_tag(&mut self.buf)?;
                }
                let concl = *self.parse_formula()?;
                let mut prems = vec![];
                if self
                    .r
                    .try_read_start(&mut self.buf, Some("Provisional-Formulas"))?
                    .is_ok()
                {
                    while let Some(p) = self.parse_proposition()? {
                        prems.push(*p)
                    }
                } else {
                    end_tag = true;
                }
                let sym = if spelling.is_empty() {
                    None
                } else {
                    Some(spelling.into())
                };
                ItemKind::SchemeHead(Box::new(SchemeHead {
                    sym,
                    nr,
                    groups,
                    concl,
                    prems,
                }))
            }
            b"Theorem-Item" => ItemKind::Theorem {
                prop: self.parse_proposition()?.unwrap(),
                just: self.parse_justification()?,
            },
            b"Reservation" => {
                let vars = self.parse_variables()?;
                let tys = if self.msm {
                    Some(self.parse_types()?)
                } else {
                    None
                };
                let ty = self.parse_type()?.unwrap();
                let fvars = if self.msm {
                    let mut out = IdxVec::default();
                    while let Ok(e) = self.r.try_read_start(&mut self.buf, Some("SetMember"))? {
                        let id = self
                            .r
                            .get_attr::<u32>(&e.try_get_attribute(b"x").unwrap().unwrap().value)?;
                        out.push(ReservedId(id - 1));
                        self.r.end_tag(&mut self.buf)?;
                    }
                    end_tag = true;
                    Some(out)
                } else {
                    None
                };
                ItemKind::Reservation(vec![Reservation {
                    vars,
                    tys,
                    ty,
                    fvars,
                }])
            }
            b"Section-Pragma" => ItemKind::Section,
            b"Choice-Statement" => {
                let mut vars = vec![];
                let conds = loop {
                    match self.parse_elem()? {
                        Elem::Binder(var) => vars.push(*var),
                        Elem::Conditions(conds) => break conds,
                        _ => panic!("expected <Conditions>"),
                    }
                };
                ItemKind::Consider {
                    vars,
                    conds,
                    just: self.parse_justification()?,
                }
            }
            b"Type-Changing-Statement" => {
                let mut vars = vec![];
                let ty = loop {
                    match self.parse_elem()? {
                        Elem::Variable(var) => vars.push(ReconsiderVar::Var(*var)),
                        Elem::Equality(var, tm) => {
                            vars.push(ReconsiderVar::Equality { var: *var, tm: *tm })
                        }
                        Elem::Type(ty) => break ty,
                        _ => panic!("expected variable, equality, or type"),
                    }
                };
                ItemKind::Reconsider {
                    vars,
                    ty,
                    just: self.parse_justification()?,
                }
            }
            b"Private-Functor-Definition" => ItemKind::DefFunc {
                var: self.parse_variable()?.unwrap(),
                tys: self.parse_types()?,
                value: self.parse_term()?.unwrap(),
            },
            b"Private-Predicate-Definition" => ItemKind::DefPred {
                var: self.parse_variable()?.unwrap(),
                tys: self.parse_types()?,
                value: self.parse_formula()?,
            },
            b"Constant-Definition" => ItemKind::Set(vec![SetDecl {
                var: self.parse_variable()?.unwrap(),
                value: self.parse_term()?.unwrap(),
            }]),
            b"Generalization" | b"Loci-Declaration" => ItemKind::Let {
                vars: vec![*self.parse_binder()?],
                conds: vec![],
            },
            b"Existential-Assumption" => {
                let mut vars = vec![];
                let conds = loop {
                    match self.parse_elem()? {
                        Elem::Binder(var) => vars.push(*var),
                        Elem::Conditions(conds) => break conds,
                        _ => panic!("expected <Conditions>"),
                    }
                };
                ItemKind::Given { vars, conds }
            }
            b"Exemplification" => {
                let (mut var, mut term) = (None, None);
                loop {
                    match self.parse_elem()? {
                        Elem::Variable(v) => var = Some(v),
                        Elem::Term(t) => term = Some(t),
                        Elem::End => break,
                        _ => panic!("unexpected element"),
                    }
                }
                end_tag = true;
                let term = term.unwrap_or_else(|| Box::new(var.as_deref().unwrap().to_term()));
                ItemKind::Take(vec![TakeDecl { var, term }])
            }
            b"Unfolding" => {
                self.r.read_start(&mut self.buf, Some("Unfolding"))?;
                ItemKind::Unfold(self.parse_references()?)
            }
            b"Per-Cases" => ItemKind::PerCasesHead(self.parse_justification()?),
            b"Regular-Statement" => {
                let shape = (*shape).try_into().unwrap();
                end_tag = true;
                ItemKind::Statement(self.parse_stmt(shape)?)
            }
            b"Conclusion" => {
                let shape = (*shape).try_into().unwrap();
                end_tag = true;
                ItemKind::Thus(self.parse_stmt(shape)?)
            }
            b"Case-Block" => {
                let mut bl = self.parse_block()?.unwrap();
                let BlockKind::Case(kind) = bl.kind else {
                    panic!("expected case or suppose block")
                };
                let hyp = Box::new(match bl.items.remove(0).kind {
                    ItemKind::CaseHead(k2, hyp) if k2 == kind => hyp,
                    _ => panic!("missing case/suppose head"),
                });
                let bl = CaseBlock {
                    end: bl.pos.1,
                    hyp,
                    items: bl.items,
                };
                let it = &mut items.last_mut().unwrap().kind;
                match it {
                    ItemKind::PerCases {
                        kind: k2, blocks, ..
                    } if *k2 == kind => blocks.push(bl),
                    _ => {
                        let ItemKind::PerCasesHead(just) = std::mem::take(it) else {
                            panic!("unexpected case block")
                        };
                        *it = ItemKind::PerCases {
                            just,
                            kind,
                            blocks: vec![bl],
                        };
                    }
                }
                self.r.end_tag(&mut self.buf)?;
                return Ok(true);
            }
            b"Case-Head" => ItemKind::CaseHead(CaseKind::Case, self.parse_assumption()?),
            b"Suppose-Head" => ItemKind::CaseHead(CaseKind::Suppose, self.parse_assumption()?),
            b"Assumption" => ItemKind::Assume(self.parse_assumption()?),
            b"Correctness-Condition" => {
                let kind = (*condition).try_into().unwrap();
                let corr = items
                    .last_mut()
                    .and_then(|it| it.kind.corr_mut())
                    .unwrap()
                    .0;
                corr.push(CorrCond {
                    pos,
                    kind,
                    just: *self.parse_justification()?,
                });
                self.r.end_tag(&mut self.buf)?;
                return Ok(true);
            }
            b"Correctness" => {
                self.r
                    .read_start(&mut self.buf, Some("CorrectnessConditions"))?;
                // let mut conds = vec![];
                while let Ok(_e) = self.r.try_read_start(&mut self.buf, Some("Correctness")) {
                    // conds.push((*e.try_get_attribute(b"condition").unwrap().unwrap().value).try_into().unwrap());
                    self.r.end_tag(&mut self.buf)?;
                }
                let corr = items
                    .last_mut()
                    .and_then(|it| it.kind.corr_mut())
                    .unwrap()
                    .1;
                assert!(corr.is_none());
                *corr = Some(Correctness {
                    pos,
                    just: *self.parse_justification()?,
                });
                self.r.end_tag(&mut self.buf)?;
                return Ok(true);
            }
            b"Property" => {
                let props = items
                    .last_mut()
                    .and_then(|it| it.kind.property_mut())
                    .unwrap();
                props.push(Property {
                    kind: property.unwrap(),
                    just: self.parse_justification()?,
                });
                self.r.end_tag(&mut self.buf)?;
                return Ok(true);
            }
            b"Mode-Definition" => {
                let (redef, pat) = match self.parse_elem()? {
                    Elem::Redefine => (true, self.parse_pattern()?),
                    Elem::Pattern(pat) => (false, pat),
                    _ => panic!("expected pattern"),
                };
                let Pattern::Mode(pat) = pat else {
                    panic!("expected a mode pattern")
                };
                let e = self.r.read_start(&mut self.buf, None)?;
                let kind = match e.local_name().as_ref() {
                    b"Expandable-Mode" => DefModeKind::Expandable {
                        expansion: self.parse_type()?.unwrap(),
                    },
                    b"Standard-Mode" => {
                        let (spec, def) = match self.parse_elem()? {
                            Elem::TypeSpecification(ty) => (Some(ty), self.parse_definiens()?),
                            Elem::Definiens(d) => (None, Some(d)),
                            Elem::End => (None, None),
                            _ => panic!("expected <Definiens>"),
                        };
                        end_tag = def.is_none();
                        DefModeKind::Standard { spec, def }
                    }
                    _ => panic!("unknown def mode kind"),
                };
                self.r.end_tag(&mut self.buf)?;
                let kind = DefinitionKind::Mode { pat, kind };
                let body = DefinitionBody {
                    redef,
                    conds: vec![],
                    corr: None,
                    props: vec![],
                };
                ItemKind::Definition(Box::new(Definition { kind, body }))
            }
            b"Attribute-Definition" => {
                let (redef, pat) = match self.parse_elem()? {
                    Elem::Redefine => (true, self.parse_pattern()?),
                    Elem::Pattern(pat) => (false, pat),
                    _ => panic!("expected pattern"),
                };
                let Pattern::Attr(pat) = pat else {
                    panic!("expected an attribute pattern")
                };
                let def = self.parse_definiens()?;
                end_tag = def.is_none();
                let kind = DefinitionKind::Attr { pat, def };
                let body = DefinitionBody {
                    redef,
                    conds: vec![],
                    corr: None,
                    props: vec![],
                };
                ItemKind::Definition(Box::new(Definition { kind, body }))
            }
            b"Predicate-Definition" => {
                let (redef, pat) = match self.parse_elem()? {
                    Elem::Redefine => (true, self.parse_pattern()?),
                    Elem::Pattern(pat) => (false, pat),
                    _ => panic!("expected pattern"),
                };
                let Pattern::Pred(pat) = pat else {
                    panic!("expected a predicate pattern")
                };
                let def = self.parse_definiens()?;
                end_tag = def.is_none();
                let kind = DefinitionKind::Pred { pat, def };
                let body = DefinitionBody {
                    redef,
                    conds: vec![],
                    corr: None,
                    props: vec![],
                };
                ItemKind::Definition(Box::new(Definition { kind, body }))
            }
            b"Functor-Definition" => {
                let shape = match &*shape {
                    b"No-Definiens" => None,
                    b"Means" => Some(false),
                    b"Equals" => Some(true),
                    _ => panic!("unexpected functor shape attribute"),
                };
                let (redef, pat) = match self.parse_elem()? {
                    Elem::Redefine => (true, self.parse_pattern()?),
                    Elem::Pattern(pat) => (false, pat),
                    _ => panic!("expected pattern"),
                };
                let Pattern::Func(pat) = pat else {
                    panic!("expected a functor pattern")
                };
                let (spec, def) = match self.parse_elem()? {
                    Elem::TypeSpecification(ty) => (Some(ty), self.parse_definiens()?),
                    Elem::Definiens(d) => (None, Some(d)),
                    Elem::End => (None, None),
                    _ => panic!("expected <Definiens>"),
                };
                end_tag = def.is_none();
                let shape2 = def.as_ref().map(|d| matches!(d.kind, DefValue::Term(_)));
                assert_eq!(shape, shape2, "unexpected shape");
                let kind = DefinitionKind::Func { pat, spec, def };
                let body = DefinitionBody {
                    redef,
                    conds: vec![],
                    corr: None,
                    props: vec![],
                };
                ItemKind::Definition(Box::new(Definition { kind, body }))
            }
            b"Structure-Definition" => {
                self.r.read_start(&mut self.buf, Some("Ancestors"))?;
                let mut parents = vec![];
                while let Some(t) = self.parse_type()? {
                    parents.push(*t);
                }
                let e = self
                    .r
                    .read_start(&mut self.buf, Some("Structure-Pattern"))?;
                let (mut sym, mut spelling) = <_>::default();
                for attr in e.attributes() {
                    let attr = attr?;
                    match attr.key.0 {
                        b"nr" => sym = StructSymId(self.r.get_attr::<u32>(&attr.value)? - 1),
                        b"spelling" => spelling = attr.unescape_value().unwrap().to_string(),
                        _ => {}
                    }
                }
                let mut args = vec![];
                self.parse_loci(&mut args)?;
                let mut fields = vec![];
                while let Ok(e) = self
                    .r
                    .try_read_start(&mut self.buf, Some("Field-Segment"))?
                {
                    let pos = self.r.get_pos(&e)?;
                    let mut vars = vec![];
                    let ty = loop {
                        match self.parse_elem()? {
                            Elem::Selector(field) => vars.push(field),
                            Elem::Type(ty) => break ty,
                            _ => panic!("expected type"),
                        }
                    };
                    self.r.end_tag(&mut self.buf)?;
                    fields.push(FieldGroup { pos, vars, ty: *ty })
                }
                let pat = PatternStruct {
                    sym: (sym, spelling),
                    args,
                };
                ItemKind::DefStruct(Box::new(DefStruct {
                    parents,
                    fields,
                    pat,
                }))
            }
            b"Pred-Synonym" => {
                let Pattern::Pred(orig) = self.parse_pattern()? else {
                    panic!("expected pred pattern")
                };
                let Pattern::Pred(new) = self.parse_pattern()? else {
                    panic!("expected pred pattern")
                };
                ItemKind::PatternRedef(PatternRedef::Pred {
                    new,
                    orig,
                    pos: true,
                })
            }
            b"Pred-Antonym" => {
                let Pattern::Pred(orig) = self.parse_pattern()? else {
                    panic!("expected pred pattern")
                };
                let Pattern::Pred(new) = self.parse_pattern()? else {
                    panic!("expected pred pattern")
                };
                ItemKind::PatternRedef(PatternRedef::Pred {
                    new,
                    orig,
                    pos: false,
                })
            }
            b"Func-Synonym" => {
                let Pattern::Func(orig) = self.parse_pattern()? else {
                    panic!("expected func pattern")
                };
                let Pattern::Func(new) = self.parse_pattern()? else {
                    panic!("expected func pattern")
                };
                ItemKind::PatternRedef(PatternRedef::Func { new, orig })
            }
            b"Mode-Synonym" => {
                let Pattern::Mode(orig) = self.parse_pattern()? else {
                    panic!("expected mode pattern")
                };
                let Pattern::Mode(new) = self.parse_pattern()? else {
                    panic!("expected mode pattern")
                };
                ItemKind::PatternRedef(PatternRedef::Mode { new, orig })
            }
            b"Attr-Synonym" => {
                let Pattern::Attr(orig) = self.parse_pattern()? else {
                    panic!("expected attr pattern")
                };
                let Pattern::Attr(new) = self.parse_pattern()? else {
                    panic!("expected attr pattern")
                };
                ItemKind::PatternRedef(PatternRedef::Attr {
                    new,
                    orig,
                    pos: true,
                })
            }
            b"Attr-Antonym" => {
                let Pattern::Attr(orig) = self.parse_pattern()? else {
                    panic!("expected attr pattern")
                };
                let Pattern::Attr(new) = self.parse_pattern()? else {
                    panic!("expected attr pattern")
                };
                ItemKind::PatternRedef(PatternRedef::Attr {
                    new,
                    orig,
                    pos: false,
                })
            }
            b"Cluster" => {
                let e = self.r.read_start(&mut self.buf, None)?;
                let kind = match e.local_name().as_ref() {
                    b"Existential-Registration" => ClusterDeclKind::Exist {
                        concl: self.parse_attrs()?,
                        ty: self.parse_type()?.unwrap(),
                    },
                    b"Conditional-Registration" => ClusterDeclKind::Cond {
                        antecedent: self.parse_attrs()?,
                        concl: self.parse_attrs()?,
                        ty: self.parse_type()?.unwrap(),
                    },
                    b"Functorial-Registration" => {
                        let term = self.parse_term()?.unwrap();
                        let concl = self.parse_attrs()?;
                        let ty = self.parse_type()?;
                        end_tag = ty.is_none();
                        ClusterDeclKind::Func { term, concl, ty }
                    }
                    _ => panic!("unexpected cluster registration kind"),
                };
                self.r.end_tag(&mut self.buf)?;
                ItemKind::Cluster(Box::new(Cluster {
                    kind,
                    conds: vec![],
                    corr: None,
                }))
            }
            b"Identify" => {
                let Pattern::Func(rhs) = self.parse_pattern()? else {
                    panic!("expected a functor pattern")
                };
                let Pattern::Func(lhs) = self.parse_pattern()? else {
                    panic!("expected a functor pattern")
                };
                let mut eqs = vec![];
                loop {
                    match self.parse_elem()? {
                        Elem::LociEquality(x1, x2) => eqs.push((*x1, *x2)),
                        Elem::End => break,
                        _ => panic!("expected <LociEquality>"),
                    }
                }
                end_tag = true;
                let id = IdentifyFunc {
                    lhs,
                    rhs,
                    eqs,
                    conds: vec![],
                    corr: None,
                };
                ItemKind::IdentifyFunc(Box::new(id))
            }
            b"Property-Registration" => {
                assert!(matches!(property.unwrap(), PropertyKind::Sethood));
                ItemKind::SethoodRegistration {
                    ty: self.parse_type()?.unwrap(),
                    just: self.parse_justification()?,
                }
            }
            b"Reduction" => {
                let (t1, t2) = (self.parse_term()?.unwrap(), self.parse_term()?.unwrap());
                let (from, to) = if self.msm { (t2, t1) } else { (t1, t2) }; // Yes, this is silly
                ItemKind::Reduction(Box::new(Reduction {
                    from,
                    to,
                    conds: vec![],
                    corr: None,
                }))
            }
            b"Pragma" => ItemKind::Pragma(spelling.unwrap().parse().unwrap()),
            _ => panic!("unrecognized item kind"),
        };
        if !end_tag {
            self.r.end_tag(&mut self.buf)?;
        }
        items.push(Item { pos, kind });
        Ok(true)
    }

    pub fn parse_items(&mut self) -> Result<Vec<Item>> {
        let mut items = vec![];
        while self.push_parse_item(&mut items)? {}
        Ok(items)
    }

    fn parse_elem(&mut self) -> Result<Elem> {
        // Note: this is only really an issue in debug mode, but this function
        // is both recursive and has a pretty big stack frame, so it is liable to run out
        // of stack space on some of the deep `p1 & p2 & ... & pn` expressions in some MML articles.
        // We use the stacker library here to allocate space on the heap to avoid stack overflow.
        stacker::maybe_grow(0x20000, 0x100000, || {
            Ok(if let Event::Start(e) = self.r.read_event(&mut self.buf)? {
                let elem = match e.local_name().as_ref() {
                    b"Block" => {
                        let (mut start, mut end, mut kind) =
                            <(Position, Position, Cow<'_, [u8]>)>::default();
                        for attr in e.attributes() {
                            let attr = attr?;
                            match attr.key.0 {
                                b"line" => start.line = self.r.get_attr(&attr.value)?,
                                b"col" => start.col = self.r.get_attr(&attr.value)?,
                                b"posline" => end.line = self.r.get_attr(&attr.value)?,
                                b"poscol" => end.col = self.r.get_attr(&attr.value)?,
                                b"kind" => kind = attr.value,
                                _ => {}
                            }
                        }
                        return Ok(Elem::Block(Box::new(Block {
                            kind: match &*kind {
                                b"Now-Reasoning" => BlockKind::Now,
                                b"Hereby-Reasoning" => BlockKind::Hereby,
                                b"Proof" => BlockKind::Proof,
                                b"Definitional-Block" => {
                                    BlockKind::Def(types::BlockKind::Definition)
                                }
                                b"Notation-Block" => BlockKind::Def(types::BlockKind::Notation),
                                b"Registration-Block" => {
                                    BlockKind::Def(types::BlockKind::Registration)
                                }
                                b"Case" => BlockKind::Case(CaseKind::Case),
                                b"Suppose" => BlockKind::Case(CaseKind::Suppose),
                                b"Scheme-Block" => BlockKind::Scheme,
                                kind => panic!(
                                    "unrecognized block kind: {}",
                                    std::str::from_utf8(kind).unwrap()
                                ),
                            },
                            pos: (start, end),
                            items: self.parse_items()?,
                        })));
                    }
                    b"Straightforward-Justification" => {
                        let pos = self.r.get_pos(&e)?;
                        let (mut link, mut refs) = (None, vec![]);
                        loop {
                            match self.parse_elem()? {
                                Elem::Link(pos) => {
                                    if self.msm {
                                        // MSM mode adds a garbage extra local ref for linked inferences.
                                        // We can't use it anyway because they aren't labeled, so we just discard it
                                        match self.parse_elem()? {
                                            Elem::Reference(r)
                                                if matches!(r.kind, ReferenceKind::Priv(_)) => {}
                                            _ => panic!("expected local reference for link"),
                                        }
                                    }
                                    link = Some(pos)
                                }
                                Elem::Reference(r) => refs.push(r),
                                Elem::End => break,
                                _ => panic!("unexpected element"),
                            }
                        }
                        return Ok(Elem::Inference(pos, InferenceKind::By { link }, refs));
                    }
                    b"Scheme-Justification" => {
                        let (mut pos, (mut art, mut sch, mut id, mut spelling)) =
                            <(Position, _)>::default();
                        for attr in e.attributes() {
                            let attr = attr?;
                            match attr.key.0 {
                                b"line" => pos.line = self.r.get_attr(&attr.value)?,
                                b"col" => pos.col = self.r.get_attr(&attr.value)?,
                                b"nr" => art = self.r.get_attr(&attr.value)?,
                                b"idnr" => id = self.r.get_attr::<u32>(&attr.value)?,
                                b"schnr" => sch = self.r.get_attr::<u32>(&attr.value)?,
                                b"spelling" => {
                                    spelling = attr.unescape_value().unwrap().to_string()
                                }
                                _ => {}
                            }
                        }
                        let refs = self.parse_references()?;
                        let sch = if art == ArticleId::SELF {
                            match sch.checked_sub(1) {
                                Some(sch) => SchRef::Resolved(art, SchId(sch)),
                                None => SchRef::UnresolvedPriv(spelling),
                            }
                        } else {
                            SchRef::Resolved(art, SchId(id - 1))
                        };
                        return Ok(Elem::Inference(pos, InferenceKind::From { sch }, refs));
                    }
                    b"Implicitly-Qualified-Segment" => {
                        let nr = match e.try_get_attribute(b"nr").unwrap() {
                            Some(attr) => Some(self.r.get_attr::<u32>(&attr.value)?),
                            None => None,
                        };
                        let var = self.parse_variable()?.unwrap();
                        let pos = var.pos;
                        let ty = if self.msm {
                            let group = ResGroupId(nr.unwrap() - 1);
                            Some(Box::new(Type::Reservation {
                                pos,
                                group,
                                subst: self.parse_substitution()?,
                            }))
                        } else {
                            self.r.end_tag(&mut self.buf)?;
                            None
                        };
                        return Ok(Elem::Binder(Box::new(BinderGroup {
                            vars: vec![*var],
                            ty,
                        })));
                    }
                    b"Explicitly-Qualified-Segment" => Elem::Binder(Box::new(BinderGroup {
                        vars: self.parse_variables()?,
                        ty: Some(self.parse_type()?.unwrap()),
                    })),
                    b"Conditions" => {
                        let mut conds = vec![];
                        while let Some(p) = self.parse_proposition()? {
                            conds.push(*p);
                        }
                        return Ok(Elem::Conditions(conds));
                    }
                    b"Variable" => Elem::Variable(self.r.parse_variable_attrs(&e)?),
                    b"Equality" => {
                        Elem::Equality(self.parse_variable()?.unwrap(), self.parse_term()?.unwrap())
                    }
                    b"Redefine" => Elem::Redefine,
                    b"Type-Specification" => Elem::TypeSpecification(self.parse_type()?.unwrap()),
                    b"Definiens" => {
                        let ((mut pos, mut kind), mut is_term) =
                            (<(Position, _)>::default(), false);
                        for attr in e.attributes() {
                            let attr = attr?;
                            match attr.key.0 {
                                b"line" => pos.line = self.r.get_attr(&attr.value)?,
                                b"col" => pos.col = self.r.get_attr(&attr.value)?,
                                b"kind" => kind = attr.value,
                                b"shape" => {
                                    is_term = match &*attr.value {
                                        b"Term-Expression" => true,
                                        b"Formula-Expression" => false,
                                        _ => panic!("unexpected definiens shape"),
                                    }
                                }
                                _ => {}
                            }
                        }
                        let (label, end_tag, kind) = match &*kind {
                            b"Simple-Definiens" if is_term => {
                                let (lab, t) = match self.parse_elem()? {
                                    Elem::Label(lab) => (lab, self.parse_term()?.unwrap()),
                                    Elem::Term(t) => (None, t),
                                    _ => panic!("expected term"),
                                };
                                (
                                    lab,
                                    false,
                                    DefValue::Term(DefBody {
                                        cases: vec![],
                                        otherwise: Some(t),
                                    }),
                                )
                            }
                            b"Simple-Definiens" => {
                                let (lab, f) = match self.parse_elem()? {
                                    Elem::Label(lab) => (lab, self.parse_formula()?),
                                    Elem::Formula(f) => (None, f),
                                    _ => panic!("expected formula"),
                                };
                                (
                                    lab,
                                    false,
                                    DefValue::Formula(DefBody {
                                        cases: vec![],
                                        otherwise: Some(f),
                                    }),
                                )
                            }
                            b"Conditional-Definiens" if is_term => {
                                let (mut lab, mut cases) = (None, vec![]);
                                let otherwise = loop {
                                    match self.parse_elem()? {
                                        Elem::Label(l) => lab = l,
                                        Elem::DefCaseTerm(case) => cases.push(case),
                                        Elem::End => break None,
                                        Elem::Term(t) => break Some(t),
                                        _ => panic!("unexpected element"),
                                    }
                                };
                                (
                                    lab,
                                    otherwise.is_none(),
                                    DefValue::Term(DefBody { cases, otherwise }),
                                )
                            }
                            b"Conditional-Definiens" => {
                                let (mut lab, mut cases) = (None, vec![]);
                                let otherwise = loop {
                                    match self.parse_elem()? {
                                        Elem::Label(l) => lab = l,
                                        Elem::DefCaseFormula(case) => cases.push(case),
                                        Elem::End => break None,
                                        Elem::Formula(t) => break Some(t),
                                        _ => panic!("unexpected element"),
                                    }
                                };
                                (
                                    lab,
                                    otherwise.is_none(),
                                    DefValue::Formula(DefBody { cases, otherwise }),
                                )
                            }
                            _ => panic!("unknown definiens kind"),
                        };
                        if !end_tag {
                            self.r.end_tag(&mut self.buf)?;
                        }
                        return Ok(Elem::Definiens(Box::new(Definiens { pos, label, kind })));
                    }
                    b"LociEquality" => Elem::LociEquality(
                        self.parse_locus()?.unwrap(),
                        self.parse_locus()?.unwrap(),
                    ),
                    b"Label" => {
                        let (mut pos, (mut id, mut spelling)) = <(Position, _)>::default();
                        for attr in e.attributes() {
                            let attr = attr?;
                            match attr.key.0 {
                                b"line" => pos.line = self.r.get_attr(&attr.value)?,
                                b"col" => pos.col = self.r.get_attr(&attr.value)?,
                                b"labelnr" => {
                                    id = self
                                        .r
                                        .get_attr::<u32>(&attr.value)?
                                        .checked_sub(1)
                                        .map(LabelId)
                                }
                                b"spelling" => spelling = attr.unescape_value().unwrap(),
                                _ => {}
                            }
                        }
                        Elem::Label((!spelling.is_empty()).then(|| {
                            Box::new(Label {
                                pos,
                                id: (id, spelling.into()),
                            })
                        }))
                    }
                    b"Link" => Elem::Link(self.r.get_pos(&e)?),
                    b"Local-Reference" => {
                        let pos = self.r.get_pos(&e)?;
                        let kind = if self.msm {
                            let attr = e.try_get_attribute(b"labelnr").unwrap().unwrap();
                            ReferenceKind::Priv(
                                self.r
                                    .get_attr::<u32>(&attr.value)?
                                    .checked_sub(1)
                                    .map(LabelId),
                            )
                        } else {
                            let attr = e.try_get_attribute(b"spelling").unwrap().unwrap();
                            ReferenceKind::UnresolvedPriv(
                                attr.unescape_value().unwrap().to_string(),
                            )
                        };
                        Elem::Reference(Reference { pos, kind })
                    }
                    b"Theorem-Reference" => {
                        let (mut pos, (mut article_nr, mut thm_nr)) = <(Position, _)>::default();
                        for attr in e.attributes() {
                            let attr = attr?;
                            match attr.key.0 {
                                b"line" => pos.line = self.r.get_attr(&attr.value)?,
                                b"col" => pos.col = self.r.get_attr(&attr.value)?,
                                b"nr" => article_nr = self.r.get_attr(&attr.value)?,
                                // b"spelling" => spelling = attr.unescape_value().unwrap(),
                                b"number" => {
                                    thm_nr = ThmId(self.r.get_attr::<u32>(&attr.value)? - 1)
                                }
                                _ => {}
                            }
                        }
                        let refs = vec![RefFragment::Thm { pos, id: thm_nr }];
                        Elem::Reference(Reference {
                            pos,
                            kind: ReferenceKind::Global(article_nr, refs),
                        })
                    }
                    b"Definition-Reference" => {
                        let (mut pos, (mut article_nr, mut def_nr)) = <(Position, _)>::default();
                        for attr in e.attributes() {
                            let attr = attr?;
                            match attr.key.0 {
                                b"line" => pos.line = self.r.get_attr(&attr.value)?,
                                b"col" => pos.col = self.r.get_attr(&attr.value)?,
                                b"nr" => article_nr = self.r.get_attr(&attr.value)?,
                                // b"spelling" => spelling = attr.unescape_value().unwrap(),
                                b"number" => {
                                    def_nr = DefId(self.r.get_attr::<u32>(&attr.value)? - 1)
                                }
                                _ => {}
                            }
                        }
                        let refs = vec![RefFragment::Def { pos, id: def_nr }];
                        Elem::Reference(Reference {
                            pos,
                            kind: ReferenceKind::Global(article_nr, refs),
                        })
                    }
                    b"Partial-Definiens" => match self.parse_elem()? {
                        Elem::Term(case) => Elem::DefCaseTerm(DefCase {
                            case,
                            guard: self.parse_formula()?,
                        }),
                        Elem::Formula(case) => Elem::DefCaseFormula(DefCase {
                            case,
                            guard: self.parse_formula()?,
                        }),
                        _ => panic!("expected term or formula"),
                    },
                    b"Placeholder-Term" => {
                        let (mut pos, mut nr) = <(Position, _)>::default();
                        for attr in e.attributes() {
                            let attr = attr?;
                            match attr.key.0 {
                                b"line" => pos.line = self.r.get_attr(&attr.value)?,
                                b"col" => pos.col = self.r.get_attr(&attr.value)?,
                                b"nr" => nr = LocusId(self.r.get_attr::<u8>(&attr.value)? - 1),
                                _ => {}
                            }
                        }
                        Elem::Term(Box::new(Term::Placeholder { pos, nr }))
                    }
                    b"Numeral-Term" => {
                        let (mut pos, mut value) = <(Position, _)>::default();
                        for attr in e.attributes() {
                            let attr = attr?;
                            match attr.key.0 {
                                b"line" => pos.line = self.r.get_attr(&attr.value)?,
                                b"col" => pos.col = self.r.get_attr(&attr.value)?,
                                b"number" => value = self.r.get_attr(&attr.value)?,
                                _ => {}
                            }
                        }
                        Elem::Term(Box::new(Term::Numeral { pos, value }))
                    }
                    b"Simple-Term" => {
                        let (mut pos, (mut kind, mut var, mut spelling)) =
                            <(Position, _)>::default();
                        for attr in e.attributes() {
                            let attr = attr?;
                            match attr.key.0 {
                                b"line" => pos.line = self.r.get_attr(&attr.value)?,
                                b"col" => pos.col = self.r.get_attr(&attr.value)?,
                                // b"idnr" => sym = self.r.get_attr::<u32>(&attr.value)? - 1,
                                b"spelling" => {
                                    spelling = attr.unescape_value().unwrap().to_string()
                                }
                                // b"origin" => origin = self.r.get_attr(&attr.value)?,
                                b"kind" => kind = self.r.get_attr(&attr.value)?,
                                // b"serialnr" => serial = self.r.get_attr(&attr.value)?,
                                b"varnr" => var = self.r.get_attr::<u32>(&attr.value)? - 1,
                                _ => {}
                            }
                        }
                        let kind = match kind {
                            IdentKind::Free => None,
                            IdentKind::Bound => Some(VarKind::Bound(BoundId(var))),
                            IdentKind::Const | IdentKind::DefConst => {
                                Some(VarKind::Const(ConstId(var)))
                            }
                            _ => unreachable!(),
                        };
                        let term = Term::Var {
                            pos,
                            kind,
                            spelling,
                        };
                        Elem::Term(Box::new(term))
                    }
                    b"Private-Functor-Term" => {
                        let (mut pos, (mut shape, mut var, mut spelling)) =
                            <(Position, _)>::default();
                        for attr in e.attributes() {
                            let attr = attr?;
                            match attr.key.0 {
                                b"line" => pos.line = self.r.get_attr(&attr.value)?,
                                b"col" => pos.col = self.r.get_attr(&attr.value)?,
                                // b"idnr" => id = self.r.get_attr::<u32>(&attr.value)? - 1,
                                b"spelling" => spelling = attr.unescape_value().unwrap(),
                                b"shape" => shape = self.r.get_attr(&attr.value)?,
                                // b"serialnr" => serial = self.r.get_attr(&attr.value)?,
                                b"nr" => var = self.r.get_attr::<u32>(&attr.value)? - 1,
                                _ => {}
                            }
                        }
                        let (spelling, args) = (spelling.into(), self.parse_terms()?);
                        let kind = match shape {
                            IdentKind::Free => None,
                            IdentKind::PrivFunc => Some(PrivFuncKind::PrivFunc(PrivFuncId(var))),
                            IdentKind::SchFunc => Some(PrivFuncKind::SchFunc(SchFuncId(var))),
                            _ => unreachable!(),
                        };
                        let term = Term::PrivFunc {
                            pos,
                            kind,
                            spelling,
                            args,
                        };
                        return Ok(Elem::Term(Box::new(term)));
                    }
                    b"Infix-Term" => {
                        let (mut pos, (mut sym, mut spelling)) = <(Position, _)>::default();
                        for attr in e.attributes() {
                            let attr = attr?;
                            match attr.key.0 {
                                b"line" => pos.line = self.r.get_attr(&attr.value)?,
                                b"col" => pos.col = self.r.get_attr(&attr.value)?,
                                b"nr" => sym = FuncSymId(self.r.get_attr::<u32>(&attr.value)? - 1),
                                b"spelling" => {
                                    spelling = attr.unescape_value().unwrap().to_string()
                                }
                                _ => {}
                            }
                        }
                        self.r.read_start(&mut self.buf, Some("Arguments"))?;
                        let mut args = self.parse_terms()?;
                        let left = args.len().try_into().unwrap();
                        self.r.read_start(&mut self.buf, Some("Arguments"))?;
                        while let Some(t) = self.parse_term()? {
                            args.push(*t);
                        }
                        Elem::Term(Box::new(Term::Infix {
                            pos,
                            sym: (sym, spelling),
                            left,
                            args,
                        }))
                    }
                    b"Circumfix-Term" => {
                        let (mut pos, (mut lsym, mut lsp)) = <(Position, _)>::default();
                        for attr in e.attributes() {
                            let attr = attr?;
                            match attr.key.0 {
                                b"line" => pos.line = self.r.get_attr(&attr.value)?,
                                b"col" => pos.col = self.r.get_attr(&attr.value)?,
                                b"nr" => {
                                    lsym = LeftBrkSymId(self.r.get_attr::<u32>(&attr.value)? - 1)
                                }
                                b"spelling" => lsp = attr.unescape_value().unwrap().to_string(),
                                _ => {}
                            }
                        }
                        let (rsym, args) = (self.parse_right_pattern()?, self.parse_terms()?);
                        return Ok(Elem::Term(Box::new(Term::Bracket {
                            pos,
                            lsym: (lsym, lsp),
                            rsym,
                            args,
                        })));
                    }
                    b"Aggregate-Term" => {
                        let (mut pos, (mut sym, mut spelling)) = <(Position, _)>::default();
                        for attr in e.attributes() {
                            let attr = attr?;
                            match attr.key.0 {
                                b"line" => pos.line = self.r.get_attr(&attr.value)?,
                                b"col" => pos.col = self.r.get_attr(&attr.value)?,
                                b"nr" => {
                                    sym = StructSymId(self.r.get_attr::<u32>(&attr.value)? - 1)
                                }
                                b"spelling" => {
                                    spelling = attr.unescape_value().unwrap().to_string()
                                }
                                _ => {}
                            }
                        }
                        let args = self.parse_terms()?;
                        return Ok(Elem::Term(Box::new(Term::Aggregate {
                            pos,
                            sym: (sym, spelling),
                            args,
                        })));
                    }
                    b"Forgetful-Functor-Term" => {
                        let (mut pos, (mut sym, mut spelling)) = <(Position, _)>::default();
                        for attr in e.attributes() {
                            let attr = attr?;
                            match attr.key.0 {
                                b"line" => pos.line = self.r.get_attr(&attr.value)?,
                                b"col" => pos.col = self.r.get_attr(&attr.value)?,
                                b"nr" => {
                                    sym = StructSymId(self.r.get_attr::<u32>(&attr.value)? - 1)
                                }
                                b"spelling" => {
                                    spelling = attr.unescape_value().unwrap().to_string()
                                }
                                _ => {}
                            }
                        }
                        let arg = self.parse_term()?.unwrap();
                        Elem::Term(Box::new(Term::SubAggr {
                            pos,
                            sym: (sym, spelling),
                            arg,
                        }))
                    }
                    b"Selector-Term" => {
                        let (mut pos, (mut sym, mut spelling)) = <(Position, _)>::default();
                        for attr in e.attributes() {
                            let attr = attr?;
                            match attr.key.0 {
                                b"line" => pos.line = self.r.get_attr(&attr.value)?,
                                b"col" => pos.col = self.r.get_attr(&attr.value)?,
                                b"nr" => sym = SelSymId(self.r.get_attr::<u32>(&attr.value)? - 1),
                                b"spelling" => {
                                    spelling = attr.unescape_value().unwrap().to_string()
                                }
                                _ => {}
                            }
                        }
                        let arg = self.parse_term()?.unwrap();
                        Elem::Term(Box::new(Term::Selector {
                            pos,
                            sym: (sym, spelling),
                            arg,
                        }))
                    }
                    b"Internal-Selector-Term" => {
                        let (mut pos, (mut sym, mut spelling, mut id)) = <(Position, _)>::default();
                        for attr in e.attributes() {
                            let attr = attr?;
                            match attr.key.0 {
                                b"line" => pos.line = self.r.get_attr(&attr.value)?,
                                b"col" => pos.col = self.r.get_attr(&attr.value)?,
                                b"nr" => sym = SelSymId(self.r.get_attr::<u32>(&attr.value)? - 1),
                                b"varnr" => {
                                    id = self
                                        .r
                                        .get_attr::<u32>(&attr.value)?
                                        .checked_sub(1)
                                        .map(ConstId)
                                }
                                b"spelling" => {
                                    spelling = attr.unescape_value().unwrap().to_string()
                                }
                                _ => {}
                            }
                        }
                        Elem::Term(Box::new(Term::InternalSelector {
                            pos,
                            sym: (sym, spelling),
                            id,
                        }))
                    }
                    b"Qualification-Term" => {
                        let pos = self.r.get_pos(&e)?;
                        let term = self.parse_term()?.unwrap();
                        let ty = self.parse_type()?.unwrap();
                        Elem::Term(Box::new(Term::Qua { pos, term, ty }))
                    }
                    b"Global-Choice-Term" => {
                        let pos = self.r.get_pos(&e)?;
                        let ty = self.parse_type()?.unwrap();
                        Elem::Term(Box::new(Term::The { pos, ty }))
                    }
                    b"Simple-Fraenkel-Term" => {
                        let pos = self.r.get_pos(&e)?;
                        let mut vars = vec![];
                        let scope = loop {
                            match self.parse_elem()? {
                                Elem::Binder(var) => vars.push(*var),
                                Elem::Term(t) => break t,
                                _ => panic!("expected <Conditions>"),
                            }
                        };
                        Elem::Term(Box::new(Term::Fraenkel {
                            pos,
                            vars,
                            scope,
                            compr: None,
                            nameck: None,
                        }))
                    }
                    b"Fraenkel-Term" => {
                        let pos = self.r.get_pos(&e)?;
                        let mut vars = vec![];
                        let scope = loop {
                            match self.parse_elem()? {
                                Elem::Binder(var) => vars.push(*var),
                                Elem::Term(t) => break t,
                                _ => panic!("expected <Conditions>"),
                            }
                        };
                        let compr = Some(self.parse_formula()?);
                        Elem::Term(Box::new(Term::Fraenkel {
                            pos,
                            vars,
                            scope,
                            compr,
                            nameck: None,
                        }))
                    }
                    b"it-Term" => {
                        let pos = self.r.get_pos(&e)?;
                        Elem::Term(Box::new(Term::It { pos }))
                    }
                    b"Standard-Type" => {
                        let (mut pos, (mut sym, mut spelling)) = <(Position, _)>::default();
                        for attr in e.attributes() {
                            let attr = attr?;
                            match attr.key.0 {
                                b"line" => pos.line = self.r.get_attr(&attr.value)?,
                                b"col" => pos.col = self.r.get_attr(&attr.value)?,
                                b"nr" => sym = ModeSymId(self.r.get_attr::<u32>(&attr.value)? - 1),
                                b"spelling" => {
                                    spelling = attr.unescape_value().unwrap().to_string()
                                }
                                _ => {}
                            }
                        }
                        let args = self.parse_terms()?;
                        return Ok(Elem::Type(Box::new(Type::Mode {
                            pos,
                            sym: (sym, spelling),
                            args,
                        })));
                    }
                    b"Struct-Type" => {
                        let (mut pos, (mut sym, mut spelling)) = <(Position, _)>::default();
                        for attr in e.attributes() {
                            let attr = attr?;
                            match attr.key.0 {
                                b"line" => pos.line = self.r.get_attr(&attr.value)?,
                                b"col" => pos.col = self.r.get_attr(&attr.value)?,
                                b"nr" => {
                                    sym = StructSymId(self.r.get_attr::<u32>(&attr.value)? - 1)
                                }
                                b"spelling" => {
                                    spelling = attr.unescape_value().unwrap().to_string()
                                }
                                _ => {}
                            }
                        }
                        let args = self.parse_terms()?;
                        return Ok(Elem::Type(Box::new(Type::Struct {
                            pos,
                            sym: (sym, spelling),
                            args,
                        })));
                    }
                    b"Clustered-Type" => Elem::Type(Box::new(Type::Cluster {
                        pos: self.r.get_pos(&e)?,
                        attrs: self.parse_attrs()?,
                        ty: self.parse_type()?.unwrap(),
                    })),
                    b"ReservedDscr-Type" if self.msm => {
                        let (mut pos, mut group) = <(Position, _)>::default();
                        for attr in e.attributes() {
                            let attr = attr?;
                            match attr.key.0 {
                                b"line" => pos.line = self.r.get_attr(&attr.value)?,
                                b"col" => pos.col = self.r.get_attr(&attr.value)?,
                                b"nr" => {
                                    group = ResGroupId(self.r.get_attr::<u32>(&attr.value)? - 1)
                                }
                                _ => {}
                            }
                        }
                        let subst = self.parse_substitution()?;
                        return Ok(Elem::Type(Box::new(Type::Reservation {
                            pos,
                            group,
                            subst,
                        })));
                    }
                    b"Negated-Formula" => {
                        let pos = self.r.get_pos(&e)?;
                        Elem::Formula(Box::new(Formula::Not {
                            pos,
                            f: self.parse_formula()?,
                        }))
                    }
                    b"Conjunctive-Formula" => {
                        let (pos, f1, f2) = (
                            self.r.get_pos(&e)?,
                            self.parse_formula()?,
                            self.parse_formula()?,
                        );
                        Elem::Formula(Box::new(Formula::Binop {
                            kind: FormulaBinop::And,
                            pos,
                            f1,
                            f2,
                        }))
                    }
                    b"Disjunctive-Formula" => {
                        let (pos, f1, f2) = (
                            self.r.get_pos(&e)?,
                            self.parse_formula()?,
                            self.parse_formula()?,
                        );
                        Elem::Formula(Box::new(Formula::Binop {
                            kind: FormulaBinop::Or,
                            pos,
                            f1,
                            f2,
                        }))
                    }
                    b"Conditional-Formula" => {
                        let (pos, f1, f2) = (
                            self.r.get_pos(&e)?,
                            self.parse_formula()?,
                            self.parse_formula()?,
                        );
                        Elem::Formula(Box::new(Formula::Binop {
                            kind: FormulaBinop::Imp,
                            pos,
                            f1,
                            f2,
                        }))
                    }
                    b"Biconditional-Formula" => {
                        let (pos, f1, f2) = (
                            self.r.get_pos(&e)?,
                            self.parse_formula()?,
                            self.parse_formula()?,
                        );
                        Elem::Formula(Box::new(Formula::Binop {
                            kind: FormulaBinop::Iff,
                            pos,
                            f1,
                            f2,
                        }))
                    }
                    b"FlexaryConjunctive-Formula" => {
                        let (pos, f1, f2) = (
                            self.r.get_pos(&e)?,
                            self.parse_formula()?,
                            self.parse_formula()?,
                        );
                        Elem::Formula(Box::new(Formula::Binop {
                            kind: FormulaBinop::FlexAnd,
                            pos,
                            f1,
                            f2,
                        }))
                    }
                    b"FlexaryDisjunctive-Formula" => {
                        let (pos, f1, f2) = (
                            self.r.get_pos(&e)?,
                            self.parse_formula()?,
                            self.parse_formula()?,
                        );
                        Elem::Formula(Box::new(Formula::Binop {
                            kind: FormulaBinop::FlexOr,
                            pos,
                            f1,
                            f2,
                        }))
                    }
                    b"Predicative-Formula" => {
                        let (mut pos, (mut sym, mut spelling)) = <(Position, _)>::default();
                        for attr in e.attributes() {
                            let attr = attr?;
                            match attr.key.0 {
                                b"line" => pos.line = self.r.get_attr(&attr.value)?,
                                b"col" => pos.col = self.r.get_attr(&attr.value)?,
                                b"nr" => sym = PredSymId(self.r.get_attr::<u32>(&attr.value)? - 1),
                                b"spelling" => {
                                    spelling = attr.unescape_value().unwrap().to_string()
                                }
                                _ => {}
                            }
                        }
                        self.r.read_start(&mut self.buf, Some("Arguments"))?;
                        let mut args = self.parse_terms()?;
                        let left = args.len().try_into().unwrap();
                        self.r.read_start(&mut self.buf, Some("Arguments"))?;
                        while let Some(t) = self.parse_term()? {
                            args.push(*t)
                        }
                        let pred = Box::new(Pred {
                            pos,
                            positive: true,
                            sym: (sym, spelling),
                            left,
                            args,
                        });
                        Elem::Formula(Box::new(Formula::Pred(pred)))
                    }
                    b"RightSideOf-Predicative-Formula" => {
                        let (mut pos, (mut sym, mut spelling)) = <(Position, _)>::default();
                        for attr in e.attributes() {
                            let attr = attr?;
                            match attr.key.0 {
                                b"line" => pos.line = self.r.get_attr(&attr.value)?,
                                b"col" => pos.col = self.r.get_attr(&attr.value)?,
                                b"nr" => sym = PredSymId(self.r.get_attr::<u32>(&attr.value)? - 1),
                                b"spelling" => {
                                    spelling = attr.unescape_value().unwrap().to_string()
                                }
                                _ => {}
                            }
                        }
                        self.r.read_start(&mut self.buf, Some("Arguments"))?;
                        let right = self.parse_terms()?;
                        Elem::PredRhs(Box::new(PredRhs {
                            pos,
                            positive: true,
                            sym: (sym, spelling),
                            right,
                        }))
                    }
                    b"Multi-Predicative-Formula" => {
                        let pos = self.r.get_pos(&e)?;
                        let first = match *self.parse_formula()? {
                            Formula::Not { f, .. } => match *f {
                                Formula::Pred(mut pred) => {
                                    pred.positive = !pred.positive;
                                    pred
                                }
                                _ => panic!("expected predicate"),
                            },
                            Formula::Pred(pred) => pred,
                            _ => panic!("expected predicate"),
                        };
                        let mut rest = vec![];
                        loop {
                            match self.parse_elem()? {
                                Elem::PredRhs(rhs) => rest.push(*rhs),
                                Elem::End => break,
                                _ => panic!("expected <RightSideOf-Predicative-Formula>"),
                            }
                        }
                        return Ok(Elem::Formula(Box::new(Formula::ChainPred {
                            pos,
                            first,
                            rest,
                        })));
                    }
                    b"Private-Predicate-Formula" => {
                        let (mut pos, (mut shape, mut var, mut spelling)) =
                            <(Position, _)>::default();
                        for attr in e.attributes() {
                            let attr = attr?;
                            match attr.key.0 {
                                b"line" => pos.line = self.r.get_attr(&attr.value)?,
                                b"col" => pos.col = self.r.get_attr(&attr.value)?,
                                // b"idnr" => id = self.r.get_attr::<u32>(&attr.value)? - 1,
                                b"spelling" => spelling = attr.unescape_value().unwrap(),
                                b"shape" => shape = self.r.get_attr(&attr.value)?,
                                b"nr" => var = self.r.get_attr::<u32>(&attr.value)? - 1,
                                _ => {}
                            }
                        }
                        let (spelling, args) = (spelling.into(), self.parse_terms()?);
                        let kind = match shape {
                            IdentKind::Free => None,
                            IdentKind::PrivPred => Some(PrivPredKind::PrivPred(PrivPredId(var))),
                            IdentKind::SchPred => Some(PrivPredKind::SchPred(SchPredId(var))),
                            _ => unreachable!(),
                        };
                        return Ok(Elem::Formula(Box::new(Formula::PrivPred {
                            pos,
                            kind,
                            spelling,
                            args,
                        })));
                    }
                    b"Attributive-Formula" => Elem::Formula(Box::new(Formula::Attr {
                        pos: self.r.get_pos(&e)?,
                        positive: true,
                        term: self.parse_term()?.unwrap(),
                        attrs: self.parse_attrs()?,
                    })),
                    b"Qualifying-Formula" => Elem::Formula(Box::new(Formula::Is {
                        pos: self.r.get_pos(&e)?,
                        positive: true,
                        term: self.parse_term()?.unwrap(),
                        ty: self.parse_type()?.unwrap(),
                    })),
                    b"Universal-Quantifier-Formula" => {
                        let pos = self.r.get_pos(&e)?;
                        let (vars, scope) = (vec![*self.parse_binder()?], self.parse_formula()?);
                        let f = Formula::Binder {
                            kind: FormulaBinder::ForAll,
                            pos,
                            vars,
                            st: None,
                            scope,
                        };
                        Elem::Formula(Box::new(f))
                    }
                    b"Existential-Quantifier-Formula" => {
                        let pos = self.r.get_pos(&e)?;
                        let (vars, scope) = (vec![*self.parse_binder()?], self.parse_formula()?);
                        let f = Formula::Binder {
                            kind: FormulaBinder::Exists,
                            pos,
                            vars,
                            st: None,
                            scope,
                        };
                        Elem::Formula(Box::new(f))
                    }
                    b"Thesis" => Elem::Formula(Box::new(Formula::Thesis {
                        pos: self.r.get_pos(&e)?,
                    })),
                    b"Contradiction" => Elem::Formula(Box::new(Formula::False {
                        pos: self.r.get_pos(&e)?,
                    })),
                    b"Predicate-Pattern" => {
                        let (mut pos, (mut sym, mut spelling)) = <(Position, _)>::default();
                        for attr in e.attributes() {
                            let attr = attr?;
                            match attr.key.0 {
                                b"line" => pos.line = self.r.get_attr(&attr.value)?,
                                b"col" => pos.col = self.r.get_attr(&attr.value)?,
                                b"nr" => sym = PredSymId(self.r.get_attr::<u32>(&attr.value)? - 1),
                                b"spelling" => {
                                    spelling = attr.unescape_value().unwrap().to_string()
                                }
                                _ => {}
                            }
                        }
                        let mut args = vec![];
                        self.parse_loci(&mut args)?;
                        let left = args.len() as u8;
                        self.parse_loci(&mut args)?;
                        let pat = PatternPred {
                            pos,
                            sym: (sym, spelling),
                            left,
                            args,
                        };
                        Elem::Pattern(Pattern::Pred(Box::new(pat)))
                    }
                    b"Operation-Functor-Pattern" => {
                        let (mut pos, (mut sym, mut spelling)) = <(Position, _)>::default();
                        for attr in e.attributes() {
                            let attr = attr?;
                            match attr.key.0 {
                                b"line" => pos.line = self.r.get_attr(&attr.value)?,
                                b"col" => pos.col = self.r.get_attr(&attr.value)?,
                                b"nr" => sym = FuncSymId(self.r.get_attr::<u32>(&attr.value)? - 1),
                                b"spelling" => {
                                    spelling = attr.unescape_value().unwrap().to_string()
                                }
                                _ => {}
                            }
                        }
                        let mut args = vec![];
                        self.parse_loci(&mut args)?;
                        let left = args.len() as u8;
                        self.parse_loci(&mut args)?;
                        let pat = PatternFunc::Func {
                            pos,
                            sym: (sym, spelling),
                            left,
                            args,
                        };
                        Elem::Pattern(Pattern::Func(Box::new(pat)))
                    }
                    b"Bracket-Functor-Pattern" => {
                        let (mut pos, (mut lsym, mut lsp)) = <(Position, _)>::default();
                        for attr in e.attributes() {
                            let attr = attr?;
                            match attr.key.0 {
                                b"line" => pos.line = self.r.get_attr(&attr.value)?,
                                b"col" => pos.col = self.r.get_attr(&attr.value)?,
                                b"nr" => {
                                    lsym = LeftBrkSymId(self.r.get_attr::<u32>(&attr.value)? - 1)
                                }
                                b"spelling" => lsp = attr.unescape_value().unwrap().to_string(),
                                _ => {}
                            }
                        }
                        let (rsym, mut args) = (self.parse_right_pattern()?, vec![]);
                        self.parse_loci(&mut args)?;
                        let pat = PatternFunc::Bracket {
                            pos,
                            lsym: (lsym, lsp),
                            rsym,
                            args,
                        };
                        Elem::Pattern(Pattern::Func(Box::new(pat)))
                    }
                    b"Mode-Pattern" => {
                        let (mut pos, (mut sym, mut spelling)) = <(Position, _)>::default();
                        for attr in e.attributes() {
                            let attr = attr?;
                            match attr.key.0 {
                                b"line" => pos.line = self.r.get_attr(&attr.value)?,
                                b"col" => pos.col = self.r.get_attr(&attr.value)?,
                                b"nr" => sym = ModeSymId(self.r.get_attr::<u32>(&attr.value)? - 1),
                                b"spelling" => {
                                    spelling = attr.unescape_value().unwrap().to_string()
                                }
                                _ => {}
                            }
                        }
                        let mut args = vec![];
                        self.parse_loci(&mut args)?;
                        let pat = PatternMode {
                            pos,
                            sym: (sym, spelling),
                            args,
                        };
                        Elem::Pattern(Pattern::Mode(Box::new(pat)))
                    }
                    b"Attribute-Pattern" => {
                        let (mut pos, (mut sym, mut spelling)) = <(Position, _)>::default();
                        for attr in e.attributes() {
                            let attr = attr?;
                            match attr.key.0 {
                                b"line" => pos.line = self.r.get_attr(&attr.value)?,
                                b"col" => pos.col = self.r.get_attr(&attr.value)?,
                                b"nr" => sym = AttrSymId(self.r.get_attr::<u32>(&attr.value)? - 1),
                                b"spelling" => {
                                    spelling = attr.unescape_value().unwrap().to_string()
                                }
                                _ => {}
                            }
                        }
                        let arg = self.parse_locus()?.unwrap();
                        let mut args = vec![];
                        self.parse_loci(&mut args)?;
                        args.push(*arg);
                        let pat = PatternAttr {
                            pos,
                            sym: (sym, spelling),
                            args,
                        };
                        Elem::Pattern(Pattern::Attr(Box::new(pat)))
                    }
                    b"Selector" => {
                        let (mut pos, (mut sym, mut spelling)) = <(Position, _)>::default();
                        for attr in e.attributes() {
                            let attr = attr?;
                            match attr.key.0 {
                                b"line" => pos.line = self.r.get_attr(&attr.value)?,
                                b"col" => pos.col = self.r.get_attr(&attr.value)?,
                                b"nr" => sym = SelSymId(self.r.get_attr::<u32>(&attr.value)? - 1),
                                b"spelling" => {
                                    spelling = attr.unescape_value().unwrap().to_string()
                                }
                                _ => {}
                            }
                        }
                        Elem::Selector(Field {
                            pos,
                            sym: (sym, spelling),
                        })
                    }
                    _ => Elem::Other,
                };
                self.r.end_tag(&mut self.buf)?;
                elem
            } else {
                Elem::End
            })
        })
    }
}
