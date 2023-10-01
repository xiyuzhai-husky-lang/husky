use husky_semantic_token_kind::SemanticTokenKind;

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
    pub fn get_field(&self, token_kind: SemanticTokenKind) -> &String {
        match token_kind {
            SemanticTokenKind::Attribute => &self.attribute,
            SemanticTokenKind::Comment => &self.comment,
            SemanticTokenKind::Keyword(_) => &self.keyword,
            SemanticTokenKind::Field => &self.field,
            SemanticTokenKind::Special => &self.special,
            SemanticTokenKind::Parameter => &self.parameter,
            SemanticTokenKind::Variable => &self.variable,
            SemanticTokenKind::ThisValue => &self.this_value,
            SemanticTokenKind::FrameVariable => &self.frame_variable,
            SemanticTokenKind::Entity(_) => &self.entity,
            SemanticTokenKind::ImplicitParameter => &self.implicit_parameter,
            SemanticTokenKind::EnumVariant => &self.enum_variant,
            SemanticTokenKind::Method => &self.method,
            SemanticTokenKind::Literal => &self.literal,
            SemanticTokenKind::HtmlTagKind => &self.html_tag_kind,
            SemanticTokenKind::WordPattern => &self.word_pattern,
            SemanticTokenKind::WordOpr => &self.word_opr,
            SemanticTokenKind::SelfType => &self.self_type,
            SemanticTokenKind::SelfValue => &self.self_value,
            SemanticTokenKind::HtmlFunctionIdent => &self.html_function_ident,
            SemanticTokenKind::HtmlPropertyIdent => &self.html_property_ident,
            SemanticTokenKind::SubmoduleIdent => &self.submodule_ident,
            SemanticTokenKind::Todo => &self.todo,
            SemanticTokenKind::Unreachable => &self.unreachable,
            SemanticTokenKind::Ident => &self.ident,
            SemanticTokenKind::Label => &self.label,
            SemanticTokenKind::Error => &self.error,
        }
    }
}
