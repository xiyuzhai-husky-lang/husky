use super::*;
use husky_sha_utils::ShaHash;
use rustc_hash::FxHashMap;
use visored_mir_expr::{
    expr::{VdMirExprArenaRef, VdMirExprData, VdMirExprIdx},
    symbol::local_defn::VdMirSymbolLocalDefnIdx,
};

pub struct Engine<'db> {
    expr_arena: VdMirExprArenaRef<'db>,
    term_arena: NonLiteralTermArena,
    interned_terms: FxHashMap<NonLiteralTermData, NonLiteralTerm>,
}

impl<'db> Engine<'db> {
    pub fn new(expr_arena: VdMirExprArenaRef<'db>) -> Self {
        Self {
            expr_arena,
            term_arena: NonLiteralTermArena::default(),
            interned_terms: FxHashMap::default(),
        }
    }
}

impl<'db> Engine<'db> {
    pub fn judge(&mut self, lopd: VdMirExprIdx, ropd: VdMirExprIdx) -> bool {
        let lterm = self.convert(lopd);
        let rterm = self.convert(ropd);
        lterm == rterm
    }

    pub fn convert(&mut self, expr: VdMirExprIdx) -> Term {
        match self.expr_arena[expr] {
            VdMirExprData::Literal(vd_literal) => todo!(),
            VdMirExprData::Variable(local_defn) => self.mk_variable(local_defn),
            VdMirExprData::Application {
                function,
                arguments,
            } => todo!(),
            VdMirExprData::FoldingSeparatedList {
                leader,
                ref followers,
            } => {
                todo!()
            }
            VdMirExprData::ChainingSeparatedList {
                leader,
                ref followers,
                joined_separator_and_signature,
            } => todo!(),
            VdMirExprData::ItemPath(vd_item_path) => todo!(),
        }
    }
}

impl<'db> Engine<'db> {
    fn intern_term(&mut self, data: NonLiteralTermData) -> NonLiteralTerm {
        if let Some(idx) = self.interned_terms.get(&data) {
            return *idx;
        }
        let sha256 = data.sha256();
        self.term_arena
            .alloc_one(NonLiteralTermEntry { data, sha256 })
    }

    fn mk_product(&mut self, factors: impl IntoIterator<Item = NonLiteralTerm>) -> NonLiteralTerm {
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
                NonLiteralTermData::Variable(local_defn) => todo!(),
            }
        }
        let data = NonLiteralTermData::Product {
            literal,
            nonliteral_atom_exponentials,
        };
        self.intern_term(data)
    }

    fn mk_variable(&mut self, local_defn: VdMirSymbolLocalDefnIdx) -> Term {
        self.intern_term(NonLiteralTermData::Variable(local_defn))
            .into()
    }
}
