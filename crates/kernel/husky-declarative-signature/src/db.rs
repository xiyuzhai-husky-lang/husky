use husky_syn_decr::DecrDb;
use husky_syn_expr::SynExprRegion;

use crate::*;

pub trait DeclarativeSignatureDb:
    salsa::DbWithJar<DeclarativeSignatureJar> + DecrDb + DeclarativeTermDb
{
    fn declarative_term_region(&self, syn_expr_region: SynExprRegion) -> &DeclarativeTermRegion;
}

impl<Db> DeclarativeSignatureDb for Db
where
    Db: salsa::DbWithJar<DeclarativeSignatureJar> + DecrDb + DeclarativeTermDb,
{
    fn declarative_term_region(&self, syn_expr_region: SynExprRegion) -> &DeclarativeTermRegion {
        declarative_term_region(self, syn_expr_region)
    }
}

// todo: remove unnecessary tracked functions
// replace them by associated functions
#[salsa::jar(db = DeclarativeSignatureDb)]
pub struct DeclarativeSignatureJar(
    declarative_term_region,
    // type
    ty_declarative_signature_template,
    EnumTypeDeclarativeSignatureTemplate,
    UnitTypeStructDeclarativeSignatureTemplate,
    TupleTypeStructDeclarativeSignatureTemplate,
    PropsTypeStructDeclarativeSignatureTemplate,
    RecordTypeDeclarativeSignatureTemplate,
    InductiveTypeDeclarativeSignatureTemplate,
    StructureTypeDeclarativeSignatureTemplate,
    ExternTypeDeclarativeSignatureTemplate,
    UnionTypeDeclarativeSignatureTemplate,
    // trait
    TraitDeclarativeSignatureTemplate,
    trai_syn_declarative_signature_template,
    // fugitive
    // fugitive_signature,
    fugitive_syn_declarative_signature_template,
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
    ty_impl_block_syn_declarative_signature_template,
    TraitForTypeImplBlockDeclarativeSignatureTemplate,
    trai_for_ty_impl_block_syn_declarative_signature_template,
    // type variant
    ty_variant_syn_declarative_signature_template,
    EnumUnitTypeVariantDeclarativeSignatureTemplate,
    EnumPropsTypeVariantDeclarativeSignatureTemplate,
    EnumTupleTypeVariantDeclarativeSignatureTemplate,
    // associated items
    // associated_item_syn_declarative_signature_from_decl,
    // type item
    ty_item_syn_declarative_signature_template,
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
    // trait item
    trai_item_syn_declarative_signature_template,
    TraitAssociatedFnDeclarativeSignatureTemplate,
    trai_associated_form_fn_declarative_signature_template,
    TraitMethodFnDeclarativeSignatureTemplate,
    trai_method_fn_declarative_signature_template,
    TraitAssociatedTypeDeclarativeSignatureTemplate,
    trai_associated_ty_declarative_signature_template,
    TraitAssociatedValDeclarativeSignatureTemplate,
    trai_associated_val_declarative_signature_template,
    // type as trait item
    trai_for_ty_item_syn_declarative_signature_from_decl,
    TraitForTypeAssociatedFnDeclarativeSignatureTemplate,
    trai_for_ty_associated_fn_declarative_signature_template,
    TraitForTypeMethodFnDeclarativeSignatureTemplate,
    trai_for_ty_method_fn_declarative_signature_template,
    TraitForTypeAssociatedTypeDeclarativeSignatureTemplate,
    trai_for_ty_associated_ty_declarative_signature_template,
    TraitForTypeAssociatedValDeclarativeSignatureTemplate,
    trai_for_ty_associated_val_declarative_signature_template,
    // decr
    DeriveDecrDeclarativeSignatureTemplate,
    // derive_decr_declarative_signature_template,
    ty_path_derive_decr_declarative_signature_templates_map,
);
