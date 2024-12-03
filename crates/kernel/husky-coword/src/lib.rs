mod ident;
pub mod jar;
mod kebab;
mod label;
mod menu;
mod style;
#[cfg(test)]
mod tests;

pub use self::ident::*;
pub use self::kebab::*;
pub use self::label::*;
pub use self::menu::*;
pub use self::style::*;
pub use base_coword::Coword;

use self::jar::CowordJar as Jar;
use salsa::Db;
