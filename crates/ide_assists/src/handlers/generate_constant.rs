use crate::assist_context::{AssistContext, Assists};
use hir::HirDisplay;
use ide_db::{
    assists::{AssistId, AssistKind},
    defs::NameRefClass,
};
use syntax::ast::{self, edit::IndentLevel};

// Assist: generate_constant
//
// Generate a named constant.
//
// ```
// struct S { i: usize }
// impl S { pub fn new(n: usize) {} }
// fn main() {
//     let v = S::new(CAPA$0CITY);
// }
// ```
// ->
// ```
// struct S { i: usize }
// impl S { pub fn new(n: usize) {} }
// fn main() {
//     const CAPACITY: usize = $0;
//     let v = S::new(CAPACITY);
// }
// ```

pub(crate) fn generate_constant(acc: &mut Assists, ctx: &AssistContext) -> Option<()> {
    todo!()
}
