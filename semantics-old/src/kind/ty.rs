use syntax_types::MembType;
use word::CustomIdentifier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ty {
    pub kind: TyKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TyKind {
    Enum,
    Struct { memb_vars: Vec<MembVar> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MembVar {
    pub ident: CustomIdentifier,
    pub ty: MembType,
}
