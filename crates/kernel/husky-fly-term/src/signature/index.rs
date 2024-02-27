mod ethereal;

pub(crate) use self::ethereal::*;

use super::*;

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FlyIndexSignature {
    Int { element_ty: FlyTerm },
    Regular { element_ty: FlyTerm },
    Index { element_ty: FlyTerm },
}

impl MemberSignature for FlyIndexSignature {
    fn expr_ty(&self, self_value_final_place: FlyPlace) -> FlyTermResult<FlyTerm> {
        match self {
            FlyIndexSignature::Int { element_ty } => {
                Ok(element_ty.with_place(self_value_final_place))
            }
            FlyIndexSignature::Regular { element_ty } => todo!(),
            FlyIndexSignature::Index { element_ty } => todo!(),
        }
    }
}

// to: better name
fn list_like_index_signature(
    engine: &mut impl FlyTermEngineMut,
    expr_idx: SynExprIdx,
    element_ty: FlyTerm,
    index_ty: FlyTerm,
) -> FlyTermMaybeResult<FlyIndexSignature> {
    match index_ty.data(engine) {
        FlyTermData::Literal(_) => todo!(),
        // todo: is this correct?
        FlyTermData::TypeOntology {
            ty_path,
            refined_ty_path,
            ty_arguments: arguments,
            ..
        } => match refined_ty_path {
            Left(prelude_ty_path) => match prelude_ty_path {
                PreludeTypePath::Num(PreludeNumTypePath::Int(prelude_int_ty_path)) => {
                    JustOk(FlyIndexSignature::Int { element_ty })
                }
                _ => todo!(),
            },
            Right(_) => todo!(),
        },
        FlyTermData::Curry {
            toolchain,
            curry_kind,
            variance,
            parameter_hvar,
            parameter_ty,
            return_ty,
            ty_ethereal_term,
        } => todo!(),
        FlyTermData::Hole(hole_kind, _) => match hole_kind {
            HoleKind::UnspecifiedIntegerType => {
                let expectation =
                    ExpectCoersion::new_pure(engine, engine.term_menu().usize_ty_ontology().into());
                engine.add_expectation(
                    ExpectationSource::new_expr(expr_idx),
                    index_ty,
                    expectation,
                );
                JustOk(FlyIndexSignature::Int { element_ty })
            }
            HoleKind::UnspecifiedFloatType => todo!(),
            HoleKind::ImplicitType => todo!(),
            HoleKind::AnyOriginal => todo!(),
            HoleKind::AnyDerived => todo!(),
        },
        FlyTermData::Sort(_) => todo!(),
        FlyTermData::Ritchie {
            ritchie_kind,
            parameter_contracted_tys,
            return_ty,
            ..
        } => todo!(),
        FlyTermData::Symbol { .. } => todo!(),
        FlyTermData::Hvar { .. } => todo!(),
        FlyTermData::TypeVariant { path } => todo!(),
    }
}
