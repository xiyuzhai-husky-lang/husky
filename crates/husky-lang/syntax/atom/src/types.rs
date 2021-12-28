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
        ty: ContractedType,
    },
    MembFunc {
        this: Contract,
        inputs: Vec<ContractedType>,
        output: ContractedType,
        args: Vec<GenericPlaceholder>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FuncKind {
    Test,
    Proc,
    Func,
    Def,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuncDecl {
    pub funcname: CustomIdentifier,
    pub placeholders: Vec<GenericPlaceholder>,
    pub inputs: Vec<(CustomIdentifier, ContractedType)>,
    pub output: ScopeId,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GenericPlaceholder {
    pub ident: CustomIdentifier,
    pub traits: Vec<ScopeId>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ContractedType {
    pub contract: Contract,
    pub ty: ScopeId,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Contract {
    PureInput,
    Share,
    Give,
}
