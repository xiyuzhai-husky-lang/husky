use husky_vfs::linktime_target_path::LinktimeTargetPath;

pub trait HasSynopsis: Copy {
    type Synopsis;

    fn synopsis(self, db: &::salsa::Db) -> Self::Synopsis;
}

pub enum LinktimeTargetSynopsis {}

impl HasSynopsis for LinktimeTargetPath {
    type Synopsis = LinktimeTargetSynopsis;

    fn synopsis(self, db: &salsa::Db) -> Self::Synopsis {
        todo!()
    }
}
