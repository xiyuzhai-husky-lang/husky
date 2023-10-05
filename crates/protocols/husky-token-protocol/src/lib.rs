use husky_entity_protocol::EntityClass;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKindProtocol {
    Attribute,
    Comment,
    Keyword(KeywordKindProtocol),
    Field,
    Special,
    Parameter,
    Variable,
    ThisValue,
    FrameVariable,
    Entity(EntityClass),
    ImplicitParameter,
    EnumVariant,
    Method,
    Literal,
    HtmlTagKind,
    WordPattern,
    WordOpr,
    SelfType,
    SelfValue,
    HtmlFunctionIdent,
    HtmlPropertyIdent,
    SubmoduleIdent,
    Todo,
    Unreachable,
    Ident,
    Label,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeywordKindProtocol {
    ControlFlow,
    Other,
}
