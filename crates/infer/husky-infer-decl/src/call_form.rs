mod output;
mod parameter;
mod variadic;

pub use output::*;
pub use parameter::*;
pub use variadic::*;

use defn_head::*;
use fold::LocalStack;
use husky_atom::{
    context::{AtomContextKind, Symbol},
    AtomContext, AtomContextStandalone,
};
use husky_implement::{Implementable, ImplementationContext};
use husky_instantiate::InstantiationContext;
use husky_print_utils::{msg_once, p};
use husky_word::IdentDict;
use map_collect::MapCollect;
use static_defn::{EntityStaticDefnVariant, StaticParameter};
use vec_like::VecMapEntry;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct CallFormDecl {
    pub opt_base_route: Option<EntityRoutePtr>,
    pub opt_this_liason: Option<ParameterLiason>,
    pub spatial_parameters: IdentDict<SpatialParameter>,
    pub primary_parameters: IdentDict<ParameterDecl>,
    pub variadic_template: VariadicTemplate,
    pub keyword_parameters: IdentDict<ParameterDecl>,
    pub output: OutputDecl,
    pub is_lazy: bool,
}

impl CallFormDecl {
    pub(crate) fn from_ast(
        db: &dyn DeclQueryGroup,
        route: EntityRoutePtr,
        ast: &Ast,
    ) -> InferResultArc<Self> {
        msg_once!("variadics");
        Ok(match ast.variant {
            AstVariant::CallFormDefnHead {
                ident,
                paradigm,
                ref spatial_parameters,
                ref parameters,
                output_ty,
                output_liason,
                opt_this_liason,
            } => Arc::new(CallFormDecl {
                opt_base_route: Some(route),
                opt_this_liason,
                spatial_parameters: spatial_parameters.clone(),
                primary_parameters: parameters
                    .iter()
                    .map(|parameter| ParameterDecl::from_parameter(db, parameter))
                    .collect::<InferResult<_>>()?,
                output: OutputDecl::new(db, output_liason, output_ty.route)?,
                keyword_parameters: Default::default(),
                variadic_template: VariadicTemplate::None,
                is_lazy: paradigm.is_lazy(),
            }),
            _ => todo!(),
        })
    }

    pub fn from_static(
        db: &dyn DeclQueryGroup,
        base_route: EntityRoutePtr,
        symbol_context: &mut dyn AtomContext,
        defn: &EntityStaticDefn,
    ) -> InferResultArc<Self> {
        Ok(match defn.variant {
            EntityStaticDefnVariant::Method {
                this_liason,
                parameters,
                output_ty,
                output_liason,
                spatial_parameters,
                method_static_defn_kind: method_kind,
                ..
            } => {
                let output_ty = symbol_context.parse_entity_route(output_ty).unwrap();
                Arc::new(Self {
                    opt_base_route: Some(base_route),
                    opt_this_liason: Some(this_liason),
                    primary_parameters: parameters
                        .map(|input| ParameterDecl::from_static(symbol_context, input)),
                    output: OutputDecl::new(db, output_liason, output_ty)?,
                    spatial_parameters: spatial_parameters.map(|static_spatial_parameter| {
                        SpatialParameter::from_static(
                            symbol_context.entity_syntax_db(),
                            static_spatial_parameter,
                        )
                    }),
                    is_lazy: false,
                    variadic_template: VariadicTemplate::None,
                    keyword_parameters: Default::default(),
                })
            }
            _ => panic!(""),
        })
    }

    pub fn ident(&self) -> CustomIdentifier {
        self.opt_base_route.unwrap().ident().custom()
    }

    pub fn nargs(&self) -> u8 {
        let nargs0: u8 = self.primary_parameters.len().try_into().unwrap();
        nargs0 + self.opt_this_liason.map(|_| 1u8).unwrap_or(0u8)
    }

    pub fn this_liason(&self) -> ParameterLiason {
        self.opt_this_liason.unwrap()
    }

    pub fn variadic_start(&self) -> usize {
        self.primary_parameters.len() + self.keyword_parameters.len()
    }

    pub fn opt_this_ty(&self) -> Option<EntityRoutePtr> {
        self.opt_this_liason
            .map(|_| self.opt_base_route.unwrap().parent())
    }
}

impl Instantiable for CallFormDecl {
    type Target = Arc<Self>;

    fn instantiate(&self, ctx: &InstantiationContext) -> Self::Target {
        Arc::new(Self {
            opt_base_route: self
                .opt_base_route
                .map(|route| route.instantiate(ctx).take_entity_route()),
            opt_this_liason: self.opt_this_liason,
            spatial_parameters: self
                .spatial_parameters
                .iter()
                .filter_map(|parameter| parameter.instantiate(ctx))
                .collect(),
            primary_parameters: self
                .primary_parameters
                .map(|parameter| parameter.instantiate(ctx)),
            output: self.output.instantiate(ctx),
            keyword_parameters: self
                .keyword_parameters
                .map(|parameter| parameter.instantiate(ctx)),
            variadic_template: self.variadic_template.instantiate(ctx),
            is_lazy: self.is_lazy,
        })
    }
}

impl Implementable for CallFormDecl {
    type Target = Arc<Self>;

    fn implement(&self, ctx: &ImplementationContext) -> Self::Target {
        Arc::new(Self {
            opt_base_route: self
                .opt_base_route
                .map(|route| route.implement(ctx).take_entity_route()),
            opt_this_liason: self.opt_this_liason,
            primary_parameters: self
                .primary_parameters
                .map(|parameter| parameter.implement(ctx)),
            keyword_parameters: self
                .keyword_parameters
                .map(|parameter| parameter.implement(ctx)),
            output: self.output.implement(ctx),
            spatial_parameters: self.spatial_parameters.clone(),
            is_lazy: self.is_lazy,
            variadic_template: self.variadic_template.implement(ctx),
        })
    }
}

pub(crate) fn entity_call_form_decl(
    db: &dyn DeclQueryGroup,
    route: EntityRoutePtr,
) -> InferQueryResultArc<CallFormDecl> {
    let source = db.entity_source(route)?;
    return match source {
        EntitySource::StaticModuleItem(static_defn) => Ok(match static_defn.variant {
            EntityStaticDefnVariant::Function { .. } => {
                routine_decl_from_static(db, vec![], route, static_defn)?
            }
            EntityStaticDefnVariant::Ty { .. } => match db.ty_decl(route)?.opt_type_call {
                Some(ref ty_call) => ty_call.clone(),
                None => return Err(query_error!(format!("no type call for {:?}", route))),
            },
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
                .iter_from(token_group_index)
                .next()
                .unwrap();
            let ast = item.value.as_ref()?;
            match ast.variant {
                AstVariant::CallFormDefnHead { .. } => Ok(CallFormDecl::from_ast(db, route, ast)?),
                // type constructor
                AstVariant::TypeDefnHead { .. } => {
                    let ty_decl = db.ty_decl(route)?;
                    Ok(ty_decl.opt_type_call.clone().expect("todo"))
                }
                _ => Err(query_error!(format!("this entity can not be called"))),
            }
        }
        EntitySource::Module { file: file_id } => todo!(),
        EntitySource::TargetInput { .. } => todo!(),
        EntitySource::StaticTypeMember(_) => match route.variant {
            EntityRouteVariant::Root { ident } => todo!(),
            EntityRouteVariant::Package { main, ident } => todo!(),
            EntityRouteVariant::Child { parent, ident } => {
                let ty_decl = derived_unwrap!(db.ty_decl(parent));
                match derived_not_none!(ty_decl
                    .ty_members
                    .iter()
                    .find(|member| member.key() == ident))?
                {
                    TyMemberDecl::Field(_) => todo!(),
                    TyMemberDecl::Method(method) => Ok(method.clone()),
                    TyMemberDecl::Call(_) => todo!(),
                }
            }
            EntityRouteVariant::TypeAsTraitMember { ty, trai, ident } => todo!(),
            EntityRouteVariant::TargetInputValue => todo!(),
            EntityRouteVariant::Any { .. } => todo!(),
            EntityRouteVariant::ThisType => todo!(),
            EntityRouteVariant::TargetOutputType => todo!(),
        },
        EntitySource::StaticTraitMember(_) => todo!(),
        EntitySource::StaticTypeAsTraitMember => match route.variant {
            EntityRouteVariant::TypeAsTraitMember { ty, trai, ident } => {
                let ty_decl = derived_unwrap!(db.ty_decl(ty));
                match derived_not_none!(ty_decl.trai_member_impl(trai, ident))? {
                    TraitMemberImplDecl::Method(method) => Ok(method.clone()),
                    TraitMemberImplDecl::AssociatedType { ident, ty } => todo!(),
                    TraitMemberImplDecl::Call {} => todo!(),
                    TraitMemberImplDecl::AssociatedConstSize {} => todo!(),
                }
            }
            _ => todo!(),
        },
        EntitySource::Any { .. } => todo!(),
        EntitySource::StaticEnumVariant(_) => todo!(),
    };
}

pub(crate) fn value_call_form_decl(
    db: &dyn DeclQueryGroup,
    ty: EntityRoutePtr,
) -> InferQueryResultArc<CallFormDecl> {
    match ty.variant {
        EntityRouteVariant::Root {
            ident: RootIdentifier::FatFp,
        } => {
            msg_once!("much more todo");
            let nargs = ty.spatial_arguments.len() - 1;
            return Ok(Arc::new(CallFormDecl {
                opt_base_route: None,
                opt_this_liason: None,
                spatial_parameters: Default::default(),
                primary_parameters: ty.spatial_arguments[0..nargs]
                    .iter()
                    .enumerate()
                    .map(|(i, spatial_argument)| ParameterDecl {
                        liason: ParameterLiason::Pure,
                        ty: spatial_argument.take_entity_route(),
                        ident: db.intern_word(&format!("arg{}", i)).custom(),
                    })
                    .collect(),
                variadic_template: Default::default(),
                keyword_parameters: Default::default(),
                output: OutputDecl::new(
                    db,
                    OutputLiason::Transfer,
                    ty.spatial_arguments.last().unwrap().take_entity_route(),
                )?,
                is_lazy: false,
            }));
        }
        EntityRouteVariant::Root {
            ident: RootIdentifier::Fn,
        } => todo!(),
        EntityRouteVariant::Root {
            ident: RootIdentifier::FnMut,
        } => todo!(),
        EntityRouteVariant::Root {
            ident: RootIdentifier::FnOnce,
        } => todo!(),
        _ => Err(query_error!(format!(
            "a value of type `{ty:?}` can not be called"
        ))),
    }
}

pub(crate) fn routine_decl_from_static(
    db: &dyn DeclQueryGroup,
    mut symbols: Vec<Symbol>,
    route: EntityRoutePtr,
    static_defn: &EntityStaticDefn,
) -> InferResultArc<CallFormDecl> {
    match static_defn.variant {
        EntityStaticDefnVariant::Function {
            ref spatial_parameters,
            ref parameters,
            output_ty,
            output_liason,
            ref linkage,
            ref variadic_template,
        } => {
            let spatial_parameters = db.spatial_parameters_from_static(spatial_parameters);
            symbols.extend(db.symbols_from_spatial_parameters(&spatial_parameters));
            let mut symbol_context = AtomContextStandalone {
                db: db.upcast(),
                opt_this_ty: None,
                opt_this_contract: None,
                symbols: (&symbols as &[Symbol]).into(),
                kind: AtomContextKind::Normal,
                opt_file: Some(db.intern_file(static_defn.dev_src.file.into())),
            };
            let parameters = parameters.map(|parameter| ParameterDecl {
                ty: symbol_context.parse_entity_route(parameter.ty).unwrap(),
                liason: parameter.liason,
                ident: db.custom_ident(parameter.name),
            });
            let output_ty = symbol_context.parse_entity_route(output_ty).unwrap();
            msg_once!("todo: keyword parameters");
            Ok(Arc::new(CallFormDecl {
                opt_base_route: Some(route),
                spatial_parameters,
                primary_parameters: parameters,
                output: OutputDecl::new(db, output_liason, output_ty)?,
                keyword_parameters: Default::default(),
                variadic_template: VariadicTemplate::from_static(
                    &mut symbol_context,
                    variadic_template,
                ),
                opt_this_liason: None,
                is_lazy: false,
            }))
        }
        _ => panic!(),
    }
}

// pub(crate) fn model_decl_from_static(
//     db: &dyn DeclQueryGroup,
//     mut symbols: Vec<Symbol>,
//     route: EntityRoutePtr,
//     static_defn: &EntityStaticDefn,
// ) -> Arc<FunctionDecl> {
//     match static_defn.variant {
//         EntityStaticDefnVariant::Model {
//             spatial_parameters: ref generic_parameters,
//             ref parameters,
//             output_ty,
//             output_liason,
//             ..
//         } => {
//             let generic_parameters = db.generic_parameters_from_static(generic_parameters);
//             symbols.extend(db.symbols_from_generic_parameters(&generic_parameters));
//             let mut symbol_context = AtomContextStandalone {
//                 opt_target_entrance: None,
//                 db: db.upcast(),
//                 opt_this_ty: None,
//                 opt_this_contract: None,
//                 symbols: (&symbols as &[Symbol]).into(),
//                 kind: AtomContextKind::Normal,
//             };
//             let parameters = parameters.map(|parameter| ParameterDecl {
//                 ty: symbol_context.parse_entity_route(parameter.ty).unwrap(),
//                 liason: parameter.liason,
//                 ident: db.custom_ident(parameter.name),
//             });
//             let output_ty = symbol_context.parse_entity_route(output_ty).unwrap();
//             msg_once!("todo: keyword parameters");
//             Arc::new(FunctionDecl {
//                 route,
//                 spatial_parameters: generic_parameters,
//                 primary_parameters: parameters,
//                 output: OutputDecl {
//                     liason: output_liason,
//                     ty: output_ty,
//                 },
//                 keyword_parameters: Default::default(),
//             })
//         }
//         _ => panic!(),
//     }
// }
