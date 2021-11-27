#![allow(dead_code)]
#![allow(unused_macros)]
#![allow(unused_imports)]
#![allow(warnings, unused)]
pub mod defn;
pub mod entity;
pub mod ovld;
pub mod project;
mod utils;

struct Component {}
enum ComponentVariant {
    Entity,
    Expr,
    Stmt,
}

pub use common::*;
pub use entity::*;
pub use ovld::*;
pub use project::*;
pub use syntax::*;

pub struct SemanticSess {}
impl SemanticSess {
    pub fn new(_ast: &AST) -> SemanticSess {
        SemanticSess {}
    }
}
