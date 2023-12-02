use crate::{ParentUseExpr, *};
use husky_coword::Ident;
use husky_token::{IdentToken, PathNameToken};

#[derive(Debug, Default, PartialEq, Eq, Clone)]
#[salsa::debug_with_db(db = EntitySynTreeDb, jar = EntitySynTreeJar)]
pub(crate) struct UseOneRules(Vec<UseOneRule>);

impl std::ops::Index<UseOneRuleIdx> for UseOneRules {
    type Output = UseOneRule;

    fn index(&self, index: UseOneRuleIdx) -> &Self::Output {
        &self.0[index.0]
    }
}

impl std::ops::IndexMut<UseOneRuleIdx> for UseOneRules {
    fn index_mut(&mut self, index: UseOneRuleIdx) -> &mut Self::Output {
        &mut self.0[index.0]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UseOneRuleIdx(usize);

impl UseOneRules {
    pub(crate) fn push(&mut self, new_rule: UseOneRule) {
        self.0.push(new_rule)
    }

    pub(crate) fn indexed_iter(&self) -> impl Iterator<Item = (UseOneRuleIdx, &UseOneRule)> {
        self.0
            .iter()
            .enumerate()
            .map(|(i, tracker)| (UseOneRuleIdx(i), tracker))
    }

    #[cfg(test)]
    pub(crate) fn check_done(&self, db: &::salsa::Db) {
        use husky_print_utils::p;

        for tracker in self.0.iter() {
            match tracker.state {
                UseOneRuleState::Unresolved => {
                    p!(tracker.debug(db));
                    panic!()
                }
                UseOneRuleState::Resolved { .. } | UseOneRuleState::Erroneous => (),
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::debug_with_db(db = EntitySynTreeDb, jar = EntitySynTreeJar)]
pub struct UseOneRule {
    ast_idx: AstIdx,
    use_expr_idx: UseExprIdx,
    visibility: Scope,
    variant: OnceUseRuleVariant,
    parent: Option<MajorEntityPath>,
    state: UseOneRuleState,
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::debug_with_db(db = EntitySynTreeDb, jar = EntitySynTreeJar)]
pub enum OnceUseRuleVariant {
    Parent {
        parent_name_token: PathNameToken,
        children: UseExprIdxRange,
    },
    Leaf {
        ident_token: IdentToken,
    },
    UseAllTypeVariants {
        parent_ty_path: TypePath,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = EntitySynTreeDb, jar = EntitySynTreeJar)]
pub enum UseOneRuleState {
    Unresolved,
    Resolved {
        original_symbol: Option<EntitySymbol>,
    },
    Erroneous,
}

impl UseOneRule {
    pub fn new_root(
        ast_idx: AstIdx,
        use_expr_root: UseExprRoot,
        visibility_expr: &VisibilityExpr,
        use_expr_arena: &UseExprArena,
        _module_path: ModulePath,
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
            visibility: visibility_expr.visibility(),
            parent: None,
            state: UseOneRuleState::Unresolved,
            variant: OnceUseRuleVariant::Parent {
                parent_name_token: *parent_name_token,
                children: children.as_ref().ok()?.idx_range(),
            },
        })
    }
    pub fn new_nonroot(
        &self,
        use_expr_idx: UseExprIdx,
        parent: MajorEntityPath,
        variant: OnceUseRuleVariant,
    ) -> Self {
        Self {
            ast_idx: self.ast_idx,
            use_expr_idx,
            visibility: self.visibility,
            parent: Some(parent),
            state: UseOneRuleState::Unresolved,
            variant,
        }
    }

    #[inline(always)]
    pub fn ast_idx(&self) -> AstIdx {
        self.ast_idx
    }

    #[inline(always)]
    pub fn state(&self) -> UseOneRuleState {
        self.state
    }

    #[inline(always)]
    pub(crate) fn mark_as_resolved(&mut self, original_symbol: impl Into<Option<EntitySymbol>>) {
        self.state = UseOneRuleState::Resolved {
            original_symbol: original_symbol.into(),
        }
    }

    #[inline(always)]
    pub(crate) fn is_unresolved(&self) -> bool {
        self.state == UseOneRuleState::Unresolved
    }

    pub(crate) fn parent(&self) -> Option<MajorEntityPath> {
        self.parent
    }

    pub(crate) fn variant(&self) -> &OnceUseRuleVariant {
        &self.variant
    }

    pub(crate) fn visibility(&self) -> Scope {
        self.visibility
    }

    pub(crate) fn children(&self) -> Option<UseExprIdxRange> {
        todo!()
        // self.children
    }

    pub(crate) fn mark_as_erroneous(&mut self) {
        self.state = UseOneRuleState::Erroneous
    }

    pub fn use_expr_idx(&self) -> ArenaIdx<UseExpr> {
        self.use_expr_idx
    }

    pub(crate) fn ident(&self) -> Option<Ident> {
        match self.variant {
            OnceUseRuleVariant::Parent {
                parent_name_token: PathNameToken::Ident(ident_token),
                ..
            }
            | OnceUseRuleVariant::Leaf { ident_token } => Some(ident_token.ident()),
            _ => None,
        }
    }
}

impl<'a> std::ops::Index<UseOneRuleIdx> for EntityTreePresheetMut<'a> {
    type Output = UseOneRule;

    fn index(&self, index: UseOneRuleIdx) -> &Self::Output {
        &self.use_one_rules[index]
    }
}

impl<'a> std::ops::IndexMut<UseOneRuleIdx> for EntityTreePresheetMut<'a> {
    fn index_mut(&mut self, index: UseOneRuleIdx) -> &mut Self::Output {
        &mut self.use_one_rules[index]
    }
}
