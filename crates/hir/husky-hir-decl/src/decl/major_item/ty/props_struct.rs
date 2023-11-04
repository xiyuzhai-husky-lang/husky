use super::*;
use husky_syn_decl::PropsStructTypeSynDecl;
use husky_syn_expr::PropsFieldSyndicate;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct PropsStructTypeHirDecl {
    pub path: TypePath,
    #[return_ref]
    pub template_parameters: HirTemplateParameters,
    #[return_ref]
    pub fields: SmallVec<[PropsFieldHirDecl; 4]>,
    pub hir_expr_region: HirEagerExprRegion,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = HirDeclDb, jar= HirDeclJar)]
pub struct PropsFieldHirDecl {
    ident: Ident,
    ty: HirType,
}

impl PropsStructTypeHirDecl {
    pub(super) fn from_syn(
        path: TypePath,
        syn_decl: PropsStructTypeSynDecl,
        db: &dyn HirDeclDb,
    ) -> Self {
        let TypeSynDecl::PropsStruct(syn_decl) = path.syn_decl(db).expect("hir stage ok") else {
            unreachable!()
        };
        let builder = HirDeclBuilder::new(syn_decl.syn_expr_region(db), db);
        let template_parameters =
            HirTemplateParameters::from_syn(syn_decl.template_parameters(db), db);
        let fields = syn_decl
            .fields(db)
            .iter()
            .map(|field| PropsFieldHirDecl::from_syn(field, &builder))
            .collect();
        Self::new(
            db,
            path,
            template_parameters,
            fields,
            hir_eager_expr_region(syn_decl.syn_expr_region(db), db),
        )
    }
}

impl PropsFieldHirDecl {
    fn from_syn(field: &PropsFieldSyndicate, builder: &HirDeclBuilder) -> Self {
        Self {
            ident: field.ident(),
            ty: builder.hir_ty(field.ty_syn_expr_idx()),
        }
    }
}
