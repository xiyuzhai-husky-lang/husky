mod alias;
mod builtin;
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
pub use query::{ScopeQuery, ScopeQueryStorage, ScopeSalsaQuery};
pub use subscope::SubscopeTable;

use word::Identifier;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Scope {
    ident: Identifier,
    parent: ScopeParent,
    generic_arguments: Option<Vec<ScopeId>>,
}

impl Scope {
    pub fn new(
        ident: Identifier,
        parent: ScopeParent,
        generic_arguments: Option<Vec<ScopeId>>,
    ) -> Scope {
        Scope {
            ident,
            parent,
            generic_arguments,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScopeParent {
    Scope(ScopeId),
    Package(FileId),
    Root,
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

mod error;
