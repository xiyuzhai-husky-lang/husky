use super::*;

#[salsa::interned(db = DecSignatureDb, jar = DecSignatureJar)]
pub struct TupleStructTypeDecTemplate {
    #[return_ref]
    pub template_parameters: DeclarativeTemplateParameterTemplates,
    pub self_ty: DecTerm,
    #[return_ref]
    pub fields: SmallVec<[TupleStructFieldDecTemplate; 4]>,
    pub instance_constructor_ritchie_ty: DecRitchie,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db]
pub struct TupleStructFieldDecTemplate {
    ty: DecTerm,
}

impl TupleStructTypeDecTemplate {
    pub fn from_decl(
        db: &::salsa::Db,
        path: TypePath,
        decl: TupleStructTypeSynDecl,
    ) -> DecSignatureResult<Self> {
        let syn_expr_region = decl.syn_expr_region(db);
        let declarative_term_region = syn_expr_dec_term_region(db, syn_expr_region);
        let declarative_term_menu = db
            .declarative_term_menu(syn_expr_region.toolchain(db))
            .unwrap();
        let template_parameters = DeclarativeTemplateParameterTemplates::from_decl(
            decl.template_parameters(db),
            declarative_term_region,
            declarative_term_menu,
        );
        let self_ty = construct_self_ty(db, path, &template_parameters);
        let fields: SmallVec<[TupleStructFieldDecTemplate; 4]> = decl
            .fields(db)
            .iter()
            .enumerate()
            .map(|(i, field)| {
                Ok(TupleStructFieldDecTemplate {
                    ty: match declarative_term_region.expr_term(field.ty()) {
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
        DeclarativeRitchieRegularParameter::new(TermContract::Move, self.ty).into()
    }

    pub fn ty(&self) -> DecTerm {
        self.ty
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db]
pub struct TupleStructTypeDecSignature {}
