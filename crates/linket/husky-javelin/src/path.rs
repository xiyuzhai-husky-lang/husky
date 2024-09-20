use either::*;
use husky_entity_kind::TypeKind;
use husky_entity_path::{
    path::{
        assoc_item::{
            trai_for_ty_item::TraitForTypeItemPath, trai_item::TraitItemPath,
            ty_item::TypeItemPath, AssocItemPath,
        },
        major_item::{form::MajorFormPath, trai::PreludeTraitPath, ty::TypePath, MajorItemPath},
        ItemPath, ItemPathId,
    },
    *,
};
use husky_hir_defn::defn::HirDefn;

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

impl Into<ItemPath> for JavPath {
    fn into(self) -> ItemPath {
        match self {
            JavPath::Form(slf) => slf.into(),
            JavPath::TypeItem(slf) => slf.into(),
            JavPath::TraitItem(slf) => slf.into(),
            JavPath::TraitForTypeItem(slf) => slf.into(),
            JavPath::Type(slf) => slf.into(),
        }
    }
}

impl std::ops::Deref for JavPath {
    type Target = ItemPathId;

    fn deref(&self) -> &Self::Target {
        unsafe { &std::mem::transmute::<_, &(u32, ItemPathId)>(self).1 }
    }
}

#[test]
fn jav_path_deref_works() {
    use crate::*;
    use husky_entity_tree::helpers::paths::module_item_paths;

    DB::ast_plain_test(
        |db, module_path| {
            for &item_path in module_item_paths(db, module_path) {
                match item_path {
                    ItemPath::TypeVariant(_, _) => (),
                    _ if let Some(jav_path) = JavPath::try_from_item_path(item_path, db) => {
                        assert_eq!(*jav_path, *item_path)
                    }
                    _ => (),
                }
            }
        },
        &AstTestConfig::new(
            "jav_path_deref",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::LINKET,
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
            ItemPath::TypeVariant(_, path) => {
                if path.parent_ty_path(db).ident(db).data(db) == "Class" {
                    use husky_print_utils::p;
                    use salsa::DebugWithDb;

                    p!(path.ident(db).debug(db));
                    todo!()
                }
                Some(path.parent_ty_path(db).into())
            }
            ItemPath::ImplBlock(_) => None,
            ItemPath::Attr(_, _) => None,
            ItemPath::Script(_, _) => todo!(),
        }
    }

    pub fn hir_defn(self, _db: &::salsa::Db) -> HirDefn {
        todo!()
    }
}
