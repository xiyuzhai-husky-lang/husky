pub mod config;
pub mod error;
/// builder builds one instance of the pipeline, returns a tracker
mod executor;
pub mod helpers;
pub mod input;
/// smallest unit of the pipeline
mod instance;
pub mod lean4;
/// runner orchestrates all instances
pub mod runner;
#[cfg(test)]
mod tests;
/// tracker keeps the values of instance execution
mod tracker;

use self::{config::*, error::*};
use all_llms::transformation::AllLlmsStringTransformation;
