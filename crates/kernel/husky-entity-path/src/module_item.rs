mod connected;
mod disconnected;

pub use connected::*;
pub use disconnected::*;

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModuleItemPath {
    Connected(ConnectedModuleItemPath),
    Disconnected(DisconnectedModuleItemPath),
}

impl From<DisconnectedModuleItemPath> for ModuleItemPath {
    fn from(v: DisconnectedModuleItemPath) -> Self {
        Self::Disconnected(v)
    }
}

impl From<ConnectedModuleItemPath> for ModuleItemPath {
    fn from(v: ConnectedModuleItemPath) -> Self {
        Self::Connected(v)
    }
}

impl salsa::DebugWithDb<dyn EntityPathDb + '_> for ModuleItemPath {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn EntityPathDb,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        match self {
            ModuleItemPath::Connected(connected_module_item_path) => f
                .debug_tuple("Connected")
                .field(&connected_module_item_path.debug_with(db, include_all_fields))
                .finish(),
            ModuleItemPath::Disconnected(disconnected_module_item_path) => f
                .debug_tuple("Connected")
                .field(&disconnected_module_item_path.debug_with(db, include_all_fields))
                .finish(),
        }
    }
}

impl<Db> salsa::DebugWithDb<Db> for ModuleItemPath
where
    Db: EntityPathDb,
{
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        self.fmt(f, db as &dyn EntityPathDb, include_all_fields)
    }
}
