#![feature(generic_const_exprs)]
#![feature(impl_trait_in_assoc_type)]
//! warning: this crate is mostly copied from `husky-sem-static-mut-deps` and adapted in an ad hoc way.
//! there should be a lot more to adapt, so expects bugs.
//!
//! var includes both static var and compterm var, thus the name is var
mod builder;
mod graph_dynamics;
pub mod helpers;
pub mod jar;
pub mod region;
#[cfg(test)]
mod tests;
pub mod var_deps;

use self::jar::SemVarDepsJar as Jar;
#[cfg(test)]
use self::tests::*;
use self::var_deps::value::SemValueVarDeps;
use husky_entity_path::path::ItemPath;

pub fn item_sem_var_deps<'db>(
    item_path: impl Into<ItemPath>,
    db: &'db ::salsa::Db,
) -> &'db SemValueVarDeps {
    crate::graph_dynamics::item_sem_var_deps(db, *item_path.into())
}
