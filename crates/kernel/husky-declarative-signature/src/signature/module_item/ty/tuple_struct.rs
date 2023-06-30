use super::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct TupleStructDeclarativeSignatureTemplate {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterDeclarativeSignatures,
    pub self_ty: DeclarativeTerm,
    #[return_ref]
    pub fields: SmallVec<[TupleStructFieldDeclarativeSignatureTemplate; 4]>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct TupleStructFieldDeclarativeSignatureTemplate {
    ty: DeclarativeTerm,
}

impl TupleStructDeclarativeSignatureTemplate {
    pub fn from_decl(
        db: &dyn DeclarativeSignatureDb,
        path: TypePath,
        decl: TupleStructTypeDecl,
    ) -> DeclarativeSignatureResult<Self> {
        let expr_region = decl.expr_region(db);
        let declarative_term_region = declarative_term_region(db, expr_region);
        let declarative_term_menu = db.declarative_term_menu(expr_region.toolchain(db)).unwrap();
        let implicit_parameters = ImplicitParameterDeclarativeSignatures::from_decl(
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
        Ok(Self::new(db, implicit_parameters, self_ty, fields))
    }
}

impl TupleStructFieldDeclarativeSignatureTemplate {
    pub fn into_ritchie_parameter_contracted_ty(
        self,
    ) -> DeclarativeTermRitchieParameterContractedType {
        DeclarativeTermRitchieParameterContractedType::new(Contract::Move, self.ty)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclarativeSignatureDb)]
pub struct TupleStructTypeDeclarativeSignature {}
