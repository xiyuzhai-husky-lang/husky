use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct FormDefn {
    kind: FormDefnKind,
    body: ExprIdx,
}

#[derive(Debug, PartialEq, Eq)]
pub enum FormDefnKind {
    Function(PhyParadigm),
    Method(PhyParadigm),
    Feature(Paradigm),
    TrackedFunction(Paradigm),
    Morphism,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Paradigm {
    Phy(PhyParadigm),
    Def,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PhyParadigm {
    Func,
    Proc,
    Fn,
}
