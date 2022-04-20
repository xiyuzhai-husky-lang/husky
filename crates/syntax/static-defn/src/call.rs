use crate::*;
use vm::*;

#[derive(Debug, PartialEq, Eq)]
pub struct StaticCallDefn {
    pub generic_placeholders: &'static [StaticGenericPlaceholder],
    pub inputs: Vec<StaticInputPlaceholder>,
    pub output_ty: &'static str,
    pub output_contract: OutputContract,
    pub linkage: Linkage,
}

#[derive(Debug, PartialEq, Eq)]
pub struct StaticInputPlaceholder {
    pub contract: InputContract,
    pub ty: &'static str,
    pub name: &'static str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct StaticGenericPlaceholder {
    pub name: &'static str,
    pub variant: StaticGenericPlaceholderVariant,
}

#[derive(Debug, PartialEq, Eq)]
pub enum StaticGenericPlaceholderVariant {
    Const,
    Type { traits: &'static [&'static str] },
}
