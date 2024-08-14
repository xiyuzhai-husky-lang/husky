use super::*;

pub(super) fn reduce_by_defn(
    pre_asts: Seq<Option<PreAst>>,
    allocated_asts: Seq<Option<Ast>>,
) -> (Seq<Option<PreAst>>, Seq<Option<Ast>>) {
    todo!()
}

#[test]
fn reduce_n_times_for_defn_works1() {
    t(
        "struct A { x: i32, y: i32 }",
        2,
        expect![[r#"
            [
                `struct`: `struct` ✓,
                `A`: "A" ✓,
                `{`: `{`,
                `x`: "x",
                `:`: "x : i32",
                `i32`: "i32",
                `,`: "x : i32, ",
                `y`: "y",
                `:`: "y : i32",
                `i32`: "i32",
                `}`: "{ x : i32, y : i32 }" ✓,
            ]
        "#]],
    )
}
