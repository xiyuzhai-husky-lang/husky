use crate::db::EternerDb;

pub trait AsId: Copy {
    fn as_id(self) -> u32;

    fn from_id(id: u32, db: &EternerDb) -> Self;
}
