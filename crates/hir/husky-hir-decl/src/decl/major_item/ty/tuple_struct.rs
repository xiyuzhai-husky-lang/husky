use super::*;
use husky_syn_decl::decl::major_item::ty::tuple_struct::TupleStructSynDecl;
use husky_syn_expr::syndicates::tuple_field::TupleFieldSyndicate;

#[salsa::interned]
pub struct TupleStructHirDecl {
    pub path: TypePath,
    #[return_ref]
    pub template_parameters: HirTemplateParameters,
    #[return_ref]
    pub fields: SmallVec<[TupleFieldHirDecl; 2]>,
    pub hir_eager_expr_region: HirEagerExprRegion,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
pub struct TupleFieldHirDecl {
    ty: HirType,
}

impl TupleStructHirDecl {
    pub(super) fn from_syn(path: TypePath, syn_decl: TupleStructSynDecl, db: &::salsa::Db) -> Self {
        let builder = HirDeclBuilder::new(syn_decl.syn_expr_region(db), db);
        let template_parameters =
            HirTemplateParameters::from_syn(syn_decl.template_parameters(db), &builder);
        let fields = syn_decl
            .fields(db)
            .iter()
            .map(|field| TupleFieldHirDecl::from_syn(field, &builder))
            .collect();
        Self::new(
            db,
            path,
            template_parameters,
            fields,
            builder.finish().eager(),
        )
    }
}

impl TupleFieldHirDecl {
    fn from_syn(field: &TupleFieldSyndicate, builder: &HirDeclBuilder) -> Self {
        Self {
            ty: builder.hir_ty(field.ty()).unwrap(),
        }
    }

    pub fn ty(self) -> HirType {
        self.ty
    }
}
