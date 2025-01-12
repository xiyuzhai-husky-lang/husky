use super::*;
use latex_math_letter::letter::LxMathLetter;
use visored_mir_expr::symbol::local_defn::VdMirSymbolLocalDefnIdx;
use visored_opr::precedence::VdPrecedenceRange;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct VdBsqAtomComnumTerm<'sess>(VdBsqComnumTermFld<'sess>);

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum VdBsqComnumAtomTermData {
    Variable(LxMathLetter, VdMirSymbolLocalDefnIdx),
}

impl<'sess> From<VdBsqAtomComnumTerm<'sess>> for VdBsqNumTerm<'sess> {
    fn from(value: VdBsqAtomComnumTerm<'sess>) -> Self {
        VdBsqNumTerm::Comnum(VdBsqComnumTerm::Atom(value))
    }
}

impl<'sess> VdBsqAtomComnumTerm<'sess> {
    pub fn new(data: VdBsqComnumAtomTermData, db: &'sess FloaterDb) -> Self {
        VdBsqAtomComnumTerm(VdBsqComnumTermFld::new(VdBsqComnumTermData::Atom(data), db))
    }
}

impl<'sess> VdBsqAtomComnumTerm<'sess> {
    pub fn data(self) -> &'sess VdBsqComnumAtomTermData {
        match self.0.data() {
            VdBsqComnumTermData::Atom(data) => data,
            _ => unreachable!(),
        }
    }
}

impl<'sess> VdBsqComnumTerm<'sess> {
    pub fn new_atom(data: VdBsqComnumAtomTermData, db: &'sess FloaterDb) -> Self {
        VdBsqComnumTerm::Atom(VdBsqAtomComnumTerm::new(data, db))
    }
}

impl<'sess> VdBsqTerm<'sess> {
    pub fn new_numeric_variable(
        lx_math_letter: LxMathLetter,
        local_defn_idx: VdMirSymbolLocalDefnIdx,
        db: &'sess FloaterDb,
    ) -> Self {
        VdBsqTerm::Comnum(VdBsqComnumTerm::Atom(VdBsqAtomComnumTerm(
            VdBsqComnumTermFld::new(
                VdBsqComnumTermData::Atom(VdBsqComnumAtomTermData::Variable(
                    lx_math_letter,
                    local_defn_idx,
                )),
                db,
            ),
        )))
    }
}

impl<'sess> VdBsqAtomComnumTerm<'sess> {
    pub fn show_fmt(
        self,
        precedence_range: VdPrecedenceRange,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        self.data().show_fmt(precedence_range, f)
    }

    pub fn outer_precedence(&self) -> VdPrecedence {
        self.data().outer_precedence()
    }
}

impl<'sess> VdBsqComnumAtomTermData {
    pub fn show_fmt(
        &self,
        precedence_range: VdPrecedenceRange,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            VdBsqComnumAtomTermData::Variable(lx_math_letter, _) => {
                write!(f, "{}", lx_math_letter.unicode())
            }
        }
    }

    pub fn outer_precedence(&self) -> VdPrecedence {
        match self {
            VdBsqComnumAtomTermData::Variable(_, _) => VdPrecedence::ATOM,
        }
    }
}

impl<'sess> VdBsqAtomComnumTerm<'sess> {
    pub fn neg(self, db: &'sess FloaterDb) -> VdBsqComnumTerm<'sess> {
        let product_base = VdBsqProductBase::new(
            [(VdBsqNonProductNumTerm::AtomComnum(self), VdBsqNumTerm::ONE)]
                .into_iter()
                .collect(),
            db,
        );
        VdBsqComnumTerm::Product((-1).into(), product_base)
    }

    pub fn mul128(self, rhs: i128, db: &'sess FloaterDb) -> VdBsqNumTerm<'sess> {
        if rhs == 0 {
            return VdBsqNumTerm::ZERO;
        }
        if rhs == 1 {
            return self.into();
        }
        let product_base = VdBsqProductBase::new(
            [(VdBsqNonProductNumTerm::AtomComnum(self), VdBsqNumTerm::ONE)]
                .into_iter()
                .collect(),
            db,
        );
        VdBsqComnumTerm::Product(rhs.into(), product_base).into()
    }

    pub fn mul_litnum(
        self,
        rhs: VdBsqLitnumTerm<'sess>,
        db: &'sess FloaterDb,
    ) -> VdBsqNumTerm<'sess> {
        if rhs == 0.into() {
            return VdBsqNumTerm::ZERO;
        }
        if rhs == 1.into() {
            return self.into();
        }
        let product_base = VdBsqProductBase::new(
            [(VdBsqNonProductNumTerm::AtomComnum(self), VdBsqNumTerm::ONE)]
                .into_iter()
                .collect(),
            db,
        );
        VdBsqComnumTerm::Product(rhs, product_base).into()
    }

    pub fn div_litnum(
        self,
        rhs: VdBsqLitnumTerm<'sess>,
        db: &'sess FloaterDb,
    ) -> Option<VdBsqComnumTerm<'sess>> {
        if rhs == 0.into() {
            return None;
        }
        if rhs == 1.into() {
            return Some(self.into());
        }
        let product_base = VdBsqProductBase::new(
            [(VdBsqNonProductNumTerm::AtomComnum(self), VdBsqNumTerm::ONE)]
                .into_iter()
                .collect(),
            db,
        );
        Some(VdBsqComnumTerm::Product(rhs.inverse().unwrap(), product_base).into())
    }
}
