mod builtin;
mod intern;
mod kind;
mod module;
mod query;
mod table;

pub use builtin::BuiltinScope;
use file::FileId;
pub use intern::{InternScope, ScopeId, ScopeInterner};
pub use kind::ScopeKind;
pub use module::Module;
pub use query::{ScopeQuery, ScopeQueryStorage, ScopeSalsaQuery};
pub use table::ScopeTable;

use word::Identifier;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Scope {
    ident: Identifier,
    parent: ScopeParent,
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
