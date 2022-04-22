mod table;
mod vec;

use static_defn::StaticEntityDefnVariant;
pub use table::*;

use check_utils::*;
use entity_route::{EntityRoute, EntityRouteKind, EntityRoutePtr, EntitySource, GenericArgument};
use map_collect::MapCollect;
use print_utils::p;
use semantics_entity::{EntityDefnQueryGroup, EntityDefnVariant, MethodDefnVariant};
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
        match opd_tys[0].kind {
            EntityRouteKind::Root {
                ident: RootIdentifier::Vec,
            } => {
                should_eq!(opd_tys.len(), 2);
                Linkage {
                    call: match access_kind {
                        MemberAccessKind::Move => virtual_vec_element_move_access,
                        MemberAccessKind::Ref => virtual_vec_element_ref_access,
                        MemberAccessKind::BorrowMut => virtual_vec_element_borrow_mut_access,
                    },
                    nargs: 2,
                }
            }
            _ => {
                let this_ty_defn = self.entity_defn(opd_tys[0]).unwrap();
                let std_ops_index_trai = self.intern_entity_route(EntityRoute {
                    kind: self.entity_route_menu().std_ops_index_trai.kind,
                    generic_arguments: vec![GenericArgument::EntityRoute(opd_tys[1])],
                });
                let index_trai_impl = this_ty_defn.trait_impl(std_ops_index_trai).unwrap();
                match index_trai_impl.member_impls[1].variant {
                    EntityDefnVariant::TraitMethodImpl {
                        method_variant:
                            MethodDefnVariant::StaticMemberAccess {
                                ref_access,
                                move_access,
                                borrow_mut_access,
                            },
                        ..
                    } => match access_kind {
                        MemberAccessKind::Move => move_access,
                        MemberAccessKind::Ref => ref_access,
                        MemberAccessKind::BorrowMut => borrow_mut_access,
                    },
                    _ => panic!(""),
                }
            }
        }
    }

    fn struct_field_access(
        &self,
        this_ty: EntityRoutePtr,
        field_ident: CustomIdentifier,
    ) -> Option<Linkage> {
        todo!()
    }

    fn method_linkage(&self, method_route: EntityRoutePtr) -> Option<Linkage> {
        if let Some(linkage) = self.linkage_table().routine(self.entity_uid(method_route)) {
            return Some(linkage);
        }
        let method_defn = self.entity_defn(method_route).unwrap();
        match method_defn.variant {
            EntityDefnVariant::Builtin => todo!(),
            EntityDefnVariant::TypeMethod { .. } => todo!(),
            EntityDefnVariant::TraitMethod { .. } => todo!(),
            EntityDefnVariant::TraitMethodImpl { .. } => todo!(),
            _ => panic!(),
        }
    }

    fn routine_linkage(&self, routine: EntityRoutePtr) -> Option<Linkage> {
        match self.entity_source(routine).unwrap() {
            EntitySource::StaticModuleItem(builtin_entity_data) => {
                match builtin_entity_data.variant {
                    StaticEntityDefnVariant::Func(ref func_decl) => Some(func_decl.linkage),
                    StaticEntityDefnVariant::Type { .. } => todo!(),
                    StaticEntityDefnVariant::Module => todo!(),
                    StaticEntityDefnVariant::Trait { .. } => todo!(),
                }
            }
            EntitySource::WithinBuiltinModule => todo!(),
            EntitySource::WithinModule { .. } => {
                self.linkage_table().routine(self.entity_uid(routine))
            }
            EntitySource::Module { file } => todo!(),
            EntitySource::Input { main } => todo!(),
            EntitySource::StaticTypeMember => todo!(),
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

    fn struct_constructor(&self, ty_uid: EntityUid) -> Option<Linkage> {
        todo!()
    }
}
