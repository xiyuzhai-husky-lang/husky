use husky_syn_expr::SynExprRegion;

use crate::*;

pub trait DeclarativeSignatureDb:
    salsa::DbWithJar<DeclarativeSignatureJar> + SynDeclDb + DeclarativeTermDb
{
    fn declarative_term_region(&self, syn_expr_region: SynExprRegion) -> &DeclarativeTermRegion;
}

impl<Db> DeclarativeSignatureDb for Db
where
    Db: salsa::DbWithJar<DeclarativeSignatureJar> + SynDeclDb + DeclarativeTermDb,
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
    UnitStructTypeDeclarativeSignatureTemplate,
    TupleStructTypeDeclarativeSignatureTemplate,
    PropsStructTypeDeclarativeSignatureTemplate,
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
    ValFugitiveDeclarativeSignatureTemplate,
    FnFugitiveDeclarativeSignatureTemplate,
    GnFugitiveDeclarativeSignatureTemplate,
    TypeAliasFugitiveDeclarativeSignatureTemplate,
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
    TypeAssociatedFnDeclarativeSignatureTemplate,
    TypeMethodFnDeclarativeSignatureTemplate,
    TypeMethodFunctionDeclarativeSignatureTemplate,
    TypeAssociatedTypeDeclarativeSignatureTemplate,
    TypeAssociatedValDeclarativeSignatureTemplate,
    TypeMemoizedFieldDeclarativeSignatureTemplate,
    // trait item
    trai_item_syn_declarative_signature_template,
    TraitAssociatedFnDeclarativeSignatureTemplate,
    TraitMethodFnDeclarativeSignatureTemplate,
    TraitAssociatedTypeDeclarativeSignatureTemplate,
    TraitAssociatedValDeclarativeSignatureTemplate,
    // type as trait item
    trai_for_ty_item_syn_declarative_signature_from_decl,
    TraitForTypeAssociatedFnDeclarativeSignatureTemplate,
    TraitForTypeMethodFnDeclarativeSignatureTemplate,
    TraitForTypeAssociatedTypeDeclarativeSignatureTemplate,
    TraitForTypeAssociatedValDeclarativeSignatureTemplate,
    // decr
    decr_declarative_signature_template,
    DeriveDecrDeclarativeSignatureTemplate,
    DeriveDecrShardDeclarativeSignatureTemplate,
);
