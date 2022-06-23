mod output;
mod parameter;

pub use output::*;
pub use parameter::*;

use atom::{
    context::{AtomContextKind, Symbol},
    AtomContext, AtomContextStandalone,
};
use defn_head::*;
use fold::LocalStack;
use implement::Implementor;
use map_collect::MapCollect;
use print_utils::{emsg_once, msg_once, p};
use static_defn::{EntityStaticDefnVariant, StaticParameter};
use word::IdentDict;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FunctionDecl {
    pub route: EntityRoutePtr,
    pub spatial_parameters: IdentDict<SpatialParameter>,
    pub primary_parameters: IdentDict<ParameterDecl>,
    pub keyword_parameters: IdentDict<ParameterDecl>,
    pub output: OutputDecl,
}

impl FunctionDecl {
    pub fn instantiate(&self, instantiator: &Instantiator) -> Arc<Self> {
        Arc::new(Self {
            route: instantiator
                .instantiate_entity_route(self.route)
                .take_entity_route(),
            spatial_parameters: self
                .spatial_parameters
                .iter()
                .filter_map(|placeholder| instantiator.instantiate_generic_placeholder(placeholder))
                .collect(),
            primary_parameters: self
                .primary_parameters
                .map(|parameter| parameter.instantiate(instantiator)),
            output: self.output.instantiate(instantiator),
            keyword_parameters: self
                .primary_parameters
                .map(|parameter| parameter.instantiate(instantiator)),
        })
    }

    pub(crate) fn from_ast(route: EntityRoutePtr, ast: &Ast) -> Arc<Self> {
        match ast.variant {
            AstVariant::CallFormDefnHead {
                ident,
                paradigm,
                spatial_parameters: ref generic_parameters,
                ref parameters,
                output_ty,
                output_liason,
                opt_this_liason,
            } => Arc::new(FunctionDecl {
                route,
                spatial_parameters: generic_parameters.clone(),
                primary_parameters: parameters
                    .iter()
                    .map(|parameter| parameter.into())
                    .collect(),
                output: OutputDecl {
                    ty: output_ty.route,
                    liason: output_liason,
                },
                keyword_parameters: Default::default(),
            }),
            _ => todo!(),
        }
    }

    pub fn nargs(&self) -> u8 {
        self.primary_parameters.len().try_into().unwrap()
    }
}

pub(crate) fn function_decl(
    db: &dyn DeclQueryGroup,
    route: EntityRoutePtr,
) -> InferQueryResultArc<FunctionDecl> {
    let locus = db.entity_locus(route)?;
    return match locus {
        EntityLocus::StaticModuleItem(static_defn) => Ok(match static_defn.variant {
            EntityStaticDefnVariant::Routine { .. } => {
                routine_decl_from_static(db, vec![], route, static_defn)
            }
            EntityStaticDefnVariant::Morphism { .. } => {
                morphism_decl_from_static(db, vec![], route, static_defn)
            }
            EntityStaticDefnVariant::Ty { .. } => match db.ty_decl(route)?.opt_type_call {
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
                AstVariant::CallFormDefnHead { .. } => Ok(FunctionDecl::from_ast(route, ast)),
                // type constructor
                AstVariant::TypeDefnHead { .. } => {
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
) -> Arc<FunctionDecl> {
    match static_defn.variant {
        EntityStaticDefnVariant::Routine {
            ref generic_parameters,
            ref parameters,
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
            let parameters = parameters.map(|parameter| ParameterDecl {
                ty: symbol_context.parse_entity_route(parameter.ty).unwrap(),
                liason: parameter.liason,
                ident: db.custom_ident(parameter.name),
            });
            let output_ty = symbol_context.parse_entity_route(output_ty).unwrap();
            msg_once!("todo: keyword parameters");
            Arc::new(FunctionDecl {
                route,
                spatial_parameters: generic_parameters,
                primary_parameters: parameters,
                output: OutputDecl {
                    liason: output_liason,
                    ty: output_ty,
                },
                keyword_parameters: Default::default(),
            })
        }
        _ => panic!(),
    }
}

pub(crate) fn morphism_decl_from_static(
    db: &dyn DeclQueryGroup,
    mut symbols: Vec<Symbol>,
    route: EntityRoutePtr,
    static_defn: &EntityStaticDefn,
) -> Arc<FunctionDecl> {
    match static_defn.variant {
        EntityStaticDefnVariant::Morphism {
            ref generic_parameters,
            ref parameters,
            output_ty,
            output_liason,
            ..
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
            let parameters = parameters.map(|parameter| ParameterDecl {
                ty: symbol_context.parse_entity_route(parameter.ty).unwrap(),
                liason: parameter.liason,
                ident: db.custom_ident(parameter.name),
            });
            let output_ty = symbol_context.parse_entity_route(output_ty).unwrap();
            msg_once!("todo: keyword parameters");
            Arc::new(FunctionDecl {
                route,
                spatial_parameters: generic_parameters,
                primary_parameters: parameters,
                output: OutputDecl {
                    liason: output_liason,
                    ty: output_ty,
                },
                keyword_parameters: Default::default(),
            })
        }
        _ => panic!(),
    }
}
