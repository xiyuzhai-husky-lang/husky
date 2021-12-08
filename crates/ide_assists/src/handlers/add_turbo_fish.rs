use husky_lang_db::defs::{Definition, NameRefClass};
use syntax::{ast, SyntaxKind};

use crate::{
    assist_context::{AssistContext, Assists},
    AssistId, AssistKind,
};

// Assist: add_turbo_fish
//
// Adds `::<_>` to a call of a generic method or function.
//
// ```
// fn make<T>() -> T { todo!() }
// fn main() {
//     let x = make$0();
// }
// ```
// ->
// ```
// fn make<T>() -> T { todo!() }
// fn main() {
//     let x = make::<${0:_}>();
// }
// ```
pub(crate) fn add_turbo_fish(acc: &mut Assists, ctx: &AssistContext) -> Option<()> {
    todo!()
}
