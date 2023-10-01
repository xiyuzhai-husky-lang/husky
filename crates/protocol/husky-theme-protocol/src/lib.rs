use husky_token_protocol::TokenProtocol;

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
    pub fn get_field(&self, token_kind: TokenProtocol) -> &String {
        match token_kind {
            TokenProtocol::Attribute => &self.attribute,
            TokenProtocol::Comment => &self.comment,
            TokenProtocol::Keyword(_) => &self.keyword,
            TokenProtocol::Field => &self.field,
            TokenProtocol::Special => &self.special,
            TokenProtocol::Parameter => &self.parameter,
            TokenProtocol::Variable => &self.variable,
            TokenProtocol::ThisValue => &self.this_value,
            TokenProtocol::FrameVariable => &self.frame_variable,
            TokenProtocol::Entity(_) => &self.entity,
            TokenProtocol::ImplicitParameter => &self.implicit_parameter,
            TokenProtocol::EnumVariant => &self.enum_variant,
            TokenProtocol::Method => &self.method,
            TokenProtocol::Literal => &self.literal,
            TokenProtocol::HtmlTagKind => &self.html_tag_kind,
            TokenProtocol::WordPattern => &self.word_pattern,
            TokenProtocol::WordOpr => &self.word_opr,
            TokenProtocol::SelfType => &self.self_type,
            TokenProtocol::SelfValue => &self.self_value,
            TokenProtocol::HtmlFunctionIdent => &self.html_function_ident,
            TokenProtocol::HtmlPropertyIdent => &self.html_property_ident,
            TokenProtocol::SubmoduleIdent => &self.submodule_ident,
            TokenProtocol::Todo => &self.todo,
            TokenProtocol::Unreachable => &self.unreachable,
            TokenProtocol::Ident => &self.ident,
            TokenProtocol::Label => &self.label,
            TokenProtocol::Error => &self.error,
        }
    }
}
