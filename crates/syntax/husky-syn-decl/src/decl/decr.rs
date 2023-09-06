use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum DecrSynNodeDecl {}

impl DecrSynNodeDecl {
    pub fn ast_idx(self, db: &dyn SynDeclDb) -> AstIdx {
        todo!()
    }
}
