//! Analysis of patterns, notably match exhaustiveness checking.

#![allow(rustc::untranslatable_diagnostic)]
#![allow(rustc::diagnostic_outside_of_impl)]

pub mod constructor;
pub mod context;
pub mod pattern;
pub mod pattern_column;
pub mod usefulness;

#[macro_use]
extern crate tracing;

use self::context::*;
use crate::pattern::DeconstructedPattern;
use husky_lifetime_utils::capture::Captures;

/// `bool` newtype that indicates whether this is a privately uninhabited field that we should skip
/// during analysis.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PrivateUninhabitedField(pub bool);

/// The arm of a match expression.
#[derive(Debug)]
pub struct MatchArm<'p, Ctx: PatternContext> {
    pub pat: &'p DeconstructedPattern<Ctx>,
    pub has_guard: bool,
    pub arm_data: Ctx::MatchArmData,
}

impl<'p, Ctx: PatternContext> Clone for MatchArm<'p, Ctx> {
    fn clone(&self) -> Self {
        Self {
            pat: self.pat,
            has_guard: self.has_guard,
            arm_data: self.arm_data,
        }
    }
}

impl<'p, Ctx: PatternContext> Copy for MatchArm<'p, Ctx> {}
