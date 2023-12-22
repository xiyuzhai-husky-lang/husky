use super::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct TupleStructTypeDeclarativeSignatureTemplate {
    #[return_ref]
    pub template_parameters: DeclarativeTemplateParameterTemplates,
    pub self_ty: DeclarativeTerm,
    #[return_ref]
    pub fields: SmallVec<[TupleStructFieldDeclarativeSignatureTemplate; 4]>,
    pub instance_constructor_ritchie_ty: DeclarativeTermRitchie,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct TupleStructFieldDeclarativeSignatureTemplate {
    ty: DeclarativeTerm,
}

impl TupleStructTypeDeclarativeSignatureTemplate {
    pub fn from_decl(
        db: &::salsa::Db,
        path: TypePath,
        decl: TupleStructTypeSynDecl,
    ) -> DeclarativeSignatureResult<Self> {
        let syn_expr_region = decl.syn_expr_region(db);
        let declarative_term_region = declarative_term_region(db, syn_expr_region);
        let declarative_term_menu = db
            .declarative_term_menu(syn_expr_region.toolchain(db))
            .unwrap();
        let template_parameters = DeclarativeTemplateParameterTemplates::from_decl(
            decl.template_parameters(db),
            declarative_term_region,
            declarative_term_menu,
        );
        let self_ty = construct_self_ty(db, path, &template_parameters);
        let fields: SmallVec<[TupleStructFieldDeclarativeSignatureTemplate; 4]> = decl
            .fields(db)
            .iter()
            .enumerate()
            .map(|(i, field)| {
                Ok(TupleStructFieldDeclarativeSignatureTemplate {
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
        let instance_constructor_ritchie_ty =
            DeclarativeTermRitchie::new(db, RitchieKind::RITCHIE_TYPE_FN, fields
                .iter()
                .copied()
                .map(
                    TupleStructFieldDeclarativeSignatureTemplate::into_ritchie_parameter_contracted_ty,
                )
                .collect(), self_ty);
        Ok(Self::new(
            db,
            template_parameters,
            self_ty,
            fields,
            instance_constructor_ritchie_ty,
        ))
    }
}

impl TupleStructFieldDeclarativeSignatureTemplate {
    pub fn into_ritchie_parameter_contracted_ty(self) -> DeclarativeRitchieParameter {
        DeclarativeRitchieRegularParameter::new(TermContract::Move, self.ty).into()
    }

    pub fn ty(&self) -> DeclarativeTerm {
        self.ty
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct TupleStructTypeDeclarativeSignature {}
