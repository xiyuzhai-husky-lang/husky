mod db;
mod intern;
mod local_term;
mod sheet;

pub use db::*;
use husky_term::*;
pub use intern::*;
pub use local_term::*;
pub use sheet::*;

#[salsa::jar(db = LocalTermDb)]
pub struct LocalTermJar();
