use super::*;

pub(super) struct TomlSectionParseIter<'a> {
    db: &'a dyn TomlAstDb,
    toml_token_text: &'a TomlTokenSheet,
    line_groups: &'a [TomlGroup],
    current: usize,
    section_errors: &'a mut Vec<TomlAstError>,
}

impl<'a> Iterator for TomlSectionParseIter<'a> {
    type Item = TomlSectionAst;

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

#[salsa::derive_debug_with_db(db = TomlAstDb)]
enum TomlSectionIterState {
    Normal,
    Err,
}

impl<'a> TomlSectionParseIter<'a> {
    pub(super) fn new(
        db: &'a dyn TomlAstDb,
        toml_token_text: &'a TomlTokenSheet,
        line_groups: &'a [TomlGroup],
        section_errors: &'a mut Vec<TomlAstError>,
    ) -> Self {
        Self {
            db,
            toml_token_text,
            line_groups,
            current: 0,
            section_errors,
        }
    }
}

impl<'a> TomlSectionParseIter<'a> {
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

    fn next_section(
        &mut self,
        title_words: SmallVec<[Word; 2]>,
        kind: TomlSectionKind,
    ) -> TomlSectionAst {
        let mut key_value_pairs = vec![];
        while let Some((i, line_group)) = self.peek_indexed_line_group() {
            match line_group {
                TomlGroup::SectionTitle { .. } => break,
                TomlGroup::KeyValue(key, value) => key_value_pairs.push((i, *key, *value)),
                TomlGroup::Comment | TomlGroup::Err => (),
            }
            self.pass()
        }
        TomlSectionAst {
            title: TomlSectionTitle::new(self.db, title_words),
            kind,
            key_value_pairs,
        }
    }

    fn ignore_until_new_section(&mut self) -> Option<TomlSectionAst> {
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
