use super::*;

pub static VEC_CYCLIC_SLICE: EntityStaticDefn = EntityStaticDefn {
    name: "cyclic_slice",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Method {
        this_contract: InputLiason::GlobalRef,
        input_parameters: &[],
        output_ty: "[%]E",
        generic_parameters: &[],
        kind: MethodStaticDefnVariant::TypeMethod {
            source: LinkageSource::Transfer(Linkage {
                call: |values| todo!(),
                nargs: 1,
            }),
        },
        output_liason: OutputLiason::MemberAccess {
            member_liason: MemberLiason::Mutable,
        },
    },
    dev_src: static_dev_src!(),
};
