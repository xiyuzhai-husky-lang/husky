use super::*;
use signature::TypeSignature;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TypeExpectation {
    ty: Type,
    source: TypeExpectationSource,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TypeExpectationSource {
    CallArgument { caller_ident: Ident, rank: Rank },
}

fn calc_ty_expectations(
    asts: Seq<Option<Ast>>,
    ranks: Seq<Option<Rank>>,
    ty_signatures: Seq<Option<TypeSignature>>,
) -> Seq<Option<TypeExpectation>> {
    let parent_asts = asts.index(asts.map(|ast| ast?.parent)).map(Option::flatten);
    let ty_expectation_sources = calc_ty_expectation_source.apply(asts, ranks);
    let retrieved_ty_signatures = ty_signatures
        .first_filtered_by_attention(
            ty_expectation_sources,
            ty_signatures,
            |ty_expection_source, ty_signature| {
                let Some(type_expectation_source) = ty_expection_source else {
                    return false;
                };
                let Some(type_signature) = ty_signature else {
                    return false;
                };
                match (type_expectation_source, type_signature.key()) {
                    (
                        TypeExpectationSource::CallArgument {
                            caller_ident,
                            rank: rank0,
                        },
                        signature::TypeSignatureKey::CallArgument {
                            fn_ident,
                            rank: rank1,
                        },
                    ) if caller_ident == fn_ident && rank0 == rank1 => true,
                    _ => false,
                }
            },
        )
        .map(Option::flatten);
    (|ty_expectation_source: Option<TypeExpectationSource>,
      retrieved_ty_signature: Option<TypeSignature>| {
        Some(TypeExpectation {
            ty: retrieved_ty_signature?.ty(),
            source: ty_expectation_source?,
        })
    })
    .apply(ty_expectation_sources, retrieved_ty_signatures)
}

fn calc_ty_expectation_source(
    ast: Option<Ast>,
    rank: Option<Rank>,
) -> Option<TypeExpectationSource> {
    let ast = ast?;
    let rank = rank?;
    match ast.data {
        AstData::Call {
            caller,
            caller_ident: Some(caller_ident),
            left_delimiter,
            right_delimiter,
            delimited_arguments,
        } => Some(TypeExpectationSource::CallArgument { caller_ident, rank }),
        _ => None,
    }
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
    let ty_expectations = calc_ty_expectations(asts, ranks, ty_signatures);
    expect.assert_debug_eq(&show_asts_mapped_values(tokens, asts, ty_expectations));
}
