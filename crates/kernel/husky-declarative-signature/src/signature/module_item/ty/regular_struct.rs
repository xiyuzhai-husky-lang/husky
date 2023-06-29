use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclarativeSignatureDb)]
pub struct PropsStructDeclarativeSignature {}

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct PropsStructDeclarativeSignatureTemplate {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterDeclarativeSignatures,
    #[return_ref]
    pub fields: SmallVec<[PropsStructFieldDeclarativeSignatureTemplate; 4]>,
}

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub fn regular_struct_declarative_signature_template(
    db: &dyn DeclarativeSignatureDb,
    decl: PropsStructTypeDecl,
) -> DeclarativeSignatureResult<PropsStructDeclarativeSignatureTemplate> {
    let expr_region = decl.expr_region(db);
    let declarative_term_region = declarative_term_region(db, expr_region);
    let declarative_term_menu = db.declarative_term_menu(expr_region.toolchain(db)).unwrap();
    Ok(PropsStructDeclarativeSignatureTemplate::new(
        db,
        ImplicitParameterDeclarativeSignatures::from_decl(
            decl.implicit_parameters(db),
            declarative_term_region,
            declarative_term_menu,
        ),
        decl.fields(db)
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
            .collect::<DeclarativeSignatureResult<SmallVec<_>>>()?,
    ))
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclarativeSignatureDb, jar= DeclarativeSignatureJar)]
pub struct PropsStructFieldDeclarativeSignatureTemplate {
    ident: Ident,
    ty: DeclarativeTerm,
}

impl PropsStructFieldDeclarativeSignatureTemplate {
    pub fn ident(&self) -> Ident {
        self.ident
    }

    pub fn ty(&self) -> DeclarativeTerm {
        self.ty
    }

    pub fn into_ritchie_parameter_contracted_ty(
        self,
    ) -> DeclarativeTermRitchieParameterContractedType {
        DeclarativeTermRitchieParameterContractedType::new(Contract::Move, self.ty)
    }
}
