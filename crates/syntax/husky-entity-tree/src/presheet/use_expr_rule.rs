use husky_token::IdentifierToken;
use husky_word::Identifier;

use crate::*;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub(crate) struct UseTreeRules(Vec<UseTreeRule>);

impl<Db: EntityTreeDb + ?Sized> salsa::DebugWithDb<Db> for UseTreeRules {
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

impl std::ops::Index<UseTreeRuleIdx> for UseTreeRules {
    type Output = UseTreeRule;

    fn index(&self, index: UseTreeRuleIdx) -> &Self::Output {
        &self.0[index.0]
    }
}

impl std::ops::IndexMut<UseTreeRuleIdx> for UseTreeRules {
    fn index_mut(&mut self, index: UseTreeRuleIdx) -> &mut Self::Output {
        &mut self.0[index.0]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct UseTreeRuleIdx(usize);

impl UseTreeRules {
    pub(crate) fn push(&mut self, new_rule: UseTreeRule) {
        self.0.push(new_rule)
    }

    pub(crate) fn indexed_iter(&self) -> impl Iterator<Item = (UseTreeRuleIdx, &UseTreeRule)> {
        self.0
            .iter()
            .enumerate()
            .map(|(i, tracker)| (UseTreeRuleIdx(i), tracker))
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
pub(crate) struct UseTreeRule {
    ast_idx: AstIdx,
    accessibility: AccessibilityProgress,
    ident_token: IdentifierToken,
    children: UseExprIdxRange,
    parent: Option<EntityPath>,
    state: EntityUseState,
}

// ad hoc
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AccessibilityProgress {
    Done { accessibility: Accessibility },
    Todo,
}

impl<Db: EntityTreeDb + ?Sized> salsa::DebugWithDb<Db> for AccessibilityProgress {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<EntityTreeJar>>::as_jar_db(db);
        match self {
            AccessibilityProgress::Done { accessibility } => f
                .debug_struct("Done")
                .field("accessibility", &accessibility.debug(db))
                .finish(),
            AccessibilityProgress::Todo => f.debug_struct("Todo").finish(),
        }
    }
}

impl AccessibilityProgress {
    fn new(expr: Option<AccessibilityExpr>, module_path: ModulePath) -> Self {
        match expr {
            Some(expr) => match expr {
                AccessibilityExpr::Public { .. } => AccessibilityProgress::Done {
                    accessibility: Accessibility::Public,
                },
                AccessibilityExpr::PublicUnder { scope, .. } => todo!(),
            },
            None => AccessibilityProgress::Done {
                accessibility: Accessibility::PublicUnder(module_path),
            },
        }
    }
}

impl<Db: EntityTreeDb + ?Sized> salsa::DebugWithDb<Db> for UseTreeRule {
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
            .field("use_tree_expr_children", &self.children)
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

impl UseTreeRule {
    pub fn new_root(
        ast_idx: AstIdx,
        accessibility_expr: Option<AccessibilityExpr>,
        use_tree_expr: &UseExpr,
        module_path: ModulePath,
    ) -> Self {
        match use_tree_expr {
            UseExpr::All { star_token } => todo!(),
            UseExpr::One { ident_token } => todo!(),
            UseExpr::Parent {
                ident_token,
                scope_resolution_token,
                children,
            } => Self {
                ast_idx,
                accessibility: AccessibilityProgress::new(accessibility_expr, module_path),
                parent: None,
                state: EntityUseState::Unresolved,
                ident_token: *ident_token,
                children: children.idx_range(),
            },
            UseExpr::Err(_) => todo!(),
        }
    }
    pub fn new_child(
        &self,
        ident_token: IdentifierToken,
        use_tree_expr_idx: UseExprIdx,
        parent: EntityPath,
        children: UseExprIdxRange,
    ) -> Self {
        Self {
            ast_idx: self.ast_idx,
            accessibility: self.accessibility,
            ident_token,
            parent: Some(parent),
            state: EntityUseState::Unresolved,
            children,
        }
    }

    pub fn ast_idx(&self) -> AstIdx {
        self.ast_idx
    }

    pub fn state(&self) -> EntityUseState {
        self.state
    }

    pub(crate) fn mark_as_resolved(&mut self) {
        self.state = EntityUseState::Resolved
    }

    pub(crate) fn is_unresolved(&self) -> bool {
        self.state == EntityUseState::Unresolved
    }

    pub(crate) fn parent(&self) -> Option<EntityPath> {
        self.parent
    }

    pub(crate) fn ident_token(&self) -> IdentifierToken {
        self.ident_token
    }

    pub(crate) fn accessibility(&self) -> Accessibility {
        match self.accessibility {
            AccessibilityProgress::Done { accessibility } => accessibility,
            AccessibilityProgress::Todo => todo!(),
        }
    }

    pub(crate) fn children(&self) -> UseExprIdxRange {
        self.children
    }
}
