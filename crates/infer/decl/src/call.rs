use atom::symbol_proxy::Symbol;
use defn_head::*;
use fold::LocalStack;
use implement::Implementor;
use map_collect::MapCollect;
use print_utils::msg_once;
use static_decl::{StaticEntityDecl, StaticFuncDecl, StaticInputDecl};
use vm::InputContract;
use word::IdentDict;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CallDecl {
    pub generic_placeholders: IdentDict<GenericPlaceholder>,
    pub inputs: Vec<InputDecl>,
    pub output: EntityRoutePtr,
}

impl From<&RoutineDefnHead> for CallDecl {
    fn from(head: &RoutineDefnHead) -> Self {
        CallDecl {
            generic_placeholders: head.generic_placeholders.clone(),
            inputs: head
                .input_placeholders
                .iter()
                .map(|input_placeholder| input_placeholder.into())
                .collect(),
            output: head.output.route,
        }
    }
}

// impl From<&MethodDefnHead> for MethodDecl {
//     fn from(head: &MethodDefnHead) -> Self {
//         Self {
//             ident: head.ident,
//             this_contract: head.this_contract,
//             inputs: head
//                 .input_placeholders
//                 .iter()
//                 .map(|input_placeholder| input_placeholder.into())
//                 .collect(),
//             output: head.output.route,
//             generic_placeholders: head.generic_placeholders.clone(),
//         }
//     }
// }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputDecl {
    pub contract: InputContract,
    pub ty: EntityRoutePtr,
    pub ident: CustomIdentifier,
}

impl InputDecl {
    pub fn from_static(
        db: &dyn DeclQueryGroup,
        input: &StaticInputDecl,
        opt_this_ty: Option<EntityRoutePtr>,
        symbols: &LocalStack<Symbol>,
    ) -> Self {
        Self {
            ty: db.parse_entity(input.ty, opt_this_ty, symbols).unwrap(),
            contract: input.contract,
            ident: db.custom_ident(input.name),
        }
    }

    pub fn instantiate(&self, instantiator: &Instantiator) -> Self {
        Self {
            ty: instantiator.instantiate_entity_route(self.ty).as_scope(),
            contract: self.contract,
            ident: self.ident,
        }
    }

    pub fn implement(&self, implementor: &Implementor) -> Self {
        todo!()
    }
}

impl Into<InputDecl> for &InputPlaceholder {
    fn into(self) -> InputDecl {
        InputDecl {
            contract: self.contract,
            ty: self.ranged_ty.route,
            ident: self.ident,
        }
    }
}

impl CallDecl {
    fn new_vec(ty: EntityRoutePtr) -> Self {
        msg_once!("new vec compiled");
        Self {
            inputs: Vec::new(),
            output: ty,
            generic_placeholders: Default::default(),
        }
    }
}

pub(crate) fn call_decl(
    db: &dyn DeclQueryGroup,
    scope: EntityRoutePtr,
) -> InferResultArc<CallDecl> {
    let source = db.entity_source(scope)?;
    return match source {
        EntitySource::Static(data) => Ok(Arc::new(match data.decl {
            StaticEntityDecl::Func(ref signature) => func_call_decl_from_static(db, signature),
            StaticEntityDecl::TyTemplate => CallDecl::new_vec(scope),
            _ => panic!(),
        })),
        EntitySource::WithinBuiltinModule => todo!(),
        EntitySource::WithinModule {
            file,
            token_group_index,
        } => {
            let ast_text = db.ast_text(file)?;
            let item = ast_text
                .folded_results
                .fold_iter(token_group_index)
                .next()
                .unwrap();
            let ast = item.value.as_ref()?;
            match ast.kind {
                AstKind::RoutineDefnHead(ref head) => Ok(Arc::new(head.into())),
                // type constructor
                AstKind::TypeDefnHead {
                    ref kind,
                    ref generic_placeholders,
                    ..
                } => match kind {
                    TyKind::Enum => todo!(),
                    TyKind::Struct => {
                        let mut inputs = vec![];
                        for subitem in item.children.unwrap() {
                            let subast = subitem.value.as_ref()?;
                            match subast.kind {
                                AstKind::FieldDefn(ref field_defn) => inputs.push(InputDecl {
                                    contract: field_defn.contract.constructor_input(),
                                    ty: field_defn.ty,
                                    ident: field_defn.ident,
                                }),
                                _ => (),
                            }
                        }
                        msg_once!("struct type call compiled");
                        Ok(Arc::new(CallDecl {
                            inputs,
                            output: scope,
                            generic_placeholders: generic_placeholders.clone(),
                        }))
                    }
                    TyKind::Record => todo!(),
                    TyKind::Primitive => todo!(),
                    TyKind::Vec => todo!(),
                    TyKind::Array => todo!(),
                    TyKind::Other => todo!(),
                },
                _ => panic!(),
            }
        }
        EntitySource::Module { file: file_id } => todo!(),
        EntitySource::Input { .. } => todo!(),
    };

    fn func_call_decl_from_static(
        db: &dyn DeclQueryGroup,
        static_decl: &StaticFuncDecl,
    ) -> CallDecl {
        let generic_placeholders =
            db.parse_generic_placeholders_from_static(static_decl.generic_placeholders);
        let symbols = db.symbols_from_generic_placeholders(&generic_placeholders);
        let inputs = static_decl.inputs.map(|input| InputDecl {
            ty: db.parse_entity(input.ty, None, &symbols).unwrap(),
            contract: input.contract,
            ident: db.custom_ident(input.name),
        });
        let output = db.parse_entity(static_decl.output, None, &symbols).unwrap();
        CallDecl {
            generic_placeholders,
            inputs,
            output,
        }
    }
}
