mod associated_fn;
mod associated_val;
mod memoized_field;
mod method;
mod method_fn;
mod method_function;

use husky_entity_tree::HasItemPaths;

pub use self::associated_fn::*;
pub use self::associated_val::*;
pub use self::memoized_field::*;
pub use self::method::*;
pub use self::method_fn::*;
pub use self::method_function::*;

use super::*;

#[derive(Debug, PartialEq, Eq)]
pub enum TypeItemEtherealSignatureTemplates {
    AssociatedFn(SmallVecImpl<TypeAssociatedFnEtherealSignatureTemplate>),
    MethodFn(SmallVecImpl<TypeMethodFnEtherealSignatureTemplate>),
    MethodFunction(SmallVecImpl<TypeMethodFunctionEtherealSignatureTemplate>),
}

pub trait HasTypeItemSignatureTemplates: Copy {
    fn ty_item_signature_templates_map<'a>(
        self,
        db: &'a dyn EtherealSignatureDb,
    ) -> EtherealSignatureResult<
        &'a [(
            Ident,
            EtherealSignatureResult<TypeItemEtherealSignatureTemplates>,
        )],
    >;

    fn ty_item_signature_templates<'a>(
        self,
        db: &'a dyn EtherealSignatureDb,
        ident: Ident,
        all_available_traits: &[()],
    ) -> EtherealSignatureMaybeResult<&'a TypeItemEtherealSignatureTemplates> {
        use vec_like::VecMapGetEntry;
        match self.ty_item_signature_templates_map(db)?.get_entry(ident) {
            Some((_, Ok(templates))) => JustOk(templates),
            Some((_, Err(e))) => JustErr(*e),
            None => Nothing,
        }
    }
}

impl HasTypeItemSignatureTemplates for TypePath {
    fn ty_item_signature_templates_map<'a>(
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
    let _ = ty_path.item_paths(db)?;
    todo!()
    // Ok(IdentPairMap::from_iter_assuming_no_repetitions(
    //     ty_path
    //         .ty_method_declarative_signature_templates_map(db)?
    //         .iter()
    //         .map(
    //             |(ident, result)| -> (
    //                 Ident,
    //                 EtherealSignatureResult<TypeMethodEtherealSignatureTemplates>,
    //             ) {
    //                 let result = match result {
    //                     Ok(templates) => match templates {
    //                         TypeMethodDeclarativeSignatureTemplates::MethodFn(templates) => {
    //                             templates
    //                                 .iter()
    //                                 .copied()
    //                                 .map(|template| template.ethereal_signature_template(db))
    //                                 .collect::<EtherealSignatureResult<SmallVecImpl<_>>>()
    //                                 .map(TypeMethodEtherealSignatureTemplates::MethodFn)
    //                         }
    //                         TypeMethodDeclarativeSignatureTemplates::MethodFunction(templates) => {
    //                             todo!()
    //                         }
    //                     },
    //                     Err(e) => Err(EtherealSignatureError::DerivedFromDeclarative),
    //                 };
    //                 (*ident, result)
    //             },
    //         ),
    // )
    // .expect("expect no repetitions"))
}
