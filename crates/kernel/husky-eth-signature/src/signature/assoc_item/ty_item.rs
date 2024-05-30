pub mod assoc_ritchie;
pub mod assoc_ty;
pub mod assoc_val;
pub mod memo_field;
pub mod method_curry;
pub mod method_ritchie;

use self::assoc_ritchie::*;
use self::memo_field::*;
use self::method_curry::*;
use self::method_ritchie::*;
use super::*;
use husky_dec_signature::signature::{assoc_item::ty_item::TypeItemDecTemplate, HasDecTemplate};
use husky_entity_kind::TypeItemKind;
use husky_entity_path::path::{assoc_item::ty_item::TypeItemPath, major_item::ty::TypePath};
use husky_entity_tree::node::assoc_item::ty_item::HasItemPathsMap;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum TypeItemEthTemplate {
    AssocRitchie(TypeAssocRitchieEthTemplate),
    MethodRitchie(TypeMethodRitchieEthTemplate),
    MethodCurry(TypeMethodCurryEthTemplate),
    MemoizedField(TypeMemoizedFieldEthTemplate),
}

impl TypeItemEthTemplate {
    pub fn path(self, db: &::salsa::Db) -> TypeItemPath {
        match self {
            TypeItemEthTemplate::AssocRitchie(slf) => slf.path(db),
            TypeItemEthTemplate::MethodRitchie(slf) => slf.path(db),
            TypeItemEthTemplate::MethodCurry(slf) => slf.path(db),
            TypeItemEthTemplate::MemoizedField(slf) => slf.path(db),
        }
    }

    pub fn self_ty(self, db: &::salsa::Db) -> Option<EthTerm> {
        match self {
            TypeItemEthTemplate::AssocRitchie(_) => None,
            TypeItemEthTemplate::MethodRitchie(template) => Some(template.self_ty(db)),
            TypeItemEthTemplate::MethodCurry(_) => todo!(),
            TypeItemEthTemplate::MemoizedField(template) => Some(template.self_ty(db)),
        }
    }
}

impl HasEthTemplate for TypeItemPath {
    type EthTemplate = TypeItemEthTemplate;

    fn eth_template(self, db: &::salsa::Db) -> EthSignatureResult<Self::EthTemplate> {
        ty_item_eth_template(db, self)
    }
}

#[salsa::tracked]
pub(crate) fn ty_item_eth_template(
    db: &::salsa::Db,
    path: TypeItemPath,
) -> EthSignatureResult<TypeItemEthTemplate> {
    Ok(match path.dec_template(db)? {
        TypeItemDecTemplate::AssocRitchie(template) => {
            TypeAssocRitchieEthTemplate::from_dec(db, path, template)?.into()
        }
        TypeItemDecTemplate::MethodRitchie(template) => {
            TypeMethodRitchieEthTemplate::from_dec(db, path, template)?.into()
        }
        TypeItemDecTemplate::AssocType(_) => todo!(),
        TypeItemDecTemplate::AssocVal(_) => todo!(),
        TypeItemDecTemplate::MemoizedField(template) => {
            TypeMemoizedFieldEthTemplate::from_dec(db, path, template)?.into()
        }
    })
}

pub trait HasTypeItemTemplates: Copy {
    fn ty_item_templates_map<'a>(
        self,
        db: &'a ::salsa::Db,
    ) -> EthSignatureResult<&'a [(Ident, EthSignatureResult<TypeItemEthTemplates>)]>;

    fn ty_item_eth_templates<'a>(
        self,
        db: &'a ::salsa::Db,
        ident: Ident,
    ) -> EthSignatureMaybeResult<&'a TypeItemEthTemplates> {
        use vec_like::VecMapGetEntry;
        match self.ty_item_templates_map(db)?.get_entry(ident) {
            Some((_, Ok(templates))) => JustOk(templates),
            Some((_, Err(e))) => JustErr(*e),
            None => Nothing,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TypeItemEthTemplates {
    AssocRitchie(SmallVecImpl<TypeAssocRitchieEthTemplate>),
    MethodFn(SmallVecImpl<TypeMethodRitchieEthTemplate>),
    MethodFunction(SmallVecImpl<TypeMethodCurryEthTemplate>),
    MemoizedField(SmallVecImpl<TypeMemoizedFieldEthTemplate>),
}

impl HasTypeItemTemplates for TypePath {
    fn ty_item_templates_map<'a>(
        self,
        db: &'a ::salsa::Db,
    ) -> EthSignatureResult<&'a [(Ident, EthSignatureResult<TypeItemEthTemplates>)]> {
        ty_item_eth_templates_map(db, self)
            .as_ref()
            .map(|v| v as &[_])
            .map_err(|e| *e)
    }
}

#[salsa::tracked(return_ref)]
pub(crate) fn ty_item_eth_templates_map(
    db: &::salsa::Db,
    ty_path: TypePath,
) -> EthSignatureResult<IdentPairMap<EthSignatureResult<TypeItemEthTemplates>>> {
    Ok(
        IdentPairMap::from_iter_assuming_no_repetitions(ty_path.item_paths_map(db).iter().map(
            |(ident, (ty_item_kind, result))| -> (Ident, EthSignatureResult<TypeItemEthTemplates>) {
                let result = match result {
                    Ok(paths) => match ty_item_kind {
                        TypeItemKind::MethodRitchie(_) => paths
                            .iter()
                            .copied()
                            .map(|path| match path.eth_template(db) {
                                Ok(TypeItemEthTemplate::MethodRitchie(template)) => Ok(template),
                                Err(e) => Err(e),
                                _ => unreachable!(),
                            })
                            .collect::<EthSignatureResult<SmallVecImpl<_>>>()
                            .map(TypeItemEthTemplates::MethodFn),
                        TypeItemKind::AssocRitchie(_) => paths
                            .iter()
                            .copied()
                            .map(|path| match path.eth_template(db) {
                                Ok(TypeItemEthTemplate::AssocRitchie(template)) => Ok(template),
                                Err(e) => Err(e),
                                _ => unreachable!(),
                            })
                            .collect::<EthSignatureResult<SmallVecImpl<_>>>()
                            .map(TypeItemEthTemplates::AssocRitchie),
                        TypeItemKind::AssocVal => todo!(),
                        TypeItemKind::AssocType => todo!(),
                        TypeItemKind::MemoizedField => paths
                            .iter()
                            .copied()
                            .map(|path| match path.eth_template(db) {
                                Ok(TypeItemEthTemplate::MemoizedField(template)) => Ok(template),
                                Err(e) => Err(e),
                                _ => unreachable!(),
                            })
                            .collect::<EthSignatureResult<SmallVecImpl<_>>>()
                            .map(TypeItemEthTemplates::MemoizedField),
                        TypeItemKind::AssocConceptual => todo!(),
                        TypeItemKind::AssocStatic => todo!(),
                        TypeItemKind::AssocTermic => todo!(),
                    },
                    Err(_e) => Err(EthSignatureError::EntityTreeError),
                };
                (*ident, result)
            },
        ))
        .expect("expect no repetitions"),
    )
}
