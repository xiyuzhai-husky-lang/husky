use either::*;
use smallvec::{Array, SmallVec};
use std::{
    collections::{HashMap, HashSet},
    convert::Infallible,
    fmt,
    rc::Rc,
    sync::Arc,
};
use vec_like::{AsVecMapEntry, InsertEntryRepeatError, VecMap, VecSet};

pub trait DebugWithDb<Db: ?Sized> {
    fn debug<'me, 'db>(&'me self, db: &'me Db) -> DebugWith<'me, Db>
    where
        Self: Sized + 'me,
    {
        DebugWith {
            value: BoxRef::Ref(self),
            db,
            level: DebugFormatLevel::root(),
        }
    }

    fn debug_with<'me, 'db>(
        &'me self,
        db: &'me Db,
        level: DebugFormatLevelExt,
    ) -> DebugWith<'me, Db>
    where
        Self: Sized + 'me,
    {
        DebugWith {
            value: BoxRef::Ref(self),
            db,
            level: level.unwrap(),
        }
    }

    // /// Be careful when using this method inside a tracked function,
    // /// because the default macro generated implementation will read all fields,
    // /// maybe introducing undesired dependencies.
    // fn debug_all<'me, 'db>(&'me self, db: &'me Db) -> DebugWith<'me, Db>
    // where
    //     Self: Sized + 'me,
    // {
    //     DebugWith {
    //         value: BoxRef::Ref(self),
    //         db,
    //         level: true,
    //     }
    // }

    // fn into_debug<'me, 'db>(self, db: &'me Db) -> DebugWith<'me, Db>
    // where
    //     Self: Sized + 'me,
    // {
    //     DebugWith {
    //         value: BoxRef::Box(Box::new(self)),
    //         db,
    //         level: false,
    //     }
    // }

    // /// Be careful when using this method inside a tracked function,
    // /// because the default macro generated implementation will read all fields,
    // /// maybe introducing undesired dependencies.
    // fn into_debug_all<'me, 'db>(self, db: &'me Db) -> DebugWith<'me, Db>
    // where
    //     Self: Sized + 'me,
    // {
    //     DebugWith {
    //         value: BoxRef::Box(Box::new(self)),
    //         db,
    //         level: true,
    //     }
    // }

    /// if `level` is `false` only identity fields should be read, which means:
    ///     - for [#\[salsa::input\]](salsa_macros::input) no fields
    ///     - for [#\[salsa::tracked\]](salsa_macros::tracked) only fields with `#[id]` attribute
    ///     - for [#\[salsa::interned\]](salsa_macros::interned) any field
    fn fmt(&self, f: &mut fmt::Formatter<'_>, db: &Db, level: DebugFormatLevel) -> fmt::Result;
}

pub struct DebugWith<'me, Db: ?Sized> {
    value: BoxRef<'me, dyn DebugWithDb<Db> + 'me>,
    db: &'me Db,
    level: DebugFormatLevel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DebugFormatLevel(u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DebugFormatLevelExt(DebugFormatLevel);

impl DebugFormatLevelExt {
    fn unwrap(self) -> DebugFormatLevel {
        self.0
    }
}

impl DebugFormatLevel {
    pub fn root() -> Self {
        DebugFormatLevel(0)
    }

    pub fn next(self) -> DebugFormatLevelExt {
        DebugFormatLevelExt(self.next_inner())
    }

    fn next_inner(self) -> Self {
        Self(self.0 + 1)
    }

    fn parallel(self) -> DebugFormatLevelExt {
        DebugFormatLevelExt(self)
    }

    pub fn is_root(self) -> bool {
        self.0 == 0
    }
}

enum BoxRef<'me, T: ?Sized> {
    Box(Box<T>),
    Ref(&'me T),
}

impl<T: ?Sized> std::ops::Deref for BoxRef<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            BoxRef::Box(b) => b,
            BoxRef::Ref(r) => r,
        }
    }
}

impl<D: ?Sized> fmt::Debug for DebugWith<'_, D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        DebugWithDb::fmt(&*self.value, f, self.db, self.level)
    }
}

impl<Db: ?Sized> DebugWithDb<Db> for () {
    fn fmt(&self, f: &mut fmt::Formatter<'_>, db: &Db, level: DebugFormatLevel) -> fmt::Result {
        f.write_str("()")
    }
}

impl<Db: ?Sized> DebugWithDb<Db> for u8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>, db: &Db, level: DebugFormatLevel) -> fmt::Result {
        <Self as std::fmt::Debug>::fmt(self, f)
    }
}

impl<Db: ?Sized> DebugWithDb<Db> for u16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>, db: &Db, level: DebugFormatLevel) -> fmt::Result {
        <Self as std::fmt::Debug>::fmt(self, f)
    }
}

impl<Db: ?Sized> DebugWithDb<Db> for u32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>, db: &Db, level: DebugFormatLevel) -> fmt::Result {
        <Self as std::fmt::Debug>::fmt(self, f)
    }
}

impl<Db: ?Sized> DebugWithDb<Db> for u64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>, db: &Db, level: DebugFormatLevel) -> fmt::Result {
        <Self as std::fmt::Debug>::fmt(self, f)
    }
}

impl<Db: ?Sized> DebugWithDb<Db> for usize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>, db: &Db, level: DebugFormatLevel) -> fmt::Result {
        <Self as std::fmt::Debug>::fmt(self, f)
    }
}

impl<Db: ?Sized, T: ?Sized> DebugWithDb<Db> for &T
where
    T: DebugWithDb<Db>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>, db: &Db, level: DebugFormatLevel) -> fmt::Result {
        T::fmt(self, f, db, level)
    }
}

impl<Db: ?Sized, T: ?Sized> DebugWithDb<Db> for Box<T>
where
    T: DebugWithDb<Db>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>, db: &Db, level: DebugFormatLevel) -> fmt::Result {
        T::fmt(self, f, db, level)
    }
}

impl<Db: ?Sized, T> DebugWithDb<Db> for Rc<T>
where
    T: DebugWithDb<Db>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>, db: &Db, level: DebugFormatLevel) -> fmt::Result {
        T::fmt(self, f, db, level)
    }
}

impl<Db: ?Sized, T: ?Sized> DebugWithDb<Db> for Arc<T>
where
    T: DebugWithDb<Db>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>, db: &Db, level: DebugFormatLevel) -> fmt::Result {
        T::fmt(self, f, db, level)
    }
}

impl<Db: ?Sized, T> DebugWithDb<Db> for [T]
where
    T: DebugWithDb<Db>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>, db: &Db, level: DebugFormatLevel) -> fmt::Result {
        let elements = self.iter().map(|e| e.debug_with(db, level.parallel()));
        f.debug_list().entries(elements).finish()
    }
}

impl<Db: ?Sized, T> DebugWithDb<Db> for Vec<T>
where
    T: DebugWithDb<Db>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>, db: &Db, level: DebugFormatLevel) -> fmt::Result {
        let elements = self.iter().map(|e| e.debug_with(db, level.parallel()));
        f.debug_list().entries(elements).finish()
    }
}

impl<Db: ?Sized, T: Array> DebugWithDb<Db> for SmallVec<T>
where
    <T as Array>::Item: DebugWithDb<Db>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>, db: &Db, level: DebugFormatLevel) -> fmt::Result {
        let elements = self.iter().map(|e| e.debug_with(db, level.parallel()));
        f.debug_list().entries(elements).finish()
    }
}

impl<Db: ?Sized, T> DebugWithDb<Db> for Option<T>
where
    T: DebugWithDb<Db>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>, db: &Db, level: DebugFormatLevel) -> fmt::Result {
        let me = self.as_ref().map(|v| v.debug_with(db, level.parallel()));
        fmt::Debug::fmt(&me, f)
    }
}

impl<Db: ?Sized, T, E> DebugWithDb<Db> for Result<T, E>
where
    T: DebugWithDb<Db>,
    E: DebugWithDb<Db>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>, db: &Db, level: DebugFormatLevel) -> fmt::Result {
        match self {
            Ok(t) => f
                .debug_tuple("Ok")
                .field(&t.debug_with(db, level.parallel()))
                .finish(),
            Err(e) => f
                .debug_tuple("Err")
                .field(&e.debug_with(db, level.parallel()))
                .finish(),
        }
    }
}

impl<Db: ?Sized, L, R> DebugWithDb<Db> for Either<L, R>
where
    L: DebugWithDb<Db>,
    R: DebugWithDb<Db>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>, db: &Db, level: DebugFormatLevel) -> fmt::Result {
        match self {
            Left(l) => f
                .debug_tuple("Left")
                .field(&l.debug_with(db, level.parallel()))
                .finish(),
            Right(r) => f
                .debug_tuple("Right")
                .field(&r.debug_with(db, level.parallel()))
                .finish(),
        }
    }
}

impl<Db: ?Sized> DebugWithDb<Db> for Infallible {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>, _db: &Db, _level: DebugFormatLevel) -> fmt::Result {
        unreachable!()
    }
}

impl<Db: ?Sized, K, V, S> DebugWithDb<Db> for HashMap<K, V, S>
where
    K: DebugWithDb<Db>,
    V: DebugWithDb<Db>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>, db: &Db, level: DebugFormatLevel) -> fmt::Result {
        let elements = self.iter().map(|(k, v)| {
            (
                k.debug_with(db, level.parallel()),
                v.debug_with(db, level.parallel()),
            )
        });
        f.debug_map().entries(elements).finish()
    }
}

impl<Db: ?Sized, K> DebugWithDb<Db> for VecSet<K>
where
    K: PartialEq + Eq + DebugWithDb<Db>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>, db: &Db, level: DebugFormatLevel) -> fmt::Result {
        let elements = self
            .data()
            .iter()
            .map(|v| v.debug_with(db, level.parallel()));
        f.debug_list().entries(elements).finish()
    }
}

impl<Db: ?Sized, K, V> DebugWithDb<Db> for VecMap<V>
where
    K: PartialEq + Eq,
    V: AsVecMapEntry<K = K> + DebugWithDb<Db>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>, db: &Db, level: DebugFormatLevel) -> fmt::Result {
        let elements = self
            .data()
            .iter()
            .map(|v| v.debug_with(db, level.parallel()));
        f.debug_list().entries(elements).finish()
    }
}

impl<Db: ?Sized, Entry> DebugWithDb<Db> for InsertEntryRepeatError<Entry>
where
    Entry: DebugWithDb<Db>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>, db: &Db, level: DebugFormatLevel) -> fmt::Result {
        f.debug_struct("EntryRepeatError::Insert")
            .field("old", &self.old)
            .field("new", &self.new.debug_with(db, level.parallel()))
            .finish()
    }
}

impl<Db: ?Sized, A, B> DebugWithDb<Db> for (A, B)
where
    A: DebugWithDb<Db>,
    B: DebugWithDb<Db>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>, db: &Db, level: DebugFormatLevel) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.0.debug_with(db, level.parallel()))
            .field(&self.1.debug_with(db, level.parallel()))
            .finish()
    }
}

impl<Db: ?Sized, A, B, C> DebugWithDb<Db> for (A, B, C)
where
    A: DebugWithDb<Db>,
    B: DebugWithDb<Db>,
    C: DebugWithDb<Db>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>, db: &Db, level: DebugFormatLevel) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.0.debug_with(db, level.parallel()))
            .field(&self.1.debug_with(db, level.parallel()))
            .field(&self.2.debug_with(db, level.parallel()))
            .finish()
    }
}

impl<Db: ?Sized, V, S> DebugWithDb<Db> for HashSet<V, S>
where
    V: DebugWithDb<Db>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>, db: &Db, level: DebugFormatLevel) -> fmt::Result {
        let elements = self.iter().map(|e| e.debug_with(db, level.parallel()));
        f.debug_list().entries(elements).finish()
    }
}

/// This is used by the macro generated code.
/// If the field type implements `DebugWithDb`, uses that, otherwise, uses `Debug`.
/// That's the "has impl" trick (https://github.com/nvzqz/impls#how-it-works)
#[doc(hidden)]
pub mod helper {
    use super::{DebugFormatLevelExt, DebugWith, DebugWithDb};
    use std::{fmt, marker::PhantomData};

    pub trait Fallback<T: fmt::Debug, Db: ?Sized> {
        fn salsa_debug<'a, 'b>(
            a: &'a T,
            _db: &'b Db,
            _level: DebugFormatLevelExt,
        ) -> &'a dyn fmt::Debug {
            a
        }
    }

    pub struct SalsaDebug<T, Db: ?Sized>(PhantomData<T>, PhantomData<Db>);

    impl<T: DebugWithDb<Db>, Db: ?Sized> SalsaDebug<T, Db> {
        #[allow(dead_code)]
        pub fn salsa_debug<'a, 'b: 'a>(
            a: &'a T,
            db: &'b Db,
            level: DebugFormatLevelExt,
        ) -> DebugWith<'a, Db> {
            a.debug_with(db, level)
        }
    }

    impl<Everything, Db: ?Sized, T: fmt::Debug> Fallback<T, Db> for Everything {}
}
