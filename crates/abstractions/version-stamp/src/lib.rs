

pub trait HasVersionStamp: Copy {
    type VersionStamp;

    fn version_stamp(self, db: &::salsa::Db) -> Self::VersionStamp;
}
