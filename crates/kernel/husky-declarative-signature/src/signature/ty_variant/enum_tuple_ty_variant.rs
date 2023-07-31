use crate::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct EnumTupleTypeVariantDeclarativeSignatureTemplate {
    pub parent_ty_template: EnumTypeDeclarativeSignatureTemplate,
    pub field_tys: SmallVec<[EnumTupleTypeVariantFieldDeclarativeSignatureTemplate; 4]>,
    pub return_ty: DeclarativeTerm,
    pub instance_constructor_ty: DeclarativeTermRitchie,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct EnumTupleTypeVariantFieldDeclarativeSignatureTemplate {
    ty: DeclarativeTerm,
}

impl EnumTupleTypeVariantDeclarativeSignatureTemplate {
    pub(super) fn from_decl(
        db: &dyn DeclarativeSignatureDb,
        parent_ty_template: EnumTypeDeclarativeSignatureTemplate,
        decl: TupleTypeVariantSynDecl,
    ) -> DeclarativeSignatureResult<Self> {
        let syn_expr_region = decl.syn_expr_region(db);
        let declarative_term_region = declarative_term_region(db, syn_expr_region);
        let fields = decl
            .fields(db)
            .iter()
            .enumerate()
            .map(|(i, field)| {
                Ok(EnumTupleTypeVariantFieldDeclarativeSignatureTemplate {
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
        let instance_constructor_ty = DeclarativeTermRitchie::new(
            db,
            RitchieKind::FnType,
            fields
                .iter()
                .copied()
                .map(
                    |field: EnumTupleTypeVariantFieldDeclarativeSignatureTemplate| {
                        DeclarativeTermRitchieRegularParameter::new(Contract::Move, field.ty).into()
                    },
                )
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
