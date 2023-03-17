use std::collections::BTreeMap;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TomlAstDb)]
pub struct TomlTable {
    data: BTreeMap<Word, TomlTableValue>,
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TomlAstDb)]
pub enum TomlTableValue {
    Table(TomlTable),
    Section(TomlSectionAstIdx),
    List(Vec<TomlTableValue>),
}

impl TomlTable {
    pub(crate) fn new(db: &dyn TomlAstDb, sections: &TomlSectionAstSheet) -> Self {
        let mut table = TomlTable {
            data: Default::default(),
        };
        for (idx, section) in sections.indexed_section_iter() {
            table.insert_section(db, idx, section)
        }
        table
    }

    fn insert_section<'a>(
        &mut self,
        db: &dyn TomlAstDb,
        idx: TomlSectionAstIdx,
        section: &TomlSectionAst,
    ) {
        match section.kind() {
            TomlSectionKind::Normal => {
                let title_words = section.title().words(db);
                if title_words.len() == 1 {
                    let key = title_words[0];
                    if self.data.contains_key(&key) {
                        todo!()
                    }
                    assert!(self
                        .data
                        .insert(key, TomlTableValue::Section(idx))
                        .is_none());
                } else {
                    todo!()
                }
            }
            TomlSectionKind::Scattered => todo!(),
        }
    }
}
