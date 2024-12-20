#[derive(Debug, thiserror::Error)]
pub enum SnlMarkupError {
    #[error("SNL Markup Error: couldn't find pattern command in `{markup_content}`")]
    CoundntFindPatternCommand { markup_content: String },
    #[error("SNL Markup Error: couldn't find pattern arg command in `{markup_content}`")]
    CoundntFindPatternArgCommand { markup_content: String },
    #[error("SNL Markup Error: couldn't parse ident in `{markup_content}`")]
    CoundntParseIdent { markup_content: String },
}

pub type SnlMarkupResult<T> = Result<T, SnlMarkupError>;
