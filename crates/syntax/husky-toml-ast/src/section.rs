use crate::*;
use husky_word::Word;
use idx_arena::{Arena, ArenaIdx};
use smallvec::SmallVec;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TomlAstDb)]
pub struct TomlSectionSheet {
    arena: Arena<TomlSection>,
    errors: Vec<TomlAstError>,
}

pub type TomlSectionArena = Arena<TomlSection>;
pub type TomlSectionIdx = ArenaIdx<TomlSection>;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TomlAstDb)]
pub struct TomlSection {
    title: SmallVec<[Word; 2]>,
    kind: TomlSectionKind,
    key_value_pairs: Vec<(usize, Word, Option<TomlExprIdx>)>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = TomlAstDb)]
pub enum TomlSectionKind {
    Normal,
    Scattered,
}

impl TomlSection {
    pub fn kind(&self) -> TomlSectionKind {
        self.kind
    }

    pub fn title(&self) -> &[Word] {
        &self.title
    }
}

impl TomlSectionSheet {
    pub(crate) fn new(toml_token_text: &TomlTokenSheet, line_groups: &[TomlGroup]) -> Self {
        let mut errors = vec![];
        Self {
            arena: TomlSectionIter::new(toml_token_text, line_groups, &mut errors).collect(),
            errors,
        }
    }

    pub fn errors(&self) -> &[TomlAstError] {
        &self.errors
    }

    pub fn indexed_section_iter(&self) -> impl Iterator<Item = (TomlSectionIdx, &TomlSection)> {
        self.arena.indexed_iter()
    }
}

struct TomlSectionIter<'a> {
    toml_token_text: &'a TomlTokenSheet,
    line_groups: &'a [TomlGroup],
    current: usize,
    section_errors: &'a mut Vec<TomlAstError>,
}

#[salsa::derive_debug_with_db(db = TomlAstDb)]
pub enum TomlSectionIterState {
    Normal,
    Err,
}

impl<'a> TomlSectionIter<'a> {
    fn new(
        toml_token_text: &'a TomlTokenSheet,
        line_groups: &'a [TomlGroup],
        section_errors: &'a mut Vec<TomlAstError>,
    ) -> Self {
        Self {
            toml_token_text,
            line_groups,
            current: 0,
            section_errors,
        }
    }
}

impl<'a> Iterator for TomlSectionIter<'a> {
    type Item = TomlSection;

    fn next(&mut self) -> Option<Self::Item> {
        let (line_group_index, line_group) = self.next_indexed_line_group()?;
        match line_group {
            TomlGroup::SectionTitle { title, kind } => {
                Some(self.next_section(title.clone(), *kind))
            }
            TomlGroup::KeyValue(_, _) => {
                self.section_errors
                    .push(TomlAstError::MisplacedKeyValue(line_group_index));
                self.next()
            }
            TomlGroup::Comment => self.next(),
            TomlGroup::Err => self.ignore_until_new_section(),
        }
    }
}

impl<'a> TomlSectionIter<'a> {
    fn next_indexed_line_group(&mut self) -> Option<(usize, &'a TomlGroup)> {
        if self.current < self.line_groups.len() {
            let index = self.current;
            self.current += 1;
            Some((index, &self.line_groups[index]))
        } else {
            None
        }
    }

    fn peek_indexed_line_group(&mut self) -> Option<(usize, &'a TomlGroup)> {
        if self.current < self.line_groups.len() {
            Some((self.current, &self.line_groups[self.current]))
        } else {
            None
        }
    }

    fn peek_line_group(&mut self) -> Option<&'a TomlGroup> {
        if self.current < self.line_groups.len() {
            Some(&self.line_groups[self.current])
        } else {
            None
        }
    }

    fn pass(&mut self) {
        self.current += 1;
    }

    fn next_section(&mut self, title: SmallVec<[Word; 2]>, kind: TomlSectionKind) -> TomlSection {
        let mut key_value_pairs = vec![];
        while let Some((i, line_group)) = self.peek_indexed_line_group() {
            match line_group {
                TomlGroup::SectionTitle { .. } => break,
                TomlGroup::KeyValue(key, value) => key_value_pairs.push((i, *key, *value)),
                TomlGroup::Comment | TomlGroup::Err => (),
            }
            self.pass()
        }
        TomlSection {
            title,
            kind,
            key_value_pairs,
        }
    }

    fn ignore_until_new_section(&mut self) -> Option<TomlSection> {
        while let Some(line_group) = self.peek_line_group() {
            match line_group {
                TomlGroup::SectionTitle { .. } => break,
                TomlGroup::KeyValue(_, _) | TomlGroup::Comment | TomlGroup::Err => {
                    self.pass();
                }
            }
        }
        self.next()
    }
}
