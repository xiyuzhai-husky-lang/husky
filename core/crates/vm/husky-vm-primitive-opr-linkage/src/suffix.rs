use husky_opn_syntax::RawSuffixOpr;
use husky_word::RootIdentifier;
use vm::__Linkage;

pub fn resolve_primitive_prefix_opr_linkage(
    opr: RawSuffixOpr,
    this_ty: RootIdentifier,
) -> __Linkage {
    match opr {
        RawSuffixOpr::Incr => todo!(),
        RawSuffixOpr::Decr => todo!(),
        RawSuffixOpr::AsTy(_) => todo!(),
        RawSuffixOpr::BePattern(_) => panic!(),
    }
}
