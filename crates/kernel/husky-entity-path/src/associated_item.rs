use crate::*;

#[salsa::interned(jar = EntityPathJar)]
pub struct TypeItemPath {
    pub parent_path: ModuleItemPath,
    pub ident: Identifier,
}

#[salsa::interned(jar = EntityPathJar)]
pub struct TraitItemPath {
    pub parent_path: ModuleItemPath,
    pub ident: Identifier,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AssociatedItemPath {
    Type(TypeItemPath),
    Trait(TraitItemPath),
}

impl From<TraitItemPath> for AssociatedItemPath {
    fn from(v: TraitItemPath) -> Self {
        Self::Trait(v)
    }
}

impl From<TypeItemPath> for AssociatedItemPath {
    fn from(v: TypeItemPath) -> Self {
        Self::Type(v)
    }
}

impl<Db> salsa::DebugWithDb<Db> for AssociatedItemPath
where
    Db: salsa::DbWithJar<EntityPathJar> + ?Sized,
{
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<EntityPathJar>>::as_jar_db(db);
        if include_all_fields {
            match self {
                AssociatedItemPath::Type(path) => f
                    .debug_tuple("Type")
                    .field(&path.debug_with(db, include_all_fields))
                    .finish(),
                AssociatedItemPath::Trait(path) => f
                    .debug_tuple("Trait")
                    .field(&path.debug_with(db, include_all_fields))
                    .finish(),
            }
        } else {
            match self {
                AssociatedItemPath::Type(path) => path.fmt(f, db, false),
                AssociatedItemPath::Trait(path) => path.fmt(f, db, false),
            }
        }
    }
}
