use hir::HasSource;
use syntax::ast;

use crate::{
    assist_context::{AssistContext, Assists},
    utils::{
        add_trait_assoc_items_to_impl, filter_assoc_items, render_snippet, Cursor, DefaultMethods,
    },
    AssistId, AssistKind,
};

// Assist: add_impl_missing_members
//
// Adds scaffold for required impl members.
//
// ```
// trait Trait<T> {
//     type X;
//     fn foo(&self) -> T;
//     fn bar(&self) {}
// }
//
// impl Trait<u32> for () {$0
//
// }
// ```
// ->
// ```
// trait Trait<T> {
//     type X;
//     fn foo(&self) -> T;
//     fn bar(&self) {}
// }
//
// impl Trait<u32> for () {
//     $0type X;
//
//     fn foo(&self) -> u32 {
//         todo!()
//     }
// }
// ```
pub(crate) fn add_missing_impl_members(acc: &mut Assists, ctx: &AssistContext) -> Option<()> {
    add_missing_impl_members_inner(
        acc,
        ctx,
        DefaultMethods::No,
        "add_impl_missing_members",
        "Implement missing members",
    )
}

// Assist: add_impl_default_members
//
// Adds scaffold for overriding default impl members.
//
// ```
// trait Trait {
//     type X;
//     fn foo(&self);
//     fn bar(&self) {}
// }
//
// impl Trait for () {
//     type X = ();
//     fn foo(&self) {}$0
// }
// ```
// ->
// ```
// trait Trait {
//     type X;
//     fn foo(&self);
//     fn bar(&self) {}
// }
//
// impl Trait for () {
//     type X = ();
//     fn foo(&self) {}
//
//     $0fn bar(&self) {}
// }
// ```
pub(crate) fn add_missing_default_members(acc: &mut Assists, ctx: &AssistContext) -> Option<()> {
    add_missing_impl_members_inner(
        acc,
        ctx,
        DefaultMethods::Only,
        "add_impl_default_members",
        "Implement default members",
    )
}

fn add_missing_impl_members_inner(
    acc: &mut Assists,
    ctx: &AssistContext,
    mode: DefaultMethods,
    assist_id: &'static str,
    label: &'static str,
) -> Option<()> {
    todo!()
}

fn try_gen_trait_body(
    ctx: &AssistContext,
    func: &ast::Fn,
    trait_: &hir::Trait,
    impl_def: &ast::Impl,
) -> Option<()> {
    todo!()
}
