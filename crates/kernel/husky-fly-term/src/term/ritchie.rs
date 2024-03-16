mod keyed;
mod regular;
mod variadic;

pub use self::keyed::*;
pub use self::regular::*;
pub use self::variadic::*;

use super::*;
use husky_coword::Ident;
use husky_eth_term::term::ritchie::EtherealRitchieParameter;
use husky_term_prelude::ritchie::RitchieKind;

impl FlyTerm {
    pub fn new_ritchie(
        engine: &mut impl FlyTermEngineMut,
        ritchie_kind: RitchieKind,
        params: Vec<FlyRitchieParameter>,
        return_ty: FlyTerm,
    ) -> FlyTermResult<Self> {
        Self::new_ritchie_inner(
            ritchie_kind,
            params,
            return_ty,
            engine.db(),
            engine.fly_terms_mut(),
        )
    }

    pub fn new_ritchie_inner(
        ritchie_kind: RitchieKind,
        params: Vec<FlyRitchieParameter>,
        return_ty: FlyTerm,
        db: &::salsa::Db,
        terms: &mut FlyTerms,
    ) -> FlyTermResult<Self> {
        let mut merger = FlyTermDataKindMerger::new(terms);
        merger.accept(params.iter().map(|param| param.ty()));
        merger.accept_one(return_ty);
        match merger.data_kind() {
            FlyTermDataKind::Err => todo!(),
            FlyTermDataKind::Ethereal => Ok(EthRitchie::new(
                db,
                ritchie_kind,
                params
                    .into_iter()
                    .map(|param| param.resolve_as_ethereal(terms).expect("todo")),
                return_ty.resolve_as_ethereal(terms).expect("todo"),
            )?
            .into()),
            FlyTermDataKind::Solid => todo!(),
            FlyTermDataKind::Hollow => Ok(terms
                .hollow_terms_mut()
                .alloc_new(HolTermData::Ritchie {
                    ritchie_kind,
                    params,
                    return_ty,
                })
                .into()),
        }
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum FlyRitchieParameter {
    Regular(FlyRitchieSimpleParameter),
    Variadic(FlyRitchieVariadicParameter),
    Keyed(FlyRitchieKeyedParameter),
}

impl FlyRitchieParameter {
    fn resolve_as_ethereal(
        self,
        terms: &impl std::borrow::Borrow<HolTerms>,
    ) -> Option<EtherealRitchieParameter> {
        Some(match self {
            FlyRitchieParameter::Regular(param) => param.resolve_as_ethereal(terms)?.into(),
            FlyRitchieParameter::Variadic(param) => todo!(),
            FlyRitchieParameter::Keyed(param) => todo!(),
        })
    }
}

impl From<EtherealRitchieParameter> for FlyRitchieParameter {
    fn from(param: EtherealRitchieParameter) -> Self {
        match param {
            EtherealRitchieParameter::Regular(param) => FlyRitchieParameter::Regular(param.into()),
            EtherealRitchieParameter::Variadic(param) => {
                FlyRitchieParameter::Variadic(param.into())
            }
            EtherealRitchieParameter::Keyed(param) => FlyRitchieParameter::Keyed(param.into()),
        }
    }
}

impl FlyInstantiate for EtherealRitchieParameter {
    type Target = FlyRitchieParameter;

    fn instantiate(
        self,
        engine: &mut impl FlyTermEngineMut,
        expr_idx: SynExprIdx,
        instantiation: &FlyInstantiation,
    ) -> Self::Target {
        match self {
            EtherealRitchieParameter::Regular(param) => {
                param.instantiate(engine, expr_idx, instantiation).into()
            }
            EtherealRitchieParameter::Variadic(param) => {
                param.instantiate(engine, expr_idx, instantiation).into()
            }
            EtherealRitchieParameter::Keyed(param) => {
                param.instantiate(engine, expr_idx, instantiation).into()
            }
        }
    }
}

impl FlyRitchieParameter {
    pub fn ty(&self) -> FlyTerm {
        match self {
            FlyRitchieParameter::Regular(param) => param.ty(),
            FlyRitchieParameter::Variadic(param) => param.ty(),
            FlyRitchieParameter::Keyed(param) => param.ty(),
        }
    }

    pub(crate) fn ty_mut(&mut self) -> &mut FlyTerm {
        match self {
            FlyRitchieParameter::Regular(param) => param.ty_mut(),
            FlyRitchieParameter::Variadic(param) => param.ty_mut(),
            FlyRitchieParameter::Keyed(param) => param.ty_mut(),
        }
    }
}
