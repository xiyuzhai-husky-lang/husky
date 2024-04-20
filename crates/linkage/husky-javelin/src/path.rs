use either::*;
use husky_entity_kind::TypeKind;
use husky_entity_path::*;
use husky_hir_defn::HirDefn;

#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum JavPath {
    Form(MajorFormPath),
    TypeItem(TypeItemPath),
    TraitItem(TraitItemPath),
    TraitForTypeItem(TraitForTypeItemPath),
    Type(TypePath),
}

impl std::ops::Deref for JavPath {
    type Target = ItemPathId;

    fn deref(&self) -> &Self::Target {
        unsafe { &std::mem::transmute::<_, &(u32, ItemPathId)>(self).1 }
    }
}

#[test]
fn javelin_item_path_deref_works() {
    use crate::*;
    use husky_entity_tree::helpers::paths::module_item_paths;

    DB::ast_plain_test(
        |db, module_path| {
            for &item_path in module_item_paths(db, module_path) {
                if let Some(javelin_item_path) = JavPath::try_from_item_path(item_path, db) {
                    assert_eq!(*javelin_item_path, *item_path)
                }
            }
        },
        &AstTestConfig::new(
            "javelin_item_path_deref",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::LINKAGE,
        ),
    )
}

impl JavPath {
    pub fn try_from_item_path(item_path: ItemPath, db: &::salsa::Db) -> Option<Self> {
        match item_path {
            ItemPath::Submodule(_, _) => None,
            ItemPath::MajorItem(path) => match path {
                MajorItemPath::Type(path) => match path.ty_kind(db) {
                    TypeKind::Struct | TypeKind::Enum => Some(path.into()),
                    TypeKind::Inductive
                    | TypeKind::Record
                    | TypeKind::Structure
                    | TypeKind::Extern => None,
                },
                MajorItemPath::Trait(_) => None,
                MajorItemPath::Form(path) => Some(path.into()),
            },
            ItemPath::AssocItem(path) => match path {
                AssocItemPath::TraitForTypeItem(path) => {
                    if path.impl_block(db).trai_path(db).refine(db)
                        == Left(PreludeTraitPath::Visualize)
                    {
                        return None;
                    }
                    Some(JavPath::TraitForTypeItem(path))
                }
                AssocItemPath::TypeItem(path) => Some(JavPath::TypeItem(path)),
                AssocItemPath::TraitItem(path) => Some(JavPath::TraitItem(path)),
            },
            ItemPath::TypeVariant(_, path) => None,
            ItemPath::ImplBlock(_) => None,
            ItemPath::Attr(_, _) => None,
        }
    }

    pub fn hir_defn(self, _db: &::salsa::Db) -> HirDefn {
        todo!()
    }
}
