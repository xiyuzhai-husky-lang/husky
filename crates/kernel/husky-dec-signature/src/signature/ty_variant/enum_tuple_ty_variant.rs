use super::*;
use husky_entity_kind::ritchie::RitchieItemKind;
use husky_syn_decl::decl::ty_variant::tuple_ty_variant::TypeTupleVariantSynDecl;

#[salsa::interned]
pub struct EnumTupleVariantDecTemplate {
    pub parent_ty_template: EnumDecTemplate,
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
        parent_ty_template: EnumDecTemplate,
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
                    DeclarativeRitchieSimpleParameter::new(Contract::Move, field.ty).into()
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
