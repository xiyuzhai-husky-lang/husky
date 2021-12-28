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
        ty: InputType,
    },
    MembFunc {
        this: InputContract,
        inputs: Vec<InputType>,
        output: InputType,
        args: Vec<GenericPlaceholder>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FuncKind {
    Test,
    Proc,
    PureFunc,
    Def,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuncDecl {
    pub funcname: CustomIdentifier,
    pub placeholders: Vec<GenericPlaceholder>,
    pub inputs: Vec<(CustomIdentifier, InputType)>,
    pub output: ScopeId,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GenericPlaceholder {
    pub ident: CustomIdentifier,
    pub traits: Vec<ScopeId>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct InputType {
    pub contract: InputContract,
    pub ty: ScopeId,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum InputContract {
    Intact,
    Share,
    Own,
}
