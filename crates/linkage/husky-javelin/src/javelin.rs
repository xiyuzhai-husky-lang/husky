use husky_entity_path::ItemPath;
use husky_hir_decl::parameter::template::item_hir_template_parameter_stats;
use husky_hir_ty::instantiation::HirInstantiation;
use husky_vfs::PackagePath;

use crate::{
    amazon::package_amazon_javelins, instantiation::JavelinInstantiation, path::JavelinPath,
    valkyrie::package_valkyrie_javelins, *,
};

#[salsa::interned(db = JavelinDb, jar = JavelinJar, constructor = pub(crate) new)]
pub struct Javelin {
    #[return_ref]
    pub data: JavelinData,
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum JavelinData {
    Coersion {},
    PathLeading {
        path: JavelinPath,
        instantiation: JavelinInstantiation,
    },
    // todo: merge into Item
    PropsStructField,
    // todo: merge into Item
    MemoizedField,
    // todo: merge into Item
    Index,
    // todo: merge into Item
    Method,
}

impl Javelin {
    pub fn from_item_path(item_path: ItemPath, db: &::salsa::Db) -> Option<Self> {
        let stats = item_hir_template_parameter_stats(db, *item_path)?;
        if stats.tys + stats.constants > 0 {
            return None;
        }
        Some(Self::new(
            db,
            JavelinData::PathLeading {
                path: JavelinPath::try_from_item_path(item_path)?,
                // ad hoc consider places
                instantiation: JavelinInstantiation::new_empty(),
            },
        ))
    }

    pub fn new_suffix(db: &::salsa::Db) -> Self {
        todo!()
    }

    pub fn new_props_struct_field(db: &::salsa::Db) -> Self {
        Self::new(db, JavelinData::PropsStructField)
    }

    pub fn new_memoized_field(db: &::salsa::Db) -> Self {
        Self::new(db, JavelinData::MemoizedField)
    }

    pub fn new_method(db: &::salsa::Db) -> Self {
        Self::new(db, JavelinData::Method)
    }

    pub fn new_index(db: &::salsa::Db) -> Self {
        Self::new(db, JavelinData::Index)
    }

    pub fn new_item(
        path: impl Into<ItemPath>,
        hir_instantiation: &HirInstantiation,
        db: &::salsa::Db,
    ) -> Self {
        Self::new(
            db,
            JavelinData::PathLeading {
                path: JavelinPath::try_from_item_path(path.into()).unwrap(),
                instantiation: JavelinInstantiation::from_hir(hir_instantiation, None, db),
            },
        )
    }
}

/// package javelins are package amazon javelins and valkyrie javelins
pub fn package_javelins<'db>(
    db: &'db ::salsa::Db,
    package_path: PackagePath,
) -> impl Iterator<Item = Javelin> + 'db {
    package_amazon_javelins(db, package_path)
        .iter()
        .map(|&amazon_javelin| *amazon_javelin)
        .chain(
            package_valkyrie_javelins(db, package_path)
                .iter()
                .map(|&valkyrie_javelin| *valkyrie_javelin),
        )
}
