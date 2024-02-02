use super::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct EnumPropsVariantDecTemplate {
    pub parent_ty_template: EnumTypeDecTemplate,
    pub field_tys: SmallVec<[EnumPropsVariantFieldDecTemplate; 4]>,
    pub return_ty: DeclarativeTerm,
    pub instance_constructor_ty: RitchieDeclarativeTerm,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct EnumPropsVariantFieldDecTemplate {
    ident: Ident,
    ty: DeclarativeTerm,
}

impl EnumPropsVariantDecTemplate {
    pub(super) fn from_decl(
        db: &::salsa::Db,
        parent_ty_template: EnumTypeDecTemplate,
        decl: TypePropsVariantSynDecl,
    ) -> DeclarativeSignatureResult<Self> {
        let syn_expr_region = decl.syn_expr_region(db);
        let declarative_term_region = declarative_term_region(db, syn_expr_region);
        let fields = decl
            .fields(db)
            .iter()
            .enumerate()
            .map(|(i, field)| {
                Ok(EnumPropsVariantFieldDecTemplate {
                    ident: field.ident(),
                    ty: declarative_term_region.expr_term(field.ty()).map_err(|_| {
                        DeclarativeSignatureError::FieldTypeDeclarativeTermError(
                            i.try_into().unwrap(),
                        )
                    })?,
                })
            })
            .collect::<DeclarativeSignatureResult<SmallVec<_>>>()?;
        // todo: GADT can override return_ty
        let return_ty = parent_ty_template.self_ty(db);
        let instance_constructor_ty = RitchieDeclarativeTerm::new(
            db,
            RitchieTypeKind::Fn.into(),
            fields
                .iter()
                .copied()
                .map(|field: EnumPropsVariantFieldDecTemplate| {
                    DeclarativeRitchieRegularParameter::new(TermContract::Move, field.ty).into()
                })
                .collect(),
            return_ty,
        );
        Ok(Self::new(
            db,
            parent_ty_template,
            fields,
            return_ty,
            instance_constructor_ty,
        ))
    }
}
