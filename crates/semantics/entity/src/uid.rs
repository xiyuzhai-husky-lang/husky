use crate::*;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityUid {
    raw: usize,
}

static ENTITY_NEXT_RAW_ID: AtomicUsize = AtomicUsize::new(0);

impl EntityUid {
    pub fn new() -> EntityUid {
        let raw = ENTITY_NEXT_RAW_ID.fetch_add(1, Ordering::Relaxed);
        EntityUid { raw }
    }
}

pub(crate) fn entity_uid(db: &dyn EntityQueryGroup, entity_route: EntityRoutePtr) -> EntityUid {
    // responds to changes in either defn or defns of dependees
    let _defn = db.entity_defn(entity_route);
    let _dependees = db.entity_dependees(entity_route);
    EntityUid::new()
}
