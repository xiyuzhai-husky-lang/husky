
use husky_print_utils::p;
use husky_word::RootIdentifier;
use vm::*;

pub fn resolve_primitive_binary_opr_linkage(
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
            panic!("{:?} is not supported in Husky", (lopd_ty, opr, ropd_ty))
        }
    }
}
