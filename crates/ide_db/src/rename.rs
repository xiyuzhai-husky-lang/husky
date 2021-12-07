//! Rename infrastructure for husky
use std::fmt;

use common::*;

use file_db::{AnchoredPathBuf, FileID, FileRange};
use either::Either;
use hir::{AsAssocItem, FieldSource, HasSource, InFile, ModuleSource, Semantics};
use stdx::never;
use syntax::{ast, SyntaxKind};
use text_edit::{TextEdit, TextEditBuilder};

use crate::{
    defs::Definition,
    helpers::node_ext::expr_as_name_ref,
    search::FileReference,
    source_change::{FileSystemEdit, SourceChange},
    IdeDatabase,
};

pub type Result<T, E = RenameError> = std::result::Result<T, E>;

#[derive(Debug)]
pub struct RenameError(pub String);

impl fmt::Display for RenameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

#[macro_export]
macro_rules! _format_err {
    ($fmt:expr) => { RenameError(format!($fmt)) };
    ($fmt:expr, $($arg:tt)+) => { RenameError(format!($fmt, $($arg)+)) }
}
pub use _format_err as format_err;

#[macro_export]
macro_rules! _bail {
    ($($tokens:tt)*) => { return Err(format_err!($($tokens)*)) }
}
pub use _bail as bail;

impl Definition {
    pub fn rename(&self, sema: &Semantics<IdeDatabase>, new_name: &str) -> Result<SourceChange> {
        match *self {
            Definition::Module(module) => rename_mod(sema, module, new_name),
            Definition::BuiltinType(_) => {
                bail!("Cannot rename builtin type")
            }
            Definition::SelfType(_) => bail!("Cannot rename `Self`"),
            def => rename_reference(sema, def, new_name),
        }
    }

    /// Textual range of the identifier which will change when renaming this
    /// `Definition`. Note that some definitions, like buitin types, can't be
    /// renamed.
    pub fn range_for_rename(self, sema: &Semantics<IdeDatabase>) -> Option<FileRange> {
        todo!()
    }
}

fn rename_mod(
    sema: &Semantics<IdeDatabase>,
    module: hir::Module,
    new_name: &str,
) -> Result<SourceChange> {
    todo!()
}

fn rename_reference(
    sema: &Semantics<IdeDatabase>,
    mut def: Definition,
    new_name: &str,
) -> Result<SourceChange> {
    todo!()
}

pub fn source_edit_from_references(
    references: &[FileReference],
    def: Definition,
    new_name: &str,
) -> TextEdit {
    todo!()
}

fn source_edit_from_name(edit: &mut TextEditBuilder, name: &ast::Name, new_name: &str) -> bool {
    todo!()
}

fn source_edit_from_name_ref(
    edit: &mut TextEditBuilder,
    name_ref: &ast::NameRef,
    new_name: &str,
    def: Definition,
) -> bool {
    todo!()
}

fn source_edit_from_def(
    sema: &Semantics<IdeDatabase>,
    def: Definition,
    new_name: &str,
) -> Result<(FileID, TextEdit)> {
    todo!()
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum IdentifierKind {
    Ident,
    Lifetime,
    Underscore,
}

impl IdentifierKind {
    pub fn classify(new_name: &str) -> Result<IdentifierKind> {
        todo!()
    }
}
