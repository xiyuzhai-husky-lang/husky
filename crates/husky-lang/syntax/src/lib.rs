use std::sync::Arc;

use common::*;

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
    let lexed = this.tokenized_text(id);
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
