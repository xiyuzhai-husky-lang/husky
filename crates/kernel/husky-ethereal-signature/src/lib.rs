mod db;

pub use self::db::*;

use husky_ethereal_term::*;

#[salsa::jar(db = EtherealSignatureDb)]
pub struct EtherealSignatureJar();
