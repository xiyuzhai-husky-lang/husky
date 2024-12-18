mod visitor;

pub use self::visitor::*;

use std::collections::BTreeMap;

use crate::*;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct TomlTable {
    data: BTreeMap<BaseCoword, TomlTableValue>,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub enum TomlTableValue {
    Table(TomlTable),
    Section(TomlSectionIdx),
    List(Vec<TomlTableValue>),
}

impl TomlTable {
    pub(crate) fn new(db: &::salsa::Db, sections: &TomlSectionSheet) -> Self {
        let mut table = TomlTable {
            data: Default::default(),
        };
        for (idx, section) in sections.indexed_section_iter() {
            table.insert_section(db, idx, section)
        }
        table
    }

    pub(crate) fn get(&self, key: BaseCoword) -> Option<&TomlTableValue> {
        self.data.get(&key)
    }

    fn insert_section<'a>(
        &mut self,
        _db: &::salsa::Db,
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
