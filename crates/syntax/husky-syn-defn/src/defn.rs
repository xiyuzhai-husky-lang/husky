mod associated_item;
mod impl_block;
mod module_item;
mod submodule;
mod ty_variant;

pub use self::associated_item::*;
pub use self::impl_block::*;
pub use self::module_item::*;
pub use self::submodule::*;
pub use self::ty_variant::*;

use crate::*;
use husky_ast::AstIdx;
use husky_entity_syn_tree::path::{module_item_paths, module_item_syn_node_paths};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = SynDefnDb)]
#[enum_class::from_variants]
pub enum SynNodeDefn {
    Submodule(SubmoduleSynNodeDefn),
    ModuleItem(ModuleItemSynNodeDefn),
    TypeVariant(TypeVariantSynNodeDefn),
    ImplBlock(ImplBlockSynNodeDecl),
    AssociatedItem(AssociatedItemSynNodeDefn),
}

impl SynNodeDefn {
    pub fn syn_node_decl(self, db: &dyn SynDefnDb) -> SynNodeDecl {
        match self {
            SynNodeDefn::Submodule(syn_node_defn) => syn_node_defn.syn_node_decl().into(),
            SynNodeDefn::ModuleItem(syn_node_defn) => syn_node_defn.syn_node_decl(db).into(),
            SynNodeDefn::TypeVariant(syn_node_defn) => syn_node_defn.syn_node_decl(db).into(),
            SynNodeDefn::ImplBlock(syn_node_decl) => syn_node_decl.into(),
            SynNodeDefn::AssociatedItem(syn_node_defn) => syn_node_defn.syn_node_decl(db).into(),
        }
    }

    pub fn ast_idx(self, db: &dyn SynDefnDb) -> AstIdx {
        self.syn_node_decl(db).ast_idx(db)
    }

    pub fn syn_expr_region(self, db: &dyn SynDefnDb) -> Option<SynExprRegion> {
        match self {
            SynNodeDefn::Submodule(_) => None,
            SynNodeDefn::ModuleItem(defn) => defn.syn_expr_region(db),
            SynNodeDefn::AssociatedItem(defn) => defn.syn_expr_region(db),
            SynNodeDefn::TypeVariant(_defn) => None,
            SynNodeDefn::ImplBlock(_) => None,
        }
    }
}

pub trait HasSynNodeDefn: Copy {
    type SynNodeDefn;

    fn syn_node_defn(self, db: &dyn SynDefnDb) -> Self::SynNodeDefn;
}

impl HasSynNodeDefn for ItemSynNodePath {
    type SynNodeDefn = SynNodeDefn;

    fn syn_node_defn(self, db: &dyn SynDefnDb) -> Self::SynNodeDefn {
        match self {
            ItemSynNodePath::Submodule(path) => path.syn_node_defn(db).into(),
            ItemSynNodePath::ModuleItem(path) => path.syn_node_defn(db).into(),
            ItemSynNodePath::TypeVariant(path) => path.syn_node_defn(db).into(),
            ItemSynNodePath::ImplBlock(path) => path.syn_node_defn(db).into(),
            ItemSynNodePath::AssociatedItem(path) => path.syn_node_defn(db).into(),
        }
    }
}

pub trait HasNodeDefns: Copy {
    fn node_defns(self, db: &dyn SynDefnDb) -> EntitySynTreeResult<&[SynNodeDefn]>;
}

impl HasNodeDefns for ModulePath {
    fn node_defns(self, db: &dyn SynDefnDb) -> EntitySynTreeResult<&[SynNodeDefn]> {
        Ok(module_syn_node_defns(db, self).as_ref()?)
    }
}

#[salsa::tracked(jar = SynDefnJar, return_ref)]
pub(crate) fn module_syn_node_defns(
    db: &dyn SynDefnDb,
    module_path: ModulePath,
) -> EntitySynTreeResult<Vec<SynNodeDefn>> {
    Ok(module_item_syn_node_paths(db, module_path)
        .as_ref()?
        .iter()
        .copied()
        .map(|syn_node_path| syn_node_path.syn_node_defn(db))
        .collect())
}

#[test]
fn module_node_defns_works() {
    use tests::*;

    DB::default()
        .ast_expect_test_debug_with_db("module_syn_node_defns", |db, module_path: ModulePath| {
            module_path.node_defns(db)
        });
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = SynDefnDb)]
#[enum_class::from_variants]
pub enum Defn {
    Submodule(SubmoduleDefn),
    ModuleItem(ModuleItemDefn),
    TypeVariant(TypeVariantDefn),
    ImplBlock(ImplBlockSynDecl),
    AssociatedItem(AssociatedItemSynDefn),
}

impl Defn {
    pub fn decl(self, db: &dyn SynDefnDb) -> Decl {
        match self {
            Defn::Submodule(defn) => Decl::Submodule(defn.decl()),
            Defn::ModuleItem(defn) => defn.decl(db).into(),
            Defn::TypeVariant(defn) => defn.decl(db).into(),
            Defn::ImplBlock(decl) => decl.into(),
            Defn::AssociatedItem(defn) => defn.decl(db).into(),
        }
    }

    pub fn ast_idx(self, db: &dyn SynDefnDb) -> AstIdx {
        todo!()
        // self.decl(db).ast_idx(db)
    }

    pub fn template_parameters<'a>(self, db: &'a dyn SynDefnDb) -> &'a [TemplateParameterDecl] {
        self.decl(db).template_parameters(db)
    }

    pub fn syn_expr_region(self, db: &dyn SynDefnDb) -> Option<SynExprRegion> {
        match self {
            Defn::Submodule(_) => None,
            Defn::ModuleItem(defn) => defn.syn_expr_region(db),
            Defn::AssociatedItem(defn) => defn.syn_expr_region(db),
            Defn::TypeVariant(_defn) => None,
            Defn::ImplBlock(_) => None,
        }
    }
}

impl Defn {
    pub fn path(self, db: &dyn SynDefnDb) -> Option<ItemPath> {
        todo!()
        // match self {
        //     Defn::Type(defn) => Some(defn.path(db).into()),
        //     Defn::Trait(defn) => Some(defn.path(db).into()),
        //     Defn::Fugitive(defn) => Some(defn.path(db).into()),
        //     Defn::AssociatedItem(defn) => defn.path(db).map(|path| path.into()),
        //     Defn::Variant(defn) => Some(defn.path(db).into()),
        //     Defn::ImplBlock(_) => None,
        // }
    }
}

pub trait HasSynDefn: Copy {
    type SynDefn;

    fn syn_defn(self, db: &dyn SynDefnDb) -> SynDefnResult<Self::SynDefn>;
}

impl HasSynDefn for ItemPath {
    type SynDefn = Defn;

    fn syn_defn(self, db: &dyn SynDefnDb) -> SynDefnResult<Self::SynDefn> {
        Ok(match self {
            ItemPath::Submodule(path) => path.syn_defn(db)?.into(),
            ItemPath::ModuleItem(path) => path.syn_defn(db)?.into(),
            ItemPath::ImplBlock(path) => path.syn_defn(db)?.into(),
            ItemPath::AssociatedItem(path) => path.syn_defn(db)?.into(),
            ItemPath::TypeVariant(_) => todo!(),
        })
    }
}

pub trait HasDefns: Copy {
    fn defns(self, db: &dyn SynDefnDb) -> EntitySynTreeResult<&[Defn]>;
}

impl HasDefns for ModulePath {
    fn defns(self, db: &dyn SynDefnDb) -> EntitySynTreeResult<&[Defn]> {
        Ok(module_syn_defns(db, self).as_ref()?)
    }
}

#[salsa::tracked(jar = SynDefnJar, return_ref)]
pub(crate) fn module_syn_defns(
    db: &dyn SynDefnDb,
    module_path: ModulePath,
) -> EntitySynTreeResult<Vec<Defn>> {
    Ok(module_item_paths(db, module_path)
        .as_ref()?
        .iter()
        .copied()
        .filter_map(|path| path.syn_defn(db).ok())
        .collect())
}

#[test]
fn module_defns_works() {
    use tests::*;

    DB::default()
        .ast_expect_test_debug_with_db("module_syn_defns", |db, module_path: ModulePath| {
            module_path.defns(db)
        });
}
