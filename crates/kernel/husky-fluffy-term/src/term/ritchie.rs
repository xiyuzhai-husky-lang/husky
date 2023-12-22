mod keyed;
mod regular;
mod variadic;

pub use self::keyed::*;
pub use self::regular::*;
pub use self::variadic::*;

use super::*;
use husky_coword::Ident;

impl FluffyTerm {
    pub fn new_ritchie(
        engine: &mut impl FluffyTermEngine,
        ritchie_kind: RitchieKind,
        params: Vec<FluffyRitchieParameter>,
        return_ty: FluffyTerm,
    ) -> FluffyTermResult<Self> {
        Self::new_ritchie_inner(
            ritchie_kind,
            params,
            return_ty,
            engine.db(),
            engine.fluffy_terms_mut(),
        )
    }

    pub fn new_ritchie_inner(
        ritchie_kind: RitchieKind,
        params: Vec<FluffyRitchieParameter>,
        return_ty: FluffyTerm,
        db: &::salsa::Db,
        terms: &mut FluffyTerms,
    ) -> FluffyTermResult<Self> {
        let mut merger = FluffyTermDataKindMerger::new(terms);
        merger.accept(params.iter().map(|param| param.ty()));
        merger.accept_one(return_ty);
        match merger.data_kind() {
            FluffyTermDataKind::Err => todo!(),
            FluffyTermDataKind::Ethereal => Ok(EtherealTermRitchie::new(
                db,
                ritchie_kind,
                params
                    .into_iter()
                    .map(|param| param.resolve_as_ethereal(terms).expect("todo")),
                return_ty.resolve_as_ethereal(terms).expect("todo"),
            )?
            .into()),
            FluffyTermDataKind::Solid => todo!(),
            FluffyTermDataKind::Hollow => Ok(terms
                .hollow_terms_mut()
                .alloc_new(HollowTermData::Ritchie {
                    ritchie_kind,
                    params,
                    return_ty,
                })
                .into()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = FluffyTermDb, jar = FluffyTermJar)]
#[enum_class::from_variants]
pub enum FluffyRitchieParameter {
    Regular(FluffyRitchieRegularParameter),
    Variadic(FluffyRitchieVariadicParameter),
    Keyed(FluffyRitchieKeyedParameter),
}

impl FluffyRitchieParameter {
    fn resolve_as_ethereal(
        self,
        terms: &impl std::borrow::Borrow<HollowTerms>,
    ) -> Option<EtherealRitchieParameter> {
        Some(match self {
            FluffyRitchieParameter::Regular(param) => param.resolve_as_ethereal(terms)?.into(),
            FluffyRitchieParameter::Variadic(param) => todo!(),
            FluffyRitchieParameter::Keyed(param) => todo!(),
        })
    }
}

impl From<EtherealRitchieParameter> for FluffyRitchieParameter {
    fn from(param: EtherealRitchieParameter) -> Self {
        match param {
            EtherealRitchieParameter::Regular(param) => {
                FluffyRitchieParameter::Regular(param.into())
            }
            EtherealRitchieParameter::Variadic(param) => {
                FluffyRitchieParameter::Variadic(param.into())
            }
            EtherealRitchieParameter::Keyed(param) => FluffyRitchieParameter::Keyed(param.into()),
        }
    }
}

impl FluffyInstantiate for EtherealRitchieParameter {
    type Target = FluffyRitchieParameter;

    fn instantiate(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: SynExprIdx,
        instantiation: &FluffyInstantiation,
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

impl FluffyRitchieParameter {
    pub fn ty(&self) -> FluffyTerm {
        match self {
            FluffyRitchieParameter::Regular(param) => param.ty(),
            FluffyRitchieParameter::Variadic(param) => param.ty(),
            FluffyRitchieParameter::Keyed(param) => param.ty(),
        }
    }

    pub(crate) fn ty_mut(&mut self) -> &mut FluffyTerm {
        match self {
            FluffyRitchieParameter::Regular(param) => param.ty_mut(),
            FluffyRitchieParameter::Variadic(param) => param.ty_mut(),
            FluffyRitchieParameter::Keyed(param) => param.ty_mut(),
        }
    }
}
