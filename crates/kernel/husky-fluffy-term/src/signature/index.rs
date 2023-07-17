mod ethereal;

pub(crate) use self::ethereal::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub struct FluffyIndexSignature {
    element_ty: FluffyTerm,
}

impl MemberSignature for FluffyIndexSignature {
    fn expr_ty(
        &self,
        indirections: &[FluffyDynamicDispatchIndirection],
    ) -> FluffyTermResult<FluffyTerm> {
        let mut expr_ty = self.element_ty;
        for indirection in indirections {
            match indirection {
                FluffyDynamicDispatchIndirection::Place(_) => todo!(),
                FluffyDynamicDispatchIndirection::Leash => todo!(),
            }
        }
        Ok(expr_ty)
    }
}

fn list_index_signature(
    engine: &mut impl FluffyTermEngine,
    expr_idx: ExprIdx,
    element_ty: FluffyTerm,
    index_ty: FluffyTerm,
) -> FluffyTermMaybeResult<FluffyIndexSignature> {
    match index_ty.data(engine) {
        FluffyTermData::Literal(_) => todo!(),
        // todo: is this correct?
        FluffyTermData::TypeOntology {
            ty_path,
            refined_ty_path,
            arguments,
            ..
        }
        | FluffyTermData::TypeOntologyAtPlace {
            ty_path,
            refined_ty_path,
            arguments,
            ..
        } => match refined_ty_path {
            Left(prelude_ty_path) => match prelude_ty_path {
                PreludeTypePath::Basic(_) => todo!(),
                PreludeTypePath::Num(prelude_num_ty_path) => match prelude_num_ty_path {
                    PreludeNumTypePath::Int(prelude_int_ty_path) => match prelude_int_ty_path {
                        PreludeIntTypePath::I8 => todo!(),
                        PreludeIntTypePath::I16 => todo!(),
                        // should we allow this?
                        // indexing list with i32?
                        PreludeIntTypePath::I32 => JustOk(FluffyIndexSignature { element_ty }),
                        PreludeIntTypePath::I64 => todo!(),
                        PreludeIntTypePath::I128 => todo!(),
                        PreludeIntTypePath::ISize => todo!(),
                        PreludeIntTypePath::U8 => todo!(),
                        PreludeIntTypePath::U16 => todo!(),
                        PreludeIntTypePath::U32 => todo!(),
                        PreludeIntTypePath::U64 => todo!(),
                        PreludeIntTypePath::U128 => todo!(),
                        PreludeIntTypePath::USize => JustOk(FluffyIndexSignature { element_ty }),
                        PreludeIntTypePath::R8 => todo!(),
                        PreludeIntTypePath::R16 => todo!(),
                        PreludeIntTypePath::R32 => todo!(),
                        PreludeIntTypePath::R64 => todo!(),
                        PreludeIntTypePath::R128 => todo!(),
                        PreludeIntTypePath::RSize => todo!(),
                    },
                    PreludeNumTypePath::Float(_) => todo!(),
                },
                PreludeTypePath::Borrow(_) => todo!(),
                PreludeTypePath::Nat => todo!(),
                PreludeTypePath::Lifetime => todo!(),
                PreludeTypePath::Module => todo!(),
                PreludeTypePath::Trait => todo!(),
                PreludeTypePath::List => todo!(),
                PreludeTypePath::Array => todo!(),
                PreludeTypePath::Array2d => todo!(),
                PreludeTypePath::Array3d => todo!(),
                PreludeTypePath::Array4d => todo!(),
                PreludeTypePath::Array5d => todo!(),
                PreludeTypePath::Slice => todo!(),
                PreludeTypePath::StringLiteral => todo!(),
                PreludeTypePath::Str => todo!(),
                PreludeTypePath::Option => todo!(),
                PreludeTypePath::Result => todo!(),
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
                JustOk(FluffyIndexSignature { element_ty })
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
        FluffyTermData::HoleAtPlace {
            place,
            hole_kind,
            hole,
        } => todo!(),
        FluffyTermData::Symbol { .. } => todo!(),
        FluffyTermData::SymbolAtPlace { .. } => todo!(),
        FluffyTermData::Variable { ty } => todo!(),
        FluffyTermData::TypeVariant { path } => todo!(),
    }
}
