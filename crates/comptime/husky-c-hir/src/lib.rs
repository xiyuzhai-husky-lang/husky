mod db;
mod engine;
mod hir;
#[cfg(test)]
mod tests;
mod transpile;

pub use db::*;
pub use hir::*;

use transpile::*;

#[salsa::jar(db = CHirDb)]
pub struct CHirJar(
    CStructDeclHir,
    CEnumDeclHir,
    CUnionDeclHir,
    CFunctionDeclHir,
    CValueDeclHir,
    CFunctionDefnHir,
    CValDefnHir,
    CAliasDefnHir,
);
