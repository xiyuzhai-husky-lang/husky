use crate::*;

pub trait AsTestInput<Db: ?Sized>: Sized {
    fn collect(db: &Db, dir: &std::path::Path) -> Vec<Self>;
    fn test_dirs() -> Vec<PathBuf>;
}

impl<Db: ?Sized> AsTestInput<Db> for String {
    fn collect(db: &Db, dir: &std::path::Path) -> Vec<Self> {
        todo!()
    }

    fn test_dirs() -> Vec<PathBuf> {
        // let cargo
        todo!()
    }
}
