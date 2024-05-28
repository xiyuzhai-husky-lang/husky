#[path = "crate/lib.rs"]
pub mod lib;
#[path = "crate/main.rs"]
pub mod main;
#[path = "crate/task.rs"]
pub mod task;

use self::lib::*;
use self::main::*;
use self::task::*;
use super::*;

pub enum CrateHirDecl {
    Lib(LibCrateHirDecl),
    Main(MainCrateHirDecl),
}
