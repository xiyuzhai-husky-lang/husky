mod alias;
mod db;
mod error;
mod subroute_table;

pub use alias::*;
pub use db::{EntityTreeDb, ModuleFromFileError};
pub use error::*;
pub use subroute_table::*;

#[salsa::jar(db = EntityTreeDb)]
pub struct EntityTreeJar();
