#![feature(trait_upcasting)]
mod db;
mod decl;
mod error;
mod parameter;
mod parser;
mod sheet;
#[cfg(test)]
mod tests;

pub use db::*;
pub use decl::*;
pub use error::*;
pub use parameter::*;
pub use sheet::*;

use husky_ast::*;
use husky_entity_path::*;
use husky_entity_tree::*;
use husky_expr::*;
use husky_token::*;
use husky_vfs::ModulePath;
use parsec::ParseContext;
use parser::*;
#[cfg(test)]
use tests::*;

#[salsa::jar(db = DeclDb)]
pub struct DeclJar(
    // type
    ty_decl,
    EnumTypeDecl,
    UnitStructTypeDecl,
    TupleStructTypeDecl,
    RegularStructTypeDecl,
    RecordTypeDecl,
    InductiveTypeDecl,
    StructureTypeDecl,
    ExternTypeDecl,
    UnionTypeDecl,
    // trait
    trai_decl,
    TraitDecl,
    // form
    form_decl,
    ValueDecl,
    FeatureDecl,
    FunctionDecl,
    MorphismDecl,
    TypeAliasDecl,
    // impl block
    ty_impl_decl,
    TypeImplDecl,
    ty_as_trai_impl_decl,
    TypeAsTraitImplDecl,
    // variant
    UnitVariantDecl,
    PropsVariantDecl,
    TupleVariantDecl,
    // associated items
    associated_item_decl,
    // type item
    TypeAssociatedFunctionDecl,
    TypeMethodDecl,
    TypeAssociatedTypeDecl,
    TypeAssociatedValueDecl,
    TypeMemoDecl,
    // trait item
    TraitAssociatedFunctionDecl,
    TraitMethodDecl,
    TraitAssociatedTypeDecl,
    TraitAssociatedValueDecl,
    // type as trait item
    TypeAsTraitAssociatedFunctionDecl,
    TypeAsTraitMethodDecl,
    TypeAsTraitAssociatedTypeDecl,
    TypeAsTraitAssociatedValueDecl,
);
