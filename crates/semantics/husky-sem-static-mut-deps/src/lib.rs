#![feature(generic_const_exprs)]
#![feature(impl_trait_in_assoc_type)]
pub mod builder;
mod graph_dynamics;
pub mod jar;
pub mod region;
pub mod static_mut_deps;
#[cfg(test)]
mod tests;

use self::jar::SemStaticMutDepsJar as Jar;
use self::static_mut_deps::SemStaticMutDeps;
#[cfg(test)]
use self::tests::*;
use husky_entity_path::path::ItemPath;

pub fn item_sem_static_mut_deps<'db>(
    item_path: ItemPath,
    db: &'db ::salsa::Db,
) -> &'db SemStaticMutDeps {
    crate::graph_dynamics::item_sem_static_mut_deps(db, *item_path)
}
