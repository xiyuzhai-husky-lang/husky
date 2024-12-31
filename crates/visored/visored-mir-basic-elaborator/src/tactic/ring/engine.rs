use crate::session::VdMirSession;

use super::*;
use husky_sha_utils::ShaHash;
use rustc_hash::FxHashMap;
use visored_mir_expr::{
    expr::{application::VdMirFunc, VdMirExprArenaRef, VdMirExprData, VdMirExprIdx},
    symbol::local_defn::VdMirSymbolLocalDefnIdx,
};
use visored_opr::separator::VdBaseSeparator;
use visored_term::term::literal::VdLiteralData;

pub struct VdMirRingTacticEngine<'db, 'sess> {
    session: &'sess VdMirSession<'db>,
    expr_arena: VdMirExprArenaRef<'db>,
    term_arena: NonLiteralTermArena,
    interned_terms: FxHashMap<NonLiteralTermData, IrrationalTerm>,
}

impl<'db, 'sess> VdMirRingTacticEngine<'db, 'sess> {
    pub fn new(session: &'sess VdMirSession<'db>, expr_arena: VdMirExprArenaRef<'db>) -> Self {
        Self {
            session,
            expr_arena,
            term_arena: NonLiteralTermArena::default(),
            interned_terms: FxHashMap::default(),
        }
    }
}

impl<'db, 'sess> VdMirRingTacticEngine<'db, 'sess> {
    pub fn judge(&mut self, lopd: VdMirExprIdx, ropd: VdMirExprIdx) -> bool {
        let lterm = self.convert(lopd);
        let rterm = self.convert(ropd);
        lterm == rterm
    }

    pub fn convert(&mut self, expr: VdMirExprIdx) -> Term {
        let expr_arena = self.expr_arena;
        match expr_arena[expr] {
            VdMirExprData::Literal(vd_literal) => match *vd_literal.data() {
                VdLiteralData::Nat128(n) => Term::Rational(RationalTerm::Nat128(n)),
                VdLiteralData::Int128(i) => Term::Rational(RationalTerm::Int128(i)),
                VdLiteralData::Float(_) => todo!(),
                VdLiteralData::SpecialConstant(vd_special_constant) => todo!(),
            },
            VdMirExprData::Variable(local_defn) => self.mk_variable(local_defn),
            VdMirExprData::Application {
                function,
                arguments,
            } => todo!(),
            VdMirExprData::FoldingSeparatedList {
                leader,
                ref followers,
            } => {
                let (VdMirFunc::NormalBaseSeparator(signature), fst_follower) = followers[0] else {
                    unreachable!()
                };
                match signature.opr() {
                    VdBaseSeparator::Space => todo!(),
                    VdBaseSeparator::Comma => todo!(),
                    VdBaseSeparator::Semicolon => todo!(),
                    VdBaseSeparator::Add => self.mk_sum(leader, followers),
                    VdBaseSeparator::Mul => todo!(),
                    VdBaseSeparator::Dot => todo!(),
                    VdBaseSeparator::Eq => todo!(),
                    VdBaseSeparator::Ne => todo!(),
                    VdBaseSeparator::Lt => todo!(),
                    VdBaseSeparator::Gt => todo!(),
                    VdBaseSeparator::Le => todo!(),
                    VdBaseSeparator::Ge => todo!(),
                    VdBaseSeparator::Subset => todo!(),
                    VdBaseSeparator::Supset => todo!(),
                    VdBaseSeparator::Subseteq => todo!(),
                    VdBaseSeparator::Supseteq => todo!(),
                    VdBaseSeparator::Subseteqq => todo!(),
                    VdBaseSeparator::Supseteqq => todo!(),
                    VdBaseSeparator::Subsetneq => todo!(),
                    VdBaseSeparator::Supsetneq => todo!(),
                    VdBaseSeparator::In => todo!(),
                    VdBaseSeparator::Notin => todo!(),
                    VdBaseSeparator::Times => todo!(),
                    VdBaseSeparator::Otimes => todo!(),
                }
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

impl<'db, 'sess> VdMirRingTacticEngine<'db, 'sess> {
    fn intern_term(&mut self, data: NonLiteralTermData) -> IrrationalTerm {
        if let Some(idx) = self.interned_terms.get(&data) {
            return *idx;
        }
        let sha256 = data.sha256();
        self.term_arena
            .alloc_one(NonLiteralTermEntry { data, sha256 })
    }

    fn mk_product(&mut self, factors: impl IntoIterator<Item = IrrationalTerm>) -> IrrationalTerm {
        let mut literal = RationalTerm::ONE;
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

    fn mk_sum(&mut self, leader: VdMirExprIdx, followers: &[(VdMirFunc, VdMirExprIdx)]) -> Term {
        let mut literal_term = RationalTerm::ZERO;
        let mut nonliteral_monomial_coefficients = NonLiteralMonomialCoefficients::default();
        let mut t = |expr: VdMirExprIdx| {
            let term = self.convert(expr);
            match term {
                Term::Rational(new_literal_term) => literal_term += new_literal_term,
                Term::Irrational(term) => match self.term_arena[term].data {
                    NonLiteralTermData::Product {
                        literal,
                        ref nonliteral_atom_exponentials,
                    } => todo!(),
                    NonLiteralTermData::Sum {
                        constant_term,
                        ref nonliteral_monomial_coefficients,
                    } => todo!(),
                    NonLiteralTermData::Atom | NonLiteralTermData::Variable(_) => {
                        match nonliteral_monomial_coefficients.get_value(term) {
                            Some(_) => todo!(),
                            None => {
                                nonliteral_monomial_coefficients.insert((term, RationalTerm::ONE))
                            }
                        }
                        nonliteral_monomial_coefficients.insert_or_update(
                            (term, RationalTerm::ONE),
                            |entry| {
                                todo!()
                                // *e += RationalTerm::ONE;
                            },
                        );
                    }
                },
            }
        };
        t(leader);
        for (func, follower) in followers {
            let VdMirFunc::NormalBaseSeparator(signature) = func else {
                unreachable!()
            };
            match signature.opr() {
                VdBaseSeparator::Add => t(*follower),
                _ => unreachable!(),
            }
        }
        if nonliteral_monomial_coefficients.is_empty() {
            literal_term.into()
        } else {
            self.intern_term(NonLiteralTermData::Sum {
                constant_term: literal_term,
                nonliteral_monomial_coefficients,
            })
            .into()
        }
    }
}
