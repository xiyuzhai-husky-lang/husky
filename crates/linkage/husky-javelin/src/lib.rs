#![feature(if_let_guard)]
//! a javelin is a group of linkages, the name inspired by Diablo 2. For instance, the type constructor javelin will contains the type constructor linkage and also the (pattern) field access linkages.
//!
//! It's easier to collect javelins. We first collect javelins and then flatten them to get linkages.
//!
//! Javelins are divided into two classes (as inspired by Diablo 2):
//! - Amazon. Those javelins doesn't contain any template parameters and thus are always collected. In a sense, amazons are always there, nonvanishing.
//! - Valkyrie. Those javelins contain nonempty template parameters and thus are collected if invoked. So we need to do some "tree walk" to completely collect valkyrie javelins. In a sense, valkyries are summoned by amazons.
/// the name amazon comes from diablo 2
pub mod amazon;
pub mod context;
pub mod instantiation;
pub mod jar;
pub mod javelin;
pub mod path;
pub mod template_argument;
#[cfg(test)]
mod tests;
/// the name valkyrie comes from diablo 2
pub mod valkyrie;
pub mod version_stamp;

use self::jar::JavelinJar as Jar;
use self::javelin::Javelin;
#[cfg(test)]
use self::tests::*;
