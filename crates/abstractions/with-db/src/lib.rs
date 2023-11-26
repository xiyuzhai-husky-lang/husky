use salsa::Db;

pub struct WithDbRef<'me, T>
where
    T: ?Sized,
{
    t: &'me T,
    db: &'me Db,
}
impl<'me, T> std::fmt::Debug for WithDbRef<'me, T>
where
    T: ?Sized + std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WithDbRef").field("t", &self.t).finish()
    }
}

pub trait WithDb {
    fn with_db<'me>(&'me self, db: &'me Db) -> WithDbRef<'me, Self>;
}

impl<T> WithDb for T
where
    T: ?Sized,
{
    fn with_db<'me>(&'me self, db: &'me Db) -> WithDbRef<'me, Self> {
        WithDbRef { t: self, db }
    }
}

impl<'a, T> PartialEq for WithDbRef<'a, T>
where
    T: ?Sized + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.db as *const Db == other.db as *const Db && self.t == other.t
    }
}

impl<'a, T> PartialOrd for WithDbRef<'a, T>
where
    T: ?Sized + PartialEq + PartialOrdWithDb,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        assert_eq!(self.db as *const Db, other.db as *const Db);
        self.t.partial_cmp_with_db(self.db, &other.t)
    }
}

pub trait PartialOrdWithDb {
    fn partial_cmp_with_db(&self, db: &::salsa::Db, other: &Self) -> Option<std::cmp::Ordering>;
}
