use crate::*;

pub trait TomlAst: Sized {
    type Key: Copy;
    type Visitor<'a>: VisitTomlAst<'a, Ast = Self>;
}

pub trait VisitTomlAst<'a>: Sized {
    type Ast;
}

impl TomlAst for TomlSection {
    type Key = TomlSectionTitle;

    type Visitor<'a> = TomlSectionAstVisitor<'a>;
}

pub trait TomlSubAst<A>
where
    A: TomlAst,
{
}

pub type TomlAstVisitor<'a, A: TomlAst> = <A as TomlAst>::Visitor<'a>;
