use super::*;
use husky_syn_decl::decl::ty_variant::props_ty_variant::TypePropsVariantSynDecl;
use husky_syn_expr::syndicates::props_field::PropsFieldSyndicate;

#[salsa::interned]
pub struct EnumPropsVariantHirDecl {
    pub path: TypeVariantPath,
    #[return_ref]
    pub fields: SmallVec<[EnumPropsVariantField; 2]>,
    pub hir_eager_expr_region: HirEagerExprRegion,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct EnumPropsVariantField {
    ident: Ident,
    ty: HirType,
}

impl EnumPropsVariantHirDecl {
    pub(super) fn from_syn(
        path: TypeVariantPath,
        syn_decl: TypePropsVariantSynDecl,
        db: &::salsa::Db,
    ) -> Self {
        let builder = HirDeclBuilder::new(syn_decl.syn_expr_region(db), db);
        let fields = syn_decl
            .fields(db)
            .iter()
            .map(|field| EnumPropsVariantField::from_syn(field, &builder))
            .collect();
        Self::new(db, path, fields, builder.finish().eager())
    }
}

impl EnumPropsVariantField {
    fn from_syn(field: &PropsFieldSyndicate, builder: &HirDeclBuilder) -> Self {
        Self {
            ident: field.ident(),
            ty: builder.hir_ty(field.ty()).unwrap(),
        }
    }

    pub fn ident(&self) -> Ident {
        self.ident
    }

    pub fn ty(self) -> HirType {
        self.ty
    }
}
