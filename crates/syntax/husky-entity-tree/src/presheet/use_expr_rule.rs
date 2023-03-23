use husky_token::IdentToken;
use husky_word::Ident;

use crate::{ParentUseExpr, *};

#[derive(Debug, Default, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub(crate) struct UseExprRules(Vec<UseExprRule>);

impl std::ops::Index<UseExprRuleIdx> for UseExprRules {
    type Output = UseExprRule;

    fn index(&self, index: UseExprRuleIdx) -> &Self::Output {
        &self.0[index.0]
    }
}

impl std::ops::IndexMut<UseExprRuleIdx> for UseExprRules {
    fn index_mut(&mut self, index: UseExprRuleIdx) -> &mut Self::Output {
        &mut self.0[index.0]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UseExprRuleIdx(usize);

impl UseExprRules {
    pub(crate) fn push(&mut self, new_rule: UseExprRule) {
        self.0.push(new_rule)
    }

    pub(crate) fn indexed_iter(&self) -> impl Iterator<Item = (UseExprRuleIdx, &UseExprRule)> {
        self.0
            .iter()
            .enumerate()
            .map(|(i, tracker)| (UseExprRuleIdx(i), tracker))
    }

    #[cfg(test)]
    pub(crate) fn check_done(&self, db: &dyn EntityTreeDb) {
        use husky_print_utils::p;

        for tracker in self.0.iter() {
            match tracker.state {
                UseExprRuleState::Unresolved => {
                    p!(tracker.debug(db));
                    panic!()
                }
                UseExprRuleState::Resolved { .. } | UseExprRuleState::Erroneous => (),
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct UseExprRule {
    ast_idx: AstIdx,
    use_expr_idx: UseExprIdx,
    accessibility: VisibilityProgress,
    variant: UseExprRuleVariant,
    parent: Option<EntityPath>,
    state: UseExprRuleState,
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub enum UseExprRuleVariant {
    Parent {
        parent_name_token: NameToken,
        children: UseExprIdxRange,
    },
    Leaf {
        ident_token: IdentToken,
    },
}

// ad hoc
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub enum VisibilityProgress {
    Done { accessibility: Visibility },
    Todo,
}

impl VisibilityProgress {
    fn new(expr: Option<VisibilityExpr>, module_path: ModulePath) -> Self {
        match expr {
            Some(expr) => match expr {
                VisibilityExpr::Public { .. } => VisibilityProgress::Done {
                    accessibility: Visibility::Public,
                },
                VisibilityExpr::PublicUnder { .. } => todo!(),
            },
            None => VisibilityProgress::Done {
                accessibility: Visibility::PublicUnder(module_path),
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub enum UseExprRuleState {
    Unresolved,
    Resolved { original_symbol: EntitySymbol },
    Erroneous,
}

impl UseExprRule {
    pub fn new_root(
        ast_idx: AstIdx,
        use_expr_root: UseExprRoot,
        accessibility_expr: Option<VisibilityExpr>,
        use_expr_arena: &UseExprArena,
        module_path: ModulePath,
    ) -> Option<Self> {
        let parent_use_expr_idx = use_expr_root.parent_use_expr_idx();
        let ParentUseExpr {
            parent_name_token,
            children,
            ..
        } = parent_use_expr_idx.index(use_expr_arena);
        Some(Self {
            ast_idx,
            use_expr_idx: parent_use_expr_idx.into(),
            accessibility: VisibilityProgress::new(accessibility_expr, module_path),
            parent: None,
            state: UseExprRuleState::Unresolved,
            variant: UseExprRuleVariant::Parent {
                parent_name_token: *parent_name_token,
                children: children.as_ref().ok()?.idx_range(),
            },
        })
    }
    pub fn new_nonroot(
        &self,
        use_expr_idx: UseExprIdx,
        parent: EntityPath,
        variant: UseExprRuleVariant,
    ) -> Self {
        Self {
            ast_idx: self.ast_idx,
            use_expr_idx,
            accessibility: self.accessibility,
            parent: Some(parent),
            state: UseExprRuleState::Unresolved,
            variant,
        }
    }

    pub fn ast_idx(&self) -> AstIdx {
        self.ast_idx
    }

    pub fn state(&self) -> UseExprRuleState {
        self.state
    }

    pub(crate) fn mark_as_resolved(&mut self, original_symbol: EntitySymbol) {
        self.state = UseExprRuleState::Resolved { original_symbol }
    }

    pub(crate) fn is_unresolved(&self) -> bool {
        self.state == UseExprRuleState::Unresolved
    }

    pub(crate) fn parent(&self) -> Option<EntityPath> {
        self.parent
    }

    pub(crate) fn variant(&self) -> &UseExprRuleVariant {
        &self.variant
    }

    pub(crate) fn accessibility(&self) -> Visibility {
        match self.accessibility {
            VisibilityProgress::Done { accessibility } => accessibility,
            VisibilityProgress::Todo => todo!(),
        }
    }

    pub(crate) fn children(&self) -> Option<UseExprIdxRange> {
        todo!()
        // self.children
    }

    pub(crate) fn mark_as_erroneous(&mut self) {
        self.state = UseExprRuleState::Erroneous
    }

    pub fn use_expr_idx(&self) -> ArenaIdx<UseExpr> {
        self.use_expr_idx
    }

    pub(crate) fn ident(&self) -> Option<Ident> {
        match self.variant {
            UseExprRuleVariant::Parent {
                parent_name_token: NameToken::Ident(ident_token),
                ..
            }
            | UseExprRuleVariant::Leaf { ident_token } => Some(ident_token.ident()),
            _ => None,
        }
    }
}
