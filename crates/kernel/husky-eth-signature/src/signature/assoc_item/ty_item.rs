mod assoc_fn;
mod assoc_ty;
mod assoc_val;
mod memo_field;
mod method_fn;
mod method_function;

pub use self::assoc_fn::*;
pub use self::assoc_ty::*;
pub use self::memo_field::*;
pub use self::method_fn::*;
pub use self::method_function::*;

use super::*;
use husky_entity_kind::TypeItemKind;
use husky_entity_tree::HasItemPathsMap;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum TypeItemEthTemplate {
    AssocFn(TypeAssocFnEthTemplate),
    MethodFn(TypeMethodFnEthTemplate),
    MethodFunction(TypeMethodFunctionEthTemplate),
    MemoizedField(TypeMemoizedFieldEthTemplate),
}

impl TypeItemEthTemplate {
    pub fn self_ty(self, db: &::salsa::Db) -> Option<EthTerm> {
        match self {
            TypeItemEthTemplate::AssocFn(_) => None,
            TypeItemEthTemplate::MethodFn(template) => Some(template.self_ty(db)),
            TypeItemEthTemplate::MethodFunction(_) => todo!(),
            TypeItemEthTemplate::MemoizedField(template) => Some(template.self_ty(db)),
        }
    }
}

impl HasEthTemplate for TypeItemPath {
    type EthTemplate = TypeItemEthTemplate;

    fn eth_template(self, db: &::salsa::Db) -> EtherealSignatureResult<Self::EthTemplate> {
        ty_item_eth_template(db, self)
    }
}

// #[salsa::tracked(jar = EtherealSignatureJar)]
pub(crate) fn ty_item_eth_template(
    db: &::salsa::Db,
    path: TypeItemPath,
) -> EtherealSignatureResult<TypeItemEthTemplate> {
    Ok(match path.dec_template(db)? {
        TypeItemDecTemplate::AssocFn(template) => {
            TypeAssocFnEthTemplate::from_dec(db, path, template)?.into()
        }
        TypeItemDecTemplate::MethodFn(template) => {
            TypeMethodFnEthTemplate::from_dec(db, path, template)?.into()
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
    ) -> EtherealSignatureResult<&'a [(Ident, EtherealSignatureResult<TypeItemEthTemplates>)]>;

    fn ty_item_eth_templates<'a>(
        self,
        db: &'a ::salsa::Db,
        ident: Ident,
    ) -> EtherealSignatureMaybeResult<&'a TypeItemEthTemplates> {
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
    AssocFn(SmallVecImpl<TypeAssocFnEthTemplate>),
    MethodFn(SmallVecImpl<TypeMethodFnEthTemplate>),
    MethodFunction(SmallVecImpl<TypeMethodFunctionEthTemplate>),
    MemoizedField(SmallVecImpl<TypeMemoizedFieldEthTemplate>),
}

impl HasTypeItemTemplates for TypePath {
    fn ty_item_templates_map<'a>(
        self,
        db: &'a ::salsa::Db,
    ) -> EtherealSignatureResult<&'a [(Ident, EtherealSignatureResult<TypeItemEthTemplates>)]> {
        ty_item_eth_templates_map(db, self)
            .as_ref()
            .map(|v| v as &[_])
            .map_err(|e| *e)
    }
}

#[salsa::tracked(jar = EtherealSignatureJar, return_ref)]
pub(crate) fn ty_item_eth_templates_map(
    db: &::salsa::Db,
    ty_path: TypePath,
) -> EtherealSignatureResult<IdentPairMap<EtherealSignatureResult<TypeItemEthTemplates>>> {
    Ok(
        IdentPairMap::from_iter_assuming_no_repetitions(ty_path.item_paths_map(db).iter().map(
            |(ident, (ty_item_kind, result))| -> (
                Ident,
                EtherealSignatureResult<TypeItemEthTemplates>,
            ) {
                let result = match result {
                    Ok(paths) => match ty_item_kind {
                        TypeItemKind::MethodFn => paths
                            .iter()
                            .copied()
                            .map(|path| match path.eth_template(db) {
                                Ok(TypeItemEthTemplate::MethodFn(template)) => {
                                    Ok(template)
                                }
                                Err(e) => Err(e),
                                _ => unreachable!(),
                            })
                            .collect::<EtherealSignatureResult<SmallVecImpl<_>>>()
                            .map(TypeItemEthTemplates::MethodFn),
                        TypeItemKind::AssocFunctionFn => paths
                            .iter()
                            .copied()
                            .map(|path| match path.eth_template(db) {
                                Ok(TypeItemEthTemplate::AssocFn(template)) => {
                                    Ok(template)
                                }
                                Err(e) => Err(e),
                                _ => unreachable!(),
                            })
                            .collect::<EtherealSignatureResult<SmallVecImpl<_>>>()
                            .map(TypeItemEthTemplates::AssocFn),
                        TypeItemKind::AssocFunctionGn => todo!(),
                        TypeItemKind::AssocVal => todo!(),
                        TypeItemKind::AssocType => todo!(),
                        TypeItemKind::MemoizedField => paths
                            .iter()
                            .copied()
                            .map(|path| match path.eth_template(db) {
                                Ok(TypeItemEthTemplate::MemoizedField(template)) => {
                                    Ok(template)
                                }
                                Err(e) => Err(e),
                                _ => unreachable!(),
                            })
                            .collect::<EtherealSignatureResult<SmallVecImpl<_>>>()
                            .map(TypeItemEthTemplates::MemoizedField),
                        TypeItemKind::AssocFormal => todo!(),
                        TypeItemKind::AssocConst => todo!(),
                    },
                    Err(_e) => Err(EtherealSignatureError::EntityTreeError),
                };
                (*ident, result)
            },
        ))
        .expect("expect no repetitions"),
    )
}
