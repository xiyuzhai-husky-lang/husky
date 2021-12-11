#![allow(warnings)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Module {
    pub file_id: file::FileId,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Type {
    module: Module,
    sentence_id: u32,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct EntityTable {
    table: Vec<(Symbol, Entity)>,
}

impl EntityTable {
    pub fn submodules(&self) -> Vec<Module> {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Symbol {
    Opr(Opr),
    Keyword(word::Keyword),
    Identifier(word::Identifier),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Opr {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Entity {
    Module(file::FileId),
    Type {
        module: file::FileId,
        sentence_id: u32,
    },
}
