use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TypeExpectation {
    ty: Type,
    source: TypeExpectationSource,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TypeExpectationSource {
    FnCallArgument,
}

fn calc_ty_expectation_step(
    asts: Seq<Option<Ast>>,
    expectation: Seq<Option<TypeExpectation>>,
) -> Seq<Option<TypeExpectation>> {
    todo!()
}
