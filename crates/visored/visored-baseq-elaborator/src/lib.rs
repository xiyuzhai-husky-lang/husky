#![feature(let_chains)]
pub mod call;
pub mod coercion;
pub mod config;
pub mod elaborator;
pub mod expr;
pub mod helpers;
pub mod hypothesis;
pub mod maneuver;
mod monad;
pub mod outcome;
pub mod session;
mod signature;
pub mod strategy;
pub mod tactic;
pub mod term;
#[cfg(test)]
mod tests;
pub mod variable;

use crate::monad::ElabM;
use elaborator::VdBsqElaboratorInner;
use eterned::db::EternerDb;
use hypothesis::{contradiction::VdBsqHypothesisResult, VdBsqHypothesisIdx};
use miracle::error::MiracleAltMaybeResult;
use visored_models::VdModels;

type Mhr<'sess> = MiracleAltMaybeResult<VdBsqHypothesisResult<'sess, VdBsqHypothesisIdx<'sess>>>;
type Heuristic<'a, 'db, 'sess, T> =
    dyn Fn(&mut VdBsqElaboratorInner<'db, 'sess>, T) -> Mhr<'sess> + 'a;
