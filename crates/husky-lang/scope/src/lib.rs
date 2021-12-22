mod alias;
mod builtin;
mod error;
mod intern;
mod kind;
mod module;
mod query;
mod subscope;

pub use alias::ScopeAliasTable;
pub use builtin::BuiltinScope;
pub use error::{def::ScopeDefError, ScopeError, ScopeResult, ScopeResultArc};
use file::FileId;
pub use intern::{new_scope_interner, InternScope, ScopeId, ScopeInterner};
pub use kind::ScopeKind;
pub use module::Module;
pub use query::{ModuleFromFileError, ScopeQuery, ScopeQueryStorage, ScopeSalsaQuery};
pub use subscope::SubscopeTable;

use word::{BuiltinIdentifier, Identifier, UserDefinedIdentifier};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Scope {
    path: ScopePath,
    // ident: Identifier,
    // parent: ScopeParent,
    pub generic_arguments: Option<Vec<ScopeId>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScopePath {
    Builtin(BuiltinIdentifier),
    Package(FileId, UserDefinedIdentifier),
    ChildScope(ScopeId, UserDefinedIdentifier),
}

impl Scope {
    pub fn package(main_file: FileId, ident: UserDefinedIdentifier) -> Self {
        Scope {
            path: ScopePath::Package(main_file, ident),
            generic_arguments: None,
        }
    }
    pub fn child_scope(
        parent_scope: ScopeId,
        ident: UserDefinedIdentifier,
        generic_arguments: Option<Vec<ScopeId>>,
    ) -> Scope {
        Scope {
            path: ScopePath::ChildScope(parent_scope, ident),
            generic_arguments,
        }
    }

    pub fn builtin(ident: BuiltinIdentifier, generic_arguments: Option<Vec<ScopeId>>) -> Scope {
        Scope {
            path: ScopePath::Builtin(ident),
            generic_arguments,
        }
    }
}

impl From<BuiltinIdentifier> for Scope {
    fn from(ident: BuiltinIdentifier) -> Self {
        Self::builtin(ident, None)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScopeSource {
    Builtin(&'static [(Identifier, ScopeKind)]),
    WithinModule {
        file_id: FileId,
        token_group_index: usize, // None means the whole file
    },
    Module {
        file_id: FileId,
    },
}

impl ScopeSource {
    pub fn from_file(file_id: FileId, token_group_index: usize) -> ScopeSource {
        ScopeSource::WithinModule {
            file_id,
            token_group_index: token_group_index,
        }
    }
}
