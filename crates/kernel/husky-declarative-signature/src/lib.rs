#![feature(trait_upcasting)]
mod db;
mod engine;
mod error;
mod parameter;
mod region;
mod signature;
#[cfg(test)]
mod tests;

pub use self::db::*;
pub use self::engine::*;
pub use self::error::*;
pub use self::parameter::*;
pub use self::region::*;
pub use self::signature::*;

use husky_decl::*;
use husky_declarative_term::*;
use husky_entity_path::*;
use husky_term_prelude::*;
use husky_word::*;
use smallvec::*;

type SmallVecImpl<T> = SmallVec<[T; 2]>;

#[salsa::jar(db = DeclarativeSignatureDb)]
pub struct DeclarativeSignatureJar(
    declarative_term_region,
    // type
    EnumDeclarativeSignatureTemplate,
    enum_declarative_signature_template,
    UnitStructDeclarativeSignatureTemplate,
    unit_struct_declarative_signature_template,
    TupleStructDeclarativeSignatureTemplate,
    tuple_struct_declarative_signature_template,
    RegularStructDeclarativeSignatureTemplate,
    regular_struct_declarative_signature_template,
    RecordDeclarativeSignatureTemplate,
    record_declarative_signature_template,
    InductiveDeclarativeSignatureTemplate,
    inductive_declarative_signature_template,
    StructureDeclarativeSignatureTemplate,
    structure_declarative_signature_template,
    ExternDeclarativeSignatureTemplate,
    extern_declarative_signature_template,
    UnionDeclarativeSignatureTemplate,
    union_declarative_signature_template,
    // trait
    TraitDeclarativeSignatureTemplate,
    trai_declarative_signature_template,
    // form
    // form_signature,
    ValDeclarativeSignatureTemplate,
    val_declarative_signature_template,
    FnDeclarativeSignatureTemplate,
    fn_declarative_signature_template,
    GnDeclarativeSignatureTemplate,
    gn_declarative_signature,
    TypeAliasDeclarativeSignatureTemplate,
    type_alias_declarative_signature,
    // impl block
    // impl_block_signature_from_decl,
    TypeImplBlockDeclarativeSignatureTemplate,
    ty_impl_block_declarative_signature_template,
    TraitForTypeImplBlockDeclarativeSignatureTemplate,
    trai_for_ty_impl_block_declarative_signature_template,
    // variant
    UnitVariantDeclarativeSignatureTemplate,
    PropsVariantDeclarativeSignatureTemplate,
    TupleVariantDeclarativeSignatureTemplate,
    // associated items
    // associated_item_declarative_signature_from_decl,
    // type item
    ty_associated_fn_declarative_signature_template,
    TypeAssociatedFnDeclarativeSignatureTemplate,
    ty_method_fn_declarative_signature_template,
    TypeMethodFnDeclarativeSignatureTemplate,
    TypeMethodFunctionDeclarativeSignatureTemplate,
    ty_method_function_declarative_signature_template,
    TypeAssociatedTypeDeclarativeSignatureTemplate,
    ty_associated_ty_declarative_signature_template,
    TypeAssociatedValDeclarativeSignatureTemplate,
    ty_associated_val_declarative_signature_template,
    TypeMemoizedFieldDeclarativeSignatureTemplate,
    ty_memoized_field_declarative_signature_template,
    ty_method_declarative_signature_templates_map,
    // trait item
    TraitAssociatedFnDeclarativeSignatureTemplate,
    trai_associated_form_fn_declarative_signature_template,
    TraitMethodFnDeclarativeSignatureTemplate,
    trai_method_fn_declarative_signature_template,
    TraitAssociatedTypeDeclarativeSignatureTemplate,
    trai_associated_ty_declarative_signature_template,
    TraitAssociatedValDeclarativeSignatureTemplate,
    trai_associated_val_declarative_signature_template,
    // type as trait item
    TraitForTypeAssociatedFnDeclarativeSignatureTemplate,
    trai_for_ty_associated_fn_declarative_signature_template,
    TraitForTypeMethodFnDeclarativeSignatureTemplateTemplate,
    trai_for_ty_method_fn_declarative_signature_template,
    TraitForTypeAssociatedTypeDeclarativeSignatureTemplate,
    trai_for_ty_associated_ty_declarative_signature_template,
    TraitForTypeAssociatedValDeclarativeSignatureTemplate,
    trai_for_ty_associated_val_declarative_signature_template,
    // decr
    DeriveDecrDeclarativeSignatureTemplate,
    derive_decr_declarative_signature_template,
);
