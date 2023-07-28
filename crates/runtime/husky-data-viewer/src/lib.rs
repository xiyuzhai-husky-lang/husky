mod db;

pub use db::*;
use husky_coword::{Ident, IdentPairMap};
use husky_ethereal_term::EtherealTerm;
use husky_vm_binding::Binding;

use husky_vm_interface::{__Linkage, __RegistrableSafe, __RegularValue, __ResolvedLinkage};

#[derive(Debug, PartialEq, Eq)]
pub enum HuskyDataViewer {
    Primitive {
        ty: Ident,
    },
    Struct {
        fields: IdentPairMap<(__Linkage, EtherealTerm)>,
    },
    Vec {
        ilen: __ResolvedLinkage,
        index: __Linkage,
        elem_ty: EtherealTerm,
    },
    CyclicSlice {
        start: __ResolvedLinkage,
        end: __ResolvedLinkage,
        index: __Linkage,
        elem_ty: EtherealTerm,
    },
}

impl HuskyDataViewer {
    pub fn print(&self, _value: &__RegularValue) -> String {
        todo!()
    }

    pub fn serialize(&self, _db: &dyn DataViewerDb, _value: &__RegularValue) -> serde_json::Value {
        todo!()
        // match self {
        //     HuskyDataViewer::Primitive { ty } => match ty {
        //         RootBuiltinIdent::Void => todo!(),
        //         RootBuiltinIdent::I32 => todo!(),
        //         RootBuiltinIdent::I64 => todo!(),
        //         RootBuiltinIdent::F32 => serde_json::to_value(value.downcast_f32()).unwrap(),
        //         RootBuiltinIdent::F64 => todo!(),
        //         RootBuiltinIdent::B32 => todo!(),
        //         RootBuiltinIdent::B64 => todo!(),
        //         RootBuiltinIdent::Bool => todo!(),
        //         _ => panic!(),
        //     },
        //     HuskyDataViewer::Struct { fields } => serde_json::Value::Object(
        //         fields
        //             .iter()
        //             .map(|(ident, (linkage, field_ty))| {
        //                 let value = linkage
        //                     .bind(Binding::TempRef)
        //                     .call(None, &mut vec![value.bind_temp_ref()]);
        //                 let field_data_viewer = db.ty_data_viewer(*field_ty);
        //                 let value: serde_json::Value = field_data_viewer.serialize(db, &value);
        //                 (ident.as_str().to_string(), value)
        //             })
        //             .collect(),
        //     ),
        //     HuskyDataViewer::Vec { elem_ty, .. } => {
        //         let elem_data_viewer = db.ty_data_viewer(*elem_ty);
        //         serde_json::Value::Array(
        //             self.member_temp_iter(value)
        //                 .map(|elem| elem_data_viewer.serialize(db, &elem))
        //                 .collect(),
        //         )
        //     }
        //     HuskyDataViewer::CyclicSlice { elem_ty, .. } => {
        //         let elem_data_viewer = db.ty_data_viewer(*elem_ty);
        //         serde_json::Value::Array(
        //             self.member_temp_iter(value)
        //                 .map(|elem| elem_data_viewer.serialize(db, &elem))
        //                 .collect(),
        //         )
        //     }
        // }
    }

    pub fn member_eval_indexed_iter<'a>(
        &'a self,
        value: &'a __RegularValue,
    ) -> impl Iterator<Item = (i32, __RegularValue)> + 'a {
        let (start, end, index) = match self {
            HuskyDataViewer::Primitive { .. } => todo!(),
            HuskyDataViewer::Struct { .. } => todo!(),
            HuskyDataViewer::Vec { ilen, index, .. } => {
                let ilen = ilen
                    .call(None, &mut vec![value.temp_bind_leash()])
                    .downcast_i32();
                let index = index.bind(Binding::Leash);
                (0, ilen, index)
            }
            HuskyDataViewer::CyclicSlice {
                start, end, index, ..
            } => {
                let start = start
                    .call(None, &mut vec![value.temp_bind_leash()])
                    .downcast_i32();
                let end = end
                    .call(None, &mut vec![value.temp_bind_leash()])
                    .downcast_i32();
                let index = index.bind(Binding::Leash);
                (start, end, index)
            }
        };
        (start..end).into_iter().map(move |i| {
            (
                i,
                index.call(None, &mut vec![value.temp_bind_leash(), i.to_register()]),
            )
        })
    }

    pub fn member_temp_iter<'a>(
        &'a self,
        value: &'a __RegularValue,
    ) -> impl Iterator<Item = __RegularValue> + 'a {
        let (start, end, index) = match self {
            HuskyDataViewer::Primitive { .. } => todo!(),
            HuskyDataViewer::Struct { .. } => todo!(),
            HuskyDataViewer::Vec { ilen, index, .. } => {
                let ilen = ilen
                    .call(None, &mut vec![value.bind_temp_ref()])
                    .downcast_i32();
                let index = index.bind(Binding::TempRef);
                (0, ilen, index)
            }
            HuskyDataViewer::CyclicSlice {
                start, end, index, ..
            } => {
                let start = start
                    .call(None, &mut vec![value.bind_temp_ref()])
                    .downcast_i32();
                let end = end
                    .call(None, &mut vec![value.bind_temp_ref()])
                    .downcast_i32();
                let index = index.bind(Binding::TempRef);
                (start, end, index)
            }
        };
        (start..end)
            .into_iter()
            .map(move |i| index.call(None, &mut vec![value.bind_temp_ref(), i.to_register()]))
    }
}
