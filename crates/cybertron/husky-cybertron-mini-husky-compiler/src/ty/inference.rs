use signature::{TypeSignature, TypeSignatureKey};

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TypeInference {
    ty: Type,
}

fn infer_tys(asts: Seq<Option<Ast>>, ty_signatures: Seq<Option<TypeSignature>>) {
    let ty_inferences = inference_literal_tys(asts).or(infer_fn_call_tys(asts, ty_signatures));
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
                AstData::Literal(_) => todo!(),
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
