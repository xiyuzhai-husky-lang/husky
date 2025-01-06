use super::*;
use latex_math_letter::letter::LxMathLetter;
use visored_mir_expr::symbol::local_defn::VdMirSymbolLocalDefnIdx;
use visored_opr::precedence::VdPrecedenceRange;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct VdBsqAtomInumTerm<'sess>(VdBsqInumTermFld<'sess>);

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum VdBsqInumAtomTermData {
    Variable(LxMathLetter, VdMirSymbolLocalDefnIdx),
}

impl<'sess> From<VdBsqAtomInumTerm<'sess>> for VdBsqNumTerm<'sess> {
    fn from(value: VdBsqAtomInumTerm<'sess>) -> Self {
        VdBsqNumTerm::Inum(VdBsqInumTerm::Atom(value))
    }
}

impl<'sess> VdBsqAtomInumTerm<'sess> {
    pub fn new(data: VdBsqInumAtomTermData, db: &'sess FloaterDb) -> Self {
        VdBsqAtomInumTerm(VdBsqInumTermFld::new(VdBsqInumTermData::Atom(data), db))
    }
}

impl<'sess> VdBsqAtomInumTerm<'sess> {
    pub fn data(self) -> &'sess VdBsqInumAtomTermData {
        match self.0.data() {
            VdBsqInumTermData::Atom(data) => data,
            _ => unreachable!(),
        }
    }
}

impl<'sess> VdBsqInumTerm<'sess> {
    pub fn new_atom(data: VdBsqInumAtomTermData, db: &'sess FloaterDb) -> Self {
        VdBsqInumTerm::Atom(VdBsqAtomInumTerm::new(data, db))
    }
}

impl<'sess> VdBsqTerm<'sess> {
    pub fn new_numeric_variable(
        lx_math_letter: LxMathLetter,
        local_defn_idx: VdMirSymbolLocalDefnIdx,
        db: &'sess FloaterDb,
    ) -> Self {
        VdBsqTerm::Inum(VdBsqInumTerm::Atom(VdBsqAtomInumTerm(
            VdBsqInumTermFld::new(
                VdBsqInumTermData::Atom(VdBsqInumAtomTermData::Variable(
                    lx_math_letter,
                    local_defn_idx,
                )),
                db,
            ),
        )))
    }
}

impl<'sess> VdBsqAtomInumTerm<'sess> {
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

impl<'sess> VdBsqInumAtomTermData {
    pub fn show_fmt(
        &self,
        precedence_range: VdPrecedenceRange,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            VdBsqInumAtomTermData::Variable(lx_math_letter, _) => {
                write!(f, "{}", lx_math_letter.unicode())
            }
        }
    }

    pub fn outer_precedence(&self) -> VdPrecedence {
        match self {
            VdBsqInumAtomTermData::Variable(_, _) => VdPrecedence::ATOM,
        }
    }
}
