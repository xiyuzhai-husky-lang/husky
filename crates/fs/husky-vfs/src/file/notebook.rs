use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct Notebook {
    sections: Vec<NotebookSection>,
    nodes: NotebookAst,
}

#[salsa::tracked(db = VfsDb, jar = VfsJar)]
pub struct NotebookSection {
    #[id]
    id: NotebookSectionId,
    #[return_ref]
    asts: Vec<NotebookAst>,
    #[return_ref]
    subsections: Vec<NotebookSubsection>,
}

#[salsa::interned(db = VfsDb, jar = VfsJar)]
pub struct NotebookSectionId {
    #[return_ref]
    title: String,
}

#[salsa::tracked(db = VfsDb, jar = VfsJar)]
pub struct NotebookSubsection {
    #[id]
    id: NotebookSubsectionId,
    #[return_ref]
    asts: Vec<NotebookAst>,
}

#[salsa::interned(db = VfsDb, jar = VfsJar)]
pub struct NotebookSubsectionId {
    #[return_ref]
    title: String,
}

#[salsa::tracked(db = VfsDb, jar = VfsJar)]
pub struct NotebookAst {
    #[id]
    id: NotebookAstId,
    #[return_ref]
    data: NotebookAstData,
    #[return_ref]
    children: Vec<NotebookAst>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NotebookAstId {
    PlainText,
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = VfsDb)]
pub enum NotebookAstData {
    PlainText(String),
    Code(String),
    MathFormula,
    Plot,
    Image,
}
