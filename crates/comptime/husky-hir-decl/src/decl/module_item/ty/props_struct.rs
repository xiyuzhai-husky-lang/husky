use super::*;

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
    pub(super) fn from_ethereal(
        path: TypePath,
        ethereal_signature_template: PropsStructTypeEtherealSignatureTemplate,
        db: &dyn HirDeclDb,
    ) -> Self {
        let TypeSynDecl::PropsStruct(syn_decl) = path.syn_decl(db).expect("hir stage ok") else {
            unreachable!()
        };
        let template_parameters = HirTemplateParameters::from_ethereal(
            ethereal_signature_template.template_parameters(db),
            db,
        );
        let fields = ethereal_signature_template
            .fields(db)
            .iter()
            .map(|field| PropsFieldHirDecl::from_ethereal(field, db))
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
    fn from_ethereal(field: &PropsFieldEtherealSignatureTemplate, db: &dyn HirDeclDb) -> Self {
        Self {
            ident: field.ident(),
            ty: HirType::from_ethereal(field.ty(), db),
        }
    }
}
