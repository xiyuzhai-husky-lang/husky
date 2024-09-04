use signature::{calc_ty_signatures, TypeSignature, TypeSignatureKey};
use term::calc_ty_terms;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TypeInference {
    ty: Type,
}

fn infer_tys(asts: Seq<Option<Ast>>, ty_signatures: Seq<Option<TypeSignature>>) {
    let mut ty_inferences = infer_tys_initial(asts, ty_signatures);
    todo!()
}

fn infer_tys_initial(
    asts: Seq<Option<Ast>>,
    ty_signatures: Seq<Option<TypeSignature>>,
) -> Seq<Option<TypeInference>> {
    inference_literal_tys(asts).or(infer_fn_call_tys(asts, ty_signatures))
}

fn infer_tys_step() {
    todo!()
}

fn inference_literal_tys(asts: Seq<Option<Ast>>) -> Seq<Option<TypeInference>> {
    asts.map(|ast| match ast?.data {
        AstData::Literal(lit) => match lit {
            Literal::Int(_) => Some(TypeInference {
                ty: Type::new_ident(Ident::new("Int")),
            }),
            Literal::Float(_) => Some(TypeInference {
                ty: Type::new_ident(Ident::new("Float")),
            }),
        },
        _ => None,
    })
}

fn infer_fn_call_tys(
    asts: Seq<Option<Ast>>,
    ty_signatures: Seq<Option<TypeSignature>>,
) -> Seq<Option<TypeInference>> {
    ty_signatures
        .first_filtered_by_attention(asts, ty_signatures, |ast, ty_signature| {
            let Some(ast) = ast else { return false };
            let Some(TypeSignature {
                key: TypeSignatureKey::FnOutput { fn_ident },
                ..
            }) = ty_signature
            else {
                return false;
            };
            match ast.data {
                AstData::Call {
                    caller,
                    caller_ident,
                    left_delimiter,
                    right_delimiter,
                    delimited_arguments,
                } if caller_ident == Some(fn_ident) => true,
                _ => false,
            }
        })
        .map(|ty_inference| {
            Some(TypeInference {
                ty: ty_inference??.ty,
            })
        })
}

#[test]
fn infer_tys_initial_works() {
    fn t(input: &str, expect: Expect) {
        let (tokens, pre_asts, asts) =
            calc_asts_from_input_together_with_tokens_and_pre_asts(input, 10);
        let roles = calc_roles(asts, 10);
        let ranks = calc_ranks(asts);
        let ty_terms = calc_ty_terms(asts, roles, 10);
        let ty_signatures = calc_ty_signatures(asts, roles, ty_terms);
        let ty_inferences_initial = infer_tys_initial(asts, ty_signatures);
        expect.assert_debug_eq(&show_asts_mapped_values(
            tokens,
            asts,
            ty_inferences_initial,
        ));
    }
    t(
        "",
        expect![[r#"
        []
    "#]],
    );
    t(
        "struct A {}",
        expect![[r#"
        [
            #0 `struct`: "struct A {}" ✓,
            #1 `A`: "A",
            #2 `{`: `{`,
            #3 `}`: "{}",
        ]
    "#]],
    );
    t(
        "fn f(x: i32) { let y = x; let t: i32 = x + 1; }",
        expect![[r#"
            [
                #0 `fn`: "fn f(x : i32) { let y = x; let t : i32 = x + 1;  }" ✓,
                #1 `f`: "f",
                #2 `(`: `(`,
                #3 `x`: "x",
                #4 `:`: "x : i32",
                #5 `i32`: "i32",
                #6 `)`: "(x : i32)",
                #7 `{`: "(x : i32) { let y = x; let t : i32 = x + 1;  }",
                #8 `let`: "let y = x",
                #9 `y`: "y",
                #10 `=`: "y = x",
                #11 `x`: "x",
                #12 `;`: "let y = x; ",
                #13 `let`: "let t : i32 = x + 1",
                #14 `t`: "t",
                #15 `:`: "t : i32",
                #16 `i32`: "i32",
                #17 `=`: "t : i32 = x + 1",
                #18 `x`: "x",
                #19 `+`: "x + 1",
                #20 `1`: "1" → TypeInference { ty: `Int` },
                #21 `;`: "let t : i32 = x + 1; ",
                #22 `}`: "{ let y = x; let t : i32 = x + 1;  }",
            ]
        "#]],
    );
    t(
        "fn f() -> i32 { return 1; } fn g() { let x = f(); g() }",
        expect![[r#"
            [
                #0 `fn`: "fn f() -> i32 { return 1;  }" ✓,
                #1 `f`: "f",
                #2 `(`: `(`,
                #3 `)`: "()",
                #4 `->`: "() -> i32",
                #5 `i32`: "i32",
                #6 `{`: "() -> i32 { return 1;  }",
                #7 `return`: "return 1",
                #8 `1`: "1" → TypeInference { ty: `Int` },
                #9 `;`: "return 1; ",
                #10 `}`: "{ return 1;  }",
                #11 `fn`: "fn g() { let x = f(); g() }" ✓,
                #12 `g`: "g",
                #13 `(`: `(`,
                #14 `)`: "()",
                #15 `{`: "() { let x = f(); g() }",
                #16 `let`: "let x = f()",
                #17 `x`: "x",
                #18 `=`: "x = f()",
                #19 `f`: "f",
                #20 `(`: "f()" → TypeInference { ty: `i32` },
                #21 `)`: "()",
                #22 `;`: "let x = f(); ",
                #23 `g`: "g",
                #24 `(`: "g()" → TypeInference { ty: `unit` },
                #25 `)`: "()",
                #26 `}`: "{ let x = f(); g() }",
            ]
        "#]],
    );
}

#[test]
fn calc_ty_inferences_works() {}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TypeDesignation {
    symbol: Symbol,
    ty: Type,
}

fn calc_ty_designations(
    roles: Seq<Option<Role>>,
    symbols: Seq<Option<Symbol>>,
    ty_inferences: Seq<Option<TypeInference>>,
    ty_terms: Seq<Option<Type>>,
) -> Seq<Option<TypeDesignation>> {
    let symbols = symbols
        .index(roles.map(|role| match role? {
            Role::LetInitInner {
                pattern,
                initial_value,
            } => Some(pattern),
            Role::FnParameter { fn_ident, rank, ty } => todo!(),
            _ => None,
        }))
        .map(Option::flatten);
    let ty_inferences = ty_inferences
        .index(roles.map(|role| match role? {
            Role::LetInitInner {
                pattern,
                initial_value,
            } => Some(initial_value),
            Role::FnParameter { fn_ident, rank, ty } => None,
            _ => None,
        }))
        .map(Option::flatten);
    let ty_terms = ty_terms
        .index(roles.map(|role| match role? {
            Role::LetInitInner {
                pattern,
                initial_value,
            } => Some(initial_value),
            Role::FnParameter { fn_ident, rank, ty } => None,
            _ => None,
        }))
        .map(Option::flatten);
    calc_ty_designation.apply(symbols, ty_inferences, ty_terms)
}

fn calc_ty_designation(
    symbol: Option<Symbol>,
    ty_inference: Option<TypeInference>,
    ty_term: Option<Type>,
) -> Option<TypeDesignation> {
    Some(TypeDesignation {
        symbol: symbol?,
        ty: ty_inference.map(|ti| ti.ty).or(ty_term)?,
    })
}

#[test]
fn calc_ty_designations_works() {}
