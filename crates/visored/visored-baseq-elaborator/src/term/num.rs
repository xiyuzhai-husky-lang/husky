use super::*;
use crate::term::sum::VdBsqSumComnumTerm;
use builder::sum::VdBsqSumBuilder;
use smallvec::*;

#[enum_class::from_variants]
#[derive(Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum VdBsqNumTerm<'sess> {
    Litnum(VdBsqLitnumTerm<'sess>),
    Comnum(VdBsqComnumTerm<'sess>),
}

impl<'sess> From<VdBsqNonProductNumTerm<'sess>> for VdBsqNumTerm<'sess> {
    fn from(term: VdBsqNonProductNumTerm<'sess>) -> Self {
        match term {
            VdBsqNonProductNumTerm::Litnum(term) => VdBsqNumTerm::Litnum(term),
            VdBsqNonProductNumTerm::AtomComnum(term) => VdBsqNumTerm::Comnum(term.into()),
            VdBsqNonProductNumTerm::SumComnum(term) => VdBsqNumTerm::Comnum(term.into()),
        }
    }
}

impl<'sess> From<i128> for VdBsqNumTerm<'sess> {
    fn from(value: i128) -> Self {
        VdBsqNumTerm::Litnum(VdBsqLitnumTerm::Int128(value))
    }
}

impl<'sess> From<VdBsqNumTerm<'sess>> for VdBsqTerm<'sess> {
    fn from(term: VdBsqNumTerm<'sess>) -> Self {
        match term {
            VdBsqNumTerm::Litnum(term) => VdBsqTerm::Litnum(term),
            VdBsqNumTerm::Comnum(term) => VdBsqTerm::Comnum(term),
        }
    }
}

impl<'sess> VdBsqNumTerm<'sess> {
    pub const ZERO: Self = VdBsqNumTerm::Litnum(VdBsqLitnumTerm::ZERO);
    pub const ONE: Self = VdBsqNumTerm::Litnum(VdBsqLitnumTerm::ONE);
}

impl<'sess> VdBsqNumTerm<'sess> {
    pub fn is_zero_trivially(self) -> bool {
        match self {
            VdBsqNumTerm::Litnum(term) => term.is_zero(),
            VdBsqNumTerm::Comnum(term) => false,
        }
    }

    pub fn is_one_trivially(self) -> bool {
        self.eqs_i128_trivially(1)
    }

    pub fn eqs_i128_trivially(self, rhs: i128) -> bool {
        match self {
            VdBsqNumTerm::Litnum(term) => term.eqs_i128(rhs),
            VdBsqNumTerm::Comnum(term) => false,
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
            VdBsqNumTerm::Litnum(term) => VdBsqNumTerm::Litnum(term.mul128(rhs, db)),
            VdBsqNumTerm::Comnum(term) => term.mul128(rhs, db),
        }
    }

    pub fn div(self, rhs: VdBsqNumTerm<'sess>, db: &'sess FloaterDb) -> VdBsqNumTerm<'sess> {
        let mut builder = VdBsqProductBuilder::new_from_num(self, db);
        builder.div_num(rhs);
        builder.finish()
    }
}

impl<'sess> VdBsqNumTerm<'sess> {
    pub fn show_fmt(
        &self,
        precedence_range: VdPrecedenceRange,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            VdBsqNumTerm::Litnum(term) => term.show_fmt(precedence_range, f),
            VdBsqNumTerm::Comnum(term) => term.show_fmt(precedence_range, f),
        }
    }
}
