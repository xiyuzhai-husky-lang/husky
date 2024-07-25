use super::*;
use husky_syn_decl::decl::major_item::ty::props_struct::PropsStructSynDecl;
use husky_term_prelude::ritchie::RitchieKind;

#[salsa::interned]
pub struct PropsStructDecTemplate {
    #[return_ref]
    pub template_parameters: DecTemplateParameters,
    pub self_ty: DecTerm,
    #[return_ref]
    pub fields: SmallVec<[PropsStructFieldDecTemplate; 4]>,
    pub instance_constructor_ritchie_ty: DecRitchie,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct PropsStructFieldDecTemplate {
    self_ty: DecTerm,
    ident: Ident,
    ty: DecTerm,
    has_initialization: bool,
}

impl PropsStructDecTemplate {
    pub(super) fn from_decl(
        db: &::salsa::Db,
        path: TypePath,
        decl: PropsStructSynDecl,
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
        let fields = decl
            .fields(db)
            .iter()
            .enumerate()
            .map(|(i, field)| {
                Ok(PropsStructFieldDecTemplate {
                    self_ty,
                    ident: field.ident(),
                    ty: match dec_term_region.expr_term(field.ty()) {
                        Ok(ty) => ty,
                        Err(_) => {
                            return Err(DecSignatureError::FieldTypeDecTermError(
                                i.try_into().unwrap(),
                            ))
                        }
                    },
                    has_initialization: field.initialization().is_some(),
                })
            })
            .collect::<DecSignatureResult<SmallVec<_>>>()?;
        let instance_constructor_ritchie_ty = DecRitchie::new(
            db,
            RitchieKind::RITCHIE_TYPE_FN,
            fields
                .iter()
                .copied()
                .filter_map(PropsStructFieldDecTemplate::into_ritchie_parameter_contracted_ty)
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

impl PropsStructFieldDecTemplate {
    pub fn self_ty(&self) -> DecTerm {
        self.self_ty
    }

    pub fn ident(&self) -> Ident {
        self.ident
    }

    pub fn ty(&self) -> DecTerm {
        self.ty
    }

    pub fn into_ritchie_parameter_contracted_ty(self) -> Option<DeclarativeRitchieParameter> {
        (!self.has_initialization)
            .then_some(DeclarativeRitchieSimpleParameter::new(Contract::Move, self.ty).into())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
pub struct PropsStructDecSignature {}
