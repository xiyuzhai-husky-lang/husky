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

#[salsa::jar(db = DeclarativeSignatureDb)]
pub struct DeclarativeSignatureJar(
    declarative_term_region,
    // type
    enum_ty_declarative_signature_template,
    EnumTypeDeclarativeSignatureTemplate,
    unit_struct_ty_declarative_signature_template,
    UnitStructTypeDeclarativeSignatureTemplate,
    tuple_struct_ty_declarative_signature_template,
    TupleStructTypeDeclarativeSignatureTemplate,
    regular_struct_ty_declarative_signature_template,
    RegularStructTypeDeclarativeSignatureTemplate,
    record_ty_declarative_signature_template,
    RecordTypeDeclarativeSignatureTemplate,
    inductive_ty_declarative_signature_template,
    InductiveTypeDeclarativeSignatureTemplate,
    structure_ty_declarative_signature_template,
    StructureTypeDeclarativeSignatureTemplate,
    alien_ty_declarative_signature_template,
    ExternTypeDeclarativeSignatureTemplate,
    union_ty_declarative_signature_template,
    UnionTypeDeclarativeSignatureTemplate,
    // trait
    trai_declarative_signature_template,
    TraitDeclarativeSignatureTemplate,
    // form
    // form_signature,
    var_signature,
    ValDeclarativeSignatureTemplate,
    fn_declarative_signature,
    FnDeclarativeSignatureTemplate,
    gn_signature,
    GnDeclarativeSignatureTemplate,
    type_alias_signature,
    TypeAliasDeclarativeSignatureTemplate,
    // impl block
    // impl_block_signature_from_decl,
    ty_impl_block_declarative_signature,
    TypeImplBlockDeclarativeSignatureTemplate,
    trai_for_ty_impl_block_declarative_signature,
    TraitForTypeImplBlockDeclarativeSignatureTemplate,
    // variant
    UnitVariantDeclarativeSignatureTemplate,
    PropsVariantDeclarativeSignatureTemplate,
    TupleVariantDeclarativeSignatureTemplate,
    // associated items
    // associated_item_declarative_signature_from_decl,
    // type item
    ty_associated_fn_declarative_signature_template,
    TypeAssociatedFnDeclarativeSignatureTemplate,
    ty_method_fn_declarative_signature,
    TypeMethodFnDeclarativeSignatureTemplate,
    ty_method_function_declarative_signature_template,
    TypeMethodFunctionDeclarativeSignatureTemplate,
    ty_associated_ty_declarative_signature_template_from_decl,
    TypeAssociatedTypeDeclarativeSignatureTemplate,
    ty_associated_val_declarative_signature_template,
    TypeAssociatedValDeclarativeSignatureTemplate,
    ty_memo_signature,
    TypeMemoizedFieldDeclarativeSignatureTemplate,
    ty_method_declarative_signature_templates_map,
    // trait item
    trai_associated_form_fn_declarative_signature,
    TraitAssociatedFnDeclarativeSignatureTemplate,
    trai_method_fn_signature,
    TraitMethodFnSignatureTempalte,
    trai_associated_ty_declarative_signature_template,
    TraitAssociatedTypeDeclarativeSignatureTemplate,
    trai_associated_val_declarative_signature,
    TraitAssociatedValDeclarativeSignatureTemplate,
    // type as trait item
    trai_for_ty_associated_fn_declarative_signature_template,
    TraitForTypeAssociatedFnDeclarativeSignatureTemplate,
    trai_for_ty_method_fn_signature,
    TraitForTypeMethodFnDeclarativeSignatureTemplateTemplate,
    trai_for_ty_associated_ty_declarative_signature_template,
    TraitForTypeAssociatedTypeDeclarativeSignatureTemplate,
    trai_for_ty_associated_val_declarative_signature_template,
    TraitForTypeAssociatedValDeclarativeSignatureTemplate,
    // decr
    DeriveDecrDeclarativeSignatureTemplate,
    derive_decr_declarative_signature,
);
