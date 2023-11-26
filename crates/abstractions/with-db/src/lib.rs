pub struct WithDbRef<'me, T, Db>
where
    T: ?Sized,
    ,
{
    t: &'me T,
    db: &'me Db,
}
impl<'me, T, Db> std::fmt::Debug for WithDbRef<'me, T, Db>
where
    T: ?Sized + std::fmt::Debug,
    ,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WithDbRef").field("t", &self.t).finish()
    }
}

pub trait WithDb<Db>
where
    ,
{
    fn with_db<'me>(&'me self, db: &'me Db) -> WithDbRef<'me, Self, Db>;
}

impl<T, Db> WithDb<Db> for T
where
    T: ?Sized,
    ,
{
    fn with_db<'me>(&'me self, db: &'me Db) -> WithDbRef<'me, Self, Db> {
        WithDbRef { t: self, db }
    }
}

impl<'a, T, Db> PartialEq for WithDbRef<'a, T, Db>
where
    T: ?Sized + PartialEq,
    ,
{
    fn eq(&self, other: &Self) -> bool {
        self.db as *const Db == other.db as *const Db && self.t == other.t
    }
}

impl<'a, T, Db> PartialOrd for WithDbRef<'a, T, Db>
where
    T: ?Sized + PartialEq + PartialOrdWithDb<Db>,
    ,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        assert_eq!(self.db as *const Db, other.db as *const Db);
        self.t.partial_cmp_with_db(self.db, &other.t)
    }
}

pub trait PartialOrdWithDb<Db>
where
    ,
{
    fn partial_cmp_with_db(&self, db: &Db, other: &Self) -> Option<std::cmp::Ordering>;
}
