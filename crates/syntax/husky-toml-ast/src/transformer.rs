mod expr;
mod transform;

use std::path::Path;

use husky_coword::Name;
use husky_vfs::error::VfsError;
use vec_like::{AsVecMapEntry, VecMap};

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
    visitor: TomlVisitor<'a, A>,
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
    pub fn db(&self) -> &'a Context::Db<'a> {
        self.db
    }

    pub fn menu(&self) -> &<Context as TomlDeserializeContext>::Menu {
        self.menu
    }

    pub fn transform_into<Target>(mut self) -> Result<Target, Context::Error>
    where
        Target: TransformFromTomlAst<Context, Ast = Ast>,
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
            return Ok(None);
        };
        Ok(Some(Self {
            db,
            visitor: toml_ast_sheet.root_visitor(),
            menu,
            path: path.path(db).parent().unwrap(), /* ad hoc */
            errors,
        }))
    }

    pub fn finish(mut self) -> Vec<Context::Error> {
        std::mem::take(self.errors)
    }

    pub fn new_root_expected(
        db: &'a Context::Db<'a>,
        path: DiffPath,
        menu: &'a Context::Menu,
        errors: &'b mut Vec<Context::Error>,
    ) -> VfsResult<Self> {
        let Some(toml_ast_sheet) = db.toml_ast_sheet(path)? else {
            Err(VfsError::FileNotExists(path))?
        };
        Ok(Self {
            db,
            visitor: toml_ast_sheet.root_visitor(),
            menu,
            path: path.path(db),
            errors,
        })
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
            visitor: TomlSectionVisitor::new(self.visitor.toml_ast_sheet(), section_idx),
            menu: self.menu,
            path: self.path,
            errors: self.errors,
        }
    }

    pub fn transform_normal_section<Target>(&mut self) -> Option<Result<Target, Context::Error>>
    where
        Target: TransformFromTomlAst<Context, Ast = TomlSection>
            + TransformFromTomlParentKeyed<Context>,
    {
        let key = <Target as TransformFromTomlParentKeyed<Context>>::key(self.menu);
        match self.visitor.visit(key) {
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
    #[inline(always)]
    fn value_transformer<'c>(
        &'c mut self,
        key: Coword,
    ) -> Option<TomlTransformer<'a, 'c, Context, TomlExpr>>
    where
        'b: 'c,
    {
        Some(TomlTransformer {
            db: self.db,
            visitor: TomlExprVisitor::new(self.visitor.toml_ast_sheet(), self.visitor.visit(key)?),
            menu: self.menu,
            path: self.path,
            errors: self.errors,
        })
    }

    #[inline(always)]
    fn entry_transformer<'c>(
        &'c mut self,
        entry: &'a TomlSectionEntry,
    ) -> TomlTransformer<'a, 'c, Context, TomlSectionEntry>
    where
        'b: 'c,
    {
        TomlTransformer {
            db: self.db,
            visitor: TomlSectionEntryVisitor::new(
                self.visitor.toml_ast_sheet(),
                self.visitor.section_idx(),
                entry,
            ),
            menu: self.menu,
            path: self.path,
            errors: self.errors,
        }
    }

    pub fn transform_value<Target>(mut self, key: Coword) -> Option<Result<Target, Context::Error>>
    where
        Target: TransformFromTomlAst<Context, Ast = TomlExpr>,
    {
        Some(Target::transform_from(self.value_transformer(key)?))
    }

    pub fn transform_all_entries_into_vec_map<Target>(mut self) -> VecMap<Target>
    where
        Target: TransformFromTomlKeyedAst<Context, KeyedAst = TomlSectionEntry>
            + AsVecMapEntry<K = Name>
            + std::fmt::Debug,
    {
        let mut targets: VecMap<Target> = Default::default();
        for entry in self.visitor.all_entries() {
            match Target::transform_from(self.entry_transformer(entry)) {
                Ok(target) => targets.insert_new(target).expect("should be distinct!"),
                Err(e) => self.errors.push(e),
            }
        }
        targets
    }

    pub fn section_idx(&self) -> TomlSectionIdx {
        self.visitor.section_idx()
    }
}

// impl section entry transformer
impl<'a, 'b, Context: TomlDeserializeContext> TomlTransformer<'a, 'b, Context, TomlSectionEntry> {
    fn value_transformer<'c>(&'c mut self) -> Option<TomlTransformer<'a, 'c, Context, TomlExpr>>
    where
        'b: 'c,
    {
        Some(TomlTransformer {
            db: self.db,
            visitor: TomlExprVisitor::new(self.visitor.toml_ast_sheet(), self.visitor.visit()?),
            menu: self.menu,
            path: self.path,
            errors: self.errors,
        })
    }

    pub fn section_idx(&self) -> TomlSectionIdx {
        self.visitor.section_idx()
    }

    pub fn line_group_idx(&self) -> TomlLineGroupIdx {
        self.visitor.line_group_idx()
    }

    pub fn key(&self) -> Coword {
        self.visitor.key()
    }
}

pub trait TransformFromTomlParentKeyed<Context>: TransformFromTomlAst<Context>
where
    Context: TomlDeserializeContext,
{
    fn key(menu: &<Context as TomlDeserializeContext>::Menu) -> Coword;
}

pub trait TransformFromTomlAst<Context>: Sized
where
    Context: TomlDeserializeContext,
{
    type Ast: TomlAst;

    fn transform_from<'a, 'b>(
        parser: TomlTransformer<'a, 'b, Context, Self::Ast>,
    ) -> Result<Self, <Context as TomlDeserializeContext>::Error>;
}

pub trait TransformFromTomlKeyedAst<Context>: Sized
where
    Context: TomlDeserializeContext,
{
    type KeyedAst: TomlAst;

    fn transform_from<'a, 'b>(
        parser: TomlTransformer<'a, 'b, Context, Self::KeyedAst>,
    ) -> Result<Self, <Context as TomlDeserializeContext>::Error>;
}

pub trait TomlDeserializeContext {
    type Db<'a>: ?Sized + TomlAstDb;
    type Menu;
    type Error;
}
