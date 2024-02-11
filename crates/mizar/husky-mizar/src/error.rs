use crate::parser::{try_to_line_col, ParseError};
use crate::types::{Article, DirectiveKind, Formula, Position};
use crate::{LocalContext, MizGlobal, MizPath};
use std::path::{Path, PathBuf};

#[derive(PartialEq, Eq)]
enum Severity {
    Error,
    #[allow(unused)]
    Warning,
}

impl ParseError {
    pub fn report(self, path: &Path) {
        if let Some(pos) = self.pos() {
            if let Ok((line, col)) = try_to_line_col(path, pos) {
                eprintln!("{}:{line}:{col}: error: {self}", path.to_string_lossy())
            } else {
                eprintln!("{}: index {pos}: error: {self}", path.to_string_lossy())
            }
        } else {
            eprintln!("{}: error: {self}", path.to_string_lossy())
        }
    }
}

pub fn report_accom_warning(kind: DirectiveKind, path: PathBuf, art: Article, pos: Position) {
    eprintln!(
        "{file}:{pos:?}: warning: {kind} for {art} not found or empty (looked in {path})",
        file = MizPath { art }
            .to_path(true, false, "miz")
            .to_string_lossy(),
        kind = kind.name(),
        path = path.to_string_lossy()
    );
}

#[derive(Debug)]
pub enum MizError {
    UnexpectedPragma(String),
    IterEqualityNotAnEquality(Box<Formula>),
}

impl MizError {
    pub fn report(self, art: Article, pos: Position, _g: &MizGlobal, lc: &LocalContext) -> bool {
        let severity = Severity::Error;
        let msg = match &self {
            MizError::UnexpectedPragma(pragma) => format!("unknown pragma '{pragma}'"),
            MizError::IterEqualityNotAnEquality(f) => format!("not an equality: {}", lc.pp(f)),
        };
        let file = MizPath { art }.to_path(true, false, "miz");
        let sev = match severity {
            Severity::Error => "error",
            Severity::Warning => "warning",
        };
        eprintln!("{}:{pos:?}: {sev}: {msg}", file.to_string_lossy());
        severity == Severity::Error
    }
}
