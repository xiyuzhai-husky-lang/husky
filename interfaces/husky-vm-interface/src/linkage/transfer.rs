#[macro_export]
macro_rules! transfer_linkage {
    ($wrapper: expr, some $raw_fp: expr) => {{
        __Linkage::Transfer(__LinkageFp {
            wrapper: $wrapper,
            opt_fp: Some($raw_fp as *const ()),
        })
    }};
    ($wrapper: expr, none) => {{
        __Linkage::Transfer(__LinkageFp {
            wrapper: $wrapper,
            opt_fp: None,
        })
    }};
}
