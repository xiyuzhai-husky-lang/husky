mod db;
mod engine;
mod hir;
#[cfg(test)]
mod tests;
mod transpile;

pub use db::*;
pub use hir::*;

#[cfg(test)]
use tests::*;
use transpile::*;

#[salsa::jar(db = CHirDb)]
pub struct CHirJar(
    CStructDeclHir,
    CEnumDeclHir,
    CUnionDeclHir,
    CFunctionDeclHir,
    CValueDeclHir,
    CFunctionDefnHir,
    CValueDefnHir,
    CAliasDefnHir,
);
