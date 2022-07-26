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
    match (lopd_ty, opr, ropd_ty) {
        (F32, Greater, F32) => transfer_linkage!(
            |_,arguments| unsafe {
                (arguments[0].downcast_f32() > arguments[1].downcast_f32()).to_register()
            },
            some f32::gt
        ),
        _ => {
            p!((lopd_ty, opr, ropd_ty));
            todo!()
        }
    }
}
