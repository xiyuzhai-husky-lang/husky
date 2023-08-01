use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TypeMemoizedFieldEtherealSignatureTemplate {
    pub impl_block: TypeImplBlockEtherealSignatureTemplate,
    pub return_ty: EtherealTerm,
}

impl TypeMemoizedFieldEtherealSignatureTemplate { 
    pub(super) fn from_declarative(
        db: &dyn EtherealSignatureDb,
        path: TypeItemPath,
        declarative_signature: TypeMemoizedFieldDeclarativeSignatureTemplate,
    ) -> EtherealSignatureResult<TypeMemoizedFieldEtherealSignatureTemplate> {
        let impl_block = path.impl_block(db).ethereal_signature_template(db)?;
        let return_ty = EtherealTerm::ty_from_declarative(db, declarative_signature.return_ty(db))?;
        Ok(TypeMemoizedFieldEtherealSignatureTemplate::new(
            db, impl_block, return_ty,
        ))
    }
    

    fn try_instantiate(
        self,
        db: &dyn EtherealSignatureDb,
        target_self_ty_arguments: &[EtherealTerm],
    ) -> EtherealSignatureMaybeResult<TypeMemoizedFieldEtherealSignature> {
        let self_ty = self.impl_block(db).self_ty(db);
        let self_ty_application_expansion = self_ty.application_expansion(db);
        let self_ty_arguments = self_ty_application_expansion.arguments(db);
        if self_ty_arguments == target_self_ty_arguments {
            return JustOk(TypeMemoizedFieldEtherealSignature {
                return_ty: self.return_ty(db),
            });
        }
        todo!()
    }
}
 
 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TypeMemoizedFieldEtherealSignature {
    return_ty: EtherealTerm,
}

impl TypeMemoizedFieldEtherealSignature {
    pub fn return_ty(&self) -> EtherealTerm {
        self.return_ty
    }
}

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

// impl HasTypeMemoizedFieldEtherealSignatureTemplates for TypePath {
//     fn ty_memoized_field_ethereal_signature_templates_map<'a>(
//         self,
//         db: &'a dyn EtherealSignatureDb,
//     ) -> EtherealSignatureResult<
//         &'a [(
//             Ident,
//             EtherealSignatureResult<SmallVecImpl<TypeMemoizedFieldEtherealSignatureTemplate>>,
//         )],
//     > {
//         ty_memoized_field_ethereal_signature_templates_map(db, self)
//             .as_ref()
//             .map(|v| v as &[_])
//             .map_err(|e| *e)
//     }
// }

// #[salsa::tracked(jar = EtherealSignatureJar, return_ref)]
// pub(crate) fn ty_memoized_field_ethereal_signature_templates_map(
//     db: &dyn EtherealSignatureDb,
//     ty_path: TypePath,
// ) -> EtherealSignatureResult<
//     IdentPairMap<EtherealSignatureResult<SmallVecImpl<TypeMemoizedFieldEtherealSignatureTemplate>>>,
// > {
//     Ok(IdentPairMap::from_iter_assuming_no_repetitions(
//         ty_path
//             .ty_memoized_field_declarative_signature_templates_map(db)?
//             .iter()
//             .map(|(ident, result)| {
//                 let result = match result {
//                     Ok(templates) => templates
//                         .iter()
//                         .copied()
//                         .map(|template| template.ethereal_signature_template(db))
//                         .collect::<EtherealSignatureResult<SmallVecImpl<_>>>(),
//                     Err(e) => Err(todo!()),
//                 };
//                 (*ident, result)
//             }),
//     )
//     .expect("no repetition"))
// }

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
        let TypeItemEtherealSignatureTemplates::MemoizedField(templates) = 
            self.ty_item_ethereal_signature_templates(db, ident)? else {
            return Nothing
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
