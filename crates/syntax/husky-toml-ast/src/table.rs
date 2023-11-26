mod visitor;

pub use self::visitor::*;

use std::collections::BTreeMap;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = TomlAstDb, jar = TomlAstJar)]
pub struct TomlTable {
    data: BTreeMap<Coword, TomlTableValue>,
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = TomlAstDb, jar = TomlAstJar)]
pub enum TomlTableValue {
    Table(TomlTable),
    Section(TomlSectionIdx),
    List(Vec<TomlTableValue>),
}

impl TomlTable {
    pub(crate) fn new(db: &dyn TomlAstDb, sections: &TomlSectionSheet) -> Self {
        let mut table = TomlTable {
            data: Default::default(),
        };
        for (idx, section) in sections.indexed_section_iter() {
            table.insert_section(db, idx, section)
        }
        table
    }

    pub(crate) fn get(&self, key: Coword) -> Option<&TomlTableValue> {
        self.data.get(&key)
    }

    fn insert_section<'a>(
        &mut self,
        _db: &dyn TomlAstDb,
        idx: TomlSectionIdx,
        section: &TomlSection,
    ) {
        match section.kind() {
            TomlSectionKind::Normal => {
                let title = section.title();
                if title.len() == 1 {
                    let key = title[0];
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
