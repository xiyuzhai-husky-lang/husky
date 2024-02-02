mod associated_fn;
mod associated_ty;
mod associated_val;
mod memoized_field;
mod method_fn;
mod method_function;

use husky_entity_kind::TypeItemKind;
use husky_entity_tree::HasItemPathsMap;

pub use self::associated_fn::*;
pub use self::associated_ty::*;

pub use self::memoized_field::*;
pub use self::method_fn::*;
pub use self::method_function::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum TypeItemEthTemplate {
    AssociatedFn(TypeAssociatedFnEthTemplate),
    MethodFn(TypeMethodFnEthTemplate),
    MethodFunction(TypeMethodFunctionEthTemplate),
    MemoizedField(TypeMemoizedFieldEthTemplate),
}

impl TypeItemEthTemplate {
    pub fn self_ty(self, db: &::salsa::Db) -> Option<EtherealTerm> {
        match self {
            TypeItemEthTemplate::AssociatedFn(_) => None,
            TypeItemEthTemplate::MethodFn(template) => Some(template.self_ty(db)),
            TypeItemEthTemplate::MethodFunction(_) => todo!(),
            TypeItemEthTemplate::MemoizedField(template) => Some(template.self_ty(db)),
        }
    }
}

impl HasEthTemplate for TypeItemPath {
    type EthTemplate = TypeItemEthTemplate;

    fn ethereal_signature_template(
        self,
        db: &::salsa::Db,
    ) -> EtherealSignatureResult<Self::EthTemplate> {
        ty_item_ethereal_signature_template(db, self)
    }
}

// #[salsa::tracked(jar = EtherealSignatureJar)]
pub(crate) fn ty_item_ethereal_signature_template(
    db: &::salsa::Db,
    path: TypeItemPath,
) -> EtherealSignatureResult<TypeItemEthTemplate> {
    Ok(match path.declarative_signature_template(db)? {
        TypeItemDecTemplate::AssociatedFn(template) => {
            TypeAssociatedFnEthTemplate::from_declarative(db, path, template)?.into()
        }
        TypeItemDecTemplate::MethodFn(template) => {
            TypeMethodFnEthTemplate::from_declarative(db, path, template)?.into()
        }
        TypeItemDecTemplate::AssociatedType(_) => todo!(),
        TypeItemDecTemplate::AssociatedVal(_) => todo!(),
        TypeItemDecTemplate::MemoizedField(template) => {
            TypeMemoizedFieldEthTemplate::from_declarative(db, path, template)?.into()
        }
    })
}

pub trait HasTypeItemTemplates: Copy {
    fn ty_item_templates_map<'a>(
        self,
        db: &'a ::salsa::Db,
    ) -> EtherealSignatureResult<&'a [(Ident, EtherealSignatureResult<TypeItemEthTemplates>)]>;

    fn ty_item_ethereal_signature_templates<'a>(
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
    AssociatedFn(SmallVecImpl<TypeAssociatedFnEthTemplate>),
    MethodFn(SmallVecImpl<TypeMethodFnEthTemplate>),
    MethodFunction(SmallVecImpl<TypeMethodFunctionEthTemplate>),
    MemoizedField(SmallVecImpl<TypeMemoizedFieldEthTemplate>),
}

impl HasTypeItemTemplates for TypePath {
    fn ty_item_templates_map<'a>(
        self,
        db: &'a ::salsa::Db,
    ) -> EtherealSignatureResult<&'a [(Ident, EtherealSignatureResult<TypeItemEthTemplates>)]> {
        ty_item_ethereal_signature_templates_map(db, self)
            .as_ref()
            .map(|v| v as &[_])
            .map_err(|e| *e)
    }
}

#[salsa::tracked(jar = EtherealSignatureJar, return_ref)]
pub(crate) fn ty_item_ethereal_signature_templates_map(
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
                            .map(|path| match path.ethereal_signature_template(db) {
                                Ok(TypeItemEthTemplate::MethodFn(template)) => {
                                    Ok(template)
                                }
                                Err(e) => Err(e),
                                _ => unreachable!(),
                            })
                            .collect::<EtherealSignatureResult<SmallVecImpl<_>>>()
                            .map(TypeItemEthTemplates::MethodFn),
                        TypeItemKind::AssociatedFunctionFn => paths
                            .iter()
                            .copied()
                            .map(|path| match path.ethereal_signature_template(db) {
                                Ok(TypeItemEthTemplate::AssociatedFn(template)) => {
                                    Ok(template)
                                }
                                Err(e) => Err(e),
                                _ => unreachable!(),
                            })
                            .collect::<EtherealSignatureResult<SmallVecImpl<_>>>()
                            .map(TypeItemEthTemplates::AssociatedFn),
                        TypeItemKind::AssociatedFunctionGn => todo!(),
                        TypeItemKind::AssociatedVal => todo!(),
                        TypeItemKind::AssociatedType => todo!(),
                        TypeItemKind::MemoizedField => paths
                            .iter()
                            .copied()
                            .map(|path| match path.ethereal_signature_template(db) {
                                Ok(TypeItemEthTemplate::MemoizedField(template)) => {
                                    Ok(template)
                                }
                                Err(e) => Err(e),
                                _ => unreachable!(),
                            })
                            .collect::<EtherealSignatureResult<SmallVecImpl<_>>>()
                            .map(TypeItemEthTemplates::MemoizedField),
                        TypeItemKind::AssociatedFormal => todo!(),
                        TypeItemKind::AssociatedConst => todo!(),
                    },
                    Err(_e) => Err(EtherealSignatureError::EntityTreeError),
                };
                (*ident, result)
            },
        ))
        .expect("expect no repetitions"),
    )
}
