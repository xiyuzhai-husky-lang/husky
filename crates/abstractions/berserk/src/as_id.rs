use crate::db::BerserkerDb;

pub trait AsBerserkId<'db>: std::fmt::Debug + Copy + Eq {
    fn as_id(self) -> u32;

    fn from_id(id: u32, db: &'db BerserkerDb) -> Self;
}

/// As a special case, we permit `()` to be converted to an `Id`.
/// This is useful for declaring functions with no arguments.
impl<'db> AsBerserkId<'db> for () {
    fn as_id(self) -> u32 {
        0
    }

    fn from_id(id: u32, _db: &'db BerserkerDb) -> Self {
        assert_eq!(0, id);
        ()
    }
}
