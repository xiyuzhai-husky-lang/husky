use crate::*;

#[derive(Subcommand)]
pub(crate) enum QualifiedTysTestOrder {
    Misc,
}

impl QualifiedTysTestOrder {
    pub(crate) fn relative_path_str(&self) -> &'static str {
        match self {
            QualifiedTysTestOrder::Misc => "tests/qualified-tys/misc",
        }
    }
}
