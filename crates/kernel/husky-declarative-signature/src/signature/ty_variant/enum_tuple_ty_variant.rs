use crate::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct EnumTupleVariantDecTemplate {
    pub parent_ty_template: EnumTypeDecTemplate,
    pub fields: SmallVec<[EnumTupleVariantFieldDecTemplate; 4]>,
    pub return_ty: DeclarativeTerm,
    pub instance_constructor_ty: RitchieDeclarativeTerm,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct EnumTupleVariantFieldDecTemplate {
    ty: DeclarativeTerm,
}

impl EnumTupleVariantDecTemplate {
    pub(super) fn from_decl(
        db: &::salsa::Db,
        parent_ty_template: EnumTypeDecTemplate,
        decl: TypeTupleVariantSynDecl,
    ) -> DeclarativeSignatureResult<Self> {
        let syn_expr_region = decl.syn_expr_region(db);
        let declarative_term_region = declarative_term_region(db, syn_expr_region);
        let fields = decl
            .fields(db)
            .iter()
            .enumerate()
            .map(|(i, field)| {
                Ok(EnumTupleVariantFieldDecTemplate {
                    ty: match declarative_term_region.expr_term(field.ty()) {
                        Ok(ty) => ty,
                        Err(_) => {
                            return Err(DeclarativeSignatureError::FieldTypeDeclarativeTermError(
                                i.try_into().unwrap(),
                            ))
                        }
                    },
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
                .map(|field: EnumTupleVariantFieldDecTemplate| {
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
