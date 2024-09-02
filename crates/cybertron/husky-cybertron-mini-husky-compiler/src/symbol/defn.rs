use super::*;
use ast::{
    calc_asts_from_input_together_with_tokens_and_pre_asts, helpers::parent_queries,
    show::show_asts_mapped_values,
};
use husky_cybertron::prelude::*;
use scope::{infer_scopes, Scope};
use token::opr::{BinaryOpr, Opr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SymbolDefn {
    pub symbol: Symbol,
    pub scope: Option<Scope>,
}

pub fn calc_symbol_defns(asts: Seq<Option<Ast>>, n: usize) -> Seq<Option<SymbolDefn>> {
    let roles = populate_roles_n_times(asts, n);
    let scopes = infer_scopes(asts, n);
    calc_symbol_defn.apply_enumerated(asts, roles, scopes)
}

fn calc_symbol_defn(
    idx: Idx,
    ast: Option<Ast>,
    role: Option<Role>,
    scope: Option<Scope>,
) -> Option<SymbolDefn> {
    match ast?.data {
        AstData::Ident(ident) => match role? {
            Role::LetInit { .. } => unreachable!(),
            Role::Todo => todo!(),
            Role::LetInitIdent => Some(SymbolDefn {
                symbol: Symbol {
                    ident,
                    source: idx,
                    data: SymbolData::Variable,
                },
                scope,
            }),
        },
        AstData::Defn {
            keyword,
            ident_idx,
            ident,
            content,
        } => Some(SymbolDefn {
            symbol: Symbol {
                ident,
                source: idx,
                data: SymbolData::Item {
                    kind: keyword.into(),
                },
            },
            scope,
        }),
        _ => None,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Role {
    LetInit { pattern: Idx },
    Todo,
    LetInitIdent,
}

impl Ast {
    fn role(self) -> Option<Role> {
        match self.data {
            AstData::LetInit {
                expr,
                pattern,
                initial_value,
            } => Some(Role::LetInit { pattern }),
            _ => None,
        }
    }
}

fn populate_roles_n_times(asts: Seq<Option<Ast>>, n: usize) -> Seq<Option<Role>> {
    let mut roles: Seq<Option<Role>> = asts.map(|ast| ast?.role());
    for _ in 0..n {
        let parent_roles = parent_queries(asts, roles);
        roles = populate_roles(asts, parent_roles, roles);
    }
    roles
}

fn populate_roles(
    asts: Seq<Option<Ast>>,
    parent_roles: Seq<Option<Role>>,
    roles: Seq<Option<Role>>,
) -> Seq<Option<Role>> {
    populate_role.apply_enumerated(asts, parent_roles, roles)
}

fn populate_role(
    idx: Idx,
    ast: Option<Ast>,
    parent_role: Option<Role>,
    role: Option<Role>,
) -> Option<Role> {
    if let Some(role) = role {
        return Some(role);
    }
    let ast = ast?;
    if let Some(role) = ast.role() {
        return Some(role);
    }
    match parent_role? {
        Role::LetInit { pattern } => match ast.data {
            AstData::Ident(ident) if idx == pattern => Some(Role::LetInitIdent),
            AstData::Binary {
                lopd,
                opr: BinaryOpr::Assign,
                ropd,
            } if lopd == pattern => Some(Role::LetInit { pattern }),
            _ => None,
        },
        Role::Todo => todo!(),
        _ => None,
    }
}

#[test]
fn calc_symbol_defns_works() {
    fn t(input: &str, expect: Expect) {
        let (tokens, pre_asts, asts) =
            calc_asts_from_input_together_with_tokens_and_pre_asts(input, 10);
        let symbol_defns = calc_symbol_defns(asts, 10);
        expect.assert_debug_eq(&show_asts_mapped_values(
            tokens,
            pre_asts,
            asts,
            symbol_defns,
        ))
    }
    t(
        "1",
        expect![[r#"
            [
                #0 `1`: "1" ✓,
            ]
        "#]],
    );
    t(
        "struct A {}",
        expect![[r#"
            [
                #0 `struct`: "struct A {}" ✓ → SymbolDefn { symbol: Symbol { ident: `A`, source: #0, data: Item { kind: Struct } }, scope: Some(`::`) },
                #1 `A`: "A",
                #2 `{`: `{`,
                #3 `}`: "{}",
            ]
        "#]],
    );
    t(
        "enum B {}",
        expect![[r#"
            [
                #0 `enum`: "enum B {}" ✓ → SymbolDefn { symbol: Symbol { ident: `B`, source: #0, data: Item { kind: Enum } }, scope: Some(`::`) },
                #1 `B`: "B",
                #2 `{`: `{`,
                #3 `}`: "{}",
            ]
        "#]],
    );
    t(
        "fn f() {}",
        expect![[r#"
            [
                #0 `fn`: "fn f() {}" ✓ → SymbolDefn { symbol: Symbol { ident: `f`, source: #0, data: Item { kind: Fn } }, scope: Some(`::`) },
                #1 `f`: "f",
                #2 `(`: `(`,
                #3 `)`: "()",
                #4 `{`: "() {}",
                #5 `}`: "{}",
            ]
        "#]],
    );
    t(
        "fn f() { let x = 1 }",
        expect![[r#"
            [
                #0 `fn`: "fn f() { let x = 1 }" ✓ → SymbolDefn { symbol: Symbol { ident: `f`, source: #0, data: Item { kind: Fn } }, scope: Some(`::`) },
                #1 `f`: "f",
                #2 `(`: `(`,
                #3 `)`: "()",
                #4 `{`: "() { let x = 1 }",
                #5 `let`: "let x = 1",
                #6 `x`: "x" → SymbolDefn { symbol: Symbol { ident: `x`, source: #6, data: Variable }, scope: Some(`::9`) },
                #7 `=`: "x = 1",
                #8 `1`: "1",
                #9 `}`: "{ let x = 1 }",
            ]
        "#]],
    );
    t(
        "fn f() { let x = 1; }",
        expect![[r#"
            [
                #0 `fn`: "fn f() { let x = 1;  }" ✓ → SymbolDefn { symbol: Symbol { ident: `f`, source: #0, data: Item { kind: Fn } }, scope: Some(`::`) },
                #1 `f`: "f",
                #2 `(`: `(`,
                #3 `)`: "()",
                #4 `{`: "() { let x = 1;  }",
                #5 `let`: "let x = 1",
                #6 `x`: "x" → SymbolDefn { symbol: Symbol { ident: `x`, source: #6, data: Variable }, scope: Some(`::10`) },
                #7 `=`: "x = 1",
                #8 `1`: "1",
                #9 `;`: "let x = 1; ",
                #10 `}`: "{ let x = 1;  }",
            ]
        "#]],
    );
}

#[test]
fn calc_symbol_defns_works1() {
    fn t(input: &str, expect: Expect) {
        let (tokens, pre_asts, asts) =
            calc_asts_from_input_together_with_tokens_and_pre_asts(input, 10);
        let symbol_defns = calc_symbol_defns(asts, 10);
        expect.assert_debug_eq(&show_asts_mapped_values(
            tokens,
            pre_asts,
            asts,
            symbol_defns,
        ))
    }
    t(
        "fn f() { let x = 1; }",
        expect![[r#"
            [
                #0 `fn`: "fn f() { let x = 1;  }" ✓ → SymbolDefn { symbol: Symbol { ident: `f`, source: #0, data: Item { kind: Fn } }, scope: Some(`::`) },
                #1 `f`: "f",
                #2 `(`: `(`,
                #3 `)`: "()",
                #4 `{`: "() { let x = 1;  }",
                #5 `let`: "let x = 1",
                #6 `x`: "x" → SymbolDefn { symbol: Symbol { ident: `x`, source: #6, data: Variable }, scope: Some(`::10`) },
                #7 `=`: "x = 1",
                #8 `1`: "1",
                #9 `;`: "let x = 1; ",
                #10 `}`: "{ let x = 1;  }",
            ]
        "#]],
    );
}
