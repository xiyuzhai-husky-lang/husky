#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DiagnosticSeverity {
    Error,
    // We don't actually emit this one yet, but we should at some point.
    // Warning,
    WeakWarning,
}

impl Into<lsp_types::DiagnosticSeverity> for DiagnosticSeverity {
    fn into(self) -> lsp_types::DiagnosticSeverity {
        match self {
            DiagnosticSeverity::Error => lsp_types::DiagnosticSeverity::ERROR,
            DiagnosticSeverity::WeakWarning => lsp_types::DiagnosticSeverity::HINT,
        }
    }
}
