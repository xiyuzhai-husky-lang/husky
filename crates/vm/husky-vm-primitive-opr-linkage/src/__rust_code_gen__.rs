use super::*;
use husky_word::RootBuiltinIdentifier;
use husky_vm_interface::*;
use husky_opn_syntax::*;

pub fn resolve_primitive_pure_binary_opr_linkage(
    lopd_ty: RootBuiltinIdentifier,
    opr: PureBinaryOpr,
    ropd_ty: RootBuiltinIdentifier,
) -> __Linkage {
    use PureBinaryOpr::*;
    use RootBuiltinIdentifier::*;
    type b32 = u32;
    type b64 = u64;

    match (lopd_ty, opr, ropd_ty) {
        (I32, Add, I32) => transfer_linkage!(
            |arguments, _| (arguments[0].downcast_i32() + arguments[1].downcast_i32()).to_register(),
            none
        ),
        (I32, Div, I32) => transfer_linkage!(
            |arguments, _| (arguments[0].downcast_i32() / arguments[1].downcast_i32()).to_register(),
            none
        ),
        (I32, Eq, I32) => transfer_linkage!(
            |arguments, _| (arguments[0].downcast_i32() == arguments[1].downcast_i32()).to_register(),
            none
        ),
        (I32, Greater, I32) => transfer_linkage!(
            |arguments, _| (arguments[0].downcast_i32() > arguments[1].downcast_i32()).to_register(),
            none
        ),
        (I32, Geq, I32) => transfer_linkage!(
            |arguments, _| (arguments[0].downcast_i32() >= arguments[1].downcast_i32()).to_register(),
            none
        ),
        (I32, Less, I32) => transfer_linkage!(
            |arguments, _| (arguments[0].downcast_i32() < arguments[1].downcast_i32()).to_register(),
            none
        ),
        (I32, Leq, I32) => transfer_linkage!(
            |arguments, _| (arguments[0].downcast_i32() <= arguments[1].downcast_i32()).to_register(),
            none
        ),
        (I32, Mul, I32) => transfer_linkage!(
            |arguments, _| (arguments[0].downcast_i32() * arguments[1].downcast_i32()).to_register(),
            none
        ),
        (I32, Neq, I32) => transfer_linkage!(
            |arguments, _| (arguments[0].downcast_i32() != arguments[1].downcast_i32()).to_register(),
            none
        ),
        (I32, Sub, I32) => transfer_linkage!(
            |arguments, _| (arguments[0].downcast_i32() - arguments[1].downcast_i32()).to_register(),
            none
        ),
        (B32, BitAnd, B32) => transfer_linkage!(
            |arguments, _| (arguments[0].downcast_b32() & arguments[1].downcast_b32()).to_register(),
            none
        ),
        (B32, BitOr, B32) => transfer_linkage!(
            |arguments, _| (arguments[0].downcast_b32() | arguments[1].downcast_b32()).to_register(),
            none
        ),
        (B32, Eq, B32) => transfer_linkage!(
            |arguments, _| (arguments[0].downcast_b32() == arguments[1].downcast_b32()).to_register(),
            none
        ),
        (B32, Neq, B32) => transfer_linkage!(
            |arguments, _| (arguments[0].downcast_b32() != arguments[1].downcast_b32()).to_register(),
            none
        ),
        (B32, Shl, I32) => transfer_linkage!(
            |arguments, _| (arguments[0].downcast_b32() << arguments[1].downcast_i32()).to_register(),
            none
        ),
        (B32, Shr, I32) => transfer_linkage!(
            |arguments, _| (arguments[0].downcast_b32() >> arguments[1].downcast_i32()).to_register(),
            none
        ),
        (F32, Add, F32) => transfer_linkage!(
            |arguments, _| (arguments[0].downcast_f32() + arguments[1].downcast_f32()).to_register(),
            none
        ),
        (F32, Div, F32) => transfer_linkage!(
            |arguments, _| (arguments[0].downcast_f32() / arguments[1].downcast_f32()).to_register(),
            none
        ),
        (F32, Eq, F32) => transfer_linkage!(
            |arguments, _| (arguments[0].downcast_f32() == arguments[1].downcast_f32()).to_register(),
            none
        ),
        (F32, Greater, F32) => transfer_linkage!(
            |arguments, _| (arguments[0].downcast_f32() > arguments[1].downcast_f32()).to_register(),
            none
        ),
        (F32, Geq, F32) => transfer_linkage!(
            |arguments, _| (arguments[0].downcast_f32() >= arguments[1].downcast_f32()).to_register(),
            none
        ),
        (F32, Less, F32) => transfer_linkage!(
            |arguments, _| (arguments[0].downcast_f32() < arguments[1].downcast_f32()).to_register(),
            none
        ),
        (F32, Leq, F32) => transfer_linkage!(
            |arguments, _| (arguments[0].downcast_f32() <= arguments[1].downcast_f32()).to_register(),
            none
        ),
        (F32, Mul, F32) => transfer_linkage!(
            |arguments, _| (arguments[0].downcast_f32() * arguments[1].downcast_f32()).to_register(),
            none
        ),
        (F32, Sub, F32) => transfer_linkage!(
            |arguments, _| (arguments[0].downcast_f32() - arguments[1].downcast_f32()).to_register(),
            none
        ),
        (Bool, And, Bool) => transfer_linkage!(
            |arguments, _| (arguments[0].downcast_bool() && arguments[1].downcast_bool()).to_register(),
            none
        ),
        (I32, Power, I32) => transfer_linkage!(
            |arguments, _| {
                num::pow(arguments[0].downcast_i32(), arguments[1].downcast_i32() as usize).to_register()
            },
            none
        ),
        (I32, RemEuclid, I32) => transfer_linkage!(
            |arguments, _| {
                let dividend = arguments[0].downcast_i32();
                let divisor = arguments[1].downcast_i32();
                dividend.rem_euclid(divisor).to_register()
            },
            none
        ),
        _ => {
            panic!("Binary operation {:?} is not supported in Husky", (lopd_ty, opr, ropd_ty))
        }
    }
}


pub fn resolve_primitive_assign_binary_opr_linkage(
    lopd_ty: RootBuiltinIdentifier,
    opt_opr: Option<PureBinaryOpr>,
    ropd_ty: RootBuiltinIdentifier,
) -> __Linkage {
    use PureBinaryOpr::*;
    use RootBuiltinIdentifier::*;
    type b32 = u32;
    type b64 = u64;

    match (lopd_ty, opt_opr, ropd_ty) {
            (Bool, None, Bool) => transfer_linkage!(
                |arguments, _| unsafe {
                    *arguments[0].downcast_temp_mut::<bool>(&__BOOL_VTABLE) = arguments[1].downcast_bool();
                    __Register::new_void()
                },
                none
            ),
            (I32, None, I32) => transfer_linkage!(
                |arguments, _| unsafe {
                    *arguments[0].downcast_temp_mut::<i32>(&__I32_VTABLE) = arguments[1].downcast_i32();
                    __Register::new_void()
                },
                none
            ),
            (I32, Some(Add), I32) => transfer_linkage!(
                |arguments, _| unsafe {
                    let new_value: i32 = arguments[0].downcast_i32() + arguments[1].downcast_i32();
                    *arguments[0].downcast_temp_mut::<i32>(&__I32_VTABLE) = new_value;
                    __Register::new_void()
                },
                none
            ),
            (I32, Some(Sub), I32) => transfer_linkage!(
                |arguments, _| unsafe {
                    let new_value: i32 = arguments[0].downcast_i32() - arguments[1].downcast_i32();
                    *arguments[0].downcast_temp_mut::<i32>(&__I32_VTABLE) = new_value;
                    __Register::new_void()
                },
                none
            ),
            (B32, None, B32) => transfer_linkage!(
                |arguments, _| unsafe {
                    *arguments[0].downcast_temp_mut::<b32>(&__B32_VTABLE) = arguments[1].downcast_b32();
                    __Register::new_void()
                },
                none
            ),
            (B32, Some(BitAnd), B32) => transfer_linkage!(
                |arguments, _| unsafe {
                    let new_value: b32 = arguments[0].downcast_b32() & arguments[1].downcast_b32();
                    *arguments[0].downcast_temp_mut::<b32>(&__B32_VTABLE) = new_value;
                    __Register::new_void()
                },
                none
            ),
            (B32, Some(BitOr), B32) => transfer_linkage!(
                |arguments, _| unsafe {
                    let new_value: b32 = arguments[0].downcast_b32() | arguments[1].downcast_b32();
                    *arguments[0].downcast_temp_mut::<b32>(&__B32_VTABLE) = new_value;
                    __Register::new_void()
                },
                none
            ),
            (F32, Some(Add), F32) => transfer_linkage!(
                |arguments, _| unsafe {
                    let new_value: f32 = arguments[0].downcast_f32() + arguments[1].downcast_f32();
                    *arguments[0].downcast_temp_mut::<f32>(&__F32_VTABLE) = new_value;
                    __Register::new_void()
                },
                none
            ),
            (F32, Some(Sub), F32) => transfer_linkage!(
                |arguments, _| unsafe {
                    let new_value: f32 = arguments[0].downcast_f32() - arguments[1].downcast_f32();
                    *arguments[0].downcast_temp_mut::<f32>(&__F32_VTABLE) = new_value;
                    __Register::new_void()
                },
                none
            ),
            (F32, None, F32) => transfer_linkage!(
                |arguments, _| unsafe {
                    *arguments[0].downcast_temp_mut::<f32>(&__F32_VTABLE) = arguments[1].downcast_f32();
                    __Register::new_void()
                },
                none
            ),
        _ => {
            panic!("Assign operation {:?} is not supported in Husky", (lopd_ty, opt_opr, ropd_ty))
        }
    }
}
