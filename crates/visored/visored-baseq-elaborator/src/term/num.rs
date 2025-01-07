use super::*;
use crate::term::sum::VdBsqSumInumTerm;
use builder::sum::VdBsqSumBuilder;
use smallvec::*;

#[enum_class::from_variants]
#[derive(Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum VdBsqNumTerm<'sess> {
    Rnum(VdBsqRnumTerm),
    Inum(VdBsqInumTerm<'sess>),
}

impl<'sess> From<VdBsqNonProductNumTerm<'sess>> for VdBsqNumTerm<'sess> {
    fn from(term: VdBsqNonProductNumTerm<'sess>) -> Self {
        match term {
            VdBsqNonProductNumTerm::Rnum(term) => VdBsqNumTerm::Rnum(term),
            VdBsqNonProductNumTerm::AtomInum(term) => VdBsqNumTerm::Inum(term.into()),
            VdBsqNonProductNumTerm::SumInum(term) => VdBsqNumTerm::Inum(term.into()),
        }
    }
}

impl<'sess> From<i128> for VdBsqNumTerm<'sess> {
    fn from(value: i128) -> Self {
        VdBsqNumTerm::Rnum(VdBsqRnumTerm::Int128(value))
    }
}

impl<'sess> From<VdBsqNumTerm<'sess>> for VdBsqTerm<'sess> {
    fn from(term: VdBsqNumTerm<'sess>) -> Self {
        match term {
            VdBsqNumTerm::Rnum(term) => VdBsqTerm::Rnum(term),
            VdBsqNumTerm::Inum(term) => VdBsqTerm::Inum(term),
        }
    }
}

impl<'sess> VdBsqNumTerm<'sess> {
    pub const ZERO: Self = VdBsqNumTerm::Rnum(VdBsqRnumTerm::ZERO);
    pub const ONE: Self = VdBsqNumTerm::Rnum(VdBsqRnumTerm::ONE);
}

impl<'sess> VdBsqNumTerm<'sess> {
    pub fn is_zero_trivially(self) -> bool {
        match self {
            VdBsqNumTerm::Rnum(term) => term.is_zero(),
            VdBsqNumTerm::Inum(term) => false,
        }
    }

    pub fn is_one_trivially(self) -> bool {
        self.eqs_i128_trivially(1)
    }

    pub fn eqs_i128_trivially(self, rhs: i128) -> bool {
        match self {
            VdBsqNumTerm::Rnum(term) => term.eqs_i128(rhs),
            VdBsqNumTerm::Inum(term) => false,
        }
    }

    pub fn add(self, rhs: VdBsqNumTerm<'sess>, db: &'sess FloaterDb) -> VdBsqNumTerm<'sess> {
        if rhs.is_zero_trivially() {
            return self;
        }
        let mut builder = VdBsqSumBuilder::new(db);
        builder.add_num(self);
        builder.add_num(rhs);
        builder.finish()
    }

    pub fn sub(self, rhs: VdBsqNumTerm<'sess>, db: &'sess FloaterDb) -> VdBsqNumTerm<'sess> {
        if rhs.is_zero_trivially() {
            return self;
        }
        let mut builder = VdBsqSumBuilder::new(db);
        builder.add_num(self);
        builder.sub_num(rhs);
        builder.finish()
    }

    pub fn add_assign(&mut self, rhs: VdBsqNumTerm<'sess>, db: &'sess FloaterDb) {
        *self = self.add(rhs, db);
    }

    pub fn mul128(self, rhs: i128, db: &'sess FloaterDb) -> VdBsqNumTerm<'sess> {
        match self {
            VdBsqNumTerm::Rnum(term) => VdBsqNumTerm::Rnum(term.mul128(rhs, db)),
            VdBsqNumTerm::Inum(term) => term.mul128(rhs, db),
        }
    }
}

impl<'sess> VdBsqNumTerm<'sess> {
    pub fn show_fmt(
        &self,
        precedence_range: VdPrecedenceRange,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            VdBsqNumTerm::Rnum(term) => term.show_fmt(precedence_range, f),
            VdBsqNumTerm::Inum(term) => term.show_fmt(precedence_range, f),
        }
    }
}
