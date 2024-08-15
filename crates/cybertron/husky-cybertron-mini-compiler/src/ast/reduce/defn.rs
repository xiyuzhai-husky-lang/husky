use token::keyword::DefnKeyword;

use super::*;

pub(super) fn reduce_by_defn(
    pre_asts: Seq<Option<PreAst>>,
    allocated_asts: Seq<Option<Ast>>,
) -> (Seq<Option<PreAst>>, Seq<Option<Ast>>) {
    let pre_asts_nearest_left2 = pre_asts.nearest_left2();
    let pre_asts_nearest_right2 = pre_asts.nearest_right2();
    let new_defn_asts =
        new_defn_ast.apply(pre_asts_nearest_left2, pre_asts, pre_asts_nearest_right2);
    let (pre_asts, new_parents) = reduce_pre_asts_by_defn(pre_asts, new_defn_asts);
    let allocated_asts =
        allocate_asts_and_update_parents(allocated_asts, new_defn_asts, new_parents);
    let pre_asts = update_pre_asts_by_new_asts(pre_asts, new_defn_asts);
    (pre_asts, allocated_asts)
}

fn new_defn_ast(
    pre_ast_nearest_left2: Option2<(Idx, PreAst)>,
    pre_ast: Option<PreAst>,
    pre_ast_nearest_right2: Option2<(Idx, PreAst)>,
) -> Option<AstData> {
    let PreAst::Keyword(Keyword::Defn(keyword)) = pre_ast? else {
        return None;
    };
    {
        let Some((ident_idx, PreAst::Ast(AstData::Ident(ident)))) = pre_ast_nearest_right2.first()
        else {
            return None;
        };
        let Some((content, PreAst::Ast(content_ast))) = pre_ast_nearest_right2.second() else {
            return None;
        };
        match keyword {
            DefnKeyword::Struct => match content_ast {
                AstData::Delimited { .. } => (),
                _ => return None,
            },
            DefnKeyword::Enum => match content_ast {
                AstData::Delimited { .. } => (),
                _ => return None,
            },
            DefnKeyword::Fn => match content_ast {
                AstData::Call { .. } => (),
                _ => return None,
            },
        }
        Some(AstData::Defn {
            keyword,
            ident,
            ident_idx,
            content,
        })
    }
}

fn reduce_pre_asts_by_defn(
    pre_asts: Seq<Option<PreAst>>,
    new_asts: Seq<Option<AstData>>,
) -> (Seq<Option<PreAst>>, Seq<Option<Idx>>) {
    let new_asts_nearest_left = new_asts.nearest_left();
    let new_asts_nearest_right = new_asts.nearest_right();
    reduce_pre_ast_by_defn
        .apply_enumerated(new_asts_nearest_left, new_asts_nearest_right, pre_asts)
        .decouple()
}

fn reduce_pre_ast_by_defn(
    idx: Idx,
    new_ast_nearest_left: Option<(Idx, AstData)>,
    new_ast_nearest_right: Option<(Idx, AstData)>,
    pre_ast: Option<PreAst>,
) -> (Option<PreAst>, Option<Idx>) {
    if let Some((idx1, ast)) = new_ast_nearest_left {
        match ast {
            AstData::Defn {
                keyword,
                ident_idx,
                ident,
                content,
                ..
            } if ident_idx == idx || content == idx => (None, Some(idx1)),
            _ => (pre_ast, None),
        }
    } else if let Some((idx1, AstData::Defn { .. })) = new_ast_nearest_right
        && false
    // todo
    {
        (None, Some(idx1))
    } else {
        (pre_ast, None)
    }
}

#[test]
fn reduce_n_times_for_defn_works1() {
    t(
        "struct A { x: i32, y: i32 }",
        5,
        expect![[r#"
            [
                `struct`: "struct A { x : i32, y : i32 }" ✓,
                `A`: "A",
                `{`: `{`,
                `x`: "x",
                `:`: "x : i32",
                `i32`: "i32",
                `,`: "x : i32, ",
                `y`: "y",
                `:`: "y : i32",
                `i32`: "i32",
                `}`: "{ x : i32, y : i32 }",
            ]
        "#]],
    );
    t(
        "enum Animal { Cat, Dog }",
        5,
        expect![[r#"
            [
                `enum`: "enum Animal { Cat, Dog }" ✓,
                `Animal`: "Animal",
                `{`: `{`,
                `Cat`: "Cat",
                `,`: "Cat, ",
                `Dog`: "Dog",
                `}`: "{ Cat, Dog }",
            ]
        "#]],
    );
    t(
        "fn f() {}",
        5,
        expect![[r#"
            [
                `fn`: "fn f() {}" ✓,
                `f`: "f",
                `(`: `(`,
                `)`: "()",
                `{`: "() {}",
                `}`: "{}",
            ]
        "#]],
    );
    t(
        "fn f() -> i32 { 1 }",
        5,
        expect![[r#"
            [
                `fn`: "fn f() -> i32 { 1 }" ✓,
                `f`: "f",
                `(`: `(`,
                `)`: "()",
                `->`: "() -> i32",
                `i32`: "i32",
                `{`: "() -> i32 { 1 }",
                `1`: "1",
                `}`: "{ 1 }",
            ]
        "#]],
    );
}

#[test]
fn reduce_n_times_for_defn_works2() {
    t(
        "struct A { x: i32, y: i32 } enum Animal { Cat, Dog }",
        5,
        expect![[r#"
            [
                `struct`: "struct A { x : i32, y : i32 }" ✓,
                `A`: "A",
                `{`: `{`,
                `x`: "x",
                `:`: "x : i32",
                `i32`: "i32",
                `,`: "x : i32, ",
                `y`: "y",
                `:`: "y : i32",
                `i32`: "i32",
                `}`: "{ x : i32, y : i32 }",
                `enum`: "enum Animal { Cat, Dog }" ✓,
                `Animal`: "Animal",
                `{`: `{`,
                `Cat`: "Cat",
                `,`: "Cat, ",
                `Dog`: "Dog",
                `}`: "{ Cat, Dog }",
            ]
        "#]],
    );
    t(
        "struct A { x: i32, y: i32 } enum Animal { Cat, Dog } fn f() {}",
        5,
        expect![[r#"
            [
                `struct`: "struct A { x : i32, y : i32 }" ✓,
                `A`: "A",
                `{`: `{`,
                `x`: "x",
                `:`: "x : i32",
                `i32`: "i32",
                `,`: "x : i32, ",
                `y`: "y",
                `:`: "y : i32",
                `i32`: "i32",
                `}`: "{ x : i32, y : i32 }",
                `enum`: "enum Animal { Cat, Dog }" ✓,
                `Animal`: "Animal",
                `{`: `{`,
                `Cat`: "Cat",
                `,`: "Cat, ",
                `Dog`: "Dog",
                `}`: "{ Cat, Dog }",
                `fn`: "fn f() {}" ✓,
                `f`: "f",
                `(`: `(`,
                `)`: "()",
                `{`: "() {}",
                `}`: "{}",
            ]
        "#]],
    );
}
