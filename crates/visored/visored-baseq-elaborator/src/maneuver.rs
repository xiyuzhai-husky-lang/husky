pub mod diff;
mod litnum_rewrite;

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
