use either::{Either, *};
use floated_sequential::{db::FloaterDb, floated};
use num_bigint::Sign;
use visored_term::term::literal::bigint::VdBigIntData;

use super::VdBsqRnumTerm;

#[floated]
pub struct VdBsqRnumTermBigInt<'sess> {
    #[return_ref]
    pub inner: VdBigIntData,
}

impl<'sess> VdBsqRnumTermBigInt<'sess> {
    pub fn sign(self) -> Sign {
        self.inner().sign()
    }

    pub fn is_nonnegative(self) -> bool {
        self.sign() != Sign::Minus
    }

    pub fn show_fmt(self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner().show_fmt(f)
    }
}

impl<'sess> VdBsqRnumTermBigInt<'sess> {
    pub fn sub(self, rhs: Self, db: &'sess FloaterDb) -> VdBsqRnumTerm<'sess> {
        let sub = self.inner() - rhs.inner();
        match sub {
            Left(inner) => VdBsqRnumTermBigInt::new(inner, db).into(),
            Right(i) => i.into(),
        }
    }
}
