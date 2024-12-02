use crate::db::EternerDb;

pub trait AsEternedId: std::fmt::Debug + Copy + Eq {
    fn as_id(self) -> u32;

    fn from_id(id: u32, db: &EternerDb) -> Self;
}

/// As a special case, we permit `()` to be converted to an `Id`.
/// This is useful for declaring functions with no arguments.
impl AsEternedId for () {
    fn as_id(self) -> u32 {
        0
    }

    fn from_id(id: u32, _db: &EternerDb) -> Self {
        assert_eq!(0, id);
        ()
    }
}
