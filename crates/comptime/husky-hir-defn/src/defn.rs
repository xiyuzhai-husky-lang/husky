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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = HirDefnDb)]
#[enum_class::from_variants]
pub enum HirDefn {
    Submodule(SubmoduleHirDefn),
    ModuleItem(ModuleItemHirDefn),
    TypeVariant(TypeVariantHirDefn),
    ImplBlock(ImplBlockHirDecl),
    AssociatedItem(AssociatedItemHirDefn),
}

impl HirDefn {
    pub fn decl(self, db: &dyn HirDefnDb) -> HirDecl {
        match self {
            HirDefn::Submodule(defn) => HirDecl::Submodule(defn.decl()),
            HirDefn::ModuleItem(defn) => defn.decl(db).into(),
            HirDefn::TypeVariant(defn) => defn.decl(db).into(),
            HirDefn::ImplBlock(decl) => decl.into(),
            HirDefn::AssociatedItem(defn) => defn.decl(db).into(),
        }
    }

    pub fn generic_parameters<'a>(self, db: &'a dyn HirDefnDb) -> &'a [EtherealGenericParameter] {
        self.decl(db).generic_parameters(db)
    }

    pub fn expr_region(self, db: &dyn HirDefnDb) -> Option<HirExprRegion> {
        match self {
            HirDefn::Submodule(_) => None,
            HirDefn::ModuleItem(defn) => defn.expr_region(db),
            HirDefn::AssociatedItem(defn) => defn.expr_region(db),
            HirDefn::TypeVariant(_defn) => None,
            HirDefn::ImplBlock(_) => None,
        }
    }
}

impl HirDefn {
    pub fn path(self, db: &dyn HirDefnDb) -> Option<EntityPath> {
        todo!()
        // match self {
        //     HirDefn::Type(defn) => Some(defn.path(db).into()),
        //     HirDefn::Trait(defn) => Some(defn.path(db).into()),
        //     HirDefn::Fugitive(defn) => Some(defn.path(db).into()),
        //     HirDefn::AssociatedItem(defn) => defn.path(db).map(|path| path.into()),
        //     HirDefn::Variant(defn) => Some(defn.path(db).into()),
        //     HirDefn::ImplBlock(_) => None,
        // }
    }
}

pub trait HasHirDefn: Copy {
    type HirDefn;

    fn hir_defn(self, db: &dyn HirDefnDb) -> Self::HirDefn;
}

impl HasHirDefn for EntityPath {
    type HirDefn = HirDefn;

    fn hir_defn(self, db: &dyn HirDefnDb) -> Self::HirDefn {
        Ok(match self {
            EntityPath::Module(path) => path.hir_defn(db)?.into(),
            EntityPath::ModuleItem(path) => path.hir_defn(db)?.into(),
            EntityPath::ImplBlock(path) => path.hir_defn(db)?.into(),
            EntityPath::AssociatedItem(path) => path.hir_defn(db)?.into(),
            EntityPath::TypeVariant(_) => todo!(),
        })
    }
}

#[salsa::tracked(jar = HirDefnJar, return_ref)]
pub(crate) fn module_hir_defns(
    db: &dyn HirDefnDb,
    module_path: ModulePath,
) -> EntityHirTreeResult<Vec<HirDefn>> {
    Ok(module_entity_paths(db, module_path)
        .as_ref()?
        .iter()
        .copied()
        .filter_map(|path| path.hir_defn(db).ok())
        .collect())
}

#[test]
fn module_defns_works() {
    use tests::*;

    DB::default()
        .ast_expect_test_debug_with_db("module_hir_defns", |db, module_path: ModulePath| {
            module_path.defns(db)
        });
}
