use enum_index::IsEnumIndex;
use husky_entity_protocol::EntityClass;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, IsEnumIndex)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TokenClass {
    Attribute,
    Comment,
    ControlFlowKeyword,
    OtherKeyword,
    Field,
    Punctuation,
    Parameter,
    Variable,
    LoopVariable,
    ModuleEntity,
    TypeEntity,
    FunctionEntity,
    ValEntity,
    TraitEntity,
    TypeVariantEntity,
    MethodEntity,
    MemoizedFieldEntity,
    ImplicitParameter,
    Method,
    Literal,
    HtmlTagKind,
    WordOpr,
    SelfType,
    SelfValue,
    HtmlFunctionIdent,
    HtmlPropertyIdent,
    Todo,
    Unreachable,
    Ident,
    Label,
    Error,
}

impl TokenClass {
    fn description(self) -> &'static str {
        match self {
            TokenClass::Attribute => "attribute token",
            TokenClass::Comment => "comment token",
            TokenClass::ControlFlowKeyword => "control flow keyword token",
            TokenClass::OtherKeyword => "other keyword token",
            TokenClass::Field => "field token",
            TokenClass::Punctuation => "special character or symbol token",
            TokenClass::Parameter => "parameter token",
            TokenClass::Variable => "variable token",
            TokenClass::LoopVariable => "frame variable token",
            TokenClass::ModuleEntity => "module entity token",
            TokenClass::TypeEntity => "type entity token",
            TokenClass::FunctionEntity => "function entity token",
            TokenClass::ValEntity => "value entity token",
            TokenClass::TraitEntity => "trait entity token",
            TokenClass::TypeVariantEntity => "type variant entity token",
            TokenClass::MethodEntity => "method entity token",
            TokenClass::MemoizedFieldEntity => "memoized field entity token",
            TokenClass::ImplicitParameter => "implicit parameter token",
            TokenClass::Method => "method token",
            TokenClass::Literal => "literal value token",
            TokenClass::HtmlTagKind => "html tag kind token",
            TokenClass::WordOpr => "word operator token",
            TokenClass::SelfType => "'self' type token",
            TokenClass::SelfValue => "'self' value token",
            TokenClass::HtmlFunctionIdent => "html function identifier token",
            TokenClass::HtmlPropertyIdent => "html property identifier token",
            TokenClass::Todo => "todo comment token",
            TokenClass::Unreachable => "unreachable code token",
            TokenClass::Ident => "identifier token",
            TokenClass::Label => "label token",
            TokenClass::Error => "error or invalid token",
        }
    }
}

pub enum KeywordClass {
    ControlFlow,
    Other,
}

impl From<KeywordClass> for TokenClass {
    fn from(class: KeywordClass) -> Self {
        match class {
            KeywordClass::ControlFlow => Self::ControlFlowKeyword,
            KeywordClass::Other => Self::OtherKeyword,
        }
    }
}

impl From<EntityClass> for TokenClass {
    fn from(class: EntityClass) -> Self {
        match class {
            EntityClass::Module => TokenClass::ModuleEntity,
            EntityClass::Type => TokenClass::TypeEntity,
            EntityClass::FunctionFn => TokenClass::FunctionEntity,
            EntityClass::FunctionGn => TokenClass::FunctionEntity,
            EntityClass::AliasType => TokenClass::TypeEntity,
            EntityClass::Val => TokenClass::ValEntity,
            EntityClass::Trait => TokenClass::TraitEntity,
            EntityClass::TypeVariant => TokenClass::TypeVariantEntity,
            EntityClass::MethodFn => TokenClass::MethodEntity,
            EntityClass::MethodGn => TokenClass::MethodEntity,
            EntityClass::AssociatedFunctionFn => TokenClass::FunctionEntity,
            EntityClass::AssociatedFunctionGn => TokenClass::FunctionEntity,
            EntityClass::MemoizedField => TokenClass::MemoizedFieldEntity,
            EntityClass::AssociatedVal => TokenClass::ValEntity,
            EntityClass::AssociatedType => TokenClass::TypeEntity,
            EntityClass::ImplBlock => unreachable!(),
            EntityClass::Attr => unreachable!(),
        }
    }
}
