use super::*;
use vec_like::ordered_small_vec_map::OrderedSmallVecPairMap;

#[floated]
pub struct VdBsqInumTerm<'sess> {
    #[return_ref]
    pub data: VdBsqInumTermData<'sess>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum VdBsqInumTermData<'sess> {
    Variable(VdMirSymbolLocalDefnIdx),
    Sum {
        constant_term: VdBsqRnumTerm,
        irrational_monomial_coefficients: VdBsqInumMonomialCoefficients<'sess>,
    },
    Product {
        rational: VdBsqRnumTerm,
        irrational_atom_exponentials: VdBsqInumAtomExponentials<'sess>,
    },
    Power {
        base: VdBsqNumTerm<'sess>,
        exponent: VdBsqNumTerm<'sess>,
    },
}

pub type VdBsqInumMonomialCoefficients<'sess> = VdBsqInumTermMap<'sess, VdBsqRnumTerm>;

pub type VdBsqInumAtomExponentials<'sess> = VdBsqInumTermMap<'sess, VdBsqInumTerm<'sess>>;

pub type VdBsqInumTermMap<'sess, T> = OrderedSmallVecPairMap<VdBsqInumTerm<'sess>, T, 4>;

impl<'sess> VdBsqTerm<'sess> {
    pub fn new_numeric_variable(
        local_defn_idx: VdMirSymbolLocalDefnIdx,
        db: &'sess FloaterDb,
    ) -> Self {
        VdBsqTerm::Inum(VdBsqInumTerm::new(
            inum::VdBsqInumTermData::Variable(local_defn_idx),
            db,
        ))
    }

    pub fn new_power(
        base: VdBsqNumTerm<'sess>,
        exponent: VdBsqNumTerm<'sess>,
        db: &'sess FloaterDb,
    ) -> Self {
        VdBsqTerm::Inum(VdBsqInumTerm::new(
            VdBsqInumTermData::Power { base, exponent },
            db,
        ))
    }
}
