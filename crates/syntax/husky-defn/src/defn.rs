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
#[salsa::derive_debug_with_db(db = DefnDb)]
#[enum_class::from_variants]
pub enum NodeDefn {
    Submodule(SubmoduleNodeDefn),
    ModuleItem(ModuleItemNodeDefn),
    TypeVariant(TypeVariantNodeDefn),
    ImplBlock(ImplBlockNodeDecl),
    AssociatedItem(AssociatedItemNodeDefn),
}

impl NodeDefn {
    pub fn node_decl(self, db: &dyn DefnDb) -> NodeDecl {
        match self {
            NodeDefn::Submodule(node_defn) => node_defn.node_decl().into(),
            NodeDefn::ModuleItem(node_defn) => node_defn.node_decl(db).into(),
            NodeDefn::TypeVariant(node_defn) => node_defn.node_decl(db).into(),
            NodeDefn::ImplBlock(node_decl) => node_decl.into(),
            NodeDefn::AssociatedItem(node_defn) => node_defn.node_decl(db).into(),
        }
    }

    pub fn ast_idx(self, db: &dyn DefnDb) -> AstIdx {
        self.node_decl(db).ast_idx(db)
    }

    pub fn expr_region(self, db: &dyn DefnDb) -> Option<ExprRegion> {
        match self {
            NodeDefn::Submodule(_) => None,
            NodeDefn::ModuleItem(defn) => defn.expr_region(db),
            NodeDefn::AssociatedItem(defn) => defn.expr_region(db),
            NodeDefn::TypeVariant(_defn) => None,
            NodeDefn::ImplBlock(_) => None,
        }
    }
}

pub trait HasNodeDefn: Copy {
    type NodeDefn;

    fn node_defn(self, db: &dyn DefnDb) -> Self::NodeDefn;
}

impl HasNodeDefn for EntityNodePath {
    type NodeDefn = NodeDefn;

    fn node_defn(self, db: &dyn DefnDb) -> Self::NodeDefn {
        match self {
            EntityNodePath::Submodule(path) => path.node_defn(db).into(),
            EntityNodePath::ModuleItem(path) => path.node_defn(db).into(),
            EntityNodePath::TypeVariant(path) => path.node_defn(db).into(),
            EntityNodePath::ImplBlock(path) => path.node_defn(db).into(),
            EntityNodePath::AssociatedItem(path) => path.node_defn(db).into(),
        }
    }
}

pub trait HasNodeDefns: Copy {
    fn node_defns(self, db: &dyn DefnDb) -> EntityTreeResult<&[NodeDefn]>;
}

impl HasNodeDefns for ModulePath {
    fn node_defns(self, db: &dyn DefnDb) -> EntityTreeResult<&[NodeDefn]> {
        Ok(module_node_defns(db, self).as_ref()?)
    }
}

#[salsa::tracked(jar = DefnJar, return_ref)]
pub(crate) fn module_node_defns(
    db: &dyn DefnDb,
    module_path: ModulePath,
) -> EntityTreeResult<Vec<NodeDefn>> {
    Ok(module_entity_node_paths(db, module_path)
        .as_ref()?
        .iter()
        .copied()
        .map(|node_path| node_path.node_defn(db))
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
#[salsa::derive_debug_with_db(db = DefnDb)]
#[enum_class::from_variants]
pub enum Defn {
    Submodule(SubmoduleDefn),
    ModuleItem(ModuleItemDefn),
    TypeVariant(TypeVariantDefn),
    ImplBlock(ImplBlockDecl),
    AssociatedItem(AssociatedItemDefn),
}

impl Defn {
    pub fn decl(self, db: &dyn DefnDb) -> Decl {
        match self {
            Defn::Submodule(defn) => Decl::Submodule(defn.decl()),
            Defn::ModuleItem(defn) => defn.decl(db).into(),
            Defn::TypeVariant(defn) => defn.decl(db).into(),
            Defn::ImplBlock(decl) => decl.into(),
            Defn::AssociatedItem(defn) => defn.decl(db).into(),
        }
    }

    pub fn ast_idx(self, db: &dyn DefnDb) -> AstIdx {
        todo!()
        // self.decl(db).ast_idx(db)
    }

    pub fn generic_parameters<'a>(self, db: &'a dyn DefnDb) -> &'a [GenericParameterDecl] {
        self.decl(db).generic_parameters(db)
    }

    pub fn expr_region(self, db: &dyn DefnDb) -> Option<ExprRegion> {
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
    pub fn path(self, db: &dyn DefnDb) -> Option<EntityPath> {
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

    fn defn(self, db: &dyn DefnDb) -> DefnResult<Self::Defn>;
}

impl HasDefn for EntityPath {
    type Defn = Defn;

    fn defn(self, db: &dyn DefnDb) -> DefnResult<Self::Defn> {
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
    fn defns(self, db: &dyn DefnDb) -> EntityTreeResult<&[Defn]>;
}

impl HasDefns for ModulePath {
    fn defns(self, db: &dyn DefnDb) -> EntityTreeResult<&[Defn]> {
        Ok(module_defns(db, self).as_ref()?)
    }
}

#[salsa::tracked(jar = DefnJar, return_ref)]
pub(crate) fn module_defns(
    db: &dyn DefnDb,
    module_path: ModulePath,
) -> EntityTreeResult<Vec<Defn>> {
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
