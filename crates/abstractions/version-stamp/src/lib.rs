pub trait HasVersionStamp<Db: ?Sized>: Copy {
    type VersionStamp;

    fn version_stamp(self, db: &Db) -> Self::VersionStamp;
}
