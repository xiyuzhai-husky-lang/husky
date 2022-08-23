use husky_entity_route::EntityRoutePtr;
use husky_primitive_literal_syntax::PrimitiveLiteralData;
use husky_vm_interface::{__Register, __RegistrableSafe};
use husky_vm_primitive_value::PrimitiveValueData;
use husky_word::RootIdentifier;

pub fn convert_primitive_literal_to_value(
    literal: PrimitiveLiteralData,
    ty: EntityRoutePtr,
) -> PrimitiveValueData {
    match literal {
        PrimitiveLiteralData::Void => todo!(),
        PrimitiveLiteralData::Integer(i) => match ty {
            EntityRoutePtr::Root(root_identifier) => match root_identifier {
                RootIdentifier::I32 => PrimitiveValueData::I32(i as i32),
                RootIdentifier::I64 => PrimitiveValueData::I64(i as i64),
                RootIdentifier::B32 => PrimitiveValueData::B32(i as u32),
                RootIdentifier::B64 => PrimitiveValueData::B64(i as u64),
                _ => panic!(),
            },
            EntityRoutePtr::Custom(_) => todo!(),
        },
        PrimitiveLiteralData::I32(_) => todo!(),
        PrimitiveLiteralData::I64(_) => todo!(),
        PrimitiveLiteralData::Float(f) => match ty {
            EntityRoutePtr::Root(root_identifier) => match root_identifier {
                RootIdentifier::F32 => PrimitiveValueData::F32(f.0 as f32),
                RootIdentifier::F64 => todo!(),
                _ => panic!(),
            },
            EntityRoutePtr::Custom(_) => todo!(),
        },
        PrimitiveLiteralData::F32(_) => todo!(),
        PrimitiveLiteralData::F64(_) => todo!(),
        PrimitiveLiteralData::Bits(_) => todo!(),
        PrimitiveLiteralData::B32(b) => PrimitiveValueData::B32(b),
        PrimitiveLiteralData::B64(_) => todo!(),
        PrimitiveLiteralData::Bool(b) => PrimitiveValueData::Bool(b),
    }
}

pub fn convert_primitive_literal_to_register(
    literal: PrimitiveLiteralData,
    ty: EntityRoutePtr,
) -> __Register<'static> {
    match literal {
        PrimitiveLiteralData::Void => todo!(),
        PrimitiveLiteralData::Integer(i) => match ty {
            EntityRoutePtr::Root(root_identifier) => match root_identifier {
                RootIdentifier::I32 => (i as i32).to_register(),
                RootIdentifier::I64 => (i as i64).to_register(),
                RootIdentifier::B32 => (i as u32).to_register(),
                RootIdentifier::B64 => (i as u64).to_register(),
                _ => panic!(),
            },
            EntityRoutePtr::Custom(_) => todo!(),
        },
        PrimitiveLiteralData::I32(_) => todo!(),
        PrimitiveLiteralData::I64(_) => todo!(),
        PrimitiveLiteralData::Float(f) => match ty {
            EntityRoutePtr::Root(root_identifier) => match root_identifier {
                RootIdentifier::F32 => (f.0 as f32).to_register(),
                RootIdentifier::F64 => todo!(),
                _ => panic!(),
            },
            EntityRoutePtr::Custom(_) => todo!(),
        },
        PrimitiveLiteralData::F32(_) => todo!(),
        PrimitiveLiteralData::F64(_) => todo!(),
        PrimitiveLiteralData::Bits(_) => todo!(),
        PrimitiveLiteralData::B32(b) => b.to_register(),
        PrimitiveLiteralData::B64(_) => todo!(),
        PrimitiveLiteralData::Bool(b) => b.to_register(),
    }
}
