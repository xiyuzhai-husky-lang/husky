use super::*;

pub(super) fn vfs_robustness_test<Db, U, R>(db: &mut Db, f: &impl Fn(&Db, U) -> R)
where
    Db: ?Sized,
    U: VfsTestUnit,
{
    todo!()
}
