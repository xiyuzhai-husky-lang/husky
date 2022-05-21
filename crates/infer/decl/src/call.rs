mod input;
mod output;

pub use input::*;
pub use output::*;

use atom::{
    symbol::{Symbol, SymbolContextKind},
    SymbolContext,
};
use defn_head::*;
use fold::LocalStack;
use implement::Implementor;
use map_collect::MapCollect;
use print_utils::{emsg_once, p};
use static_defn::{EntityStaticDefnVariant, StaticInputParameter};
use vm::{InputContract, OutputLiason};
use word::IdentDict;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CallDecl {
    pub route: EntityRoutePtr,
    pub generic_placeholders: IdentDict<GenericPlaceholder>,
    pub parameters: Vec<InputDecl>,
    pub output: OutputDecl,
}

impl CallDecl {
    pub fn instantiate(&self, instantiator: &Instantiator) -> Arc<Self> {
        Arc::new(Self {
            route: instantiator
                .instantiate_entity_route(self.route)
                .as_entity_route(),
            generic_placeholders: self
                .generic_placeholders
                .iter()
                .filter_map(|placeholder| instantiator.instantiate_generic_placeholder(placeholder))
                .collect(),
            parameters: self.parameters.map(|input| input.instantiate(instantiator)),
            output: self.output.instantiate(instantiator),
        })
    }

    pub(crate) fn from_ast(route: EntityRoutePtr, head: &RoutineDefnHead) -> Arc<Self> {
        Arc::new(CallDecl {
            route,
            generic_placeholders: head.generic_placeholders.clone(),
            parameters: head
                .parameters
                .iter()
                .map(|input_placeholder| input_placeholder.into())
                .collect(),
            output: OutputDecl {
                ty: head.output_ty.route,
                liason: head.output_contract,
            },
        })
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
                AstKind::RoutineDefnHead(ref head) => Ok(CallDecl::from_ast(route, head)),
                AstKind::TypeAssociatedRoutineDefnHead(ref head) => {
                    Ok(CallDecl::from_ast(route, head))
                }
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
            ref generic_placeholders,
            input_placeholders: ref inputs,
            output_ty,
            output_contract,
            linkage,
            routine_kind,
        } => {
            let generic_placeholders = db.generic_placeholders_from_static(generic_placeholders);
            symbols.extend(db.symbols_from_generic_placeholders(&generic_placeholders));
            let symbol_context = SymbolContext {
                opt_package_main: None,
                db: db.upcast(),
                opt_this_ty: None,
                opt_this_contract: None,
                symbols: (&symbols as &[Symbol]).into(),
                kind: SymbolContextKind::Normal,
            };
            let inputs = inputs.map(|input| InputDecl {
                ty: symbol_context.entity_route_from_str(input.ty).unwrap(),
                contract: input.contract,
                ident: db.custom_ident(input.name),
            });
            let output_ty = symbol_context.entity_route_from_str(output_ty).unwrap();
            Arc::new(CallDecl {
                route,
                generic_placeholders,
                parameters: inputs,
                output: OutputDecl {
                    liason: output_contract,
                    ty: output_ty,
                },
            })
        }
        _ => panic!(),
    }
}
