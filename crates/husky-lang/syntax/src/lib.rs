use std::{marker::PhantomData, sync::Arc};

use common::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseResult<T> {
    _ty: PhantomData<fn() -> T>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SingleFileParseTree {}

pub mod ast;
pub struct SyntaxElementChildren {}
pub struct SyntaxToken {}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyntaxKind {}
pub struct SyntaxElement {}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyntaxNode {}
pub struct SyntaxNodePtr {}
pub mod token;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GreenNode {}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Direction {
    Next,
    Prev,
}
pub struct TokenAtOffset<T> {
    phantom: std::marker::PhantomData<T>,
}
pub struct NodeOrToken<S, T> {
    phantoms: std::marker::PhantomData<S>,
    phantomt: std::marker::PhantomData<T>,
}

#[salsa::query_group(SyntaxQueryStorage)]
pub trait SyntaxSalsaQuery: file::FileQuery + lex::LexQuery {
    fn subentities(&self, id: file::FileId) -> Result<Arc<EntityTable>, SyntaxError> {}
}

use entity::*;
mod entity;

fn subentities(
    this: &dyn SyntaxSalsaQuery,
    id: file::FileId,
) -> Result<Arc<EntityTable>, SyntaxError> {
    let lexed = this.lex_result(id);
    ep!(lexed);
    todo!()
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SyntaxError {
    FileNotExist,
}

impl std::fmt::Debug for SyntaxError {
    fn fmt(&self, f: &mut common::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FileNotExist => write!(f, "FileNotExist"),
        }
    }
}
impl std::fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut common::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FileNotExist => write!(f, "FileNotExist"),
        }
    }
}

impl std::error::Error for SyntaxError {}

pub trait SyntaxQuery: SyntaxSalsaQuery {
    fn all_modules(&self) -> Vec<Module> {
        self.all_main_files()
            .iter()
            .map(|id| self.collect_submodules(*id))
            .flatten()
            .collect()
    }

    fn collect_submodules(&self, id: file::FileId) -> Vec<Module> {
        self.subentities(id)
            .expect("bug")
            .submodules()
            .into_iter()
            .map(|module| self.collect_submodules(module.file_id))
            .flatten()
            .collect()
    }
}

#[cfg(test)]
mod tests;
