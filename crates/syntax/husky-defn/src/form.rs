use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct FormDefn {
    kind: FormDefnKind,
    body: ExprIdx,
}

#[derive(Debug, PartialEq, Eq)]
pub enum FormDefnKind {
    Function(EagerParadigm),
    Method(EagerParadigm),
    Feature(Paradigm),
    TrackedFunction(Paradigm),
    Morphism,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Paradigm {
    Eager(EagerParadigm),
    Def,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EagerParadigm {
    Func,
    Proc,
}
