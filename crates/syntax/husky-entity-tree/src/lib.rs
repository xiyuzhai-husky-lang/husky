mod alias;
mod db;
mod error;
mod source;
mod subroute_table;

pub use alias::*;
pub use db::{EntityTreeDb, ModuleFromFileError};
pub use error::*;
pub use source::*;
pub use subroute_table::*;

#[salsa::jar(db = EntityTreeDb)]
pub struct EntityTreeJar();
