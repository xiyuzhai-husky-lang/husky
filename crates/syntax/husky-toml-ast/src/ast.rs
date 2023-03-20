use crate::*;

pub trait TomlAst: Sized {
    type Visitor<'a>;
}

impl TomlAst for TomlTable {
    type Visitor<'a> = TomlTableVisitor<'a>;
}

impl TomlAst for TomlSection {
    type Visitor<'a> = TomlSectionVisitor<'a>;
}

pub trait TomlSubAst<A>
where
    A: TomlAst,
{
}

pub type TomlVisitor<'a, A: TomlAst> = <A as TomlAst>::Visitor<'a>;

pub struct TomlParser<'a, 'b, Context: TomlParseContext, A: TomlAst> {
    db: &'a Context::Db<'a>,
    toml_ast_visitor: TomlVisitor<'a, A>,
    menu: &'a Context::Menu,
    errors: &'b Vec<Context::Error>,
}

impl<'a, 'b, Context: TomlParseContext, A: TomlAst> Drop for TomlParser<'a, 'b, Context, A> {
    fn drop(&mut self) {
        // todo!("put unvisited asts as error")
    }
}

impl<'a, 'b, Context: TomlParseContext> TomlParser<'a, 'b, Context, TomlTable> {
    pub fn new_root(
        db: &'a Context::Db<'a>,
        toml_ast_sheet: &'a TomlAstSheet,
        menu: &'a Context::Menu,
        errors: &'b Vec<Context::Error>,
    ) -> Self {
        Self {
            db,
            toml_ast_visitor: toml_ast_sheet.root_visitor(),
            menu,
            errors,
        }
    }

    fn section_parser<'c>(
        &'c mut self,
        section_idx: TomlSectionIdx,
    ) -> TomlParser<'a, 'c, Context, TomlSection>
    where
        'b: 'c,
    {
        TomlParser {
            db: self.db,
            toml_ast_visitor: TomlSectionVisitor::new(
                self.toml_ast_visitor.toml_ast_sheet(),
                section_idx,
            ),
            menu: self.menu,
            errors: self.errors,
        }
    }

    pub fn parse_normal_section<Target>(&mut self) -> Option<Result<Target, Context::Error>>
    where
        Target: ParseFromToml<Context = Context, Ast = TomlSection> + ParseFromTomlParentKeyed,
    {
        let key = <Target as ParseFromTomlParentKeyed>::key(self.menu);
        match self.toml_ast_visitor.visit(key) {
            Some(TomlTableValue::Section(section_idx)) => {
                let section_parser = self.section_parser(*section_idx);
                Some(section_parser.parse_into())
            }
            Some(_) => Some(Err(todo!())),
            None => None,
        }
    }
}

impl<'a, 'b, Context: TomlParseContext, Ast: TomlAst> TomlParser<'a, 'b, Context, Ast> {
    pub fn parse_into<Target>(mut self) -> Result<Target, Context::Error>
    where
        Target: ParseFromToml<Context = Context, Ast = Ast>,
    {
        Target::parse_from(self)
    }
}

pub trait ParseFromTomlParentKeyed: ParseFromToml {
    fn key(menu: &<Self::Context as TomlParseContext>::Menu) -> Word;
}

pub trait ParseFromToml: Sized {
    type Context: TomlParseContext;
    type Ast: TomlAst;

    fn parse_from<'a, 'b>(
        parser: TomlParser<'a, 'b, Self::Context, Self::Ast>,
    ) -> Result<Self, <Self::Context as TomlParseContext>::Error>;
}

pub trait TomlParseContext {
    type Db<'a>: ?Sized;
    type Menu;
    type Error;
}
