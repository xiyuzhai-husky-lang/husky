use husky_word::Identifier;

use crate::*;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub(crate) struct UseOneTrackers(Vec<UseOneTracker>);

impl<Db: EntityTreeDb + ?Sized> salsa::DebugWithDb<Db> for UseOneTrackers {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<EntityTreeJar>>::as_jar_db(db);
        f.debug_tuple("EntityUseExprTrackers")
            .field(&self.0.debug_with(db, include_all_fields))
            .finish()
    }
}

impl std::ops::Index<UseOneTrackerIdx> for UseOneTrackers {
    type Output = UseOneTracker;

    fn index(&self, index: UseOneTrackerIdx) -> &Self::Output {
        &self.0[index.0]
    }
}

impl std::ops::IndexMut<UseOneTrackerIdx> for UseOneTrackers {
    fn index_mut(&mut self, index: UseOneTrackerIdx) -> &mut Self::Output {
        &mut self.0[index.0]
    }
}

pub(crate) struct UseOneTrackerIdx(usize);

impl UseOneTrackers {
    pub(crate) fn push(&mut self, tracker: UseOneTracker) {
        self.0.push(tracker)
    }

    pub(crate) fn indexed_iter(&self) -> impl Iterator<Item = (UseOneTrackerIdx, &UseOneTracker)> {
        self.0
            .iter()
            .enumerate()
            .map(|(i, tracker)| (UseOneTrackerIdx(i), tracker))
    }

    #[cfg(test)]
    pub(crate) fn check_done(&self, db: &dyn EntityTreeDb) {
        use husky_print_utils::p;

        for tracker in self.0.iter() {
            match tracker.state {
                EntityUseState::Unresolved => {
                    p!(tracker.debug(db));
                    panic!()
                }
                EntityUseState::Resolved | EntityUseState::Erroneous => (),
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct UseOneTracker {
    ast_idx: AstIdx,
    accessibility: Accessibility,
    ident: Identifier,
    use_expr_idx: UseExprIdx,
    parent: Option<EntityPath>,
    state: EntityUseState,
}

impl<Db: EntityTreeDb + ?Sized> salsa::DebugWithDb<Db> for UseOneTracker {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<EntityTreeJar>>::as_jar_db(db);
        f.debug_struct("UseTracker")
            .field("ast_idx", &self.ast_idx)
            .field("accessibility", &self.accessibility.debug(db))
            .field("ident", &self.ident.debug(db))
            .field("use_expr_idx", &self.use_expr_idx)
            .field("parent", &self.parent.debug(db))
            .field("state", &self.state)
            .finish()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EntityUseState {
    Unresolved,
    Resolved,
    Erroneous,
}

impl UseOneTracker {
    pub fn new_root(
        ast_idx: AstIdx,
        accessibility: Accessibility,
        ident: Identifier,
        use_expr_idx: UseExprIdx,
    ) -> Self {
        Self {
            ast_idx,
            accessibility,
            use_expr_idx,
            ident,
            parent: None,
            state: EntityUseState::Unresolved,
        }
    }

    pub fn ast_idx(&self) -> AstIdx {
        self.ast_idx
    }

    pub fn accessibility(&self) -> Accessibility {
        self.accessibility
    }

    pub fn use_expr_idx(&self) -> UseExprIdx {
        self.use_expr_idx
    }

    pub fn state(&self) -> EntityUseState {
        self.state
    }

    pub fn ident(&self) -> Identifier {
        self.ident
    }

    pub(crate) fn mark_as_resolved(&mut self) {
        self.state = EntityUseState::Resolved
    }

    pub(crate) fn is_unresolved(&self) -> bool {
        self.state == EntityUseState::Unresolved
    }
}
