use crate::*;
use husky_entity_path::path::ItemPath;
use husky_hir_lazy_expr::helpers::hir_lazy_expr_source_map;
use husky_ki::{Ki, KiArgument, KiOpn};
use husky_sem_var_deps::{
    item_sem_var_deps,
    region::item_defn_sem_var_deps_region,
    var_deps::{SemVarDep, SemVarDeps},
};
use source::{KiReprExpansionSource, KiReprSource};
use vec_like::OrderedSmallVecSet;

/// todo: make this an interned value, and there should be a dedicated linket to compute this
#[salsa::derive_debug_with_db]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct KiVarDeps(OrderedSmallVecSet<ItemPath, 4>);

impl std::ops::Deref for KiVarDeps {
    type Target = OrderedSmallVecSet<ItemPath, 4>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IntoIterator for &KiVarDeps {
    type Item = ItemPath;

    type IntoIter = impl Iterator<Item = ItemPath>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter().copied()
    }
}

// ad hoc, maybe some other consideration about template arguments?
impl From<&SemVarDeps> for KiVarDeps {
    fn from(value: &SemVarDeps) -> Self {
        Self(
            value
                .iter()
                .map(|&dep| match dep {
                    SemVarDep::Item(item_path) => item_path,
                })
                .collect(),
        )
    }
}

impl KiVarDeps {
    pub(crate) fn merge(&mut self, other: &Self) {
        self.0.extend(&other.0);
    }

    pub(crate) fn insert(&mut self, item_path: ItemPath) {
        self.0.insert(item_path);
    }
}

impl KiRepr {
    pub fn var_deps(self, db: &::salsa::Db) -> &KiVarDeps {
        ki_repr_var_deps(db, self)
    }
}

// ad hoc
#[salsa::tracked(return_ref)]
fn ki_repr_var_deps(db: &::salsa::Db, ki_repr: KiRepr) -> KiVarDeps {
    match ki_repr.source(db) {
        KiReprSource::Val(val_path) => item_sem_var_deps(val_path, db).into(),
        KiReprSource::Expansion {
            parent_ki_repr,
            source,
        } => {
            let parent_item_path: ItemPath = match parent_ki_repr.opn(db) {
                KiOpn::Return => todo!(),
                KiOpn::Require => todo!(),
                KiOpn::Assert => todo!(),
                KiOpn::Val(path) => path.into(),
                KiOpn::FunctionRitchie(_) => todo!(),
                KiOpn::Prefix(_) => todo!(),
                KiOpn::Suffix(_) => todo!(),
                KiOpn::Binary(_) => todo!(),
                KiOpn::Linket(_) => todo!(),
                KiOpn::EvalDiscarded => todo!(),
                KiOpn::Literal(_) => todo!(),
                KiOpn::Branches => todo!(),
                KiOpn::TypeVariant(_) => todo!(),
                KiOpn::Be { pattern_data } => todo!(),
                KiOpn::Unwrap {} => todo!(),
                KiOpn::Index => todo!(),
            };
            let region = item_defn_sem_var_deps_region(db, *parent_item_path).unwrap();
            let parent_expr_source_map_data = hir_lazy_expr_source_map(parent_item_path, db)
                .unwrap()
                .data(db);
            match source {
                KiReprExpansionSource::LetVariable { variable_idx, .. } => {
                    let current_variable_idx = parent_expr_source_map_data
                        .hir_lazy_to_current_variable(variable_idx)
                        .unwrap();
                    (&region.variable_var_deps_table(db)[current_variable_idx]).into()
                }
                // ad hoc, refine this
                KiReprExpansionSource::RequireCondition { stmt }
                | KiReprExpansionSource::AssertCondition { stmt }
                | KiReprExpansionSource::IfCondition { stmt }
                | KiReprExpansionSource::ElifCondition { stmt, .. } => {
                    let stmt = parent_expr_source_map_data
                        .hir_lazy_to_sem_stmt_idx(stmt)
                        .unwrap();
                    (&region.stmt_value_var_deps_table(db)[stmt]).into()
                }
                KiReprExpansionSource::Expr { expr } => {
                    let expr = parent_expr_source_map_data
                        .hir_lazy_to_sem_expr_idx(expr)
                        .unwrap();
                    (&region.expr_value_var_deps_table(db)[expr]).into()
                }
                KiReprExpansionSource::RequireDefault { stmt }
                | KiReprExpansionSource::Stmt { stmt } => {
                    let stmt = parent_expr_source_map_data
                        .hir_lazy_to_sem_stmt_idx(stmt)
                        .unwrap();
                    (&region.stmt_value_var_deps_table(db)[stmt]).into()
                }
            }
        }
    }
}

impl KiDomainRepr {
    pub fn var_deps(self, db: &::salsa::Db) -> Option<&KiVarDeps> {
        match self {
            KiDomainRepr::Omni => None,
            KiDomainRepr::ConditionSatisfied(ki_repr)
            | KiDomainRepr::ConditionNotSatisfied(ki_repr)
            | KiDomainRepr::StmtNotReturned(ki_repr)
            | KiDomainRepr::ExprNotReturned(ki_repr) => Some(ki_repr.var_deps(db)),
        }
    }
}
