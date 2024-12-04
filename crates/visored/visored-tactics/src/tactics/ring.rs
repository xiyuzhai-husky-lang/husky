use berserk::*;
use db::BerserkerDb;

// #[berserk::berserk]
// pub struct RingTerm<'db> {
//     #[return_ref]
//     data: RingTermData<'db>,
// }

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct __RingTermData<'db> {
    data: RingTermData<'db>,
}

impl<'db> AsStatic for __RingTermData<'db> {
    type Static = RingTermData<'static>;

    fn as_static(self) -> Self::Static {
        unsafe { std::mem::transmute(self.data) }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RingTerm<'db>(::berserk::Berserk<'db, __RingTermData<'db>>);

impl<'db> ::berserk::as_id::AsBerserkId<'db> for RingTerm<'db> {
    fn as_id(self) -> u32 {
        self.0.as_id()
    }
    fn from_id(id: u32, db: &'db ::berserk::db::BerserkerDb) -> Self {
        Self(berserk::Berserk::from_id(id, db))
    }
}
impl<'db> RingTerm<'db> {
    pub fn new(data: RingTermData<'db>, db: &'db ::berserk::db::BerserkerDb) -> Self {
        use berserk::Berserk;
        let data = __RingTermData { data };
        RingTerm(db.berserk(data))
    }
    pub fn data(self) -> &'db RingTermData<'db> {
        &self.0 .0.value.data
    }
}
impl<'db, Q: ?Sized> std::borrow::Borrow<Q> for __RingTermData<'db>
where
    RingTermData<'db>: std::borrow::Borrow<Q>,
{
    fn borrow(&self) -> &Q {
        self.data.borrow()
    }
}
impl<'db, 'a, Q: ?Sized> From<&'a Q> for __RingTermData<'db>
where
    RingTermData<'db>: From<&'a Q>,
{
    fn from(q: &'a Q) -> Self {
        Self { data: q.into() }
    }
}
impl<'db> RingTerm<'db> {
    pub fn from_ref<Q: Eq + std::hash::Hash + ?Sized>(
        q: &Q,
        db: &'db ::berserk::db::BerserkerDb,
    ) -> Self
    where
        RingTermData<'db>: std::borrow::Borrow<Q> + for<'a> From<&'a Q>,
    {
        RingTerm(db.berserk_ref::<__RingTermData<'db>, Q>(q))
    }
}

impl<'db> std::fmt::Debug for RingTerm<'db> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum RingTermData<'db> {
    Atom,
    Product(Vec<RingTerm<'db>>),
}

impl<'db> RingTerm<'db> {
    pub fn product(factors: impl IntoIterator<Item = Self>, db: &'db BerserkerDb) -> Self {
        Self::new(RingTermData::Product(factors.into_iter().collect()), db)
    }
}
