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
    pub(super) fn from_ethereal(
        path: TypePath,
        ethereal_signature_template: TupleStructTypeEtherealSignatureTemplate,
        db: &dyn HirDeclDb,
    ) -> Self {
        let mut builder = HirEagerExprBuilder::new(db);
        let template_parameters = HirTemplateParameters::from_ethereal(
            ethereal_signature_template.template_parameters(db),
            db,
        );
        let fields = ethereal_signature_template
            .fields(db)
            .iter()
            .map(|field| TupleFieldHirDecl::from_ethereal(field, db))
            .collect();
        let hir_expr_region = builder.finish();
        Self::new(db, path, template_parameters, fields, hir_expr_region)
    }
}

impl TupleFieldHirDecl {
    fn from_ethereal(field: &TupleFieldEtherealSignatureTemplate, db: &dyn HirDeclDb) -> Self {
        Self {
            ty: HirType::from_ethereal(field.ty(), db),
        }
    }
}
