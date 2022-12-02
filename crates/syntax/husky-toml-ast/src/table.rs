use std::collections::BTreeMap;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TomlTable {
    data: BTreeMap<Word, TomlTableValue>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TomlTableValue {
    Table(TomlTable),
    Section(TomlSection),
    List(Vec<TomlTableValue>),
}
