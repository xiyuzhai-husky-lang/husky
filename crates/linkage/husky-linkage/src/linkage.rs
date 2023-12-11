use crate::*;
use husky_entity_path::{FugitivePath, TypeItemPath};
use husky_hir_decl::parameter::template::item_hir_template_parameter_stats;
use husky_hir_defn::HasHirDefn;
use husky_hir_ty::{instantiation::HirInstantiation, HirTemplateArgument, HirTemplateArguments};
use husky_javelin::javelin::{package_javelins, Javelin, JavelinData};
use husky_vfs::PackagePath;
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
    match javelin.data(db) {
        JavelinData::PathLeading {
            path,
            instantiation,
        } => smallvec![Linkage::new(db, javelin, todo!())],
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
    DB::default()
        .ast_expect_test_debug_with_db(package_linkages, &AstTestConfig::new("package_linkages"))
}
