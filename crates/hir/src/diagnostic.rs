use common::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Severity {
    Error,
    // We don't actually emit this one yet, but we should at some point.
    // Warning,
    WeakWarning,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DiagnosticVariant {
    Todo,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Diagnostic {
    pub severity: Severity,
    pub range: TextRange,
    pub message: String,
    pub code: String,
    pub variant: DiagnosticVariant,
}

impl Diagnostic {
    pub fn todo() -> Diagnostic {
        Diagnostic {
            severity: Severity::Error,
            range: TextRange::new(TextSize::from(0), TextSize::from(10)),
            message: "todo".into(),
            code: "todo".into(),
            variant: DiagnosticVariant::Todo,
        }
    }
}

pub struct FileDiagnostics {
    pub file_id: vfs::SourceFileId,
    pub diagnostics: Vec<Diagnostic>,
}
