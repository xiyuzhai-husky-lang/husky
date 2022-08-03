use crate::*;
use entity_kind::TyKind;
use husky_compile_time::*;
use husky_entity_route::EntityRoutePtr;
use husky_vm_binding::Binding;
use husky_word::RootIdentifier;
use std::sync::Arc;

#[salsa::query_group(HuskyDataViewerQueryGroupStorage)]
pub trait HuskyDataViewerQueryGroup: AskCompileTime {
    fn ty_data_viewer(&self, ty: EntityRoutePtr) -> Arc<HuskyDataViewer>;
}

fn ty_data_viewer(db: &dyn HuskyDataViewerQueryGroup, ty: EntityRoutePtr) -> Arc<HuskyDataViewer> {
    let ty_decl: Arc<TyDecl> = db.compile_time().ty_decl(ty).unwrap();
    let comptime = db.compile_time();
    Arc::new(match ty_decl.ty_kind {
        TyKind::Enum => todo!(),
        TyKind::Record => todo!(),
        TyKind::Struct => HuskyDataViewer::Struct {
            fields: {
                let mut fields: IdentPairDict<(__Linkage, EntityRoutePtr)> = Default::default();
                for ty_member in ty_decl.ty_members.iter() {
                    match ty_member {
                        TyMemberDecl::Field(field) => fields
                            .insert_new((
                                field.ident,
                                (comptime.field_linkage(ty, field.ident).unwrap(), field.ty),
                            ))
                            .unwrap(),
                        _ => break,
                    }
                }
                fields
            },
        },
        TyKind::Primitive => HuskyDataViewer::Primitive { ty: ty.root() },
        TyKind::Vec => HuskyDataViewer::Vec {
            ilen: comptime
                .method_linkage(comptime.subroute(
                    ty,
                    comptime.intern_word("ilen").custom(),
                    Default::default(),
                ))
                .unwrap()
                .transfer(),
            index: comptime.index_linkage(vec![ty, RootIdentifier::I32.into()]),
            elem_ty: ty.spatial_arguments[0].take_entity_route(),
        },
        TyKind::Slice => todo!(),
        TyKind::CyclicSlice => HuskyDataViewer::CyclicSlice {
            start: comptime
                .field_linkage_fp(ty, comptime.intern_word("start").custom(), Binding::Copy)
                .unwrap(),
            end: comptime
                .field_linkage_fp(ty, comptime.intern_word("end").custom(), Binding::Copy)
                .unwrap(),
            index: comptime.index_linkage(vec![ty, RootIdentifier::I32.into()]),
            elem_ty: ty.spatial_arguments[0].take_entity_route(),
        },
        TyKind::Array => todo!(),
        TyKind::Tuple => todo!(),
        TyKind::Mor => todo!(),
        TyKind::Fp => todo!(),
        TyKind::AssociatedAny => todo!(),
        TyKind::ThisAny => todo!(),
        TyKind::SpatialPlaceholderAny => todo!(),
        TyKind::BoxAny => todo!(),
        TyKind::HigherKind => todo!(),
        TyKind::Ref => todo!(),
        TyKind::Option => todo!(),
    })
}
