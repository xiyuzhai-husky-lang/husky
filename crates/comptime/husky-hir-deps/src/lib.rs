pub mod db;
mod deps;

pub use self::db::*;
pub use self::deps::*;

pub trait HasDeps: Copy {
    type Deps;

    fn deps(self, db: &dyn HirDepsDb) -> Self::Deps;
}
