use super::*;
use crate::signature::impl_block::ty_impl_block::TypeImplBlockEthTemplate;
use husky_dec_signature::signature::assoc_item::ty_item::memo_field::TypeMemoizedFieldDecTemplate;
use package::PackageEthSignatureData;

#[salsa::interned]
pub struct TypeMemoizedFieldEthTemplate {
    pub path: TypeItemPath,
    pub impl_block: TypeImplBlockEthTemplate,
    pub return_ty: EthTerm,
}

impl TypeMemoizedFieldEthTemplate {
    pub fn self_ty(self, db: &::salsa::Db) -> EthTerm {
        self.impl_block(db).self_ty(db)
    }
}

impl TypeMemoizedFieldEthTemplate {
    pub(super) fn from_dec(
        db: &::salsa::Db,
        path: TypeItemPath,
        declarative_signature: TypeMemoizedFieldDecTemplate,
    ) -> EthSignatureResult<TypeMemoizedFieldEthTemplate> {
        let impl_block = path.impl_block(db).eth_template(db)?;
        let return_ty = EthTerm::ty_from_dec(db, declarative_signature.return_ty(db))?;
        Ok(TypeMemoizedFieldEthTemplate::new(
            db, path, impl_block, return_ty,
        ))
    }

    fn try_instantiate<'db>(
        self,
        target_self_ty_arguments: &[EthTerm],
        ctx: &EthSignatureBuilderContext,
        db: &'db ::salsa::Db,
    ) -> EthSignatureMaybeResult<TypeMemoizedFieldEtherealSignature> {
        let self_ty = self.impl_block(db).self_ty(db);
        let mut builder = self
            .impl_block(db)
            .template_parameters(db)
            .empty_instantiation_builder(self.path(db).into(), true, ctx);
        builder.try_add_rules_from_application(self_ty, target_self_ty_arguments, db)?;
        let instantiation = builder.try_into_instantiation().expect("business done");
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
    instantiation: EthInstantiation,
    return_ty: EthTerm,
}

impl TypeMemoizedFieldEtherealSignature {
    pub fn path(&self) -> TypeItemPath {
        self.path
    }

    pub fn instantiation(&self) -> &EthInstantiation {
        &self.instantiation
    }

    pub fn return_ty(&self) -> EthTerm {
        self.return_ty
    }
}

pub trait HasTypeMemoizedFieldEthTemplates: Copy {
    fn ty_memo_field_eth_templates_map<'a>(
        self,
        db: &'a ::salsa::Db,
    ) -> EthSignatureResult<
        &'a [(
            Ident,
            EthSignatureResult<SmallVecImpl<TypeMemoizedFieldEthTemplate>>,
        )],
    >;

    fn ty_memo_field_eth_templates<'a>(
        self,
        db: &'a ::salsa::Db,
        ident: Ident,
    ) -> EthSignatureMaybeResult<&'a [TypeMemoizedFieldEthTemplate]> {
        use vec_like::VecMapGetEntry;
        match self.ty_memo_field_eth_templates_map(db)?.get_entry(ident) {
            Some((_, Ok(templates))) => JustOk(templates),
            Some((_, Err(e))) => JustErr(*e),
            None => Nothing,
        }
    }
}

pub trait HasTypeMemoizedFieldEtherealSignature: Copy {
    fn ty_memo_field_ethereal_signature<'db>(
        self,
        arguments: &'db [EthTerm],
        ident: Ident,
        ctx: &EthSignatureBuilderContext,
        db: &'db ::salsa::Db,
    ) -> EthSignatureMaybeResult<TypeMemoizedFieldEtherealSignature>;
}

impl HasTypeMemoizedFieldEtherealSignature for TypePath {
    fn ty_memo_field_ethereal_signature<'db>(
        self,
        arguments: &'db [EthTerm],
        ident: Ident,
        ctx: &EthSignatureBuilderContext,
        db: &'db ::salsa::Db,
    ) -> EthSignatureMaybeResult<TypeMemoizedFieldEtherealSignature> {
        let TypeItemEthTemplates::MemoizedField(tmpls) = self.ty_item_eth_templates(db, ident)?
        else {
            return Nothing;
        };
        for tmpl in tmpls {
            if let Some(signature) = tmpl
                .try_instantiate(arguments, ctx, db)
                .into_result_option()?
            {
                return JustOk(signature);
            }
        }
        Nothing
    }
}
