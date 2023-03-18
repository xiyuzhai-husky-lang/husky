use super::*;

pub(super) struct TomlSectionParseIter<'a> {
    db: &'a dyn TomlAstDb,
    toml_token_text: &'a TomlTokenSheet,
    line_groups: &'a [TomlLineGroup],
    current: usize,
    errors: &'a mut Vec<TomlAstError>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct TomlLineGroupIdx(usize);

impl<'a> Iterator for TomlSectionParseIter<'a> {
    type Item = TomlSection;

    fn next(&mut self) -> Option<Self::Item> {
        let (line_group_index, line_group) = self.next_indexed_line_group()?;
        match line_group {
            TomlLineGroup::SectionTitle { title, kind } => {
                Some(self.next_section(title.clone(), *kind))
            }
            TomlLineGroup::KeyValue(_, _) => {
                self.errors
                    .push(TomlAstError::MisplacedKeyValue(line_group_index));
                self.next()
            }
            TomlLineGroup::Comment => self.next(),
            TomlLineGroup::Err => self.ignore_until_new_section(),
        }
    }
}

#[salsa::derive_debug_with_db(db = TomlAstDb)]
enum TomlSectionIterState {
    Normal,
    Err,
}

impl<'a> TomlSectionParseIter<'a> {
    pub(super) fn new(
        db: &'a dyn TomlAstDb,
        toml_token_text: &'a TomlTokenSheet,
        line_groups: &'a [TomlLineGroup],
        section_errors: &'a mut Vec<TomlAstError>,
    ) -> Self {
        Self {
            db,
            toml_token_text,
            line_groups,
            current: 0,
            errors: section_errors,
        }
    }
}

impl<'a> TomlSectionParseIter<'a> {
    fn next_indexed_line_group(&mut self) -> Option<(usize, &'a TomlLineGroup)> {
        if self.current < self.line_groups.len() {
            let index = self.current;
            self.current += 1;
            Some((index, &self.line_groups[index]))
        } else {
            None
        }
    }

    fn peek_indexed_line_group(&mut self) -> Option<(TomlLineGroupIdx, &'a TomlLineGroup)> {
        if self.current < self.line_groups.len() {
            Some((
                TomlLineGroupIdx(self.current),
                &self.line_groups[self.current],
            ))
        } else {
            None
        }
    }

    fn peek_line_group(&mut self) -> Option<&'a TomlLineGroup> {
        if self.current < self.line_groups.len() {
            Some(&self.line_groups[self.current])
        } else {
            None
        }
    }

    fn pass(&mut self) {
        self.current += 1;
    }

    fn next_section(
        &mut self,
        title_words: SmallVec<[Word; 2]>,
        kind: TomlSectionKind,
    ) -> TomlSection {
        let mut entries = VecMap::default();
        while let Some((i, line_group)) = self.peek_indexed_line_group() {
            match line_group {
                TomlLineGroup::SectionTitle { .. } => break,
                TomlLineGroup::KeyValue(key, value) => {
                    let new_entry = TomlSectionEntry {
                        line_group_idx: i,
                        key: *key,
                        value: *value,
                    };
                    if let Err(e) = entries.insert_new(new_entry) {
                        todo!()
                    }
                }
                TomlLineGroup::Comment | TomlLineGroup::Err => (),
            }
            self.pass()
        }
        TomlSection {
            title: TomlSectionTitle::new(self.db, title_words),
            kind,
            entries,
        }
    }

    fn ignore_until_new_section(&mut self) -> Option<TomlSection> {
        while let Some(line_group) = self.peek_line_group() {
            match line_group {
                TomlLineGroup::SectionTitle { .. } => break,
                TomlLineGroup::KeyValue(_, _) | TomlLineGroup::Comment | TomlLineGroup::Err => {
                    self.pass();
                }
            }
        }
        self.next()
    }
}
