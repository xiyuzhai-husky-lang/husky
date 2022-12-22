use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Accessibility {
    Public,                 // everyone can access it
    PubicUnder(ModulePath), // everyone under a path can access it
    Private,                // only self
}

impl Accessibility {
    pub fn is_accessible_from(self, db: &dyn VfsDb, module_path: ModulePath) -> bool {
        match self {
            Accessibility::Public => true,
            Accessibility::PubicUnder(parent_module) => module_path.starts_with(db, parent_module),
            Accessibility::Private => todo!(),
        }
    }
}

impl salsa::DebugWithDb<dyn VfsDb + '_> for Accessibility {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn VfsDb,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        match self {
            Self::Public => write!(f, "Public"),
            Self::PubicUnder(module_path) => f
                .debug_tuple("PubicUnder")
                .field(&module_path.debug_with(db, include_all_fields))
                .finish(),
            Self::Private => write!(f, "Private"),
        }
    }
}

impl<Db: VfsDb> salsa::DebugWithDb<Db> for Accessibility {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        todo!()
    }
}
