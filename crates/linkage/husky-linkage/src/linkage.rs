use crate::*;
use husky_entity_kind::{FugitiveKind, TraitItemKind, TypeItemKind, TypeKind};
use husky_entity_path::{FugitivePath, TypeItemPath};
use husky_entity_path::{TraitForTypeItemPath, TypePath};
use husky_hir_decl::parameter::template::item_hir_template_parameter_stats;
use husky_hir_defn::HasHirDefn;
use husky_hir_ty::{instantiation::HirInstantiation, HirTemplateArgument, HirTemplateArguments};
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
    pub javelin: Javelin,
    #[return_ref]
    pub data: LinkageData,
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum LinkageData {
    FunctionFnItem(FugitivePath),
    ValItem(FugitivePath),
    TypeMethodFn(TypeItemPath),
    TypeAssociatedFunctionFn(TypeItemPath),
    TraitForTypeMethodFn(TraitForTypeItemPath),
    TraitForTypeAssociatedFunctionFn(TraitForTypeItemPath),
    TypeConstructor(TypePath),
}

impl Linkage {
    pub fn from_item_path(item_path: ItemPath, db: &::salsa::Db) -> Option<Self> {
        let stats = item_hir_template_parameter_stats(db, *item_path)?;
        if stats.tys + stats.constants > 0 {
            return None;
        }
        Some(Self::new(
            db,
            Javelin::from_item_path(item_path, db)?,
            todo!(),
        ))
    }

    pub fn new_suffix(db: &::salsa::Db) -> Self {
        todo!()
    }

    pub fn new_props_struct_field(db: &::salsa::Db) -> Self {
        Self::new(db, Javelin::new_props_struct_field(db), todo!())
    }

    pub fn new_memoized_field(db: &::salsa::Db) -> Self {
        Self::new(db, Javelin::new_memoized_field(db), todo!())
    }

    pub fn new_method(db: &::salsa::Db) -> Self {
        Self::new(db, Javelin::new_method(db), todo!())
    }

    pub fn new_index(db: &::salsa::Db) -> Self {
        Self::new(db, Javelin::new_index(db), todo!())
    }

    pub fn new_item(
        path: impl Into<ItemPath>,
        hir_instantiation: &HirInstantiation,
        db: &::salsa::Db,
    ) -> Self {
        Self::new(db, Javelin::new_item(path, hir_instantiation, db), todo!())
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
                    smallvec![Linkage::new(db, javelin, LinkageData::FunctionFnItem(path))]
                }
                FugitiveKind::FunctionGn => smallvec![],
                FugitiveKind::AliasType => smallvec![],
                FugitiveKind::Val => {
                    smallvec![Linkage::new(db, javelin, LinkageData::ValItem(path))]
                }
            },
            JavelinPath::TypeItem(path) => match path.item_kind(db) {
                TypeItemKind::MethodFn => {
                    smallvec![Linkage::new(db, javelin, LinkageData::TypeMethodFn(path))]
                }
                TypeItemKind::AssociatedFunctionFn => {
                    smallvec![Linkage::new(
                        db,
                        javelin,
                        LinkageData::TypeAssociatedFunctionFn(path)
                    )]
                }
                TypeItemKind::AssociatedVal => todo!(),
                TypeItemKind::AssociatedType => smallvec![],
                TypeItemKind::MemoizedField => todo!(),
            },
            JavelinPath::TraitItem(_) => todo!(),
            JavelinPath::TraitForTypeItem(path) => match path.item_kind(db) {
                TraitItemKind::MethodFn => {
                    smallvec![Linkage::new(
                        db,
                        javelin,
                        LinkageData::TraitForTypeMethodFn(path)
                    )]
                }
                TraitItemKind::AssociatedType => smallvec![],
                TraitItemKind::AssociatedVal => todo!(),
                TraitItemKind::AssociatedFunctionFn => {
                    smallvec![Linkage::new(
                        db,
                        javelin,
                        LinkageData::TraitForTypeAssociatedFunctionFn(path)
                    )]
                }
            },
            JavelinPath::TypeConstructor(path) => match path.ty_kind(db) {
                TypeKind::Enum => smallvec![],
                TypeKind::Inductive => unreachable!(),
                TypeKind::Record => unreachable!(),
                TypeKind::Struct => smallvec![Linkage::new(
                    db,
                    javelin,
                    LinkageData::TypeConstructor(path)
                )],
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
