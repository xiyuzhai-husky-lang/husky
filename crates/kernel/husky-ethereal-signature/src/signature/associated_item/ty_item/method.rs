// use super::*;

// pub trait HasTypeMethodEtherealSignatureTemplates: Copy {
//     fn ty_method_ethereal_signature_templates_map<'a>(
//         self,
//         db: &'a dyn EtherealSignatureDb,
//     ) -> EtherealSignatureResult<
//         &'a [(
//             Ident,
//             EtherealSignatureResult<TypeMethodEtherealSignatureTemplates>,
//         )],
//     >;

//     fn ty_method_ethereal_signature_templates<'a>(
//         self,
//         db: &'a dyn EtherealSignatureDb,
//         ident: Ident,
//     ) -> EtherealSignatureMaybeResult<&'a TypeMethodEtherealSignatureTemplates> {
//         use vec_like::VecMapGetEntry;
//         match self
//             .ty_method_ethereal_signature_templates_map(db)?
//             .get_entry(ident)
//         {
//             Some((_, Ok(templates))) => JustOk(templates),
//             Some((_, Err(e))) => JustErr(*e),
//             None => Nothing,
//         }
//     }
// }

// impl HasTypeMethodEtherealSignatureTemplates for TypePath {
//     fn ty_method_ethereal_signature_templates_map<'a>(
//         self,
//         db: &'a dyn EtherealSignatureDb,
//     ) -> EtherealSignatureResult<
//         &'a [(
//             Ident,
//             EtherealSignatureResult<TypeMethodEtherealSignatureTemplates>,
//         )],
//     > {
//         ty_method_ethereal_signature_templates_map(db, self)
//             .as_ref()
//             .map(|v| v as &[_])
//             .map_err(|e| *e)
//     }
// }

// #[salsa::tracked(jar = EtherealSignatureJar, return_ref)]
// pub(crate) fn ty_method_ethereal_signature_templates_map(
//     db: &dyn EtherealSignatureDb,
//     ty_path: TypePath,
// ) -> EtherealSignatureResult<
//     IdentPairMap<EtherealSignatureResult<TypeMethodEtherealSignatureTemplates>>,
// > {
//     Ok(IdentPairMap::from_iter_assuming_no_repetitions(
//         ty_path
//             .ty_method_declarative_signature_templates_map(db)?
//             .iter()
//             .map(
//                 |(ident, result)| -> (
//                     Ident,
//                     EtherealSignatureResult<TypeMethodEtherealSignatureTemplates>,
//                 ) {
//                     let result = match result {
//                         Ok(templates) => match templates {
//                             TypeMethodDeclarativeSignatureTemplates::MethodFn(templates) => {
//                                 templates
//                                     .iter()
//                                     .copied()
//                                     .map(|template| template.ethereal_signature_template(db))
//                                     .collect::<EtherealSignatureResult<SmallVecImpl<_>>>()
//                                     .map(TypeMethodEtherealSignatureTemplates::MethodFn)
//                             }
//                             TypeMethodDeclarativeSignatureTemplates::MethodFunction(templates) => {
//                                 todo!()
//                             }
//                         },
//                         Err(e) => Err(EtherealSignatureError::DerivedFromDeclarative),
//                     };
//                     (*ident, result)
//                 },
//             ),
//     )
//     .expect("expect no repetitions"))
// }
