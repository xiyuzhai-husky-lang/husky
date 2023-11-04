use husky_syn_decl::TupleStructTypeSynDecl;
use husky_syn_expr::TupleFieldObelisk;

use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct TupleStructTypeHirDecl {
    pub path: TypePath,
    #[return_ref]
    pub template_parameters: HirTemplateParameters,
    #[return_ref]
    pub fields: SmallVec<[TupleFieldHirDecl; 2]>,
    pub hir_expr_region: HirEagerExprRegion,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = HirDeclDb, jar= HirDeclJar)]
pub struct TupleFieldHirDecl {
    ty: HirType,
}

impl TupleStructTypeHirDecl {
    pub(super) fn from_syn(
        path: TypePath,
        syn_decl: TupleStructTypeSynDecl,
        db: &dyn HirDeclDb,
    ) -> Self {
        let TypeSynDecl::TupleStruct(syn_decl) = path.syn_decl(db).expect("hir stage ok") else {
            unreachable!()
        };
        let hir_eager_expr_region = hir_eager_expr_region(syn_decl.syn_expr_region(db), db);
        let template_parameters =
            HirTemplateParameters::from_syn(syn_decl.template_parameters(db), db);
        let fields = syn_decl
            .fields(db)
            .iter()
            .map(|field| TupleFieldHirDecl::from_syn(field, db, hir_eager_expr_region))
            .collect();
        Self::new(db, path, template_parameters, fields, hir_eager_expr_region)
    }
}

impl TupleFieldHirDecl {
    fn from_syn(
        field: &TupleFieldObelisk,
        db: &dyn HirDeclDb,
        hir_eager_expr_region: HirEagerExprRegion,
    ) -> Self {
        Self {
            ty: HirType::from_syn(field.ty(), db),
        }
    }
}
