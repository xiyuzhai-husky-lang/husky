use crate::*;
use husky_coword::Ident;
use husky_entity_kind::{FugitiveKind, TraitItemKind, TypeItemKind, TypeKind};
use husky_entity_path::{AssociatedItemPath, FugitivePath, TypeItemPath, TypeVariantPath};
use husky_entity_path::{TraitForTypeItemPath, TypePath};
use husky_hir_decl::parameter::template::item_hir_template_parameter_stats;
use husky_hir_defn::HasHirDefn;
use husky_hir_ty::{
    instantiation::HirInstantiation, HirTemplateArgument, HirTemplateArguments, HirType,
};
use husky_javelin::{
    javelin::{package_javelins, Javelin, JavelinData},
    path::JavelinPath,
};
use husky_print_utils::p;
use husky_vfs::PackagePath;
use salsa::DebugWithDb;
use smallvec::{smallvec, SmallVec};

#[salsa::interned(db = LinkageDb, jar = LinkageJar, constructor = pub(crate) new)]
pub struct Linkage {
    #[return_ref]
    pub data: LinkageData,
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum LinkageData {
    FunctionFnItem(FugitivePath),
    ValItem(FugitivePath),
    MemoizedField(AssociatedItemPath),
    MethodFn(AssociatedItemPath),
    AssociatedFunctionFn(AssociatedItemPath),
    TypeConstructor(TypePath),
    TypeVariantConstructor(TypeVariantPath),
    PropsStructField {
        ty_path: TypePath,
        instantiation: LinkageInstantiation,
        ident: Ident,
    },
    Index,
}

impl Linkage {
    pub fn from_item_path(item_path: ItemPath, db: &::salsa::Db) -> Option<Self> {
        let stats = item_hir_template_parameter_stats(db, *item_path)?;
        if stats.tys + stats.constants > 0 {
            return None;
        }
        Some(Self::new(db, todo!()))
    }

    pub fn new_suffix(db: &::salsa::Db) -> Self {
        todo!()
    }

    // todo: linkage_instantiation
    // todo: change to `JavelinType`
    pub fn new_props_struct_field(db: &::salsa::Db, owner_base_ty: HirType, ident: Ident) -> Self {
        let data = match owner_base_ty {
            HirType::PathLeading(hir_ty) => LinkageData::PropsStructField {
                ty_path: hir_ty.ty_path(db),
                instantiation: LinkageInstantiation::new_ad_hoc(),
                ident,
            },
            HirType::Symbol(_) => todo!(),
            HirType::TypeAssociatedType(_) => todo!(),
            HirType::TraitAssociatedType(_) => todo!(),
            HirType::Ritchie(_) => todo!(),
        };
        Self::new(db, data)
    }

    // todo: linkage_instantiation
    pub fn new_memoized_field(db: &::salsa::Db, path: AssociatedItemPath) -> Self {
        Self::new(db, LinkageData::MemoizedField(path))
    }

    pub fn new_method(db: &::salsa::Db, path: AssociatedItemPath) -> Self {
        Self::new(db, LinkageData::MethodFn(path))
    }

    pub fn new_index(db: &::salsa::Db) -> Self {
        Self::new(db, LinkageData::Index)
    }

    pub fn new_ty_constructor_fn(
        path: TypePath,
        hir_instantiation: &HirInstantiation,
        db: &::salsa::Db,
    ) -> Self {
        Self::new(db, LinkageData::TypeConstructor(path))
    }

    pub fn new_ty_variant_constructor_fn(
        path: TypeVariantPath,
        hir_instantiation: &HirInstantiation,
        db: &::salsa::Db,
    ) -> Self {
        Self::new(db, LinkageData::TypeVariantConstructor(path))
    }

    pub fn new_function_fn_item(
        path: FugitivePath,
        hir_instantiation: &HirInstantiation,
        db: &::salsa::Db,
    ) -> Self {
        Self::new(db, LinkageData::FunctionFnItem(path))
    }

    pub fn new_associated_function_fn_item(
        path: AssociatedItemPath,
        hir_instantiation: &HirInstantiation,
        db: &::salsa::Db,
    ) -> Self {
        Self::new(db, LinkageData::AssociatedFunctionFn(path))
    }
}

impl HasVersionStamp for Linkage {
    type VersionStamp = LinkageVersionStamp;

    fn version_stamp(self, db: &::salsa::Db) -> LinkageVersionStamp {
        todo!()
    }
}

#[deprecated(note = "ad hoc implementation")]
#[salsa::tracked(jar = LinkageJar, return_ref)]
fn linkages_emancipated_by_javelin(db: &::salsa::Db, javelin: Javelin) -> SmallVec<[Linkage; 4]> {
    match *javelin.data(db) {
        JavelinData::PathLeading {
            path,
            ref instantiation,
        } => match path {
            JavelinPath::Fugitive(path) => match path.fugitive_kind(db) {
                FugitiveKind::FunctionFn => {
                    smallvec![Linkage::new(db, LinkageData::FunctionFnItem(path))]
                }
                FugitiveKind::FunctionGn => smallvec![],
                FugitiveKind::AliasType => smallvec![],
                FugitiveKind::Val => {
                    smallvec![Linkage::new(db, LinkageData::ValItem(path))]
                }
            },
            JavelinPath::TypeItem(path) => match path.item_kind(db) {
                TypeItemKind::MethodFn => {
                    smallvec![Linkage::new(db, LinkageData::MethodFn(path.into()))]
                }
                TypeItemKind::AssociatedFunctionFn => {
                    smallvec![Linkage::new(
                        db,
                        LinkageData::AssociatedFunctionFn(path.into())
                    )]
                }
                TypeItemKind::AssociatedVal => todo!(),
                TypeItemKind::AssociatedType => smallvec![],
                TypeItemKind::MemoizedField => todo!(),
            },
            JavelinPath::TraitItem(_) => todo!(),
            JavelinPath::TraitForTypeItem(path) => match path.item_kind(db) {
                TraitItemKind::MethodFn => {
                    smallvec![Linkage::new(db, LinkageData::MethodFn(path.into()))]
                }
                TraitItemKind::AssociatedType => smallvec![],
                TraitItemKind::AssociatedVal => todo!(),
                TraitItemKind::AssociatedFunctionFn => {
                    smallvec![Linkage::new(
                        db,
                        LinkageData::AssociatedFunctionFn(path.into())
                    )]
                }
            },
            JavelinPath::TypeConstructor(path) => match path.ty_kind(db) {
                TypeKind::Enum => smallvec![],
                TypeKind::Inductive => unreachable!(),
                TypeKind::Record => unreachable!(),
                TypeKind::Struct => smallvec![Linkage::new(db, LinkageData::TypeConstructor(path))],
                TypeKind::Structure => unreachable!(),
                TypeKind::Extern => {
                    p!(path.debug(db));
                    unreachable!()
                }
            },
            JavelinPath::TypeVariantConstructor(_) => todo!(),
        },
    }
}

#[salsa::tracked(jar = LinkageJar, return_ref)]
pub fn package_linkages(db: &::salsa::Db, package_path: PackagePath) -> Vec<Linkage> {
    package_javelins(db, package_path)
        .map(|javelin| linkages_emancipated_by_javelin(db, javelin).iter().copied())
        .flatten()
        .collect()
}

#[test]
fn package_linkages_works() {
    DB::default().ast_expect_test_debug_with_db(
        package_linkages,
        &AstTestConfig::new("package_linkages")
            .with_vfs_test_domains_config(VfsTestDomainsConfig::ExcludeLibrary),
    )
}
