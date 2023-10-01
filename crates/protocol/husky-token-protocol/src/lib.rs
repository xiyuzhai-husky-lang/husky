use husky_entity_protocol::EntityProtocol;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenProtocol {
    Attribute,
    Comment,
    Keyword(KeywordProtocol),
    Field,
    Special,
    Parameter,
    Variable,
    ThisValue,
    FrameVariable,
    Entity(EntityProtocol),
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
pub enum KeywordProtocol {
    ControlFlow,
    Other,
}
