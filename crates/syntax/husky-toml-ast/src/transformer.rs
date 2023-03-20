mod expr;
mod transform;

use std::path::Path;

use self::expr::*;
use self::transform::*;
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

pub struct TomlTransformer<'a, 'b, Context: TomlDeserializeContext, A: TomlAst> {
    db: &'a Context::Db<'a>,
    toml_ast_visitor: TomlVisitor<'a, A>,
    menu: &'a Context::Menu,
    path: &'a Path,
    errors: &'b mut Vec<Context::Error>,
}

impl<'a, 'b, Context: TomlDeserializeContext, A: TomlAst> Drop
    for TomlTransformer<'a, 'b, Context, A>
{
    fn drop(&mut self) {
        // todo!("put unvisited asts as error")
    }
}

// impl general case

impl<'a, 'b, Context: TomlDeserializeContext, Ast: TomlAst> TomlTransformer<'a, 'b, Context, Ast> {
    pub fn menu(&self) -> &<Context as TomlDeserializeContext>::Menu {
        self.menu
    }

    pub fn transform_into<Target>(mut self) -> Result<Target, Context::Error>
    where
        Target: TransformFromToml<Context, Ast = Ast>,
    {
        Target::transform_from(self)
    }
}

// impl table transformer
impl<'a, 'b, Context: TomlDeserializeContext> TomlTransformer<'a, 'b, Context, TomlTable> {
    pub fn new_root(
        db: &'a Context::Db<'a>,
        path: DiffPath,
        menu: &'a Context::Menu,
        errors: &'b mut Vec<Context::Error>,
    ) -> VfsResult<Option<Self>> {
        let Some(toml_ast_sheet) = db.toml_ast_sheet(path)? else {
            return Ok(None)
        };
        Ok(Some(Self {
            db,
            toml_ast_visitor: toml_ast_sheet.root_visitor(),
            menu,
            path: path.path(db),
            errors,
        }))
    }

    fn section_transformer<'c>(
        &'c mut self,
        section_idx: TomlSectionIdx,
    ) -> TomlTransformer<'a, 'c, Context, TomlSection>
    where
        'b: 'c,
    {
        TomlTransformer {
            db: self.db,
            toml_ast_visitor: TomlSectionVisitor::new(
                self.toml_ast_visitor.toml_ast_sheet(),
                section_idx,
            ),
            menu: self.menu,
            path: self.path,
            errors: self.errors,
        }
    }

    pub fn transform_normal_section<Target>(&mut self) -> Option<Result<Target, Context::Error>>
    where
        Target:
            TransformFromToml<Context, Ast = TomlSection> + TransformFromTomlParentKeyed<Context>,
    {
        let key = <Target as TransformFromTomlParentKeyed<Context>>::key(self.menu);
        match self.toml_ast_visitor.visit(key) {
            Some(TomlTableValue::Section(section_idx)) => {
                let section_transformer = self.section_transformer(*section_idx);
                Some(section_transformer.transform_into())
            }
            Some(_) => Some(Err(todo!())),
            None => None,
        }
    }
}

// impl section transformer
impl<'a, 'b, Context: TomlDeserializeContext> TomlTransformer<'a, 'b, Context, TomlSection> {
    fn value_transformer<'c>(
        &'c mut self,
        key: Word,
    ) -> Option<TomlTransformer<'a, 'c, Context, TomlExpr>>
    where
        'b: 'c,
    {
        Some(TomlTransformer {
            db: self.db,
            toml_ast_visitor: TomlExprVisitor::new(
                self.toml_ast_visitor.toml_ast_sheet(),
                self.toml_ast_visitor.visit(key)?,
            ),
            menu: self.menu,
            path: self.path,
            errors: self.errors,
        })
    }

    pub fn transform_value<Target>(mut self, key: Word) -> Option<Result<Target, Context::Error>>
    where
        Target: TransformFromToml<Context, Ast = TomlExpr>,
    {
        Some(Target::transform_from(self.value_transformer(key)?))
    }
}

pub trait TransformFromTomlParentKeyed<Context>: TransformFromToml<Context>
where
    Context: TomlDeserializeContext,
{
    fn key(menu: &<Context as TomlDeserializeContext>::Menu) -> Word;
}

pub trait TransformFromToml<Context>: Sized
where
    Context: TomlDeserializeContext,
{
    type Ast: TomlAst;

    fn transform_from<'a, 'b>(
        parser: TomlTransformer<'a, 'b, Context, Self::Ast>,
    ) -> Result<Self, <Context as TomlDeserializeContext>::Error>;
}

pub trait TomlDeserializeContext {
    type Db<'a>: ?Sized + TomlAstDb;
    type Menu;
    type Error;
}
