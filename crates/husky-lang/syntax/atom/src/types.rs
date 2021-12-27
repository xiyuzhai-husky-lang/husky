use scope::ScopeId;
use word::CustomIdentifier;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TypeKind {
    Enum(Vec<CustomIdentifier>),
    Struct,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MembKind {
    MembVar {
        ty: LiasonedType,
    },
    MembFunc {
        this: Contract,
        inputs: Vec<LiasonedType>,
        output: LiasonedType,
        args: Vec<GenericPlaceholder>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FuncDef {
    pub kind: FuncKind,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FuncKind {
    Test,
    Proc,
    Func,
    Def,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GenericPlaceholder {
    pub ident: CustomIdentifier,
    pub traits: Vec<ScopeId>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LiasonedType {
    pub contract: Contract,
    pub ty: ScopeId,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Contract {
    Borrow,
    Share,
    Give,
}
