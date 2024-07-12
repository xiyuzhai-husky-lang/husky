#![feature(generic_const_exprs)]
#![feature(impl_trait_in_assoc_type)]
//! warning: this crate is mostly copied from `husky-sem-static-mut-deps` and adapted in an ad hoc way.
//! there should be a lot more to adapt, so expects bugs.
mod builder;
mod graph_dynamics;
pub mod jar;
pub mod region;
pub mod static_var_deps;
#[cfg(test)]
mod tests;

use self::jar::SemStaticVarDepsJar as Jar;
use self::static_var_deps::SemStaticVarDeps;
#[cfg(test)]
use self::tests::*;
use husky_entity_path::path::ItemPath;

pub fn item_sem_static_var_deps<'db>(
    item_path: ItemPath,
    db: &'db ::salsa::Db,
) -> &'db SemStaticVarDeps {
    crate::graph_dynamics::item_sem_static_var_deps(db, *item_path)
}
