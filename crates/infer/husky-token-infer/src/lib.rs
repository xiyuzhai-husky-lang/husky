mod db;
#[cfg(test)]
mod tests;

pub use db::*;

use husky_token::*;
use husky_vfs::*;

#[salsa::jar(db =  TokenInferDb)]
pub struct TokenInferJar();
