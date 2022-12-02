use crate::{TomlAstError, TomlExprIdx, TomlLineGroup};
use husky_toml_token_text::TomlTokenText;
use husky_word::Word;

#[derive(Debug, PartialEq, Eq)]
pub struct TomlSection {
    title: Vec<Word>,
    is_scattered: bool,
    key_value_pairs: Vec<(usize, Word, TomlExprIdx)>,
}

impl TomlSection {
    pub(crate) fn collect_from_line_groups(
        toml_token_text: &TomlTokenText,
        line_groups: &[TomlLineGroup],
    ) -> (Vec<Self>, Vec<TomlAstError>) {
        let mut section_errors = vec![];
        (
            TomlSectionIter::new(toml_token_text, line_groups, &mut section_errors).collect(),
            section_errors,
        )
    }
}

struct TomlSectionIter<'a> {
    toml_token_text: &'a TomlTokenText,
    line_groups: &'a [TomlLineGroup],
    current: usize,
    section_errors: &'a mut Vec<TomlAstError>,
}

pub enum TomlSectionIterState {
    Normal,
    Err,
}

impl<'a> TomlSectionIter<'a> {
    fn new(
        toml_token_text: &'a TomlTokenText,
        line_groups: &'a [TomlLineGroup],
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
            TomlLineGroup::SectionTitle { .. } => Some(self.next_section()),
            TomlLineGroup::KeyValue(_, _) => {
                self.section_errors
                    .push(TomlAstError::MisplacedKeyValue(line_group_index));
                self.next()
            }
            TomlLineGroup::Comment => todo!(),
            TomlLineGroup::Err => self.ignore_until_new_section(),
        }
    }
}

impl<'a> TomlSectionIter<'a> {
    fn next_indexed_line_group(&mut self) -> Option<(usize, &'a TomlLineGroup)> {
        if self.current < self.line_groups.len() {
            let index = self.current;
            self.current += 1;
            Some((index, &self.line_groups[self.current]))
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

    fn next_section(&mut self) -> TomlSection {
        todo!()
    }

    fn ignore_until_new_section(&mut self) -> Option<TomlSection> {
        while let Some(line_group) = self.peek_line_group() {
            match line_group {
                TomlLineGroup::SectionTitle { .. } => break,
                TomlLineGroup::KeyValue(_, _) | TomlLineGroup::Comment | TomlLineGroup::Err => {
                    self.next();
                }
            }
        }
        self.next()
    }
}
