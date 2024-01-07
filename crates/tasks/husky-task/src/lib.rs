#![feature(associated_type_bounds)]
pub mod dev_ascension;
pub mod helpers;
pub mod linktime;

use self::dev_ascension::*;
use self::linktime::*;


pub trait IsTask: Send + 'static {
    type DevAscension: IsDevAscension;
}
