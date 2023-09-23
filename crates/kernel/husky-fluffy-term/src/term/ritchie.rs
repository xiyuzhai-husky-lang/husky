mod keyed;
mod regular;
mod variadic;

pub use self::keyed::*;
pub use self::regular::*;
pub use self::variadic::*;

use super::*;
use husky_coword::Ident;

impl FluffyTerm {
    pub(crate) fn new_ritchie(
        engine: &mut impl FluffyTermEngine,
        expr_idx: SynExprIdx,
        ritchie_kind: RitchieKind,
        params: Vec<FluffyTermRitchieParameter>,
        return_ty: FluffyTerm,
    ) -> FluffyTermResult<Self> {
        let mut merger = FluffyTermDataKindMerger::new(engine.fluffy_term_region());
        merger.accept(params.iter().map(|param| param.ty()));
        merger.accept_one(return_ty);
        match merger.data_kind() {
            FluffyTermDataKind::Err => todo!(),
            FluffyTermDataKind::Ethereal => Ok(EtherealTermRitchie::new(
                engine.db(),
                ritchie_kind,
                params.into_iter().map(|param| {
                    param
                        .resolve_as_ethereal(engine.fluffy_term_region())
                        .expect("todo")
                }),
                return_ty
                    .resolve_as_ethereal(engine.fluffy_term_region())
                    .expect("todo"),
            )?
            .into()),
            FluffyTermDataKind::Solid => todo!(),
            FluffyTermDataKind::Hollow => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = FluffyTermDb, jar = FluffyTermJar)]
#[enum_class::from_variants]
pub enum FluffyTermRitchieParameter {
    Regular(FluffyTermRitchieRegularParameter),
    Variadic(FluffyTermRitchieVariadicParameter),
    Keyed(FluffyTermRitchieKeyedParameter),
}

impl FluffyTermRitchieParameter {
    fn resolve_as_ethereal(
        self,
        terms: &impl std::borrow::Borrow<HollowTerms>,
    ) -> Option<EtherealTermRitchieParameter> {
        Some(match self {
            FluffyTermRitchieParameter::Regular(param) => param.resolve_as_ethereal(terms)?.into(),
            FluffyTermRitchieParameter::Variadic(param) => todo!(),
            FluffyTermRitchieParameter::Keyed(param) => todo!(),
        })
    }
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

impl FluffyTermInstantiate for EtherealTermRitchieParameter {
    type Target = FluffyTermRitchieParameter;

    fn instantiate(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: SynExprIdx,
        instantiation: &mut FluffyTermInstantiation,
    ) -> Self::Target {
        match self {
            EtherealTermRitchieParameter::Regular(param) => {
                param.instantiate(engine, expr_idx, instantiation).into()
            }
            EtherealTermRitchieParameter::Variadic(_) => todo!(),
            EtherealTermRitchieParameter::Keyed(_) => todo!(),
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
        terms: &mut FluffyTerms,
        ritchie_kind: RitchieKind,
        parameter_contracted_tys: Vec<FluffyTermRitchieParameter>,
        return_ty: FluffyTerm,
    ) -> Self {
        let mut solid_flag = false;
        let mut hollow_flag = false;
        for parameter_contracted_ty in &parameter_contracted_tys {
            match parameter_contracted_ty.ty().base_resolved_inner(terms) {
                FluffyTermBase::Ethereal(_) => (),
                FluffyTermBase::Solid(_) => solid_flag = true,
                FluffyTermBase::Hollow(_) => hollow_flag = true,
                FluffyTermBase::Place => todo!(),
            }
        }
        match return_ty.base_resolved_inner(terms) {
            FluffyTermBase::Ethereal(_) => (),
            FluffyTermBase::Solid(_) => solid_flag = true,
            FluffyTermBase::Hollow(_) => hollow_flag = true,
            FluffyTermBase::Place => todo!(),
        }
        if hollow_flag {
            terms
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
