use super::*;
use husky_syn_decl::TypePropsVariantSynDecl;
use husky_syn_expr::PropsFieldSyndicate;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct EnumPropsVariantHirDecl {
    pub path: TypeVariantPath,
    #[return_ref]
    pub fields: SmallVec<[EnumPropsVariantField; 2]>,
    pub hir_eager_expr_region: HirEagerExprRegion,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = HirDeclDb, jar= HirDeclJar)]
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
            ty: builder.hir_ty(field.ty_syn_expr_idx()).unwrap(),
        }
    }

    pub fn ty(self) -> HirType {
        self.ty
    }
}
