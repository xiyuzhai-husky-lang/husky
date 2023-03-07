mod db;

pub use db::*;
use husky_term::Term;
use husky_vm_binding::Binding;
use husky_word::{IdentPairMap, Identifier};

use husky_vm_interface::{__Linkage, __Register, __RegistrableSafe, __ResolvedLinkage};

#[derive(Debug, PartialEq, Eq)]
pub enum HuskyDataViewer {
    Primitive {
        ty: Identifier,
    },
    Struct {
        fields: IdentPairMap<(__Linkage, Term)>,
    },
    Vec {
        ilen: __ResolvedLinkage,
        index: __Linkage,
        elem_ty: Term,
    },
    CyclicSlice {
        start: __ResolvedLinkage,
        end: __ResolvedLinkage,
        index: __Linkage,
        elem_ty: Term,
    },
}

impl HuskyDataViewer {
    pub fn print<'eval>(&self, _value: &__Register<'eval>) -> String {
        todo!()
    }

    pub fn serialize<'eval>(
        &self,
        _db: &dyn DataViewerDb,
        _value: &__Register<'eval>,
    ) -> serde_json::Value {
        todo!()
        // match self {
        //     HuskyDataViewer::Primitive { ty } => match ty {
        //         RootBuiltinIdentifier::Void => todo!(),
        //         RootBuiltinIdentifier::I32 => todo!(),
        //         RootBuiltinIdentifier::I64 => todo!(),
        //         RootBuiltinIdentifier::F32 => serde_json::to_value(value.downcast_f32()).unwrap(),
        //         RootBuiltinIdentifier::F64 => todo!(),
        //         RootBuiltinIdentifier::B32 => todo!(),
        //         RootBuiltinIdentifier::B64 => todo!(),
        //         RootBuiltinIdentifier::Bool => todo!(),
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

    pub fn member_eval_indexed_iter<'a, 'eval>(
        &'a self,
        value: &'a __Register<'eval>,
    ) -> impl Iterator<Item = (i32, __Register<'eval>)> + 'a {
        let (start, end, index) = match self {
            HuskyDataViewer::Primitive { .. } => todo!(),
            HuskyDataViewer::Struct { .. } => todo!(),
            HuskyDataViewer::Vec { ilen, index, .. } => {
                let ilen = ilen
                    .call(None, &mut vec![value.temp_bind_eval_ref()])
                    .downcast_i32();
                let index = index.bind(Binding::Leash);
                (0, ilen, index)
            }
            HuskyDataViewer::CyclicSlice {
                start, end, index, ..
            } => {
                let start = start
                    .call(None, &mut vec![value.temp_bind_eval_ref()])
                    .downcast_i32();
                let end = end
                    .call(None, &mut vec![value.temp_bind_eval_ref()])
                    .downcast_i32();
                let index = index.bind(Binding::Leash);
                (start, end, index)
            }
        };
        (start..end).into_iter().map(move |i| {
            (
                i,
                index.call(None, &mut vec![value.temp_bind_eval_ref(), i.to_register()]),
            )
        })
    }

    pub fn member_temp_iter<'a, 'eval>(
        &'a self,
        value: &'a __Register<'eval>,
    ) -> impl Iterator<Item = __Register<'eval>> + 'a {
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
