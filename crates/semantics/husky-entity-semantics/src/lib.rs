mod call_form;
mod dependence;
mod feature;
mod function;
mod module;
mod query;
mod repr;
mod subentities;
mod trai;
mod ty;
mod utils;
mod verify;
mod visual;

pub use call_form::*;
pub use feature::*;
pub use function::*;
use husky_entity_path::EntityPathItd;
pub use query::*;
pub use repr::*;
pub use trai::*;
pub use ty::*;
pub use visual::*;

use avec::Avec;
use fold::{FoldIterItem, FoldableStorage};
use husky_ast::AstVariant;
use husky_defn_head::*;
use husky_eager_semantics::*;
use husky_entity_kind::*;
use husky_entity_syntax::EntitySource;
use husky_expr_syntax::*;
use husky_term::Ty;

use husky_file::PathItd;

use husky_lazy_semantics::{LazyExpr, LazyExprVariant, LazyOpnKind, LazyStmt, LazyStmtVariant};
use husky_liason_semantics::*;
use husky_print_utils::{msg_once, p};
use husky_semantics_error::*;
use husky_static_defn::{EntityStaticDefn, EntityStaticDefnVariant};
use husky_static_visualizer::StaticVisualTy;
use husky_text::*;
use husky_vm::*;
use husky_word::{
    ContextualIdentifier, CustomIdentifier, IdentDict, Identifier, RootBuiltinIdentifier,
};
use map_collect::MapCollect;
use module::module_defn;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use thin_vec::thin_vec;
use vec_like::VecMapEntry;

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
pub struct EntityDefnUid {
    raw: usize,
}

static ENTITY_DEFN_NEXT_RAW_ID: AtomicUsize = AtomicUsize::new(0);

impl EntityDefnUid {
    pub fn new() -> EntityDefnUid {
        let raw = ENTITY_DEFN_NEXT_RAW_ID.fetch_add(1, Ordering::Relaxed);
        EntityDefnUid { raw }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct EntityDefn {
    pub ident: Identifier,
    pub variant: EntityDefnVariant,
    pub subentities: Arc<Vec<Arc<EntityDefn>>>,
    pub base_route: Ty,
    pub file: PathItd,
    pub range: TextRange,
}

impl VecMapEntry<CustomIdentifier> for EntityDefn {
    fn key(&self) -> CustomIdentifier {
        self.ident.custom()
    }
}

impl EntityDefn {
    pub fn from_generic(
        db: &dyn EntityDefnQueryGroup,
        ident: CustomIdentifier,
        entity_path: EntityPathItd,
        file: PathItd,
        range: TextRange,
    ) -> Arc<Self> {
        todo!()
        // Self::new(db, ident.into(), EntityDefnVariant::Any, route, file, range)
    }

    pub fn this_type(db: &dyn EntityDefnQueryGroup, file: PathItd, range: TextRange) -> Arc<Self> {
        todo!()
        // Self::new(
        //     db,
        //     Identifier::Contextual(ContextualIdentifier::ThisType),
        //     EntityDefnVariant::Any,
        //     db.intern_entity_route(EntityRoute {
        //         variant: EntityRouteVariant::ThisType { file, range },
        //         temporal_arguments: Default::default(),
        //         spatial_arguments: Default::default(),
        //     }),
        //     file,
        //     range,
        // )
    }

    pub(crate) fn new(
        db: &dyn EntityDefnQueryGroup,
        ident: Identifier,
        variant: EntityDefnVariant,
        base_route: Ty,
        file: PathItd,
        range: TextRange,
    ) -> Arc<EntityDefn> {
        let entity_defn = Self {
            ident,
            subentities: variant.subentities(),
            variant,
            base_route,
            file,
            range,
        };
        entity_defn.verify(db);
        Arc::new(entity_defn)
    }

    pub fn is_builtin(&self) -> bool {
        match self.variant {
            EntityDefnVariant::Builtin => true,
            _ => false,
        }
    }

    pub fn trait_impl(&self, trai: Ty) -> Option<&Arc<TraitImplDefn>> {
        match self.variant {
            EntityDefnVariant::Ty {
                ref trait_impls, ..
            } => trait_impls
                .iter()
                .find(|trait_impl| trait_impl.trai == trai),
            _ => panic!(""),
        }
    }

    pub fn trait_member_defn(&self, ident: CustomIdentifier) -> Option<&Arc<EntityDefn>> {
        match self.variant {
            EntityDefnVariant::Trait { ref members, .. } => members.get_entry(ident),
            _ => panic!(""),
        }
    }

    pub fn spatial_parameters(&self) -> &[SpatialParameter] {
        match self.variant {
            EntityDefnVariant::Module { .. } => panic!(),
            EntityDefnVariant::Feature { .. } => panic!(),
            EntityDefnVariant::Function {
                ref spatial_parameters,
                ..
            } => spatial_parameters,
            EntityDefnVariant::Method {
                ref spatial_parameters,
                ..
            } => spatial_parameters,
            EntityDefnVariant::Func {
                ref spatial_parameters,
                ..
            } => spatial_parameters,
            EntityDefnVariant::Proc {
                ref spatial_parameters,
                ..
            } => spatial_parameters,
            EntityDefnVariant::Ty {
                ref spatial_parameters,
                ..
            } => spatial_parameters,
            EntityDefnVariant::Trait {
                ref spatial_parameters,
                ..
            } => spatial_parameters,
            EntityDefnVariant::EnumVariant { .. } => {
                msg_once!("enum spatial parameters");
                todo!()
            }
            EntityDefnVariant::Builtin => todo!(),
            EntityDefnVariant::TyField { .. } => todo!(),
            EntityDefnVariant::TraitAssociatedTypeImpl { .. } => todo!(),
            EntityDefnVariant::TraitAssociatedConstSizeImpl { .. } => todo!(),
            EntityDefnVariant::TargetInput { .. } => todo!(),
            EntityDefnVariant::Any => todo!(),
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum EntityDefnVariant {
    Module {
        module_items: Avec<EntityDefn>,
        opt_main_defn: Option<Arc<MainDefn>>,
    },
    Feature {
        defn_repr: Arc<DefinitionRepr>,
    },
    Function {
        spatial_parameters: IdentDict<SpatialParameter>,
        parameters: Arc<Vec<Parameter>>,
        source: CallFormSource,
    },
    Method {
        spatial_parameters: IdentDict<SpatialParameter>,
        this_modifier: ParameterModifier,
        parameters: Arc<Vec<Parameter>>,
        output_modifier: OutputModifier,
        method_defn_kind: MethodDefnKind,
        opt_source: Option<CallFormSource>,
    },
    Func {
        spatial_parameters: IdentDict<SpatialParameter>,
        parameters: Arc<Vec<Parameter>>,
        stmts: Arc<Vec<Arc<FuncStmt>>>,
    },
    Proc {
        spatial_parameters: IdentDict<SpatialParameter>,
        parameters: Arc<Vec<Parameter>>,
        stmts: Avec<ProcStmt>,
    },
    Ty {
        spatial_parameters: IdentDict<SpatialParameter>,
        ty_members: IdentDict<Arc<EntityDefn>>,
        variants: IdentDict<Arc<EntityDefn>>,
        ty_kind: TyKind,
        trait_impls: Vec<Arc<TraitImplDefn>>,
        members: Avec<EntityDefn>,
        opt_type_call: Option<Arc<TypeCallDefn>>,
        visualizer: Arc<Visualizer>,
    },
    Trait {
        spatial_parameters: IdentDict<SpatialParameter>,
        members: IdentDict<Arc<EntityDefn>>,
    },
    EnumVariant {
        enum_variant_defn_variant: EnumVariantDefnVariant,
    },
    Builtin,
    TyField {
        field_ty: Ty,
        field_variant: FieldDefnVariant,
        liason: MemberModifier,
        opt_linkage: Option<__Linkage>,
    },
    TraitAssociatedTypeImpl {
        trai: Ty,
        ty: Ty,
    },
    TraitAssociatedConstSizeImpl {
        value: usize,
    },
    TargetInput,
    Any,
}

impl std::fmt::Debug for EntityDefnVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityDefnVariant::Module { .. } => f.write_str("Module { ... }"),
            EntityDefnVariant::Feature { .. } => f.write_str("Feature { ... }"),
            EntityDefnVariant::Function { .. } => f.write_str("Function { ... }"),
            EntityDefnVariant::Method { .. } => f.write_str("Method { ... }"),
            EntityDefnVariant::Func { .. } => f.write_str("Func { ... }"),
            EntityDefnVariant::Proc { .. } => f.write_str("Proc { ... }"),
            EntityDefnVariant::Ty { .. } => f.write_str("Ty { ... }"),
            EntityDefnVariant::Trait { .. } => f.write_str("Trait { ... }"),
            EntityDefnVariant::EnumVariant { .. } => f.write_str("EnumVariant { ... }"),
            EntityDefnVariant::Builtin => f.write_str("Builtin { ... }"),
            EntityDefnVariant::TyField { .. } => f.write_str("TyField"),
            EntityDefnVariant::TraitAssociatedTypeImpl { .. } => {
                f.write_str("TraitAssociatedTypeImpl { ... }")
            }
            EntityDefnVariant::TraitAssociatedConstSizeImpl { .. } => {
                f.write_str("TraitAssociatedConstSizeImpl { ... }")
            }
            EntityDefnVariant::TargetInput { .. } => f.write_str("Input"),
            EntityDefnVariant::Any => f.write_str("Generic"),
        }
    }
}

pub(crate) fn main_defn(
    _this: &dyn EntityDefnQueryGroup,
    _target_entrance: husky_file::PathItd,
) -> SemanticResultArc<MainDefn> {
    todo!()
    // let ast_text = this.ast_text(target_entrance).unwrap();
    // for item in ast_text.folded_results.iter() {
    //     match item.value.as_ref().unwrap().variant {
    //         AstVariant::MainDefnHead => {
    //             let ty = Ty {
    //                 route: this.target_output_ty().unwrap(),
    //                 range: Default::default(),
    //             };
    //             return Ok(Arc::new(MainDefn {
    //                 defn_repr: DefinitionRepr::LazyBlock {
    //                     stmts: parse_lazy_stmts(
    //                         this.upcast(),
    //                         &ast_text.arena,
    //                         not_none!(item.opt_children),
    //                         target_entrance,
    //                         ty,
    //                     )?,
    //                     ty,
    //                 },
    //                 file: target_entrance,
    //             }));
    //         }
    //         _ => (),
    //     }
    // }
    // err!("main not found")
}

pub(crate) fn entity_defn(
    db: &dyn EntityDefnQueryGroup,
    entity_path: EntityPathItd,
) -> SemanticResultArc<EntityDefn> {
    // let source = db.entity_source(entity_path).unwrap();
    todo!()
    // match source {
    //     EntitySource::StaticModuleItem(static_defn) => Ok(EntityDefn::from_static(
    //         db,
    //         &mut AtomContextStandalone {
    //             db: db.upcast(),
    //             opt_this_ty: None,
    //             opt_this_contract: None,
    //             symbols: (&[] as &[Symbol]).into(),
    //             kind: AtomContextKind::Normal,
    //             opt_file: Some(db.intern_path(static_defn.dev_src.file.into())),
    //         },
    //         entity_path,
    //         static_defn,
    //     )),
    //     EntitySource::WithinBuiltinModule => todo!(),
    //     EntitySource::WithinModule {
    //         file,
    //         token_group_index,
    //     } => {
    //         let ast_text = db.ast_text(file).unwrap();
    //         let arena = &ast_text.arena;
    //         let FoldIterItem {
    //             value,
    //             opt_children,
    //             ..
    //         } = ast_text
    //             .folded_results
    //             .iter_from(token_group_index)
    //             .next()
    //             .unwrap();
    //         let ast = value.as_ref().unwrap();

    //         let (ident, husky_entity_kind) = match ast.variant {
    //             AstVariant::TypeDefnHead { ident, .. } => (
    //                 ident,
    //                 EntityDefnVariant::ty_from_ast(
    //                     db,
    //                     entity_path,
    //                     ast,
    //                     not_none!(opt_children),
    //                     arena,
    //                     file,
    //                 )?,
    //             ),
    //             AstVariant::CallFormDefnHead {
    //                 opt_this_liason,
    //                 ident,
    //                 ..
    //             } => match opt_this_liason {
    //                 Some(_) => return Ok(db.member_defn(entity_path)),
    //                 None => (
    //                     ident,
    //                     EntityDefnVariant::function(db, ast, not_none!(opt_children), arena, file)?,
    //                 ),
    //             },
    //             AstVariant::FieldDefnHead { .. } => return Ok(db.member_defn(entity_path)),
    //             AstVariant::Use { .. } => todo!(),
    //             AstVariant::MainDefnHead
    //             | AstVariant::DatasetConfigDefnHead
    //             | AstVariant::Stmt(_) => {
    //                 panic!()
    //             }
    //             AstVariant::EnumVariantDefnHead {
    //                 ident,
    //                 variant_class,
    //             } => (
    //                 ident,
    //                 EntityDefnVariant::enum_variant(db, ident, variant_class, opt_children),
    //             ),
    //             AstVariant::FeatureDefnHead {
    //                 paradigm,
    //                 ident,
    //                 return_ty: output_ty,
    //             } => (
    //                 ident,
    //                 EntityDefnVariant::feature(
    //                     db,
    //                     entity_path,
    //                     paradigm,
    //                     output_ty,
    //                     opt_children,
    //                     arena,
    //                     file,
    //                 )?,
    //             ),
    //             AstVariant::Submodule {
    //                 ident: _,
    //                 source_file: _,
    //             } => todo!(),
    //             AstVariant::Visual => todo!(),
    //         };
    //         Ok(EntityDefn::new(
    //             db,
    //             ident.ident.into(),
    //             husky_entity_kind,
    //             entity_path,
    //             file,
    //             ast.range,
    //         ))
    //     }
    //     EntitySource::Module { file } => module_defn(db, entity_path, file),
    //     EntitySource::TargetInput => {
    //         msg_once!("use task config for input defn");
    //         Ok(Arc::new(EntityDefn {
    //             ident: entity_path.ident(),
    //             variant: EntityDefnVariant::TargetInput,
    //             subentities: Default::default(),
    //             base_route: entity_path,
    //             file: db.opt_target_entrance().unwrap(),
    //             range: Default::default(),
    //         }))
    //     }
    //     EntitySource::StaticTypeMember(_) => match entity_path.variant {
    //         EntityRouteVariant::Child { parent, ident } => {
    //             let ty_defn = db.entity_defn(parent).unwrap();
    //             match ty_defn.variant {
    //                 EntityDefnVariant::Ty { ref ty_members, .. } => Ok(ty_members[ident].clone()),
    //                 _ => panic!(),
    //             }
    //         }
    //         _ => panic!(),
    //     },
    //     EntitySource::StaticTypeAsTraitMember => match entity_path.variant {
    //         EntityRouteVariant::TypeAsTraitMember { ty, trai, ident } => match trai {
    //             Ty::Root(RootBuiltinIdentifier::CloneTrait) => {
    //                 msg_once!("this is a temporary ugly solution");
    //                 let clone_trait_defn = db
    //                     .entity_defn(Ty::Root(RootBuiltinIdentifier::CloneTrait))
    //                     .unwrap();
    //                 Ok(clone_trait_defn.trait_member_defn(ident).unwrap().clone())
    //             }
    //             _ => {
    //                 let ty_defn = db.entity_defn(ty)?;
    //                 p!(ty, trai);
    //                 let trai_impl_defn = ty_defn
    //                     .trait_impl(trai)
    //                     .expect("todo: trait_impl not found");
    //                 Ok(trai_impl_defn.member_impl(ident).clone())
    //             }
    //         },
    //         _ => panic!(),
    //     },
    //     EntitySource::StaticTraitMember(_) => match entity_path.variant {
    //         EntityRouteVariant::Child { parent, ident } => {
    //             let trai_defn = db.entity_defn(parent).unwrap();
    //             match trai_defn.variant {
    //                 EntityDefnVariant::Trait { ref members, .. } => Ok(members[ident].clone()),
    //                 _ => panic!(),
    //             }
    //         }
    //         _ => panic!(),
    //     },
    //     EntitySource::Any {
    //         ident,
    //         file,
    //         range,
    //         entity_path,
    //     } => Ok(EntityDefn::from_generic(db, ident, route, file, range)),
    //     EntitySource::StaticEnumVariant(_) => todo!(),
    //     EntitySource::ThisType { file, range } => Ok(EntityDefn::this_type(db, file, range)),
    // }
}

pub(crate) fn subentity_defns(
    this: &dyn EntityDefnQueryGroup,
    scope_id: Ty,
) -> SemanticResultArc<Vec<Arc<EntityDefn>>> {
    todo!()
    // let mut defns = Vec::new();
    // for defn_result in this
    //     .subentity_routes(scope_id)
    //     .iter()
    //     .map(|scope| this.entity_defn(*scope))
    // {
    //     let defn = defn_result?;
    //     defns.push(defn)
    // }
    // Ok(Arc::new(defns))
}

pub(crate) fn entity_defn_uid(db: &dyn EntityDefnQueryGroup, entity_path: Ty) -> EntityDefnUid {
    todo!()
    // let _defn = db.entity_defn(entity_path);
    // EntityDefnUid::new()
}
