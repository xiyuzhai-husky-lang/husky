use crate::*;
use husky_hir_decl::parameter::template::item_hir_template_parameter_stats;
use husky_hir_defn::HasHirDefn;
use husky_hir_ty::{instantiation::HirInstantiation, HirTemplateArgument, HirTemplateArguments};
use husky_javelin::javelin::Javelin;

#[salsa::interned(db = LinkageDb, jar = LinkageJar, constructor = pub(crate) new)]
pub struct Linkage {
    pub javelin: Javelin,
    #[return_ref]
    pub data: LinkageData,
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum LinkageData {
    Coersion {},
    PathLeading {
        // path: JavelinPath,
        // instantiation: LinkageInstantiation,
    },
    // todo: merge into Item
    PropsStructField,
    // todo: merge into Item
    MemoizedField,
    // todo: merge into Item
    Index,
    // todo: merge into Item
    Method,
    Todo,
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
            LinkageData::PathLeading {
                // path: JavelinPath::try_from_item_path(item_path)?,
                // // ad hoc consider places
                // instantiation: JavelinInstantiation::new_first_born(),
            },
        ))
    }

    pub fn new_suffix(db: &::salsa::Db) -> Self {
        todo!()
    }

    pub fn new_props_struct_field(db: &::salsa::Db) -> Self {
        Self::new(
            db,
            Javelin::new_props_struct_field(db),
            LinkageData::PropsStructField,
        )
    }

    pub fn new_memoized_field(db: &::salsa::Db) -> Self {
        Self::new(
            db,
            Javelin::new_memoized_field(db),
            LinkageData::MemoizedField,
        )
    }

    pub fn new_method(db: &::salsa::Db) -> Self {
        Self::new(db, Javelin::new_method(db), LinkageData::Method)
    }

    pub fn new_index(db: &::salsa::Db) -> Self {
        Self::new(db, Javelin::new_index(db), LinkageData::Index)
    }

    pub fn new_item(
        path: impl Into<ItemPath>,
        hir_instantiation: &HirInstantiation,
        db: &::salsa::Db,
    ) -> Self {
        Self::new(
            db,
            Javelin::new_item(path, hir_instantiation, db),
            LinkageData::PathLeading {
                // path: JavelinPath::try_from_item_path(path.into()).unwrap(),
                // instantiation: JavelinInstantiation::from_hir(hir_instantiation, None, db),
            },
        )
    }
}

impl HasVersionStamp for Linkage {
    type VersionStamp = LinkageVersionStamp;

    fn version_stamp(self, db: &::salsa::Db) -> LinkageVersionStamp {
        todo!()
    }
}
