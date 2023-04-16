use crate::*;

use husky_ethereal_term::EtherealTerm;

use std::sync::Arc;

pub trait DataViewerDb {
    fn ty_data_viewer(&self, ty: EtherealTerm) -> Arc<HuskyDataViewer>;
}

fn ty_data_viewer(_db: &dyn DataViewerDb, _ty: EtherealTerm) -> Arc<HuskyDataViewer> {
    todo!()
    // let ty_decl: Arc<TyDecl> = db.ty_decl(ty).unwrap();
    // let comptime = db;
    // Arc::new(match ty_decl.ty_kind {
    //     TyKind::Enum => todo!(),
    //     TyKind::Record => todo!(),
    //     TyKind::Struct => HuskyDataViewer::Struct {
    //         fields: {
    //             let mut fields: IdentPairMap<(__Linkage, EntityRoutePtr)> = Default::default();
    //             for ty_member in ty_decl.ty_members.iter() {
    //                 match ty_member {
    //                     TyMemberDecl::Field(field) => fields
    //                         .insert_new((
    //                             field.ident,
    //                             (comptime.field_linkage(ty, field.ident).unwrap(), field.ty),
    //                         ))
    //                         .unwrap(),
    //                     _ => break,
    //                 }
    //             }
    //             fields
    //         },
    //     },
    //     TyKind::Primitive => HuskyDataViewer::Primitive { ty: ty.root() },
    //     TyKind::Vec => HuskyDataViewer::Vec {
    //         ilen: comptime
    //             .method_linkage(comptime.subroute(
    //                 ty,
    //                 comptime.it_word("ilen").custom(),
    //                 Default::default(),
    //             ))
    //             .unwrap()
    //             .transfer(),
    //         index: comptime.index_linkage(vec![ty, RootBuiltinIdent::I32.into()]),
    //         elem_ty: ty.entity_route_argument(0),
    //     },
    //     TyKind::Slice => todo!(),
    //     TyKind::CyclicSlice => HuskyDataViewer::CyclicSlice {
    //         start: comptime
    //             .field_linkage_resolved(ty, comptime.it_word("start").custom(), Binding::Copy)
    //             .unwrap(),
    //         end: comptime
    //             .field_linkage_resolved(ty, comptime.it_word("end").custom(), Binding::Copy)
    //             .unwrap(),
    //         index: comptime.index_linkage(vec![ty, RootBuiltinIdent::I32.into()]),
    //         elem_ty: ty.entity_route_argument(0),
    //     },
    //     TyKind::Array => todo!(),
    //     TyKind::Tuple => todo!(),
    //     TyKind::Mor => todo!(),
    //     TyKind::ThickFp => todo!(),
    //     TyKind::AssociatedAny => todo!(),
    //     TyKind::TargetOutputAny => todo!(),
    //     TyKind::ThisAny => todo!(),
    //     TyKind::SpatialPlaceholderAny => todo!(),
    //     TyKind::BoxAny => todo!(),
    //     TyKind::HigherKind => todo!(),
    //     TyKind::Ref => todo!(),
    //     TyKind::Option => todo!(),
    // })
}
