mod table;
mod vec;

use entity_syntax::EntityLocus;
use static_defn::{EntityStaticDefnVariant, LinkageSource};
pub use table::*;

use check_utils::*;
use entity_route::{EntityRoute, EntityRouteKind, EntityRoutePtr, SpatialArgument};
use map_collect::MapCollect;
use print_utils::p;
use semantics_entity::{
    EntityDefnQueryGroup, EntityDefnVariant, MethodDefnVariant, MethodSource, TyCallSource,
};
use std::collections::HashMap;
use sync_utils::ARwLock;
use vec::*;
use vm::{Binding, EntityUid};
use vm::{EvalValue, Linkage, OwnedValue, VMRuntimeResult, VMValue};
use word::{CustomIdentifier, RootIdentifier};

pub trait ResolveLinkage: EntityDefnQueryGroup {
    fn linkage_table(&self) -> &LinkageTable;

    fn element_access_linkage(
        &self,
        opd_tys: Vec<EntityRoutePtr>,
        element_binding: Binding,
    ) -> Linkage {
        if let Some(linkage) = self
            .linkage_table()
            .element_access(opd_tys.map(|ty| self.entity_uid(*ty)), element_binding)
        {
            return linkage;
        }
        let this_ty_defn = self.entity_defn(opd_tys[0]).unwrap();
        let std_ops_index_trai = self.intern_entity_route(EntityRoute {
            kind: self.entity_route_menu().std_ops_index_trai.kind,
            spatial_arguments: vec![SpatialArgument::EntityRoute(opd_tys[1])],
        });
        let index_trai_impl = this_ty_defn.trait_impl(std_ops_index_trai).unwrap();
        match index_trai_impl.member_impls[1].variant {
            EntityDefnVariant::Method {
                ref method_variant, ..
            } => match method_variant {
                MethodDefnVariant::TraitMethodImpl { trai, opt_source } => {
                    if let Some(source) = opt_source {
                        match source {
                            MethodSource::Func { stmts } => todo!(),
                            MethodSource::Proc { stmts } => todo!(),
                            MethodSource::Pattern { stmts } => todo!(),
                            MethodSource::Static(linkage_source) => {
                                linkage_source.bind(Some(element_binding))
                            }
                        }
                    } else {
                        todo!()
                    }
                }
                _ => {
                    p!(method_variant, this_ty_defn.file, this_ty_defn.range);
                    panic!("")
                }
            },
            _ => panic!(""),
        }
    }

    fn struct_field_access(
        &self,
        this_ty: EntityRoutePtr,
        field_ident: CustomIdentifier,
    ) -> Option<LinkageSource> {
        if let Some(linkage) = self
            .linkage_table()
            .struct_field_access(self.entity_uid(this_ty), field_ident)
        {
            return Some(LinkageSource::Transfer(linkage));
        } else {
            None
        }
    }

    fn method_linkage(
        &self,
        method_route: EntityRoutePtr,
        opt_output_binding: Option<Binding>,
    ) -> Option<Linkage> {
        let opt_linkage = if let Some(linkage) =
            self.linkage_table().routine(self.entity_uid(method_route))
        {
            Some(linkage)
        } else {
            let method_defn = self.entity_defn(method_route).unwrap();
            match method_defn.variant {
                EntityDefnVariant::Builtin => todo!(),
                EntityDefnVariant::Method {
                    ref method_variant, ..
                } => match method_variant {
                    MethodDefnVariant::TypeMethod { method_source, .. } => match method_source {
                        MethodSource::Func { .. }
                        | MethodSource::Proc { .. }
                        | MethodSource::Pattern { .. } => None,
                        MethodSource::Static(linkage_source) => {
                            Some(linkage_source.bind(opt_output_binding))
                        }
                    },
                    MethodDefnVariant::TraitMethod {
                        trai,
                        ref opt_default_source,
                    } => opt_default_source.as_ref().map(|source| match source {
                        MethodSource::Func { stmts } => todo!(),
                        MethodSource::Proc { stmts } => todo!(),
                        MethodSource::Pattern { stmts } => todo!(),
                        MethodSource::Static(linkage_source) => {
                            linkage_source.bind(opt_output_binding)
                        }
                    }),
                    MethodDefnVariant::TraitMethodImpl { trai, opt_source } => {
                        if let Some(source) = opt_source {
                            return match source {
                                MethodSource::Func { ref stmts } => todo!(),
                                MethodSource::Proc { ref stmts } => todo!(),
                                MethodSource::Pattern { ref stmts } => todo!(),
                                MethodSource::Static(linkage_source) => {
                                    Some(linkage_source.bind(opt_output_binding))
                                }
                            };
                        }
                        let trai_defn = self.entity_defn(*trai).unwrap();
                        match trai_defn.variant {
                            EntityDefnVariant::Trait {
                                ref generic_parameters,
                                ref members,
                            } => {
                                let member = members
                                    .iter()
                                    .find(|member| member.ident == method_defn.ident)
                                    .unwrap();
                                match member.variant {
                                    EntityDefnVariant::Method {
                                        method_variant:
                                            MethodDefnVariant::TraitMethod {
                                                trai,
                                                ref opt_default_source,
                                            },
                                        ..
                                    } => match opt_default_source.as_ref().unwrap() {
                                        MethodSource::Func { ref stmts } => todo!(),
                                        MethodSource::Proc { ref stmts } => todo!(),
                                        MethodSource::Pattern { ref stmts } => todo!(),
                                        MethodSource::Static(linkage_source) => {
                                            Some(linkage_source.bind(opt_output_binding))
                                        }
                                    },
                                    _ => panic!(),
                                }
                            }
                            _ => panic!(),
                        }
                    }
                },
                _ => todo!(),
            }
        };
        let method_decl = self.method_decl(method_route).unwrap();
        if let Some(linkage) = opt_linkage {
            should_eq!(linkage.nargs, method_decl.nargs());
        }
        opt_linkage
    }

    fn routine_linkage(&self, routine: EntityRoutePtr) -> Option<Linkage> {
        let opt_linkage = match self.entity_locus(routine).unwrap() {
            EntityLocus::StaticModuleItem(static_defn) => match static_defn.variant {
                EntityStaticDefnVariant::Routine { linkage, .. } => Some(linkage),
                _ => todo!(),
            },
            EntityLocus::WithinBuiltinModule => todo!(),
            EntityLocus::WithinModule { .. } => {
                self.linkage_table().routine(self.entity_uid(routine))
            }
            EntityLocus::Module { file } => todo!(),
            EntityLocus::Input { main } => todo!(),
            EntityLocus::StaticTypeMember => todo!(),
            EntityLocus::StaticTypeAsTraitMember => todo!(),
        };
        let call_decl = self.call_decl(routine).unwrap();
        if let Some(linkage) = opt_linkage {
            should_eq!(linkage.nargs, call_decl.nargs());
        }
        opt_linkage
    }

    fn field_access_fp(
        &self,
        this_ty: EntityRoutePtr,
        field_ident: CustomIdentifier,
    ) -> Option<Linkage> {
        self.linkage_table()
            .struct_field_access(self.entity_uid(this_ty), field_ident)
    }

    fn type_call_linkage(&self, ty: EntityRoutePtr) -> Option<Linkage> {
        if let Some(linkage) = self.linkage_table().type_call(self.entity_uid(ty)) {
            return Some(linkage);
        }
        let ty_defn = self.entity_defn(ty).unwrap();
        match ty_defn.variant {
            EntityDefnVariant::Type {
                ref opt_type_call, ..
            } => opt_type_call
                .as_ref()
                .map(|type_call| match type_call.source {
                    TyCallSource::GenericStruct => None,
                    TyCallSource::Static(linkage) => Some(linkage),
                    TyCallSource::GenericRecord => todo!(),
                })
                .flatten(),
            _ => panic!(),
        }
    }
}
