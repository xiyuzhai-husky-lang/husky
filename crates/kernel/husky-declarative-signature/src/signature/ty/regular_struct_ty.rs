use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclarativeSignatureDb)]
pub struct RegularStructTypeDeclarativeSignature {}

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub fn regular_struct_ty_declarative_signature_template(
    db: &dyn DeclarativeSignatureDb,
    decl: RegularStructTypeDecl,
) -> DeclarativeSignatureResult<RegularStructTypeDeclarativeSignatureTemplate> {
    let expr_region = decl.expr_region(db);
    let declarative_term_region = declarative_term_region(db, expr_region);
    let declarative_term_menu = db.declarative_term_menu(expr_region.toolchain(db)).unwrap();
    Ok(RegularStructTypeDeclarativeSignatureTemplate::new(
        db,
        ImplicitParameterDeclarativeSignatureTemplates::from_decl(
            decl.implicit_parameters(db),
            declarative_term_region,
            declarative_term_menu,
        ),
        decl.fields(db)
            .iter()
            .enumerate()
            .map(|(i, field)| {
                Ok(RegularStructFieldSignature {
                    ident: field.ident(),
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
            .collect::<DeclarativeSignatureResult<Vec<_>>>()?,
    ))
}

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct RegularStructTypeDeclarativeSignatureTemplate {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterDeclarativeSignatureTemplates,
    #[return_ref]
    pub fields: Vec<RegularStructFieldSignature>,
}

impl RegularStructTypeDeclarativeSignatureTemplate {
    pub fn field_signature<FT: Copy>(self) -> RegularFieldSignature<FT> {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclarativeSignatureDb, jar= DeclarativeSignatureJar)]
pub struct RegularStructFieldSignature {
    ident: Ident,
    ty: DeclarativeTerm,
}

impl RegularStructFieldSignature {
    pub fn into_ritchie_parameter_contracted_ty(
        self,
    ) -> DeclarativeTermRitchieParameterContractedType {
        DeclarativeTermRitchieParameterContractedType::new(Contract::Move, self.ty)
    }

    pub fn ident(&self) -> Ident {
        self.ident
    }

    pub fn ty(&self) -> DeclarativeTerm {
        self.ty
    }
}
