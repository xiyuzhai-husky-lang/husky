use atom::symbol_proxy::Symbol;
use defn_head::*;
use fold::LocalStack;
use implement::Implementor;
use map_collect::MapCollect;
use print_utils::{msg_once, p};
use static_decl::{StaticCallDecl, StaticEntityDecl, StaticInputDecl};
use vm::InputContract;
use word::IdentDict;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CallDecl {
    pub generic_placeholders: IdentDict<GenericPlaceholder>,
    pub inputs: Vec<InputDecl>,
    pub output: EntityRoutePtr,
}

impl CallDecl {
    pub fn instantiate(&self, instantiator: &Instantiator) -> Arc<Self> {
        Arc::new(Self {
            generic_placeholders: self
                .generic_placeholders
                .iter()
                .filter_map(|placeholder| instantiator.instantiate_generic_placeholder(placeholder))
                .collect(),
            inputs: self.inputs.map(|input| input.instantiate(instantiator)),
            output: instantiator
                .instantiate_entity_route(self.output)
                .as_scope(),
        })
    }
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
        symbols: &[Symbol],
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

pub(crate) fn call_decl(
    db: &dyn DeclQueryGroup,
    route: EntityRoutePtr,
) -> InferResultArc<CallDecl> {
    let source = db.entity_source(route)?;
    return match source {
        EntitySource::StaticModuleItem(data) => Ok(match data.decl {
            StaticEntityDecl::Func(ref signature) => call_decl_from_static(db, signature),
            StaticEntityDecl::Type(_) => db.type_decl(route)?.opt_type_call.clone().expect("todo"),
            _ => panic!(),
        }),
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
                    TypeKind::Enum => todo!(),
                    TypeKind::Struct => {
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
                            output: route,
                            generic_placeholders: generic_placeholders.clone(),
                        }))
                    }
                    TypeKind::Record => todo!(),
                    TypeKind::Primitive => todo!(),
                    TypeKind::Vec => todo!(),
                    TypeKind::Array => todo!(),
                    TypeKind::Other => todo!(),
                },
                _ => panic!(),
            }
        }
        EntitySource::Module { file: file_id } => todo!(),
        EntitySource::Input { .. } => todo!(),
        EntitySource::StaticTypeMember => todo!(),
    };
}

pub(crate) fn call_decl_from_static(
    db: &dyn DeclQueryGroup,
    static_decl: &StaticCallDecl,
) -> Arc<CallDecl> {
    let generic_placeholders =
        db.parse_generic_placeholders_from_static(static_decl.generic_placeholders);
    let symbols = db.symbols_from_generic_placeholders(&generic_placeholders);
    let inputs = static_decl.inputs.map(|input| InputDecl {
        ty: db.parse_entity(input.ty, None, &symbols).unwrap(),
        contract: input.contract,
        ident: db.custom_ident(input.name),
    });
    let output = db.parse_entity(static_decl.output, None, &symbols).unwrap();
    Arc::new(CallDecl {
        generic_placeholders,
        inputs,
        output,
    })
}
