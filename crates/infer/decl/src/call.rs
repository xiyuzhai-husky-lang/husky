mod input;
mod output;

pub use input::*;
pub use output::*;

use atom::{
    context::{AtomContextKind, Symbol},
    AtomContext, AtomContextStandalone,
};
use defn_head::*;
use fold::LocalStack;
use implement::Implementor;
use map_collect::MapCollect;
use print_utils::{emsg_once, p};
use static_defn::{EntityStaticDefnVariant, StaticInputParameter};
use vm::{InputLiason, OutputLiason};
use word::IdentDict;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CallDecl {
    pub route: EntityRoutePtr,
    pub generic_parameters: IdentDict<GenericParameter>,
    pub parameters: Vec<InputDecl>,
    pub output: OutputDecl,
}

impl CallDecl {
    pub fn instantiate(&self, instantiator: &Instantiator) -> Arc<Self> {
        Arc::new(Self {
            route: instantiator
                .instantiate_entity_route(self.route)
                .take_entity_route(),
            generic_parameters: self
                .generic_parameters
                .iter()
                .filter_map(|placeholder| instantiator.instantiate_generic_placeholder(placeholder))
                .collect(),
            parameters: self.parameters.map(|input| input.instantiate(instantiator)),
            output: self.output.instantiate(instantiator),
        })
    }

    pub(crate) fn from_ast(route: EntityRoutePtr, head: &CallableDefnHead) -> Arc<Self> {
        Arc::new(CallDecl {
            route,
            generic_parameters: head.generic_parameters.clone(),
            parameters: head
                .parameters
                .iter()
                .map(|input_placeholder| input_placeholder.into())
                .collect(),
            output: OutputDecl {
                ty: head.output_ty.route,
                liason: head.output_liason,
            },
        })
    }

    pub fn nargs(&self) -> u8 {
        self.parameters.len().try_into().unwrap()
    }
}

pub(crate) fn call_decl(
    db: &dyn DeclQueryGroup,
    route: EntityRoutePtr,
) -> InferQueryResultArc<CallDecl> {
    let locus = db.entity_locus(route)?;
    return match locus {
        EntityLocus::StaticModuleItem(static_defn) => Ok(match static_defn.variant {
            EntityStaticDefnVariant::Routine { .. } => {
                routine_decl_from_static(db, vec![], route, static_defn)
            }
            EntityStaticDefnVariant::Type { .. } => match db.ty_decl(route)?.opt_type_call {
                Some(ref ty_call) => ty_call.clone(),
                None => return Err(query_error!(format!("no type call for {:?}", route))),
            },
            _ => panic!(),
        }),
        EntityLocus::WithinBuiltinModule => todo!(),
        EntityLocus::WithinModule {
            file,
            token_group_index,
        } => {
            let ast_text = db.ast_text(file)?;
            let item = ast_text
                .folded_results
                .iter_from(token_group_index)
                .next()
                .unwrap();
            let ast = item.value.as_ref()?;
            match ast.variant {
                AstKind::CallFormDefnHead(ref head) => Ok(CallDecl::from_ast(route, head)),
                AstKind::CallFormDefnHead(ref head) => Ok(CallDecl::from_ast(route, head)),
                // type constructor
                AstKind::TypeDefnHead { .. } => {
                    let ty_decl = db.ty_decl(route)?;
                    Ok(ty_decl.opt_type_call.clone().expect("todo"))
                }
                _ => panic!(),
            }
        }
        EntityLocus::Module { file: file_id } => todo!(),
        EntityLocus::Input { .. } => todo!(),
        EntityLocus::StaticTypeMember => todo!(),
        EntityLocus::StaticTypeAsTraitMember => todo!(),
    };
}

pub(crate) fn routine_decl_from_static(
    db: &dyn DeclQueryGroup,
    mut symbols: Vec<Symbol>,
    route: EntityRoutePtr,
    static_defn: &EntityStaticDefn,
) -> Arc<CallDecl> {
    match static_defn.variant {
        EntityStaticDefnVariant::Routine {
            ref generic_parameters,
            parameters: ref inputs,
            output_ty,
            output_liason,
            linkage,
            routine_kind: paradigm,
        } => {
            let generic_parameters = db.generic_parameters_from_static(generic_parameters);
            symbols.extend(db.symbols_from_generic_parameters(&generic_parameters));
            let mut symbol_context = AtomContextStandalone {
                opt_package_main: None,
                db: db.upcast(),
                opt_this_ty: None,
                opt_this_contract: None,
                symbols: (&symbols as &[Symbol]).into(),
                kind: AtomContextKind::Normal,
            };
            let inputs = inputs.map(|input| InputDecl {
                ty: symbol_context.entity_route_from_str(input.ty).unwrap(),
                liason: input.contract,
                ident: db.custom_ident(input.name),
            });
            let output_ty = symbol_context.entity_route_from_str(output_ty).unwrap();
            Arc::new(CallDecl {
                route,
                generic_parameters,
                parameters: inputs,
                output: OutputDecl {
                    liason: output_liason,
                    ty: output_ty,
                },
            })
        }
        _ => panic!(),
    }
}
