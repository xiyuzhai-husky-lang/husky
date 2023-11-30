/// the name amazon comes from diablo 2
pub mod amazon;
/// the name valkyrie comes from diablo 2
pub mod valkyrie;

use crate::{instantiation::JavelinInstantiation, path::JavelinItemPath, *};

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
        path: JavelinItemPath,
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
