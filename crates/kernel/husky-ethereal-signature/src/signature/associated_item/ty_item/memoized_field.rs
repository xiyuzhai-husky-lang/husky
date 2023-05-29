use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TypeMemoizedFieldEtherealSignatureTemplate {
    pub impl_block: TypeImplBlockEtherealSignatureTemplate,
    pub return_ty: EtherealTerm,
}

impl HasEtherealSignatureTemplate for TypeMemoizedFieldDeclarativeSignatureTemplate {
    type EtherealSignatureTemplate = TypeMemoizedFieldEtherealSignatureTemplate;

    fn ethereal_signature_template(
        self,
        db: &dyn EtherealSignatureDb,
    ) -> EtherealSignatureResult<Self::EtherealSignatureTemplate> {
        ty_memoized_field_ethereal_signature_template(db, self)
    }
}

#[salsa::tracked(jar = EtherealSignatureJar)]
pub(crate) fn ty_memoized_field_ethereal_signature_template(
    db: &dyn EtherealSignatureDb,
    declarative_signature: TypeMemoizedFieldDeclarativeSignatureTemplate,
) -> EtherealSignatureResult<TypeMemoizedFieldEtherealSignatureTemplate> {
    let impl_block = declarative_signature
        .impl_block(db)
        .ethereal_signature_template(db)?;
    let return_ty = EtherealTerm::ty_from_declarative(db, declarative_signature.return_ty(db))?;
    Ok(TypeMemoizedFieldEtherealSignatureTemplate::new(
        db, impl_block, return_ty,
    ))
}

pub struct TypeMemoizedFieldEtherealSignature {}

pub trait HasTypeMemoizedFieldEtherealSignatureTemplates: Copy {
    fn ty_memoized_field_ethereal_signature_templates_map<'a>(
        self,
        db: &'a dyn EtherealSignatureDb,
    ) -> EtherealSignatureResult<
        &'a [(
            Ident,
            EtherealSignatureResult<SmallVecImpl<TypeMemoizedFieldEtherealSignatureTemplate>>,
        )],
    >;

    fn ty_memoized_field_ethereal_signature_templates<'a>(
        self,
        db: &'a dyn EtherealSignatureDb,
        ident: Ident,
    ) -> EtherealSignatureMaybeResult<&'a [TypeMemoizedFieldEtherealSignatureTemplate]> {
        use vec_like::VecMapGetEntry;
        match self
            .ty_memoized_field_ethereal_signature_templates_map(db)?
            .get_entry(ident)
        {
            Some((_, Ok(templates))) => JustOk(templates),
            Some((_, Err(e))) => JustErr(*e),
            None => Nothing,
        }
    }
}

impl HasTypeMemoizedFieldEtherealSignatureTemplates for TypePath {
    fn ty_memoized_field_ethereal_signature_templates_map<'a>(
        self,
        db: &'a dyn EtherealSignatureDb,
    ) -> EtherealSignatureResult<
        &'a [(
            Ident,
            EtherealSignatureResult<SmallVecImpl<TypeMemoizedFieldEtherealSignatureTemplate>>,
        )],
    > {
        ty_memoized_field_ethereal_signature_templates_map(db, self)
            .as_ref()
            .map(|v| v as &[_])
            .map_err(|e| *e)
    }
}

#[salsa::tracked(jar = EtherealSignatureJar, return_ref)]
pub(crate) fn ty_memoized_field_ethereal_signature_templates_map(
    db: &dyn EtherealSignatureDb,
    ty_path: TypePath,
) -> EtherealSignatureResult<
    IdentPairMap<EtherealSignatureResult<SmallVecImpl<TypeMemoizedFieldEtherealSignatureTemplate>>>,
> {
    Ok(ty_path
        .ty_memoized_field_declarative_signature_templates_map(db)?
        .iter()
        .map(|(ident, result)| {
            let result = match result {
                Ok(templates) => templates
                    .iter()
                    .copied()
                    .map(|template| template.ethereal_signature_template(db))
                    .collect::<EtherealSignatureResult<SmallVecImpl<_>>>(),
                Err(e) => Err(todo!()),
            };
            (*ident, result)
        })
        .collect())
}

pub trait HasTypeMemoizedFieldEtherealSignature: Copy {
    fn ty_memoized_field_ethereal_signature<'a>(
        self,
        db: &'a dyn EtherealSignatureDb,
        arguments: &[EtherealTerm],
        ident: Ident,
    ) -> EtherealSignatureMaybeResult<TypeMemoizedFieldEtherealSignature>;
}

impl HasTypeMemoizedFieldEtherealSignature for TypePath {
    fn ty_memoized_field_ethereal_signature<'a>(
        self,
        db: &'a dyn EtherealSignatureDb,
        arguments: &[EtherealTerm],
        ident: Ident,
    ) -> EtherealSignatureMaybeResult<TypeMemoizedFieldEtherealSignature> {
        let templates = self.ty_memoized_field_ethereal_signature_templates(db, ident)?;
        todo!()
    }
}
