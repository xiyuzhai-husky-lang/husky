use super::*;

impl EntityTreePresheet {
    pub(super) fn can_access(&self, db: &dyn VfsDb, accessibility: Accessibility) -> bool {
        accessibility.is_accessible_from(db, self.module_path)
    }
}
