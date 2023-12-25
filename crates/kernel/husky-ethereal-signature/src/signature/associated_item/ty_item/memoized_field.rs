use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TypeMemoizedFieldEtherealSignatureTemplate {
    pub path: TypeItemPath,
    pub impl_block: TypeImplBlockEtherealSignatureTemplate,
    pub return_ty: EtherealTerm,
}

impl TypeMemoizedFieldEtherealSignatureTemplate {
    pub fn self_ty(self, db: &::salsa::Db) -> EtherealTerm {
        self.impl_block(db).self_ty(db)
    }
}

impl TypeMemoizedFieldEtherealSignatureTemplate {
    pub(super) fn from_declarative(
        db: &::salsa::Db,
        path: TypeItemPath,
        declarative_signature: TypeMemoizedFieldDeclarativeSignatureTemplate,
    ) -> EtherealSignatureResult<TypeMemoizedFieldEtherealSignatureTemplate> {
        let impl_block = path.impl_block(db).ethereal_signature_template(db)?;
        let return_ty = EtherealTerm::ty_from_declarative(db, declarative_signature.return_ty(db))?;
        Ok(TypeMemoizedFieldEtherealSignatureTemplate::new(
            db, path, impl_block, return_ty,
        ))
    }

    fn try_instantiate(
        self,
        db: &::salsa::Db,
        target_self_ty_arguments: &[EtherealTerm],
    ) -> EtherealSignatureMaybeResult<TypeMemoizedFieldEtherealSignature> {
        let self_ty = self.impl_block(db).self_ty(db);
        let mut instantiation_builder = self
            .impl_block(db)
            .template_parameters(db)
            .empty_instantiation_builder(true);
        instantiation_builder.try_add_rules_from_application(
            self_ty,
            target_self_ty_arguments,
            db,
        )?;
        let instantiation = instantiation_builder
            .try_into_instantiation()
            .expect("business done");
        debug_assert!(instantiation.separator().is_some());
        JustOk(TypeMemoizedFieldEtherealSignature {
            path: self.path(db),
            instantiation,
            return_ty: self.return_ty(db),
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TypeMemoizedFieldEtherealSignature {
    path: TypeItemPath,
    instantiation: EtherealInstantiation,
    return_ty: EtherealTerm,
}

impl TypeMemoizedFieldEtherealSignature {
    pub fn path(&self) -> TypeItemPath {
        self.path
    }

    pub fn instantiation(&self) -> &EtherealInstantiation {
        &self.instantiation
    }

    pub fn return_ty(&self) -> EtherealTerm {
        self.return_ty
    }
}

pub trait HasTypeMemoizedFieldEtherealSignatureTemplates: Copy {
    fn ty_memoized_field_ethereal_signature_templates_map<'a>(
        self,
        db: &'a ::salsa::Db,
    ) -> EtherealSignatureResult<
        &'a [(
            Ident,
            EtherealSignatureResult<SmallVecImpl<TypeMemoizedFieldEtherealSignatureTemplate>>,
        )],
    >;

    fn ty_memoized_field_ethereal_signature_templates<'a>(
        self,
        db: &'a ::salsa::Db,
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

pub trait HasTypeMemoizedFieldEtherealSignature: Copy {
    fn ty_memoized_field_ethereal_signature<'a>(
        self,
        db: &'a ::salsa::Db,
        arguments: &[EtherealTerm],
        ident: Ident,
    ) -> EtherealSignatureMaybeResult<TypeMemoizedFieldEtherealSignature>;
}

impl HasTypeMemoizedFieldEtherealSignature for TypePath {
    fn ty_memoized_field_ethereal_signature<'a>(
        self,
        db: &'a ::salsa::Db,
        arguments: &[EtherealTerm],
        ident: Ident,
    ) -> EtherealSignatureMaybeResult<TypeMemoizedFieldEtherealSignature> {
        let TypeItemEtherealSignatureTemplates::MemoizedField(templates) =
            self.ty_item_ethereal_signature_templates(db, ident)?
        else {
            return Nothing;
        };
        for template in templates {
            if let template = template {
                if let Some(signature) = template
                    .try_instantiate(db, arguments)
                    .into_result_option()?
                {
                    return JustOk(signature);
                }
            }
        }
        Nothing
    }
}
