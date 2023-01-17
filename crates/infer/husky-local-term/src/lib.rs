mod db;
mod intern;
mod local_term;

pub use db::*;
pub use intern::*;
pub use local_term::*;

use husky_term::*;

#[salsa::jar(db = LocalTermDb)]
pub struct LocalTermJar();
