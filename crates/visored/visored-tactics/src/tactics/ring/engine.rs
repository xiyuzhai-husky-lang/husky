use super::*;
use husky_sha_utils::ShaHash;
use rustc_hash::FxHashMap;

pub(super) struct Engine {
    term_arena: NonLiteralTermArena,
    interned_terms: FxHashMap<NonLiteralTermData, NonLiteralTerm>,
}

impl Engine {
    pub(super) fn intern_term(&mut self, data: NonLiteralTermData) -> NonLiteralTerm {
        if let Some(idx) = self.interned_terms.get(&data) {
            return *idx;
        }
        let sha256 = data.sha256();
        self.term_arena
            .alloc_one(NonLiteralTermEntry { data, sha256 })
    }

    pub fn product(&mut self, factors: impl IntoIterator<Item = NonLiteralTerm>) -> NonLiteralTerm {
        let mut literal = LiteralTerm::ONE;
        let mut nonliteral_atom_exponentials = NonLiteralAtomExponentials::default();
        for factor in factors {
            match self.term_arena[factor].data {
                NonLiteralTermData::Atom => todo!(),
                NonLiteralTermData::Product {
                    literal,
                    ref nonliteral_atom_exponentials,
                } => todo!(),
                NonLiteralTermData::Sum {
                    ref nonliteral_monomial_coefficients,
                    constant_term,
                } => todo!(),
            }
        }
        let data = NonLiteralTermData::Product {
            literal,
            nonliteral_atom_exponentials,
        };
        self.intern_term(data)
    }
}
