use crate::*;
use husky_entity_path::path::ItemPath;
use husky_hir_lazy_expr::helpers::hir_lazy_expr_source_map;
use husky_ki::{Ki, KiArgument, KiOpn};
use husky_sem_var_deps::{
    item_sem_var_deps,
    region::item_defn_sem_var_deps_region,
    var_deps::{SemVarDep, SemVarDeps},
};
use source::KiReprExpansionSource;
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
        source::KiReprSource::Val(val_path) => item_sem_var_deps(val_path, db).into(),
        source::KiReprSource::Expansion {
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
                KiReprExpansionSource::LetVariable { stmt, variable_idx } => todo!(),
                KiReprExpansionSource::RequireDefault { stmt } => todo!(),
                KiReprExpansionSource::RequireCondition { stmt } => todo!(),
                KiReprExpansionSource::AssertCondition { stmt } => todo!(),
                KiReprExpansionSource::IfCondition { stmt } => todo!(),
                KiReprExpansionSource::ElifCondition { stmt, branch_idx } => todo!(),
                KiReprExpansionSource::Expr { expr } => {
                    let expr = parent_expr_source_map_data
                        .hir_lazy_to_sem_expr_idx(expr)
                        .unwrap();
                    (&region.expr_value_var_deps_table(db)[expr]).into()
                }
                KiReprExpansionSource::Stmt { stmt } => todo!(),
            }
        }
    }
}
