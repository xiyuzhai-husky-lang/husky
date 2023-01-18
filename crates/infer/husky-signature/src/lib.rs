mod db;
mod error;
mod parameter;
mod signature;

pub use db::*;
pub use error::*;
pub use parameter::*;
pub use signature::*;

use husky_decl::*;
use husky_term::*;
use husky_word::*;

#[salsa::jar(db = SignatureDb)]
pub struct SignatureJar(
    // type
    EnumTypeSignature,
    UnitStructTypeSignature,
    TupleStructTypeSignature,
    PropsStructTypeSignature,
    RecordTypeSignature,
    InductiveTypeSignature,
    StructureTypeSignature,
    AlienTypeSignature,
    UnionTypeSignature,
    // trait
    // trait_signature,
    TraitSignature,
    // form
    // form_signature,
    ValueSignature,
    FeatureSignature,
    FunctionSignature,
    MorphismSignature,
    TypeAliasSignature,
    // impl block
    // impl_block_signature,
    TypeImplBlockSignature,
    TypeAsTraitImplBlockSignature,
    // variant
    UnitVariantSignature,
    PropsVariantSignature,
    TupleVariantSignature,
    // associated items
    // associated_item_signature,
    // type item
    TypeAssociatedFunctionSignature,
    TypeMethodSignature,
    TypeAssociatedTypeSignature,
    TypeAssociatedValueSignature,
    TypeMemoSignature,
    // trait item
    TraitAssociatedFunctionSignature,
    TraitMethodSignature,
    TraitAssociatedTypeSignature,
    TraitAssociatedValueSignature,
    // type as trait item
    TypeAsTraitAssociatedFunctionSignature,
    TypeAsTraitMethodSignature,
    TypeAsTraitAssociatedTypeSignature,
    TypeAsTraitAssociatedValueSignature,
);
