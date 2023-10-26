use super::*;

#[salsa::interned(db = TraceDb, jar = TraceJar, constructor = new)]
pub struct ValItemTracePath {
    pub val_item_path: FugitivePath,
}

#[salsa::tracked(db = TraceDb, jar = TraceJar, constructor = new)]
pub struct ValItemTrace {
    #[id]
    pub path: ValItemTracePath,
}

impl ValItemTrace {
    pub fn from_val_item_path(val_item_path: FugitivePath, db: &dyn TraceDb) -> Self {
        debug_assert_eq!(val_item_path.fugitive_kind(db), FugitiveKind::Val);
        ValItemTrace::new(db, ValItemTracePath::new(db, val_item_path))
    }

    pub fn view_data<'a>(self, db: &'a dyn TraceDb) -> &'a TraceViewData {
        todo!()
    }
}
