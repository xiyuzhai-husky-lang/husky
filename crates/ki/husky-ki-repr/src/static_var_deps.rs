use crate::*;
use husky_entity_path::path::ItemPath;
use husky_ki::{Ki, KiArgument, KiOpn};
use husky_sem_static_var_deps::static_var_deps::SemStaticVarDeps;
use source::KiReprExpansionSource;
use vec_like::OrderedSmallVecSet;

#[salsa::derive_debug_with_db]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct KiStaticVarDeps(OrderedSmallVecSet<ItemPath, 4>);

// ad hoc, maybe some other consideration about template arguments?
impl From<&SemStaticVarDeps> for KiStaticVarDeps {
    fn from(value: &SemStaticVarDeps) -> Self {
        Self((**value).clone())
    }
}

impl KiStaticVarDeps {
    pub(crate) fn merge(&mut self, other: &Self) {
        self.0.extend(&other.0);
    }

    pub(crate) fn insert(&mut self, item_path: ItemPath) {
        self.0.insert(item_path);
    }
}

#[salsa::tracked]
fn ki_repr_ki_static_var_deps(db: &::salsa::Db, ki_repr: KiRepr) -> KiStaticVarDeps {
    match ki_repr.source(db) {
        source::KiReprSource::Val(_) => todo!(),
        source::KiReprSource::Expansion {
            parent_ki_repr,
            source,
        } => match source {
            KiReprExpansionSource::LetVariable { stmt, variable_idx } => todo!(),
            KiReprExpansionSource::RequireDefault { stmt } => todo!(),
            KiReprExpansionSource::RequireCondition { stmt } => todo!(),
            KiReprExpansionSource::AssertCondition { stmt } => todo!(),
            KiReprExpansionSource::IfCondition { stmt } => todo!(),
            KiReprExpansionSource::ElifCondition { stmt, branch_idx } => todo!(),
            KiReprExpansionSource::Expr { expr } => todo!(),
            KiReprExpansionSource::Stmt { stmt } => todo!(),
        },
    }
}
