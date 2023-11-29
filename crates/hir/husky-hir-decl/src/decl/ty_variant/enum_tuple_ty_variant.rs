use super::*;
use husky_syn_decl::TypeTupleVariantSynDecl;
use husky_syn_expr::TupleFieldSyndicate;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct EnumTupleVariantHirDecl {
    pub path: TypeVariantPath,
    #[return_ref]
    pub fields: SmallVec<[EnumTupleVariantField; 2]>,
    pub hir_eager_expr_region: HirEagerExprRegion,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db]
pub struct EnumTupleVariantField {
    ty: HirType,
}

impl EnumTupleVariantHirDecl {
    pub(super) fn from_syn(
        path: TypeVariantPath,
        syn_decl: TypeTupleVariantSynDecl,
        db: &::salsa::Db,
    ) -> Self {
        let builder = HirDeclBuilder::new(syn_decl.syn_expr_region(db), db);
        let fields = syn_decl
            .fields(db)
            .iter()
            .map(|field| EnumTupleVariantField::from_syn(field, &builder))
            .collect();
        Self::new(db, path, fields, builder.finish().eager())
    }
}

impl EnumTupleVariantField {
    fn from_syn(field: &TupleFieldSyndicate, builder: &HirDeclBuilder) -> Self {
        Self {
            ty: builder.hir_ty(field.ty()),
        }
    }

    pub fn ty(self) -> HirType {
        self.ty
    }
}
