use crate::*;
use husky_entity_path::path::ItemPath;
use husky_ki::{Ki, KiArgument, KiOpn};
use husky_sem_static_var_deps::{
    item_sem_static_var_deps,
    static_var_deps::{SemStaticVarDep, SemStaticVarDeps},
};
use source::KiReprExpansionSource;
use vec_like::OrderedSmallVecSet;

#[salsa::derive_debug_with_db]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct KiStaticVarDeps(OrderedSmallVecSet<ItemPath, 4>);
impl std::ops::Deref for KiStaticVarDeps {
    type Target = OrderedSmallVecSet<ItemPath, 4>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IntoIterator for &KiStaticVarDeps {
    type Item = ItemPath;

    type IntoIter = impl Iterator<Item = ItemPath>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter().copied()
    }
}

// ad hoc, maybe some other consideration about template arguments?
impl From<&SemStaticVarDeps> for KiStaticVarDeps {
    fn from(value: &SemStaticVarDeps) -> Self {
        Self(
            value
                .iter()
                .map(|&dep| match dep {
                    SemStaticVarDep::Item(item_path) => item_path,
                })
                .collect(),
        )
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

impl KiRepr {
    pub fn static_var_deps(self, db: &::salsa::Db) -> &KiStaticVarDeps {
        ki_repr_ki_static_var_deps(db, self)
    }
}

#[salsa::tracked(return_ref)]
fn ki_repr_ki_static_var_deps(db: &::salsa::Db, ki_repr: KiRepr) -> KiStaticVarDeps {
    match ki_repr.source(db) {
        source::KiReprSource::Val(val_path) => item_sem_static_var_deps(val_path, db).into(),
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
