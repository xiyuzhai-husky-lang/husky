use super::*;
use crate::{rank::Rank, symbol::Symbol, token::ident::Ident};
use husky_cybertron::seq::Seq;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TypeSignature {
    key: TypeSignatureKey,
    ty: Type,
}

impl TypeSignature {
    pub fn key(self) -> TypeSignatureKey {
        self.key
    }

    pub fn ty(&self) -> Type {
        self.ty
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TypeSignatureKey {
    CallArgument { fn_ident: Ident, rank: Rank },
    Field { ty_ident: Ident, field_ident: Ident },
}

pub(super) fn calc_ty_signatures(
    asts: Seq<Option<Ast>>,
    roles: Seq<Option<Role>>,
    ty_terms: Seq<Option<Type>>,
) -> Seq<Option<TypeSignature>> {
    calc_ty_signature.apply(roles, ty_terms)
}

fn calc_ty_signature(role: Option<Role>, ty_term: Option<Type>) -> Option<TypeSignature> {
    let ty = ty_term?;
    let key = match role? {
        Role::FnDefnCallFormParameterType { fn_ident, rank } => {
            TypeSignatureKey::CallArgument { fn_ident, rank }
        }
        Role::StructFieldType {
            ty_ident,
            field_ident,
        } => TypeSignatureKey::Field {
            ty_ident,
            field_ident,
        },
        _ => return None,
    };
    Some(TypeSignature { key, ty })
}

#[cfg(test)]
fn t(input: &str, expect: Expect) {
    use signature::calc_ty_signatures;
    use term::calc_ty_terms;

    let (tokens, pre_asts, asts) =
        calc_asts_from_input_together_with_tokens_and_pre_asts(input, 10);
    let roles = calc_roles(asts, 10);
    let ranks = calc_ranks(asts);
    let ty_terms = calc_ty_terms(asts, roles, 10);
    let ty_signatures = calc_ty_signatures(asts, roles, ty_terms);
    p!(show_asts_mapped_values(tokens, asts, roles));
    p!(show_asts_mapped_values(tokens, asts, ty_terms));
    todo!();
    expect.assert_debug_eq(&show_asts_mapped_values(tokens, asts, ty_signatures));
}

#[test]
fn ty_signatures_works() {
    // t(
    //     "",
    //     expect![[r#"
    //     []
    // "#]],
    // );
    t(
        "struct A { x: i32, y: Vec[i32] }",
        expect![[r#"
            [
                #0 `struct`: `struct`,
                #1 `A`: "A" ✓,
                #2 `{`: `{`,
                #3 `x`: "x",
                #4 `:`: "x : i32",
                #5 `i32`: "i32",
                #6 `,`: "x : i32, " ✓,
                #7 `y`: "y" ✓,
                #8 `:`: `:`,
                #9 `Vec`: "Vec",
                #10 `[`: "Vec[i32]" ✓,
                #11 `i32`: "i32",
                #12 `]`: "[i32]",
                #13 `}`: `}`,
            ]
        "#]],
    );
    t(
        "",
        expect![[r#"
        []
    "#]],
    );
}
