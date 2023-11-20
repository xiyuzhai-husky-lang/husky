use super::*;
use husky_syn_decl::TupleTypeVariantSynDecl;
use husky_syn_expr::TupleFieldSyndicate;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct EnumTupleTypeVariantHirDecl {
    pub path: TypeVariantPath,
    #[return_ref]
    pub fields: SmallVec<[EnumTupleTypeVariantField; 2]>,
    pub hir_eager_expr_region: HirEagerExprRegion,
}

impl EnumTupleTypeVariantHirDecl {
    pub(super) fn from_syn(
        path: TypeVariantPath,
        syn_decl: TupleTypeVariantSynDecl,
        db: &dyn HirDeclDb,
    ) -> Self {
        let builder = HirDeclBuilder::new(syn_decl.syn_expr_region(db), db);
        let fields = syn_decl
            .fields(db)
            .iter()
            .map(|field| EnumTupleTypeVariantField::from_syn(field, &builder))
            .collect();
        Self::new(db, path, fields, builder.finish().eager())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = HirDeclDb)]
pub struct EnumTupleTypeVariantField {
    ty: HirType,
}

impl EnumTupleTypeVariantField {
    fn from_syn(field: &TupleFieldSyndicate, builder: &HirDeclBuilder) -> Self {
        Self {
            ty: builder.hir_ty(field.ty()),
        }
    }

    pub fn ty(self) -> HirType {
        self.ty
    }
}
