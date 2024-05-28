use super::*;
use husky_syn_decl::decl::major_item::ty::tuple_struct::TupleStructSynDecl;
use husky_term_prelude::ritchie::RitchieKind;

#[salsa::interned]
pub struct TupleStructDecTemplate {
    #[return_ref]
    pub template_parameters: DecTemplateParameters,
    pub self_ty: DecTerm,
    #[return_ref]
    pub fields: SmallVec<[TupleStructFieldDecTemplate; 4]>,
    pub instance_constructor_ritchie_ty: DecRitchie,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct TupleStructFieldDecTemplate {
    ty: DecTerm,
}

impl TupleStructDecTemplate {
    pub fn from_decl(
        db: &::salsa::Db,
        path: TypePath,
        decl: TupleStructSynDecl,
    ) -> DecSignatureResult<Self> {
        let syn_expr_region = decl.syn_expr_region(db);
        let dec_term_region = syn_expr_dec_term_region(db, syn_expr_region);
        let dec_term_menu = db.dec_term_menu(syn_expr_region.toolchain(db)).unwrap();
        let template_parameters = DecTemplateParameters::from_decl(
            decl.template_parameters(db),
            dec_term_region,
            dec_term_menu,
        );
        let self_ty = construct_self_ty(db, path, &template_parameters);
        let fields: SmallVec<[TupleStructFieldDecTemplate; 4]> = decl
            .fields(db)
            .iter()
            .enumerate()
            .map(|(i, field)| {
                Ok(TupleStructFieldDecTemplate {
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
        let instance_constructor_ritchie_ty = DecRitchie::new(
            db,
            RitchieKind::RITCHIE_TYPE_FN,
            fields
                .iter()
                .copied()
                .map(TupleStructFieldDecTemplate::into_ritchie_parameter_contracted_ty)
                .collect(),
            self_ty,
        );
        Ok(Self::new(
            db,
            template_parameters,
            self_ty,
            fields,
            instance_constructor_ritchie_ty,
        ))
    }
}

impl TupleStructFieldDecTemplate {
    pub fn into_ritchie_parameter_contracted_ty(self) -> DeclarativeRitchieParameter {
        DeclarativeRitchieSimpleParameter::new(Contract::Move, self.ty).into()
    }

    pub fn ty(&self) -> DecTerm {
        self.ty
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
pub struct TupleStructDecSignature {}
