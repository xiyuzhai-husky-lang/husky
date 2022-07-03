#[macro_export]
macro_rules! field_linkage {
    ($Type: ty, $field: ident) => {{
        __Linkage::Member(&__MemberLinkage {
            copy_access: field_copy_fp!($Type, $field),
            eval_ref_access: __SpecificRoutineFp(|values| todo!()),
            temp_ref_access: __SpecificRoutineFp(|values| todo!()),
            temp_mut_access: __SpecificRoutineFp(|values| todo!()),
            move_access: __SpecificRoutineFp(|values| todo!()),
            nargs: 1,
            dev_src: __static_dev_src!(),
        })
    }};
}
#[macro_export]
macro_rules! index_linkage {
    ($Type: ty) => {{
        __Linkage::Member(&__MemberLinkage {
            copy_access: index_copy_fp!($Type),
            eval_ref_access: index_eval_ref_fp!($Type),
            temp_ref_access: index_temp_ref_fp!($Type),
            temp_mut_access: index_temp_mut_fp!($Type),
            move_access: index_move_fp!($Type),
            nargs: 1,
            dev_src: __static_dev_src!(),
        })
    }};
}
