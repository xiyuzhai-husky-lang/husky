use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct Notebook {
    sections: Vec<NotebookSection>,
    nodes: LiveAst,
}

#[salsa::tracked(db = VfsDb, jar = VfsJar)]
pub struct NotebookSection {
    #[id]
    id: NotebookSectionId,
    #[return_ref]
    asts: Vec<LiveAst>,
    #[return_ref]
    subsections: Vec<NotebookSubsection>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NotebookSectionId {}

#[salsa::tracked(db = VfsDb, jar = VfsJar)]
pub struct NotebookSubsection {
    #[id]
    id: NotebookSubsectionId,
    #[return_ref]
    asts: Vec<LiveAst>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NotebookSubsectionId {}

#[salsa::tracked(db = VfsDb, jar = VfsJar)]
pub struct LiveAst {
    #[return_ref]
    data: LiveAstData,
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = VfsDb)]
pub enum LiveAstData {
    Hello,
}
