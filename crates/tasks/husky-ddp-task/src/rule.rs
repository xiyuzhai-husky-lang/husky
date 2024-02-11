// basically rules forms a monoid
pub trait IsRule {
    type Storage;

    fn compose(self, other: Self, storage: &mut Self::Storage) -> Self;
}
