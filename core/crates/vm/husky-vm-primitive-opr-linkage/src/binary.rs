
use husky_print_utils::p;
use husky_word::RootIdentifier;
use vm::*;

pub fn resolve_primitive_pure_binary_opr_linkage(
    lopd_ty: RootIdentifier,
    opr: PureBinaryOpr,
    ropd_ty: RootIdentifier,
) -> __Linkage {
    use PureBinaryOpr::*;
    use RootIdentifier::*;
    type b32 = u32;
    type b64 = u64;

    match (lopd_ty, opr, ropd_ty) {
        (I32, Eq, I32) => transfer_linkage!(
            |_,arguments| unsafe {
                (arguments[0].downcast_i32() == arguments[1].downcast_i32()).to_register()
            },
            none
        ),
        (I32, Greater, I32) => transfer_linkage!(
            |_,arguments| unsafe {
                (arguments[0].downcast_i32() > arguments[1].downcast_i32()).to_register()
            },
            none
        ),
        (I32, Geq, I32) => transfer_linkage!(
            |_,arguments| unsafe {
                (arguments[0].downcast_i32() >= arguments[1].downcast_i32()).to_register()
            },
            none
        ),
        (I32, Less, I32) => transfer_linkage!(
            |_,arguments| unsafe {
                (arguments[0].downcast_i32() < arguments[1].downcast_i32()).to_register()
            },
            none
        ),
        (I32, Leq, I32) => transfer_linkage!(
            |_,arguments| unsafe {
                (arguments[0].downcast_i32() <= arguments[1].downcast_i32()).to_register()
            },
            none
        ),
        (F32, Greater, F32) => transfer_linkage!(
            |_,arguments| unsafe {
                (arguments[0].downcast_f32() > arguments[1].downcast_f32()).to_register()
            },
            none
        ),
        (F32, Geq, F32) => transfer_linkage!(
            |_,arguments| unsafe {
                (arguments[0].downcast_f32() >= arguments[1].downcast_f32()).to_register()
            },
            none
        ),
        (F32, Less, F32) => transfer_linkage!(
            |_,arguments| unsafe {
                (arguments[0].downcast_f32() < arguments[1].downcast_f32()).to_register()
            },
            none
        ),
        (F32, Leq, F32) => transfer_linkage!(
            |_,arguments| unsafe {
                (arguments[0].downcast_f32() <= arguments[1].downcast_f32()).to_register()
            },
            none
        ),
        _ => {
            panic!("Binary operation {:?} is not supported in Husky", (lopd_ty, opr, ropd_ty))
        }
    }
}


pub fn resolve_primitive_assign_binary_opr_linkage(
    lopd_ty: RootIdentifier,
    opt_opr: Option<PureBinaryOpr>,
    ropd_ty: RootIdentifier,
) -> __Linkage {
    use PureBinaryOpr::*;
    use RootIdentifier::*;
    type b32 = u32;
    type b64 = u64;

    match (lopd_ty, opt_opr, ropd_ty) {
            (I32, Some(Add), I32) => transfer_linkage!(
                |_,arguments| unsafe {
                    let new_value: i32 = (arguments[0].downcast_i32() + arguments[1].downcast_i32());
                    *arguments[0].downcast_temp_mut::<i32>() = new_value;
                    __Register::new_void()
                },
                none
            ),
            (I32, Some(Sub), I32) => transfer_linkage!(
                |_,arguments| unsafe {
                    let new_value: i32 = (arguments[0].downcast_i32() - arguments[1].downcast_i32());
                    *arguments[0].downcast_temp_mut::<i32>() = new_value;
                    __Register::new_void()
                },
                none
            ),
            (B32, None, B32) => transfer_linkage!(
                |_,arguments| unsafe {
                    *arguments[0].downcast_temp_mut::<b32>() = arguments[1].downcast_b32();
                    __Register::new_void()
                },
                none
            ),
        _ => {
            panic!("Assign operation {:?} is not supported in Husky", (lopd_ty, opt_opr, ropd_ty))
        }
    }
}
