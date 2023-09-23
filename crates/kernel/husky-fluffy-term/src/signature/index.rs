mod ethereal;

pub(crate) use self::ethereal::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = FluffyTermDb)]
pub enum FluffyIndexSignature {
    Int { element_ty: FluffyTerm },
    Regular { element_ty: FluffyTerm },
    Index { element_ty: FluffyTerm },
}

impl MemberSignature for FluffyIndexSignature {
    fn expr_ty(&self) -> FluffyTermResult<FluffyTerm> {
        match self {
            FluffyIndexSignature::Int { element_ty } => Ok(*element_ty),
            FluffyIndexSignature::Regular { element_ty } => todo!(),
            FluffyIndexSignature::Index { element_ty } => todo!(),
        }
    }
}

// to: better name
fn list_like_index_signature(
    engine: &mut impl FluffyTermEngine,
    expr_idx: SynExprIdx,
    element_ty: FluffyTerm,
    index_ty: FluffyTerm,
) -> FluffyTermMaybeResult<FluffyIndexSignature> {
    match index_ty.data(engine) {
        FluffyTermData::Literal(_) => todo!(),
        // todo: is this correct?
        FluffyTermData::TypeOntology {
            ty_path,
            refined_ty_path,
            ty_arguments: arguments,
            ..
        } => match refined_ty_path {
            Left(prelude_ty_path) => match prelude_ty_path {
                PreludeTypePath::Num(PreludeNumTypePath::Int(prelude_int_ty_path)) => {
                    JustOk(FluffyIndexSignature::Int { element_ty })
                }
                _ => todo!(),
            },
            Right(_) => todo!(),
        },
        FluffyTermData::Curry {
            curry_kind,
            variance,
            parameter_variable,
            parameter_ty,
            return_ty,
            ty_ethereal_term,
        } => todo!(),
        FluffyTermData::Hole(hole_kind, _) => match hole_kind {
            HoleKind::UnspecifiedIntegerType => {
                let expectation =
                    ExpectCoersion::new_pure(engine, engine.term_menu().usize_ty_ontology().into());
                engine.fluffy_term_region_mut().add_expectation(
                    ExpectationSource::new_expr(expr_idx),
                    index_ty,
                    expectation,
                );
                JustOk(FluffyIndexSignature::Int { element_ty })
            }
            HoleKind::UnspecifiedFloatType => todo!(),
            HoleKind::ImplicitType => todo!(),
            HoleKind::Any => todo!(),
        },
        FluffyTermData::Category(_) => todo!(),
        FluffyTermData::Ritchie {
            ritchie_kind,
            parameter_contracted_tys,
            return_ty,
            ..
        } => todo!(),
        FluffyTermData::Symbol { .. } => todo!(),
        FluffyTermData::Variable { ty } => todo!(),
        FluffyTermData::TypeVariant { path } => todo!(),
    }
}
