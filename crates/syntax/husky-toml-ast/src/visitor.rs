use crate::*;

pub struct TomlAstVisitor<'a, A>
where
    A: TomlAst,
{
    data: Vec<&'a A>,
}
