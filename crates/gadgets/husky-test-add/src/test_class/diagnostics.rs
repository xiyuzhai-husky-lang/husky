use crate::*;

#[derive(Subcommand)]
pub(crate) enum DiagnosticsTestOrder {
    Misc,
}

impl DiagnosticsTestOrder {
    pub(crate) fn relative_path_str(&self) -> &'static str {
        match self {
            DiagnosticsTestOrder::Misc => "tests/diagnostics/misc",
        }
    }
}
