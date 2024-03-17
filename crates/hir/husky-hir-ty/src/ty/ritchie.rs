use super::*;
use husky_eth_term::term::ritchie::{
    EthRitchie, EthRitchieSimpleParameter, EtherealRitchieParameter,
};
use husky_fly_term::FlyRitchieSimpleParameter;
use husky_term_prelude::{ritchie::RitchieTypeKind, Contract};

#[salsa::interned(db = HirTypeDb, jar = HirTypeJar, constructor = new)]
pub struct HirRitchieType {
    pub ritchie_ty_kind: RitchieTypeKind,
    #[return_ref]
    pub parameters: HirRitchieParameters,
    pub return_ty: HirType,
}

impl HirRitchieType {
    pub fn from_eth(term: EthRitchie, db: &::salsa::Db) -> Self {
        hir_ty_from_eth_term_ritchie(db, term)
    }
}

#[salsa::tracked(jar = HirTypeJar)]
fn hir_ty_from_eth_term_ritchie(db: &::salsa::Db, term_ritchie: EthRitchie) -> HirRitchieType {
    HirRitchieType::new(
        db,
        term_ritchie
            .ritchie_kind(db)
            .ritchie_ty_kind()
            .expect("should be type"),
        HirRitchieParameters::from_eth(term_ritchie.parameter_contracted_tys(db), db),
        HirType::from_eth(term_ritchie.return_ty(db), db).unwrap(),
    )
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[salsa::derive_debug_with_db]
pub struct HirRitchieParameters {
    data: SmallVec<[HirRitchieParameter; 4]>,
}

impl HirRitchieParameters {
    pub(crate) fn from_eth(params: &[EtherealRitchieParameter], db: &::salsa::Db) -> Self {
        HirRitchieParameters {
            data: params
                .iter()
                .copied()
                .map(|param| HirRitchieParameter::from_eth(param, db))
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
#[salsa::derive_debug_with_db]
pub enum HirRitchieParameter {
    Simple(HirRitchieSimpleParameter),
    Variadic(HirRitchieVariadicParameter),
    Keyed(HirRitchieKeyedParameter),
}

impl HirRitchieParameter {
    pub fn from_eth(param: EtherealRitchieParameter, db: &::salsa::Db) -> Self {
        match param {
            EtherealRitchieParameter::Simple(param) => Self::from_eth_regular(param, db),
            EtherealRitchieParameter::Variadic(_) => todo!(),
            EtherealRitchieParameter::Keyed(_) => todo!(),
        }
    }

    pub fn from_eth_regular(param: EthRitchieSimpleParameter, db: &::salsa::Db) -> Self {
        HirRitchieSimpleParameter {
            contract: HirContract::from_contract(param.contract()),
            ty: HirType::from_eth(param.ty(), db).unwrap(),
        }
        .into()
    }
}

#[salsa::derive_debug_with_db]
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct HirRitchieSimpleParameter {
    pub contract: HirContract,
    pub ty: HirType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HirContract {
    Pure,
    Move,
    Borrow,
    BorrowMut,
    Const,
    Leash,
    At,
}

impl HirContract {
    pub fn from_contract(contract: Contract) -> Self {
        match contract {
            Contract::Pure => HirContract::Pure,
            Contract::Move => HirContract::Move,
            Contract::Borrow => HirContract::Borrow,
            Contract::BorrowMut => HirContract::BorrowMut,
            Contract::Const => HirContract::Const,
            Contract::Leash => HirContract::Leash,
            Contract::At => HirContract::At,
        }
    }

    pub fn is_destructive(self) -> bool {
        match self {
            HirContract::Move => true,
            HirContract::Pure
            | HirContract::Borrow
            | HirContract::BorrowMut
            | HirContract::Const
            | HirContract::Leash
            | HirContract::At => false,
        }
    }
}

impl HirRitchieSimpleParameter {
    pub fn contract(&self) -> HirContract {
        self.contract
    }

    pub fn ty(&self) -> HirType {
        self.ty
    }

    pub fn from_fly(
        param: &FlyRitchieSimpleParameter,
        db: &::salsa::Db,
        fly_terms: &FlyTerms,
    ) -> Self {
        Self {
            contract: HirContract::from_contract(param.contract),
            ty: HirType::from_fly(param.ty, db, fly_terms).unwrap(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct HirRitchieVariadicParameter {}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct HirRitchieKeyedParameter {}
