use crate::{
    ast::{helpers::parent_queries, Ast, AstData},
    *,
};
use ast::{
    calc_asts_from_input, calc_asts_from_input_together_with_tokens_and_pre_asts,
    show::show_asts_mapped_values,
};
use husky_cybertron::{abstractions::bounded_vec::BoundedVec, prelude::*, seq::Seq};
use token::delimiter::{LCURL, RCURL};

const D: usize = 8usize;

#[derive(Default, Clone, Copy)]
pub struct Scope {
    enclosing_blocks: BoundedVec<Idx, D>,
}

impl std::fmt::Debug for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("`")?;
        if self.enclosing_blocks.is_empty() {
            f.write_str("::")?;
        }
        for idx in self.enclosing_blocks {
            f.write_str("::")?;
            idx.index().fmt(f)?
        }
        f.write_str("`")
    }
}

impl Scope {
    pub fn from_ast(idx: Idx, ast: AstData, parent_scope: Scope) -> Self {
        match ast {
            AstData::Delimited {
                left_delimiter_idx,
                left_delimiter: LCURL,
                right_delimiter: RCURL,
            } => Self {
                enclosing_blocks: parent_scope.enclosing_blocks.append(idx),
            },
            _ => parent_scope,
        }
    }
}

pub fn infer_scopes(asts: Seq<Option<Ast>>, n: usize) -> Seq<Option<Scope>> {
    let mut scopes = initial_scope.apply_enumerated(asts);
    for _ in 0..n {
        let parent_scopes = parent_queries(asts, scopes);
        scopes = infer_scopes_step(asts, parent_scopes, scopes);
    }
    scopes
}

fn initial_scope(idx: Idx, ast: Option<Ast>) -> Option<Scope> {
    let ast = ast?;
    if ast.parent.is_some() {
        return None;
    }
    let scope = Scope::default();
    Some(Scope::from_ast(idx, ast.data, scope))
}

fn infer_scopes_step(
    asts: Seq<Option<Ast>>,
    parent_scopes: Seq<Option<Scope>>,
    scopes: Seq<Option<Scope>>,
) -> Seq<Option<Scope>> {
    infer_scope_step.apply_enumerated(asts, parent_scopes, scopes)
}

fn infer_scope_step(
    idx: Idx,
    ast: Option<Ast>,
    parent_scope: Option<Scope>,
    scope: Option<Scope>,
) -> Option<Scope> {
    if let Some(scope) = scope {
        return Some(scope);
    }
    Some(Scope::from_ast(idx, ast?.data, parent_scope?))
}

#[test]
fn infer_scopes_works() {
    fn t(input: &str, (n, m): (usize, usize), expect: Expect) {
        let (tokens, (pre_asts, asts)) =
            calc_asts_from_input_together_with_tokens_and_pre_asts(input, n);
        let scopes = infer_scopes(asts, m);
        expect.assert_debug_eq(&show_asts_mapped_values(tokens, pre_asts, asts, scopes));
    }
    t(
        "",
        (1, 1),
        expect![[r#"
        []
    "#]],
    );
    t(
        "()",
        (1, 1),
        expect![[r#"
            [
                `(`: `(`,
                `)`: "()" ✓ → `::`,
            ]
        "#]],
    );
    t(
        "{}",
        (1, 1),
        expect![[r#"
            [
                `{`: `{`,
                `}`: "{}" ✓ → `::1`,
            ]
        "#]],
    );
    t(
        "{x}",
        (2, 2),
        expect![[r#"
            [
                `{`: `{`,
                `x`: "x" → `::2`,
                `}`: "{ x }" ✓ → `::2`,
            ]
        "#]],
    );
    t(
        "{ x { y } }",
        (3, 1),
        expect![[r#"
            [
                `{`: `{`,
                `x`: "x",
                `{`: "x { y }" → `::5`,
                `y`: "y",
                `}`: "{ y }",
                `}`: "{ x { y } }" ✓ → `::5`,
            ]
        "#]],
    );
    t(
        "{ x { y } }",
        (3, 3),
        expect![[r#"
            [
                `{`: `{`,
                `x`: "x" → `::5`,
                `{`: "x { y }" → `::5`,
                `y`: "y" → `::5::4`,
                `}`: "{ y }" → `::5::4`,
                `}`: "{ x { y } }" ✓ → `::5`,
            ]
        "#]],
    );
}
