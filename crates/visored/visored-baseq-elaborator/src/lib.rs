#![feature(let_chains)]
pub mod call;
pub mod coercion;
pub mod config;
pub mod elaborator;
pub mod expr;
pub mod helpers;
pub mod hypothesis;
pub mod maneuver;
pub mod outcome;
pub mod session;
mod signature;
pub mod strategy;
pub mod tactic;
pub mod term;
#[cfg(test)]
mod tests;
pub mod variable;

use eterned::db::EternerDb;
use visored_models::VdModels;
