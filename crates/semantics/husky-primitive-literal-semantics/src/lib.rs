use husky_entity_route::EntityRouteItd;
use husky_primitive_literal_syntax::RawLiteralData;
use husky_vm_interface::{__Register, __RegistrableSafe};
use husky_vm_primitive_value::PrimitiveValueData;
use husky_word::RootBuiltinIdentifier;

pub fn convert_primitive_literal_to_value(
    literal: RawLiteralData,
    ty: EntityRouteItd,
) -> PrimitiveValueData {
    match literal {
        RawLiteralData::Unit => todo!(),
        RawLiteralData::Integer(i) => match ty {
            EntityRouteItd::Root(root_identifier) => match root_identifier {
                RootBuiltinIdentifier::I32 => PrimitiveValueData::I32(i as i32),
                RootBuiltinIdentifier::I64 => PrimitiveValueData::I64(i as i64),
                RootBuiltinIdentifier::B32 => PrimitiveValueData::B32(i as u32),
                RootBuiltinIdentifier::B64 => PrimitiveValueData::B64(i as u64),
                _ => panic!(),
            },
            EntityRouteItd::Custom(_) => todo!(),
        },
        RawLiteralData::I32(_) => todo!(),
        RawLiteralData::I64(_) => todo!(),
        RawLiteralData::Float(f) => match ty {
            EntityRouteItd::Root(root_identifier) => match root_identifier {
                RootBuiltinIdentifier::F32 => PrimitiveValueData::F32(f.0 as f32),
                RootBuiltinIdentifier::F64 => todo!(),
                _ => panic!(),
            },
            EntityRouteItd::Custom(_) => todo!(),
        },
        RawLiteralData::F32(_) => todo!(),
        RawLiteralData::F64(_) => todo!(),
        RawLiteralData::Bits(_) => todo!(),
        RawLiteralData::B32(b) => PrimitiveValueData::B32(b),
        RawLiteralData::B64(_) => todo!(),
        RawLiteralData::Bool(b) => PrimitiveValueData::Bool(b),
    }
}

pub fn convert_primitive_literal_to_register(
    literal: RawLiteralData,
    ty: EntityRouteItd,
) -> __Register<'static> {
    // literal is guaranteed to be not nan
    match literal {
        RawLiteralData::Unit => todo!(),
        RawLiteralData::Integer(i) => match ty {
            EntityRouteItd::Root(root_identifier) => match root_identifier {
                RootBuiltinIdentifier::I32 => (i as i32).to_register(),
                RootBuiltinIdentifier::I64 => (i as i64).to_register(),
                RootBuiltinIdentifier::B32 => (i as u32).to_register(),
                RootBuiltinIdentifier::B64 => (i as u64).to_register(),
                _ => panic!(),
            },
            EntityRouteItd::Custom(_) => todo!(),
        },
        RawLiteralData::I32(_) => todo!(),
        RawLiteralData::I64(_) => todo!(),
        RawLiteralData::Float(f) => match ty {
            EntityRouteItd::Root(root_identifier) => match root_identifier {
                RootBuiltinIdentifier::F32 => (f.0 as f32).to_register(),
                RootBuiltinIdentifier::F64 => todo!(),
                _ => panic!(),
            },
            EntityRouteItd::Custom(_) => todo!(),
        },
        RawLiteralData::F32(_) => todo!(),
        RawLiteralData::F64(_) => todo!(),
        RawLiteralData::Bits(_) => todo!(),
        RawLiteralData::B32(b) => b.to_register(),
        RawLiteralData::B64(_) => todo!(),
        RawLiteralData::Bool(b) => b.to_register(),
    }
}
