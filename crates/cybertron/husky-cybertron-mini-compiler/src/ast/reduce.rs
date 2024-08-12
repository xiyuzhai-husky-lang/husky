pub mod call;
pub mod defn;
pub mod delimited;
pub mod opr;
pub mod stmt;
mod utils;

use self::{
    call::reduce_by_call, defn::reduce_by_defn, delimited::reduce_by_delimited, opr::reduce_by_opr,
    show::show_asts, stmt::reduce_by_stmt, utils::add_pre_asts,
};
use super::*;
use husky_cybertron::{
    debug::{is_debug, set_debug},
    prelude::*,
    seq::any::AnySeq,
};
use indexmap::IndexMap;

pub fn reduce(
    pre_asts: Seq<Option<PreAst>>,
    allocated_asts: Seq<Option<Ast>>,
) -> (Seq<Option<PreAst>>, Seq<Option<Ast>>) {
    let (pre_asts, allocated_asts) = reduce_by_opr(pre_asts, allocated_asts);
    let (pre_asts, allocated_asts) = reduce_by_delimited(pre_asts, allocated_asts);
    let (pre_asts, allocated_asts) = reduce_by_call(pre_asts, allocated_asts);
    // let (pre_asts, allocated_asts) = reduce_by_stmt(pre_asts, allocated_asts);
    // let (pre_asts, allocated_asts) = reduce_by_defn(pre_asts, allocated_asts);
    (pre_asts, allocated_asts)
}

pub fn reduce_n_times(
    mut pre_asts: Seq<Option<PreAst>>,
    mut allocated_asts: Seq<Option<Ast>>,
    n: usize,
) -> (Seq<Option<PreAst>>, Seq<Option<Ast>>) {
    for _ in 0..n {
        let (pre_asts1, allocated_asts1) = reduce(pre_asts, allocated_asts);
        pre_asts = pre_asts1;
        allocated_asts = allocated_asts1;
    }
    (pre_asts, allocated_asts)
}

#[cfg(test)]
fn t(input: &str, n: usize, expect: Expect) {
    let tokens = tokenize(input);
    let pre_asts = calc_pre_ast_initial_seq(tokens);
    let allocated_asts: Seq<Option<Ast>> = tokens.map(|token| token.into());
    let (pre_asts, allocated_asts) = reduce_n_times(pre_asts, allocated_asts, n);
    expect.assert_debug_eq(&show_asts(tokens, allocated_asts));
}

#[test]
fn reduce_n_times_works1() {
    t(
        "1",
        0,
        expect![[r#"
            [
                `1`: "1",
            ]
        "#]],
    );
    t(
        "1+1",
        1,
        expect![[r#"
            [
                `1`: "1",
                `+`: "1 + 1",
                `1`: "1",
            ]
        "#]],
    );
}

#[test]
fn reduce_n_times_works2() {
    t(
        "()",
        1,
        expect![[r#"
            [
                `(`: "",
                `)`: "()",
            ]
        "#]],
    );
    t(
        "(,)",
        1,
        expect![[r#"
            [
                `(`: "",
                `,`: ",",
                `)`: "(,)",
            ]
        "#]],
    );
    t(
        "(,,)",
        1,
        expect![[r#"
            [
                `(`: "",
                `,`: ",",
                `,`: ",",
                `)`: "(,,)",
            ]
        "#]],
    );
    t(
        "(1)",
        1,
        expect![[r#"
            [
                `(`: "",
                `1`: "1",
                `)`: "(1)",
            ]
        "#]],
    );
    t(
        "(1,)",
        1,
        expect![[r#"
            [
                `(`: "",
                `1`: "1",
                `,`: "1,",
                `)`: "(1,)",
            ]
        "#]],
    );
    t(
        "(1,,)",
        1,
        expect![[r#"
            [
                `(`: "",
                `1`: "1",
                `,`: "1,",
                `,`: ",",
                `)`: "(1,,)",
            ]
        "#]],
    );
    t(
        "(1,1)",
        1,
        expect![[r#"
            [
                `(`: "",
                `1`: "1",
                `,`: "1,",
                `1`: "1",
                `)`: "(1,1)",
            ]
        "#]],
    );
    t(
        "((1,1),1)",
        1,
        expect![[r#"
            [
                `(`: "",
                `(`: "",
                `1`: "1",
                `,`: "1,",
                `1`: "1",
                `)`: "(1,1)",
                `,`: "",
                `1`: "1",
                `)`: "",
            ]
        "#]],
    );
}

#[test]
fn reduce_n_times_works3() {
    t(
        "{}",
        1,
        expect![[r#"
            [
                `{`: "",
                `}`: "{}",
            ]
        "#]],
    );
    t(
        "{,}",
        1,
        expect![[r#"
            [
                `{`: "",
                `,`: ",",
                `}`: "{,}",
            ]
        "#]],
    );
    t(
        "{,,}",
        1,
        expect![[r#"
            [
                `{`: "",
                `,`: ",",
                `,`: ",",
                `}`: "{,,}",
            ]
        "#]],
    );
    t(
        "{1}",
        1,
        expect![[r#"
            [
                `{`: "",
                `1`: "1",
                `}`: "{1}",
            ]
        "#]],
    );
    t(
        "{1,}",
        1,
        expect![[r#"
            [
                `{`: "",
                `1`: "1",
                `,`: "1,",
                `}`: "{1,}",
            ]
        "#]],
    );
    t(
        "{1,,}",
        1,
        expect![[r#"
            [
                `{`: "",
                `1`: "1",
                `,`: "1,",
                `,`: ",",
                `}`: "{1,,}",
            ]
        "#]],
    );
    t(
        "{1,1}",
        1,
        expect![[r#"
            [
                `{`: "",
                `1`: "1",
                `,`: "1,",
                `1`: "1",
                `}`: "{1,1}",
            ]
        "#]],
    );
    t(
        "{{1,1},1}",
        1,
        expect![[r#"
            [
                `{`: "",
                `{`: "",
                `1`: "1",
                `,`: "1,",
                `1`: "1",
                `}`: "{1,1}",
                `,`: "",
                `1`: "1",
                `}`: "",
            ]
        "#]],
    );
}

#[test]
fn reduce_n_times_works4() {
    t(
        "{}",
        1,
        expect![[r#"
            [
                `{`: "",
                `}`: "{}",
            ]
        "#]],
    );
    t(
        "{;}",
        1,
        expect![[r#"
            [
                `{`: "",
                `;`: ";",
                `}`: "{;}",
            ]
        "#]],
    );
    t(
        "{;;}",
        1,
        expect![[r#"
            [
                `{`: "",
                `;`: ";",
                `;`: ";",
                `}`: "{;;}",
            ]
        "#]],
    );
    t(
        "{1}",
        1,
        expect![[r#"
            [
                `{`: "",
                `1`: "1",
                `}`: "{1}",
            ]
        "#]],
    );
    t(
        "{1;}",
        1,
        expect![[r#"
            [
                `{`: "",
                `1`: "1",
                `;`: "1;",
                `}`: "{1;}",
            ]
        "#]],
    );
    t(
        "{1;;}",
        1,
        expect![[r#"
            [
                `{`: "",
                `1`: "1",
                `;`: "1;",
                `;`: ";",
                `}`: "{1;;}",
            ]
        "#]],
    );
    t(
        "{1;1}",
        1,
        expect![[r#"
            [
                `{`: "",
                `1`: "1",
                `;`: "1;",
                `1`: "1",
                `}`: "{1;1}",
            ]
        "#]],
    );
    t(
        "{{1;1};1}",
        1,
        expect![[r#"
            [
                `{`: "",
                `{`: "",
                `1`: "1",
                `;`: "1;",
                `1`: "1",
                `}`: "{1;1}",
                `;`: "",
                `1`: "1",
                `}`: "",
            ]
        "#]],
    );
}

#[test]
fn reduce_n_times_works5() {
    t(
        "f()",
        2,
        expect![[r#"
            [
                `f`: "f",
                `(`: "f()",
                `)`: "()",
            ]
        "#]],
    );
    t(
        "f()()",
        2,
        expect![[r#"
            [
                `f`: "f",
                `(`: "f()",
                `)`: "()",
                `(`: "f()()",
                `)`: "()",
            ]
        "#]],
    );
    t(
        "f()+1",
        2,
        expect![[r#"
            [
                `f`: "f",
                `(`: "f()",
                `)`: "()",
                `+`: "f() + 1",
                `1`: "1",
            ]
        "#]],
    );
    t(
        "A {}",
        2,
        expect![[r#"
            [
                `A`: "A",
                `{`: "A{}",
                `}`: "{}",
            ]
        "#]],
    );
    // t(
    //     "{;}",
    //     1,
    //     expect![[r#"
    //         [
    //             `{`: "",
    //             `;`: ";",
    //             `}`: "{;}",
    //         ]
    //     "#]],
    // );
    // t(
    //     "{;;}",
    //     1,
    //     expect![[r#"
    //         [
    //             `{`: "",
    //             `;`: ";",
    //             `;`: ";",
    //             `}`: "{;;}",
    //         ]
    //     "#]],
    // );
    // t(
    //     "{1}",
    //     1,
    //     expect![[r#"
    //         [
    //             `{`: "",
    //             `1`: "1",
    //             `}`: "{1}",
    //         ]
    //     "#]],
    // );
    // t(
    //     "{1;}",
    //     1,
    //     expect![[r#"
    //         [
    //             `{`: "",
    //             `1`: "1",
    //             `;`: "1;",
    //             `}`: "{1;}",
    //         ]
    //     "#]],
    // );
    // t(
    //     "{1;;}",
    //     1,
    //     expect![[r#"
    //         [
    //             `{`: "",
    //             `1`: "1",
    //             `;`: "1;",
    //             `;`: ";",
    //             `}`: "{1;;}",
    //         ]
    //     "#]],
    // );
    // t(
    //     "{1;1}",
    //     1,
    //     expect![[r#"
    //         [
    //             `{`: "",
    //             `1`: "1",
    //             `;`: "1;",
    //             `1`: "1",
    //             `}`: "{1;1}",
    //         ]
    //     "#]],
    // );
    // t(
    //     "{{1;1};1}",
    //     2,
    //     expect![[r#"
    //         [
    //             `{`: "",
    //             `{`: "",
    //             `1`: "1",
    //             `;`: "1;",
    //             `1`: "1",
    //             `}`: "{1;1}",
    //             `;`: "",
    //             `1`: "1",
    //             `}`: "",
    //         ]
    //     "#]],
    // );
}
