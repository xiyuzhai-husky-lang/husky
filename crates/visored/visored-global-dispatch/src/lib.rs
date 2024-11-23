pub mod default_table;
pub mod dispatch;
pub mod jar;
pub mod menu;
#[cfg(test)]
mod tests;

use self::jar::VdGlobalDispatchJar as Jar;
#[cfg(test)]
use self::tests::*;
