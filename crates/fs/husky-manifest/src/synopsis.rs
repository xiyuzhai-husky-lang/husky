pub mod linktime_target;
pub mod package;

use crate::*;

pub trait HasSynopsis: Copy {
    type Synopsis;

    fn synopsis<'a>(self, db: &'a ::salsa::Db) -> ManifestResultRef<'a, &'a Self::Synopsis>;
}
