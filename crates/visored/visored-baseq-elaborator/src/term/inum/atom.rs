use super::*;
use visored_mir_expr::symbol::local_defn::VdMirSymbolLocalDefnIdx;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct VdBsqInumAtomTerm<'sess>(VdBsqInumTermFld<'sess>);

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum VdBsqInumAtomTermData {
    Variable(VdMirSymbolLocalDefnIdx),
}

impl<'sess> VdBsqInumAtomTerm<'sess> {
    pub fn new(data: VdBsqInumAtomTermData, db: &'sess FloaterDb) -> Self {
        VdBsqInumAtomTerm(VdBsqInumTermFld::new(VdBsqInumTermData::Atom(data), db))
    }
}

impl<'sess> VdBsqInumAtomTerm<'sess> {
    pub fn data(self) -> &'sess VdBsqInumAtomTermData {
        match self.0.data() {
            VdBsqInumTermData::Atom(data) => data,
            _ => unreachable!(),
        }
    }
}

impl<'sess> VdBsqInumTerm<'sess> {
    pub fn new_atom(data: VdBsqInumAtomTermData, db: &'sess FloaterDb) -> Self {
        VdBsqInumTerm::Atom(VdBsqInumAtomTerm::new(data, db))
    }
}

impl<'sess> VdBsqTerm<'sess> {
    pub fn new_numeric_variable(
        local_defn_idx: VdMirSymbolLocalDefnIdx,
        db: &'sess FloaterDb,
    ) -> Self {
        VdBsqTerm::Inum(VdBsqInumTerm::Atom(VdBsqInumAtomTerm(
            VdBsqInumTermFld::new(
                VdBsqInumTermData::Atom(VdBsqInumAtomTermData::Variable(local_defn_idx)),
                db,
            ),
        )))
    }
}
