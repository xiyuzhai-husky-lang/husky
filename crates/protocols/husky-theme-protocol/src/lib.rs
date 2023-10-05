use husky_token_protocol::TokenKindProtocol;

pub struct Theme {
    semantic_token: SemanticTokenTheme,
}

#[derive(Debug, Clone)]
pub struct SemanticTokenTheme {
    pub attribute: String,
    pub comment: String,
    pub keyword: String,
    pub field: String,
    pub special: String,
    pub parameter: String,
    pub variable: String,
    pub this_value: String,
    pub frame_variable: String,
    pub entity: String,
    pub implicit_parameter: String,
    pub enum_variant: String,
    pub method: String,
    pub literal: String,
    pub html_tag_kind: String,
    pub word_pattern: String,
    pub word_opr: String,
    pub self_type: String,
    pub self_value: String,
    pub html_function_ident: String,
    pub html_property_ident: String,
    pub submodule_ident: String,
    pub todo: String,
    pub unreachable: String,
    pub ident: String,
    pub label: String,
    pub error: String,
}

impl SemanticTokenTheme {
    pub fn get_field(&self, token_kind: TokenKindProtocol) -> &String {
        match token_kind {
            TokenKindProtocol::Attribute => &self.attribute,
            TokenKindProtocol::Comment => &self.comment,
            TokenKindProtocol::Keyword(_) => &self.keyword,
            TokenKindProtocol::Field => &self.field,
            TokenKindProtocol::Special => &self.special,
            TokenKindProtocol::Parameter => &self.parameter,
            TokenKindProtocol::Variable => &self.variable,
            TokenKindProtocol::ThisValue => &self.this_value,
            TokenKindProtocol::FrameVariable => &self.frame_variable,
            TokenKindProtocol::Entity(_) => &self.entity,
            TokenKindProtocol::ImplicitParameter => &self.implicit_parameter,
            TokenKindProtocol::EnumVariant => &self.enum_variant,
            TokenKindProtocol::Method => &self.method,
            TokenKindProtocol::Literal => &self.literal,
            TokenKindProtocol::HtmlTagKind => &self.html_tag_kind,
            TokenKindProtocol::WordPattern => &self.word_pattern,
            TokenKindProtocol::WordOpr => &self.word_opr,
            TokenKindProtocol::SelfType => &self.self_type,
            TokenKindProtocol::SelfValue => &self.self_value,
            TokenKindProtocol::HtmlFunctionIdent => &self.html_function_ident,
            TokenKindProtocol::HtmlPropertyIdent => &self.html_property_ident,
            TokenKindProtocol::SubmoduleIdent => &self.submodule_ident,
            TokenKindProtocol::Todo => &self.todo,
            TokenKindProtocol::Unreachable => &self.unreachable,
            TokenKindProtocol::Ident => &self.ident,
            TokenKindProtocol::Label => &self.label,
            TokenKindProtocol::Error => &self.error,
        }
    }
}
