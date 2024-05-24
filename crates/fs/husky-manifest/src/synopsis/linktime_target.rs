use super::HasSynopsis;
use crate::ManifestResultRef;
use husky_vfs::linktime_target_path::LinktimeTargetPath;

#[derive(Debug, PartialEq, Eq)]
pub enum LinktimeTargetSynopsis {}

impl HasSynopsis for LinktimeTargetPath {
    type Synopsis = LinktimeTargetSynopsis;

    fn synopsis<'a>(self, db: &'a salsa::Db) -> ManifestResultRef<'a, &'a Self::Synopsis> {
        todo!()
    }
}
