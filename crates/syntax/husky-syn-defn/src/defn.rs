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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = SynDefnDb)]
#[enum_class::from_variants]
pub enum SynNodeDefn {
    Submodule(SubmoduleSynNodeDefn),
    ModuleItem(ModuleItemSynNodeDefn),
    TypeVariant(TypeVariantSynNodeDefn),
    ImplBlock(ImplBlockSynNodeDecl),
    AssociatedItem(AssociatedItemSynNodeDefn),
}

impl SynNodeDefn {
    pub fn node_decl(self, db: &dyn SynDefnDb) -> SynNodeDecl {
        match self {
            SynNodeDefn::Submodule(node_defn) => node_defn.node_decl().into(),
            SynNodeDefn::ModuleItem(node_defn) => node_defn.node_decl(db).into(),
            SynNodeDefn::TypeVariant(node_defn) => node_defn.node_decl(db).into(),
            SynNodeDefn::ImplBlock(node_decl) => node_decl.into(),
            SynNodeDefn::AssociatedItem(node_defn) => node_defn.node_decl(db).into(),
        }
    }

    pub fn ast_idx(self, db: &dyn SynDefnDb) -> AstIdx {
        self.node_decl(db).ast_idx(db)
    }

    pub fn expr_region(self, db: &dyn SynDefnDb) -> Option<SynExprRegion> {
        match self {
            SynNodeDefn::Submodule(_) => None,
            SynNodeDefn::ModuleItem(defn) => defn.expr_region(db),
            SynNodeDefn::AssociatedItem(defn) => defn.expr_region(db),
            SynNodeDefn::TypeVariant(_defn) => None,
            SynNodeDefn::ImplBlock(_) => None,
        }
    }
}

pub trait HasSynNodeDefn: Copy {
    type NodeDefn;

    fn node_defn(self, db: &dyn SynDefnDb) -> Self::NodeDefn;
}

impl HasSynNodeDefn for EntitySynNodePath {
    type NodeDefn = SynNodeDefn;

    fn node_defn(self, db: &dyn SynDefnDb) -> Self::NodeDefn {
        match self {
            EntitySynNodePath::Submodule(path) => path.node_defn(db).into(),
            EntitySynNodePath::ModuleItem(path) => path.node_defn(db).into(),
            EntitySynNodePath::TypeVariant(path) => path.node_defn(db).into(),
            EntitySynNodePath::ImplBlock(path) => path.node_defn(db).into(),
            EntitySynNodePath::AssociatedItem(path) => path.node_defn(db).into(),
        }
    }
}

pub trait HasNodeDefns: Copy {
    fn node_defns(self, db: &dyn SynDefnDb) -> EntitySynTreeResult<&[SynNodeDefn]>;
}

impl HasNodeDefns for ModulePath {
    fn node_defns(self, db: &dyn SynDefnDb) -> EntitySynTreeResult<&[SynNodeDefn]> {
        Ok(module_node_defns(db, self).as_ref()?)
    }
}

#[salsa::tracked(jar = SynDefnJar, return_ref)]
pub(crate) fn module_node_defns(
    db: &dyn SynDefnDb,
    module_path: ModulePath,
) -> EntitySynTreeResult<Vec<SynNodeDefn>> {
    Ok(module_entity_syn_node_paths(db, module_path)
        .as_ref()?
        .iter()
        .copied()
        .map(|syn_node_path| syn_node_path.node_defn(db))
        .collect())
}

#[test]
fn module_node_defns_works() {
    use tests::*;

    DB::default()
        .ast_expect_test_debug_with_db("module_node_defns", |db, module_path: ModulePath| {
            module_path.node_defns(db)
        });
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = SynDefnDb)]
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

    pub fn generic_parameters<'a>(self, db: &'a dyn SynDefnDb) -> &'a [GenericParameterDecl] {
        self.decl(db).generic_parameters(db)
    }

    pub fn expr_region(self, db: &dyn SynDefnDb) -> Option<SynExprRegion> {
        match self {
            Defn::Submodule(_) => None,
            Defn::ModuleItem(defn) => defn.expr_region(db),
            Defn::AssociatedItem(defn) => defn.expr_region(db),
            Defn::TypeVariant(_defn) => None,
            Defn::ImplBlock(_) => None,
        }
    }
}

impl Defn {
    pub fn path(self, db: &dyn SynDefnDb) -> Option<EntityPath> {
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

pub trait HasDefn: Copy {
    type Defn;

    fn defn(self, db: &dyn SynDefnDb) -> DefnResult<Self::Defn>;
}

impl HasDefn for EntityPath {
    type Defn = Defn;

    fn defn(self, db: &dyn SynDefnDb) -> DefnResult<Self::Defn> {
        Ok(match self {
            EntityPath::Module(path) => path.defn(db)?.into(),
            EntityPath::ModuleItem(path) => path.defn(db)?.into(),
            EntityPath::ImplBlock(path) => path.defn(db)?.into(),
            EntityPath::AssociatedItem(path) => path.defn(db)?.into(),
            EntityPath::TypeVariant(_) => todo!(),
        })
    }
}

pub trait HasDefns: Copy {
    fn defns(self, db: &dyn SynDefnDb) -> EntitySynTreeResult<&[Defn]>;
}

impl HasDefns for ModulePath {
    fn defns(self, db: &dyn SynDefnDb) -> EntitySynTreeResult<&[Defn]> {
        Ok(module_defns(db, self).as_ref()?)
    }
}

#[salsa::tracked(jar = SynDefnJar, return_ref)]
pub(crate) fn module_defns(
    db: &dyn SynDefnDb,
    module_path: ModulePath,
) -> EntitySynTreeResult<Vec<Defn>> {
    Ok(module_entity_paths(db, module_path)
        .as_ref()?
        .iter()
        .copied()
        .filter_map(|path| path.defn(db).ok())
        .collect())
}

#[test]
fn module_defns_works() {
    use tests::*;

    DB::default().ast_expect_test_debug_with_db("module_defns", |db, module_path: ModulePath| {
        module_path.defns(db)
    });
}
