mod parse_iter;
mod title;
mod visitor;

pub use self::parse_iter::*;
pub use self::title::*;
pub use self::visitor::*;

use crate::*;
use husky_word::Word;
use idx_arena::{Arena, ArenaIdx};
use smallvec::SmallVec;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TomlAstDb)]
pub struct TomlSectionAstSheet {
    arena: Arena<TomlSectionAst>,
    errors: Vec<TomlAstError>,
}

pub type TomlSectionAstArena = Arena<TomlSectionAst>;
pub type TomlSectionAstIdx = ArenaIdx<TomlSectionAst>;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TomlAstDb)]
pub struct TomlSectionAst {
    title: TomlSectionTitle,
    kind: TomlSectionKind,
    key_value_pairs: Vec<(usize, Word, Option<TomlExprIdx>)>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = TomlAstDb)]
pub enum TomlSectionKind {
    Normal,
    Scattered,
}

impl TomlSectionAst {
    pub fn kind(&self) -> TomlSectionKind {
        self.kind
    }

    pub fn title(&self) -> TomlSectionTitle {
        self.title
    }
}

impl TomlSectionAstSheet {
    pub(crate) fn parse_collect(
        db: &dyn TomlAstDb,
        toml_token_text: &TomlTokenSheet,
        line_groups: &[TomlGroup],
    ) -> Self {
        let mut errors = vec![];
        Self {
            arena: TomlSectionParseIter::new(db, toml_token_text, line_groups, &mut errors)
                .collect(),
            errors,
        }
    }

    pub fn errors(&self) -> &[TomlAstError] {
        &self.errors
    }

    pub fn indexed_section_iter(
        &self,
    ) -> impl Iterator<Item = (TomlSectionAstIdx, &TomlSectionAst)> {
        self.arena.indexed_iter()
    }
}
