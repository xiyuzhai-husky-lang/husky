pub mod diff;
pub mod litnum_rewrite;

use crate::{elaborator::VdBsqElaboratorInner, *};

#[derive(Debug, PartialEq, Eq)]
pub enum VdBsqManeuver {
    Diff,
    LitnumRewrite,
}

#[derive(Debug, PartialEq, Eq)]
pub enum VdBsqManeuverCall {
    Diff,
    LitnumRewrite,
}

impl VdBsqManeuverCall {
    pub fn wrap<'db, 'sess, R>(self, m: impl ElabM<'db, 'sess, R>) -> impl ElabM<'db, 'sess, R>
    where
        'db: 'sess,
    {
        call::stack::with_call(self, m)
    }
}
