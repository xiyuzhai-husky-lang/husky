mod table;
mod vec;

use entity_route_query::EntitySource;
use static_defn::{EntityStaticDefnVariant, LinkageSource};
pub use table::*;

use check_utils::*;
use entity_route::{EntityRoute, EntityRouteKind, EntityRoutePtr, GenericArgument};
use map_collect::MapCollect;
use print_utils::p;
use semantics_entity::{
    EntityDefnQueryGroup, EntityDefnVariant, MethodDefnVariant, MethodSource, TyCallSource,
};
use std::collections::HashMap;
use sync_utils::ARwLock;
use vec::*;
use vm::EntityUid;
use vm::{BoxedValue, EvalValue, Linkage, StackValue, VMResult};
use word::{CustomIdentifier, RootIdentifier};

pub trait ResolveLinkage: EntityDefnQueryGroup {
    fn linkage_table(&self) -> &LinkageTable;

    fn element_access_linkage(
        &self,
        opd_tys: Vec<EntityRoutePtr>,
        access_kind: MemberAccessKind,
    ) -> Linkage {
        if let Some(linkage) = self
            .linkage_table()
            .element_access(opd_tys.map(|ty| self.entity_uid(*ty)), access_kind)
        {
            return linkage;
        }
        let this_ty_defn = self.entity_defn(opd_tys[0]).unwrap();
        let std_ops_index_trai = self.intern_entity_route(EntityRoute {
            kind: self.entity_route_menu().std_ops_index_trai.kind,
            generic_arguments: vec![GenericArgument::EntityRoute(opd_tys[1])],
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
                            MethodSource::Static(static_linkage_source) => {
                                match static_linkage_source {
                                    LinkageSource::MemberAccess {
                                        copy_access,
                                        ref_access,
                                        move_access,
                                        ref_mut_access: borrow_mut_access,
                                    } => match access_kind {
                                        MemberAccessKind::Move => *move_access,
                                        MemberAccessKind::Ref => *ref_access,
                                        MemberAccessKind::BorrowMut => *borrow_mut_access,
                                        MemberAccessKind::Copy => *copy_access,
                                    },
                                    LinkageSource::PureOutput(_) => todo!(),
                                }
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
            return Some(LinkageSource::PureOutput(linkage));
        } else {
            None
        }
    }

    fn method_linkage_source(&self, method_route: EntityRoutePtr) -> Option<LinkageSource> {
        if let Some(linkage) = self.linkage_table().routine(self.entity_uid(method_route)) {
            return Some(LinkageSource::PureOutput(linkage));
        }
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
                    MethodSource::Static(linkage) => Some(*linkage),
                },
                MethodDefnVariant::TraitMethod {
                    trai,
                    opt_default_source,
                } => todo!(),
                MethodDefnVariant::TraitMethodImpl { trai, opt_source } => {
                    if let Some(source) = opt_source {
                        return match source {
                            MethodSource::Func { ref stmts } => todo!(),
                            MethodSource::Proc { ref stmts } => todo!(),
                            MethodSource::Pattern { ref stmts } => todo!(),
                            MethodSource::Static(linkage_source) => Some(*linkage_source),
                        };
                    }
                    let trai_defn = self.entity_defn(*trai).unwrap();
                    match trai_defn.variant {
                        EntityDefnVariant::Trait {
                            ref generic_placeholders,
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
                                    MethodSource::Static(linkage_source) => Some(*linkage_source),
                                },
                                _ => panic!(),
                            }
                        }
                        _ => panic!(),
                    }
                }
            },
            _ => {
                panic!()
            }
        }
    }

    fn routine_linkage(&self, routine: EntityRoutePtr) -> Option<Linkage> {
        match self.entity_source(routine).unwrap() {
            EntitySource::StaticModuleItem(static_defn) => match static_defn.variant {
                EntityStaticDefnVariant::Routine { linkage, .. } => Some(linkage),
                EntityStaticDefnVariant::Type { .. } => todo!(),
                EntityStaticDefnVariant::Module => todo!(),
                EntityStaticDefnVariant::Trait { .. } => todo!(),
                EntityStaticDefnVariant::TypeField { .. } => todo!(),
                EntityStaticDefnVariant::Method { .. } => todo!(),
                EntityStaticDefnVariant::TraitAssociatedType { .. } => todo!(),
                EntityStaticDefnVariant::TraitAssociatedConstSize => todo!(),
                EntityStaticDefnVariant::TraitAssociatedTypeImpl { ty } => todo!(),
            },
            EntitySource::WithinBuiltinModule => todo!(),
            EntitySource::WithinModule { .. } => {
                self.linkage_table().routine(self.entity_uid(routine))
            }
            EntitySource::Module { file } => todo!(),
            EntitySource::Input { main } => todo!(),
            EntitySource::StaticTypeMember => todo!(),
            EntitySource::StaticTypeAsTraitMember => todo!(),
        }
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
