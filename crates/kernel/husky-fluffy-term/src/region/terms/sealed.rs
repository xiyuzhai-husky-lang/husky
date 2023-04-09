use super::*;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct SolidTerms {}

#[derive(Debug, PartialEq, Eq)]
pub struct SolidTermEntry {
    src: SolidTermSource,
    data: SolidTermData,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SolidTerm {}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SolidTermSource {}

#[derive(Debug, PartialEq, Eq)]
pub enum SolidTermData {}
