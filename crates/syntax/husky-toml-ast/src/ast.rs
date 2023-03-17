use crate::*;

pub trait TomlAst {}

impl TomlAst for TomlSectionAst {}

pub trait TomlSubAst<A>
where
    A: TomlAst,
{
}

impl TomlAstSheet {
    pub fn section_visitor<'a>(&'a self) -> TomlAstVisitor<'a, TomlSectionAst> {
        todo!()
    }
}
