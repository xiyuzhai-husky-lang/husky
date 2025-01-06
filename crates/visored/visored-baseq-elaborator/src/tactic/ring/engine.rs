use crate::session::VdBsqSession;

use super::*;
use husky_sha_utils::ShaHash;
use rustc_hash::FxHashMap;
use visored_mir_expr::{
    expr::{application::VdMirFunc, VdMirExprArenaRef, VdMirExprData, VdMirExprIdx},
    symbol::local_defn::VdMirSymbolLocalDefnIdx,
};
use visored_mir_opr::separator::VdMirBaseSeparator;
use visored_term::term::literal::VdLiteralData;

pub struct VdMirRingTacticEngine<'db, 'sess> {
    session: &'sess VdBsqSession<'db>,
    expr_arena: VdMirExprArenaRef<'db>,
    term_arena: IrrationalTermArena,
    interned_terms: FxHashMap<IrrationalTermData, IrrationalTerm>,
}

impl<'db, 'sess> VdMirRingTacticEngine<'db, 'sess> {
    pub fn new(session: &'sess VdBsqSession<'db>, expr_arena: VdMirExprArenaRef<'db>) -> Self {
        Self {
            session,
            expr_arena,
            term_arena: IrrationalTermArena::default(),
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
        match *expr_arena[expr].data() {
            VdMirExprData::Literal(vd_literal) => match *vd_literal.data() {
                VdLiteralData::Int128(i) => Term::Rational(RationalTerm::Int128(i)),
                VdLiteralData::BigInt(n) => todo!(),
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
                    VdMirBaseSeparator::CommRingAdd => self.mk_sum(leader, followers),
                    VdMirBaseSeparator::CommRingMul => todo!(),
                    VdMirBaseSeparator::Eq => todo!(),
                    VdMirBaseSeparator::Ne => todo!(),
                    VdMirBaseSeparator::Lt => todo!(),
                    VdMirBaseSeparator::Gt => todo!(),
                    VdMirBaseSeparator::Le => todo!(),
                    VdMirBaseSeparator::Ge => todo!(),
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
    fn intern_term(&mut self, data: IrrationalTermData) -> IrrationalTerm {
        if let Some(idx) = self.interned_terms.get(&data) {
            return *idx;
        }
        let sha256 = data.sha256();
        self.term_arena
            .alloc_one(IrrationalTermEntry { data, sha256 })
    }

    fn mk_product(&mut self, factors: impl IntoIterator<Item = IrrationalTerm>) -> IrrationalTerm {
        let mut literal = RationalTerm::ONE;
        let mut irrational_atom_exponentials = IrrationalAtomExponentials::default();
        for factor in factors {
            match self.term_arena[factor].data {
                IrrationalTermData::Atom => todo!(),
                IrrationalTermData::Product {
                    literal,
                    ref irrational_atom_exponentials,
                } => todo!(),
                IrrationalTermData::Sum {
                    ref irrational_monomial_coefficients,
                    constant_term,
                } => todo!(),
                IrrationalTermData::Variable(local_defn) => todo!(),
            }
        }
        let data = IrrationalTermData::Product {
            literal,
            irrational_atom_exponentials,
        };
        self.intern_term(data)
    }

    fn mk_variable(&mut self, local_defn: VdMirSymbolLocalDefnIdx) -> Term {
        self.intern_term(IrrationalTermData::Variable(local_defn))
            .into()
    }

    fn mk_sum(&mut self, leader: VdMirExprIdx, followers: &[(VdMirFunc, VdMirExprIdx)]) -> Term {
        let mut literal_term = RationalTerm::ZERO;
        let mut irrational_monomial_coefficients = IrrationalMonomialCoefficients::default();
        let mut t = |expr: VdMirExprIdx| {
            let term = self.convert(expr);
            match term {
                Term::Rational(new_literal_term) => literal_term += new_literal_term,
                Term::Irrational(term) => match self.term_arena[term].data {
                    IrrationalTermData::Product {
                        literal,
                        ref irrational_atom_exponentials,
                    } => todo!(),
                    IrrationalTermData::Sum {
                        constant_term,
                        ref irrational_monomial_coefficients,
                    } => todo!(),
                    IrrationalTermData::Atom | IrrationalTermData::Variable(_) => {
                        match irrational_monomial_coefficients.get_value(term) {
                            Some(_) => todo!(),
                            None => {
                                irrational_monomial_coefficients.insert((term, RationalTerm::ONE))
                            }
                        }
                        irrational_monomial_coefficients.insert_or_update(
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
                VdMirBaseSeparator::CommRingAdd => t(*follower),
                _ => unreachable!(),
            }
        }
        if irrational_monomial_coefficients.is_empty() {
            literal_term.into()
        } else {
            self.intern_term(IrrationalTermData::Sum {
                constant_term: literal_term,
                irrational_monomial_coefficients,
            })
            .into()
        }
    }
}
