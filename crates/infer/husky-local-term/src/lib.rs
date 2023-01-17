mod db;
mod intern;
mod local_term;
mod sheet;

pub use db::*;
pub use intern::*;
pub use local_term::*;
pub use sheet::*;

use husky_term::*;

#[salsa::jar(db = LocalTermDb)]
pub struct LocalTermJar();
