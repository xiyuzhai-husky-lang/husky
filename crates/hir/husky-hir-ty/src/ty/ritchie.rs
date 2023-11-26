use super::*;
use husky_ethereal_signature::EtherealTermParenateParameters;
use husky_ethereal_term::{EtherealRitchieParameter, EtherealRitchieRegularParameter};
use husky_term_prelude::{Contract, RitchieTypeKind};

#[salsa::interned(db = HirTypeDb, jar = HirTypeJar, constructor = new)]
pub struct HirRitchieType {
    pub ritchie_ty_kind: RitchieTypeKind,
    #[return_ref]
    pub parameters: HirRitchieParameters,
    pub return_ty: HirType,
}

impl HirRitchieType {
    pub fn from_ethereal(term: EtherealTermRitchie, db: &dyn HirTypeDb) -> Self {
        hir_ty_from_ethereal_term_ritchie(db, term)
    }
}

#[salsa::tracked(jar = HirTypeJar)]
fn hir_ty_from_ethereal_term_ritchie(
    db: &dyn HirTypeDb,
    term_ritchie: EtherealTermRitchie,
) -> HirRitchieType {
    HirRitchieType::new(
        db,
        term_ritchie
            .ritchie_kind(db)
            .ritchie_ty_kind()
            .expect("should be type"),
        HirRitchieParameters::from_ethereal(term_ritchie.parameter_contracted_tys(db), db),
        HirType::from_ethereal(term_ritchie.return_ty(db), db),
    )
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[salsa::debug_with_db(db = HirTypeDb, jar = HirTypeJar)]
pub struct HirRitchieParameters {
    data: SmallVec<[HirRitchieParameter; 4]>,
}

impl HirRitchieParameters {
    pub(crate) fn from_ethereal(params: &[EtherealRitchieParameter], db: &dyn HirTypeDb) -> Self {
        HirRitchieParameters {
            data: params
                .iter()
                .copied()
                .map(|param| HirRitchieParameter::from_ethereal(param, db))
                .collect(),
        }
    }
}

impl std::ops::Deref for HirRitchieParameters {
    type Target = [HirRitchieParameter];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
#[salsa::debug_with_db(db = HirTypeDb, jar = HirTypeJar)]
pub enum HirRitchieParameter {
    Regular(HirRitchieRegularParameter),
    Variadic(HirRitchieVariadicParameter),
    Keyed(HirRitchieKeyedParameter),
}

impl HirRitchieParameter {
    pub fn from_ethereal(param: EtherealRitchieParameter, db: &dyn HirTypeDb) -> Self {
        match param {
            EtherealRitchieParameter::Regular(param) => Self::from_ethereal_regular(param, db),
            EtherealRitchieParameter::Variadic(_) => todo!(),
            EtherealRitchieParameter::Keyed(_) => todo!(),
        }
    }

    pub fn from_ethereal_regular(
        param: EtherealRitchieRegularParameter,
        db: &dyn HirTypeDb,
    ) -> Self {
        HirRitchieRegularParameter {
            contract: param.contract(),
            ty: HirType::from_ethereal(param.ty(), db),
        }
        .into()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = HirTypeDb, jar = HirTypeJar)]
pub struct HirRitchieRegularParameter {
    contract: Contract,
    ty: HirType,
}

impl HirRitchieRegularParameter {
    pub fn contract(&self) -> Contract {
        self.contract
    }

    pub fn ty(&self) -> HirType {
        self.ty
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct HirRitchieVariadicParameter {}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct HirRitchieKeyedParameter {}
