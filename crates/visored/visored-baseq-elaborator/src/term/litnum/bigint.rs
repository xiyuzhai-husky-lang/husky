use either::{Either, *};
use floated_sequential::{db::FloaterDb, floated};
use num_bigint::Sign;
use visored_term::term::literal::bigint::VdBigIntData;

use super::VdBsqLitnumTerm;

#[floated]
pub struct VdBsqBigInt<'sess> {
    #[return_ref]
    pub inner: VdBigIntData,
}

impl<'sess> std::fmt::Debug for VdBsqBigInt<'sess> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("VdBsqBigInt(`")?;
        self.show_fmt(f)?;
        f.write_str("`)")
    }
}

impl<'sess> VdBsqBigInt<'sess> {
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

impl<'sess> VdBsqBigInt<'sess> {
    pub fn sub(self, rhs: Self, db: &'sess FloaterDb) -> VdBsqLitnumTerm<'sess> {
        let sub = self.inner() - rhs.inner();
        match sub {
            Left(inner) => VdBsqBigInt::new(inner, db).into(),
            Right(i) => i.into(),
        }
    }
}
