use crate::*;

#[derive(Subcommand)]
pub(crate) enum SemanticTokensTestOrder {
    Misc,
}

impl SemanticTokensTestOrder {
    pub(crate) fn relative_path_str(&self) -> &'static str {
        match self {
            SemanticTokensTestOrder::Misc => "tests/analyzer/semantic_tokens/misc",
        }
    }
}
