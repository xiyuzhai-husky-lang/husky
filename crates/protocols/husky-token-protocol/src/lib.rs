use enum_index::IsEnumIndex;
use husky_entity_protocol::EntityClass;

#[derive(Debug, Clone, Copy, PartialEq, Eq, IsEnumIndex)]
pub enum TokenClass {
    Attribute,
    Comment,
    ControlFlowKeyword,
    OtherKeyword,
    Field,
    Special,
    Parameter,
    Variable,
    FrameVariable,
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
    WordPattern,
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
            TokenClass::Special => "special character or symbol token",
            TokenClass::Parameter => "parameter token",
            TokenClass::Variable => "variable token",
            TokenClass::FrameVariable => "frame variable token",
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
            TokenClass::WordPattern => "word pattern token",
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
            EntityClass::Module => todo!(),
            EntityClass::Type => todo!(),
            EntityClass::FunctionFn => todo!(),
            EntityClass::FunctionGn => todo!(),
            EntityClass::AliasType => todo!(),
            EntityClass::Val => todo!(),
            EntityClass::Trait => todo!(),
            EntityClass::TypeVariant => todo!(),
            EntityClass::MethodFn => todo!(),
            EntityClass::MethodGn => todo!(),
            EntityClass::AssociatedFunctionFn => todo!(),
            EntityClass::AssociatedFunctionGn => todo!(),
            EntityClass::MemoizedField => todo!(),
            EntityClass::AssociatedVal => todo!(),
            EntityClass::AssociatedType => todo!(),
            EntityClass::ImplBlock => todo!(),
            EntityClass::Attr => todo!(),
        }
    }
}
