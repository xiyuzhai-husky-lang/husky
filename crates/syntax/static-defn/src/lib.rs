mod root;

pub use root::*;

use static_decl::*;

#[derive(Debug, PartialEq, Eq)]
pub struct StaticEntityDefn {
    pub subscopes: &'static [(&'static str, &'static StaticEntityDefn)],
    pub decl: StaticEntityDecl,
}
