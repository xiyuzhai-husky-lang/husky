use crate::*;
use husky_entity_kind::ritchie::RitchieItemKind;

#[salsa::interned(db = DecSignatureDb, jar = DecSignatureJar)]
pub struct EnumTupleVariantDecTemplate {
    pub parent_ty_template: EnumTypeDecTemplate,
    pub fields: SmallVec<[EnumTupleVariantFieldDecTemplate; 4]>,
    pub return_ty: DecTerm,
    pub instance_constructor_ty: DecRitchie,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct EnumTupleVariantFieldDecTemplate {
    ty: DecTerm,
}

impl EnumTupleVariantDecTemplate {
    pub(super) fn from_decl(
        db: &::salsa::Db,
        parent_ty_template: EnumTypeDecTemplate,
        decl: TypeTupleVariantSynDecl,
    ) -> DecSignatureResult<Self> {
        let syn_expr_region = decl.syn_expr_region(db);
        let dec_term_region = syn_expr_dec_term_region(db, syn_expr_region);
        let fields = decl
            .fields(db)
            .iter()
            .enumerate()
            .map(|(i, field)| {
                Ok(EnumTupleVariantFieldDecTemplate {
                    ty: match dec_term_region.expr_term(field.ty()) {
                        Ok(ty) => ty,
                        Err(_) => {
                            return Err(DecSignatureError::FieldTypeDecTermError(
                                i.try_into().unwrap(),
                            ))
                        }
                    },
                })
            })
            .collect::<DecSignatureResult<SmallVec<_>>>()?;
        // todo: GADT can override return_ty
        let return_ty = parent_ty_template.self_ty(db);
        let instance_constructor_ty = DecRitchie::new(
            db,
            RitchieItemKind::Fn.into(),
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
