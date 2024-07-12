pub mod devend;
pub mod helpers;
pub mod linktime;

use self::devend::*;
use self::linktime::*;

pub trait HasDevBackend {
    type DevBackend: IsDevBackend;
}

pub trait IsDevBackend {}
