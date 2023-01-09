use crate::*;

#[derive(Debug)]
pub struct ImplBlock {
    kind: ImplBlockKind,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ImplBlockKind {
    Type(TypePath),
    Todo,
}

pub type ImplBlockArena = Arena<ImplBlock>;
pub type ImplBlockIdx = ArenaIdx<ImplBlock>;
pub type ImplBlockIdxRange = ArenaIdxRange<ImplBlock>;

impl ImplBlock {
    pub(crate) fn parse_from_token_group<'a>(crate_prelude: CratePrelude<'a>) -> Self {
        Self {
            kind: ImplBlockKind::Todo,
        }
    }
}
