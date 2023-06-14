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
#[salsa::derive_debug_with_db(db = DeclarativeTypeDb)]
pub enum OriginalVarianceError {
    Todo,
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = DeclarativeTypeDb)]
pub enum DerivedVarianceError {
    DeclError,
    SignatureError,
    TypeItemNotFound,
}

pub(crate) fn entity_variances(
    db: &dyn DeclarativeTypeDb,
    path: EntityPath,
) -> VarianceResultRef<&[Variance]> {
    match path {
        EntityPath::Module(_) => Ok(&[]),
        EntityPath::ModuleItem(path) => match path {
            ModuleItemPath::Type(path) => ty_entity_variances(db, path).as_ref().map(Vec::as_ref),
            ModuleItemPath::Trait(path) => {
                trai_entity_variances(db, path).as_ref().map(Vec::as_ref)
            }
            ModuleItemPath::Fugitive(path) => {
                form_entity_variances(db, path).as_ref().map(Vec::as_ref)
            }
        },
        EntityPath::AssociatedItem(_) => todo!(),
        EntityPath::TypeVariant(_) => todo!(),
        EntityPath::ImplBlock(_) => todo!(),
    }
}

#[salsa::tracked(jar = DeclarativeTypeJar, return_ref)]
pub(crate) fn ty_entity_variances(
    db: &dyn DeclarativeTypeDb,
    path: TypePath,
) -> VarianceResult<Vec<Variance>> {
    calc_entity_variances(db, path)
}

#[salsa::tracked(jar = DeclarativeTypeJar, return_ref)]
pub(crate) fn trai_entity_variances(
    db: &dyn DeclarativeTypeDb,
    path: TraitPath,
) -> VarianceResult<Vec<Variance>> {
    calc_entity_variances(db, path)
}

#[salsa::tracked(jar = DeclarativeTypeJar, return_ref)]
pub(crate) fn form_entity_variances(
    db: &dyn DeclarativeTypeDb,
    path: FugitivePath,
) -> VarianceResult<Vec<Variance>> {
    calc_entity_variances(db, path)
}

#[salsa::tracked(jar = DeclarativeTypeJar, return_ref)]
pub(crate) fn ty_item_entity_variances(
    db: &dyn DeclarativeTypeDb,
    path: TypeItemPath,
) -> VarianceResult<Vec<Variance>> {
    calc_entity_variances(db, path)
}

fn calc_entity_variances(
    db: &dyn DeclarativeTypeDb,
    path: impl Into<EntityPath>,
) -> VarianceResult<Vec<Variance>> {
    let mut graph = VarianceGraph::new(db, path.into())?;
    graph.propagate(1000).unwrap();
    Ok(graph.finish())
}
