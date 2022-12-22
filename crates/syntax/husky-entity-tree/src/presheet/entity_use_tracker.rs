use husky_word::Identifier;

use crate::*;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub(crate) struct EntityUseExprTrackers(Vec<EntityUseTracker>);

impl std::ops::Index<EntityUseExprTrackerIdx> for EntityUseExprTrackers {
    type Output = EntityUseTracker;

    fn index(&self, index: EntityUseExprTrackerIdx) -> &Self::Output {
        &self.0[index.0]
    }
}

impl std::ops::IndexMut<EntityUseExprTrackerIdx> for EntityUseExprTrackers {
    fn index_mut(&mut self, index: EntityUseExprTrackerIdx) -> &mut Self::Output {
        &mut self.0[index.0]
    }
}

pub(crate) struct EntityUseExprTrackerIdx(usize);

impl EntityUseExprTrackers {
    pub(crate) fn push(&mut self, tracker: EntityUseTracker) {
        self.0.push(tracker)
    }

    pub(crate) fn indexed_iter(
        &self,
    ) -> impl Iterator<Item = (EntityUseExprTrackerIdx, &EntityUseTracker)> {
        self.0
            .iter()
            .enumerate()
            .map(|(i, tracker)| (EntityUseExprTrackerIdx(i), tracker))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct EntityUseTracker {
    ast_idx: AstIdx,
    accessibility: Accessibility,
    ident: Identifier,
    use_expr_idx: UseExprIdx,
    parent: Option<EntityPath>,
    state: EntityUseState,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EntityUseState {
    Unresolved,
    Resolved,
    Erroneous,
}

impl EntityUseTracker {
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
