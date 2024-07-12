pub mod dev_ascension;
pub mod helpers;
pub mod linktime;

use self::dev_ascension::*;
use self::linktime::*;

pub trait HasDevBackend {
    type DevBackend: IsDevBackend;
}

pub trait IsDevBackend {}
