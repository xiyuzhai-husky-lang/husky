mod keyed;
mod regular;
mod variadic;

pub use self::keyed::*;
pub use self::regular::*;
pub use self::variadic::*;

use super::*;
use husky_coword::Ident;
use husky_ethereal_signature::ExplicitParameterEtherealSignatureTemplate;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = FluffyTermDb, jar = FluffyTermJar)]
#[enum_class::from_variants]
pub enum FluffyTermRitchieParameter {
    Regular(FluffyTermRitchieRegularParameter),
    Variadic(FluffyTermRitchieVariadicParameter),
    Keyed(FluffyTermRitchieKeyedParameter),
}

impl From<EtherealTermRitchieParameter> for FluffyTermRitchieParameter {
    fn from(param: EtherealTermRitchieParameter) -> Self {
        match param {
            EtherealTermRitchieParameter::Regular(param) => {
                FluffyTermRitchieParameter::Regular(param.into())
            }
            EtherealTermRitchieParameter::Variadic(param) => {
                FluffyTermRitchieParameter::Variadic(param.into())
            }
            EtherealTermRitchieParameter::Keyed(param) => {
                FluffyTermRitchieParameter::Keyed(param.into())
            }
        }
    }
}

impl InstantiateRef for ExplicitParameterEtherealSignatureTemplate {
    type Target = FluffyTermRitchieParameter;

    fn instantiate(
        &self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: ExprIdx,
        instantiation: &mut FluffyTermInstantiation,
    ) -> Self::Target {
        match self {
            ExplicitParameterEtherealSignatureTemplate::Regular(signature_template) => {
                FluffyTermRitchieRegularParameter::new(
                    signature_template.contract(),
                    signature_template
                        .ty()
                        .instantiate(engine, expr_idx, instantiation),
                )
                .into()
            }
            ExplicitParameterEtherealSignatureTemplate::Variadic(_) => todo!(),
            ExplicitParameterEtherealSignatureTemplate::Keyed(_) => todo!(),
        }
    }
}

impl FluffyTermRitchieParameter {
    pub fn ty(&self) -> FluffyTerm {
        match self {
            FluffyTermRitchieParameter::Regular(param) => param.ty(),
            FluffyTermRitchieParameter::Variadic(param) => param.ty(),
            FluffyTermRitchieParameter::Keyed(param) => param.ty(),
        }
    }

    pub(crate) fn ty_mut(&mut self) -> &mut FluffyTerm {
        match self {
            FluffyTermRitchieParameter::Regular(param) => param.ty_mut(),
            FluffyTermRitchieParameter::Variadic(param) => param.ty_mut(),
            FluffyTermRitchieParameter::Keyed(param) => param.ty_mut(),
        }
    }
}

impl FluffyTerm {
    pub(crate) fn new_richie(
        db: &dyn FluffyTermDb,
        fluffy_terms: &mut FluffyTerms,
        ritchie_kind: RitchieKind,
        parameter_contracted_tys: Vec<FluffyTermRitchieParameter>,
        return_ty: FluffyTerm,
    ) -> Self {
        let mut solid_flag = false;
        let mut hollow_flag = false;
        for parameter_contracted_ty in &parameter_contracted_tys {
            match parameter_contracted_ty.ty().nested() {
                NestedFluffyTerm::Ethereal(_) => (),
                NestedFluffyTerm::Solid(_) => solid_flag = true,
                NestedFluffyTerm::Hollow(_) => hollow_flag = true,
            }
        }
        match return_ty.nested() {
            NestedFluffyTerm::Ethereal(_) => (),
            NestedFluffyTerm::Solid(_) => solid_flag = true,
            NestedFluffyTerm::Hollow(_) => hollow_flag = true,
        }
        if hollow_flag {
            fluffy_terms
                .hollow_terms_mut()
                .alloc_new(HollowTermData::Ritchie {
                    ritchie_kind,
                    params: parameter_contracted_tys,
                    return_ty,
                })
                .into()
        } else if solid_flag {
            todo!()
        } else {
            todo!()
        }
    }
}
