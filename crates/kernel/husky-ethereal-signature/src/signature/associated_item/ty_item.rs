mod associated_fn;
mod associated_val;
mod memoized_field;
mod method_fn;
mod method_function;

use husky_item_taxonomy::TypeItemKind;
use husky_item_tree::HasItemPathsMap;

pub use self::associated_fn::*;
pub use self::associated_val::*;
pub use self::memoized_field::*;
pub use self::method_fn::*;
pub use self::method_function::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum TypeItemEtherealSignatureTemplate {
    AssociatedFn(TypeAssociatedFnEtherealSignatureTemplate),
    MethodFn(TypeMethodFnEtherealSignatureTemplate),
    MethodFunction(TypeMethodFunctionEtherealSignatureTemplate),
    MemoizedField(TypeMemoizedFieldEtherealSignatureTemplate),
}

impl HasEtherealSignatureTemplate for TypeItemPath {
    type EtherealSignatureTemplate = TypeItemEtherealSignatureTemplate;

    fn ethereal_signature_template(
        self,
        db: &dyn EtherealSignatureDb,
    ) -> EtherealSignatureResult<Self::EtherealSignatureTemplate> {
        ty_item_ethereal_signature_template(db, self)
    }
}

#[salsa::tracked(jar = EtherealSignatureJar)]
pub(crate) fn ty_item_ethereal_signature_template(
    db: &dyn EtherealSignatureDb,
    path: TypeItemPath,
) -> EtherealSignatureResult<TypeItemEtherealSignatureTemplate> {
    Ok(match path.declarative_signature_template(db)? {
        TypeItemDeclarativeSignatureTemplate::AssociatedFn(template) => {
            TypeAssociatedFnEtherealSignatureTemplate::from_declarative(db, path, template)?.into()
        }
        TypeItemDeclarativeSignatureTemplate::MethodFn(template) => {
            TypeMethodFnEtherealSignatureTemplate::from_declarative(db, template)?.into()
        }
        TypeItemDeclarativeSignatureTemplate::AssociatedType(_) => todo!(),
        TypeItemDeclarativeSignatureTemplate::AssociatedVal(_) => todo!(),
        TypeItemDeclarativeSignatureTemplate::MemoizedField(template) => {
            TypeMemoizedFieldEtherealSignatureTemplate::from_declarative(db, template)?.into()
        }
    })
}

pub trait HasTypeItemTemplates: Copy {
    fn ty_item_templates_map<'a>(
        self,
        db: &'a dyn EtherealSignatureDb,
    ) -> EtherealSignatureResult<
        &'a [(
            Ident,
            EtherealSignatureResult<TypeItemEtherealSignatureTemplates>,
        )],
    >;

    fn ty_item_ethereal_signature_templates<'a>(
        self,
        db: &'a dyn EtherealSignatureDb,
        ident: Ident,
    ) -> EtherealSignatureMaybeResult<&'a TypeItemEtherealSignatureTemplates> {
        use vec_like::VecMapGetEntry;
        match self.ty_item_templates_map(db)?.get_entry(ident) {
            Some((_, Ok(templates))) => JustOk(templates),
            Some((_, Err(e))) => JustErr(*e),
            None => Nothing,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TypeItemEtherealSignatureTemplates {
    AssociatedFn(SmallVecImpl<TypeAssociatedFnEtherealSignatureTemplate>),
    MethodFn(SmallVecImpl<TypeMethodFnEtherealSignatureTemplate>),
    MethodFunction(SmallVecImpl<TypeMethodFunctionEtherealSignatureTemplate>),
    MemoizedField(SmallVecImpl<TypeMemoizedFieldEtherealSignatureTemplate>),
}

impl HasTypeItemTemplates for TypePath {
    fn ty_item_templates_map<'a>(
        self,
        db: &'a dyn EtherealSignatureDb,
    ) -> EtherealSignatureResult<
        &'a [(
            Ident,
            EtherealSignatureResult<TypeItemEtherealSignatureTemplates>,
        )],
    > {
        ty_item_ethereal_signature_templates_map(db, self)
            .as_ref()
            .map(|v| v as &[_])
            .map_err(|e| *e)
    }
}

#[salsa::tracked(jar = EtherealSignatureJar, return_ref)]
pub(crate) fn ty_item_ethereal_signature_templates_map(
    db: &dyn EtherealSignatureDb,
    ty_path: TypePath,
) -> EtherealSignatureResult<
    IdentPairMap<EtherealSignatureResult<TypeItemEtherealSignatureTemplates>>,
> {
    Ok(
        IdentPairMap::from_iter_assuming_no_repetitions(ty_path.item_paths_map(db)?.iter().map(
            |(ident, (ty_item_kind, result))| -> (
                Ident,
                EtherealSignatureResult<TypeItemEtherealSignatureTemplates>,
            ) {
                let result = match result {
                    Ok(paths) => match ty_item_kind {
                        TypeItemKind::MethodFn => paths
                            .iter()
                            .copied()
                            .map(|path| match path.ethereal_signature_template(db) {
                                Ok(TypeItemEtherealSignatureTemplate::MethodFn(template)) => {
                                    Ok(template)
                                }
                                Err(e) => Err(e),
                                _ => unreachable!(),
                            })
                            .collect::<EtherealSignatureResult<SmallVecImpl<_>>>()
                            .map(TypeItemEtherealSignatureTemplates::MethodFn),
                        TypeItemKind::AssociatedFn => paths
                            .iter()
                            .copied()
                            .map(|path| match path.ethereal_signature_template(db) {
                                Ok(TypeItemEtherealSignatureTemplate::AssociatedFn(template)) => {
                                    Ok(template)
                                }
                                Err(e) => Err(e),
                                _ => unreachable!(),
                            })
                            .collect::<EtherealSignatureResult<SmallVecImpl<_>>>()
                            .map(TypeItemEtherealSignatureTemplates::AssociatedFn),
                        TypeItemKind::AssociatedVal => todo!(),
                        TypeItemKind::AssociatedType => todo!(),
                        TypeItemKind::MemoizedField => paths
                            .iter()
                            .copied()
                            .map(|path| match path.ethereal_signature_template(db) {
                                Ok(TypeItemEtherealSignatureTemplate::MemoizedField(template)) => {
                                    Ok(template)
                                }
                                Err(e) => Err(e),
                                _ => unreachable!(),
                            })
                            .collect::<EtherealSignatureResult<SmallVecImpl<_>>>()
                            .map(TypeItemEtherealSignatureTemplates::MemoizedField),
                    },
                    // match templates {
                    //     TypeMethodDeclarativeSignatureTemplates::MethodFn(templates) => templates
                    //         .iter()
                    //         .copied()
                    //         .map(|template| template.ethereal_signature_template(db))
                    //         .collect::<EtherealSignatureResult<SmallVecImpl<_>>>()
                    //         .map(TypeItemEtherealSignatureTemplates::MethodFn),
                    //     TypeMethodDeclarativeSignatureTemplates::MethodFunction(templates) => {
                    //         todo!()
                    //     }
                    // },
                    Err(e) => Err(EtherealSignatureError::DerivedFromDeclarative),
                };
                (*ident, result)
            },
        ))
        .expect("expect no repetitions"),
    )
}
