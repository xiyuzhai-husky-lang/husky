use super::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct PropsStructDeclarativeSignatureTemplate {
    #[return_ref]
    pub implicit_parameters: DeclarativeGenericParameters,
    pub self_ty: DeclarativeTerm,
    #[return_ref]
    pub fields: SmallVec<[PropsStructFieldDeclarativeSignatureTemplate; 4]>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclarativeSignatureDb, jar= DeclarativeSignatureJar)]
pub struct PropsStructFieldDeclarativeSignatureTemplate {
    ident: Ident,
    ty: DeclarativeTerm,
}

impl PropsStructDeclarativeSignatureTemplate {
    pub(super) fn from_decl(
        db: &dyn DeclarativeSignatureDb,
        path: TypePath,
        decl: PropsStructTypeDecl,
    ) -> DeclarativeSignatureResult<Self> {
        let expr_region = decl.expr_region(db);
        let declarative_term_region = declarative_term_region(db, expr_region);
        let declarative_term_menu = db.declarative_term_menu(expr_region.toolchain(db)).unwrap();
        let implicit_parameters = DeclarativeGenericParameters::from_decl(
            decl.implicit_parameters(db),
            declarative_term_region,
            declarative_term_menu,
        );
        let self_ty = construct_self_ty(db, path, &implicit_parameters);
        let fields = decl
            .fields(db)
            .iter()
            .enumerate()
            .map(|(i, field)| {
                Ok(PropsStructFieldDeclarativeSignatureTemplate {
                    ident: field.ident(),
                    ty: match declarative_term_region.expr_term(field.ty_expr_idx()) {
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
        Ok(Self::new(db, implicit_parameters, self_ty, fields))
    }
}

impl PropsStructFieldDeclarativeSignatureTemplate {
    pub fn ident(&self) -> Ident {
        self.ident
    }

    pub fn ty(&self) -> DeclarativeTerm {
        self.ty
    }

    pub fn into_ritchie_parameter_contracted_ty(self) -> DeclarativeTermRitchieParameter {
        DeclarativeTermRitchieRegularParameter::new(Contract::Move, self.ty).into()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclarativeSignatureDb)]
pub struct PropsStructDeclarativeSignature {}
