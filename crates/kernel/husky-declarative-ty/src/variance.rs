mod default;
mod dependency;
mod graph;
mod repr;

pub(crate) use dependency::*;
pub(crate) use repr::*;

use super::*;
use graph::*;
use propagate::*;

#[derive(Debug, PartialEq, Eq)]
pub enum VarianceError {
    Original(OriginalVarianceError),
    Derived(DerivedVarianceError),
}

impl From<DerivedVarianceError> for VarianceError {
    fn from(v: DerivedVarianceError) -> Self {
        Self::Derived(v)
    }
}

impl From<OriginalVarianceError> for VarianceError {
    fn from(v: OriginalVarianceError) -> Self {
        Self::Original(v)
    }
}

pub type VarianceResult<T> = Result<T, VarianceError>;
pub type VarianceResultRef<'a, T> = Result<T, &'a VarianceError>;

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = DeclarativeTypeDb, jar = DeclarativeTypeJar)]
pub enum OriginalVarianceError {
    Todo,
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = DeclarativeTypeDb, jar = DeclarativeTypeJar)]
pub enum DerivedVarianceError {
    DeclError,
    SignatureError,
    TypeItemNotFound,
}

pub(crate) fn item_variances(db: &::salsa::Db, path: ItemPath) -> VarianceResultRef<&[Variance]> {
    match path {
        ItemPath::Submodule(_, _) => Ok(&[]),
        ItemPath::MajorItem(path) => match path {
            MajorItemPath::Type(path) => ty_path_variances(db, path).as_ref().map(Vec::as_ref),
            MajorItemPath::Trait(path) => trai_item_variances(db, path).as_ref().map(Vec::as_ref),
            MajorItemPath::Fugitive(path) => {
                todo!()
                // form_item_variances(db, path).as_ref().map(Vec::as_ref)
            }
        },
        ItemPath::AssociatedItem(_) => todo!(),
        ItemPath::TypeVariant(_, _) => todo!(),
        ItemPath::ImplBlock(_) => todo!(),
        ItemPath::Attr(_, _) => todo!(),
    }
}

#[salsa::tracked(jar = DeclarativeTypeJar, return_ref)]
pub(crate) fn ty_path_variances(db: &::salsa::Db, path: TypePath) -> VarianceResult<Vec<Variance>> {
    calc_item_path_variances(db, path)
}

#[salsa::tracked(jar = DeclarativeTypeJar, return_ref)]
pub(crate) fn trai_item_variances(
    db: &::salsa::Db,
    path: TraitPath,
) -> VarianceResult<Vec<Variance>> {
    calc_item_path_variances(db, path)
}

#[salsa::tracked(jar = DeclarativeTypeJar, return_ref)]
pub(crate) fn fugitive_path_variances(
    db: &::salsa::Db,
    path: FugitivePath,
) -> VarianceResult<Vec<Variance>> {
    calc_item_path_variances(db, path)
}

#[salsa::tracked(jar = DeclarativeTypeJar, return_ref)]
pub fn ty_item_path_variances(
    db: &::salsa::Db,
    path: TypeItemPath,
) -> VarianceResult<Vec<Variance>> {
    calc_item_path_variances(db, path)
}

fn calc_item_path_variances(
    db: &::salsa::Db,
    path: impl Into<ItemPath>,
) -> VarianceResult<Vec<Variance>> {
    let mut graph = VarianceGraph::new(db, path.into())?;
    graph.propagate(1000).unwrap();
    Ok(graph.finish())
}

pub trait HasVariances: Copy {
    fn variances<'a>(self, db: &'a ::salsa::Db) -> VarianceResultRef<'a, &'a [Variance]>;
}

impl HasVariances for TypePath {
    fn variances<'a>(self, db: &'a salsa::Db) -> VarianceResultRef<'a, &'a [Variance]> {
        ty_path_variances(db, self).as_ref().map(|v| v as &[_])
    }
}
