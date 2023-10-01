use husky_entity_kind::EntityKind;
use husky_keyword_kind::KeywordKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemanticTokenKind {
    Attribute,
    Comment,
    Keyword(KeywordKind),
    Field,
    Special,
    Parameter,
    Variable,
    ThisValue,
    FrameVariable,
    Entity(EntityKind),
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
