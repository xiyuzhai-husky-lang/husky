use super::*;
use ast::{calc_asts_from_input_together_with_tokens_and_pre_asts, helpers::parent_queries};
use husky_cybertron::prelude::*;
use scope::{infer_scopes, Scope};
use token::opr::{BinaryOpr, Opr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SymbolDefn {
    pub symbol: Symbol,
    pub scope: Option<Scope>,
}

pub fn calc_symbol_defns(
    asts: Seq<Option<Ast>>,
    scopes: Seq<Option<Scope>>,
    n: usize,
) -> Seq<Option<SymbolDefn>> {
    let roles = calc_roles(asts, n);
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
            Role::LetInitIdent => Some(SymbolDefn {
                symbol: Symbol {
                    ident,
                    source: idx,
                    data: SymbolData::Variable,
                },
                scope,
            }),
            Role::StructDefn(_) => todo!(),
            Role::EnumDefn(_) => todo!(),
            Role::FnDefn(_) => todo!(),
            Role::FnDefnCallForm(_) => todo!(),
            Role::FnParameters { .. } => todo!(),
            Role::FnBody(_) => todo!(),
            Role::StructFields(_) => todo!(),
            Role::FnParameter { fn_ident, rank, ty } => todo!(),
            Role::FnParameterType { fn_ident, rank } => todo!(),
            Role::StructField {
                field_ident,
                ty_idx,
                ty_ident,
            } => todo!(),
            Role::StructFieldType {
                ty_ident,
                field_ident,
            } => todo!(),
            Role::TypeArgument => todo!(),
            Role::TypeArguments => todo!(),
            Role::StructFieldSeparated(_) => todo!(),
            Role::FnParametersAndReturnType {
                fn_ident,
                parameters,
                return_ty,
            } => todo!(),
            Role::FnOutputType { fn_ident } => todo!(),
            Role::FnParameterSeparated { fn_ident, rank } => todo!(),
            Role::LetInitInner {
                pattern,
                initial_value,
            } => todo!(),
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

#[test]
fn calc_symbol_defns_works() {
    fn t(input: &str, expect: Expect) {
        let (tokens, pre_asts, asts) =
            calc_asts_from_input_together_with_tokens_and_pre_asts(input, 10);
        let scopes = infer_scopes(asts, 10);
        let symbol_defns = calc_symbol_defns(asts, scopes, 10);
        expect.assert_debug_eq(&show_asts_mapped_values(tokens, asts, symbol_defns))
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
