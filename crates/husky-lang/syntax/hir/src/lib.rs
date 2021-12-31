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
        args: Vec<(CustomIdentifier, SpaceParamKind)>,
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
    pub time_params: Vec<SpaceParameter>,
    pub space_params: Vec<SpaceParameter>,
    pub inputs: Vec<(CustomIdentifier, InputType)>,
    pub output: ScopeId,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TimeParameter {
    pub ident: CustomIdentifier,
    pub traits: Vec<CustomIdentifier>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SpaceParamKind {
    Const,
    Type { traits: Vec<ScopeId> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpaceParameter {
    pub ident: CustomIdentifier,
    pub kind: SpaceParamKind,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct InputType {
    pub contract: InputContract,
    pub ty: ScopeId,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InputContract {
    Intact,
    Share,
    Own,
}
