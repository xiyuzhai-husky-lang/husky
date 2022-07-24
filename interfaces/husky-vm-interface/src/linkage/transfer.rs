#[macro_export]
macro_rules! transfer_linkage {
    ($fp: expr, some $raw_fp: expr) => {{
        __Linkage::Transfer(__LinkageFp {
            fp: $fp,
            opt_raw_fp: Some($raw_fp as *const ()),
        })
    }};
    ($fp: expr, none) => {{
        __Linkage::Transfer(__LinkageFp {
            fp: $fp,
            opt_raw_fp: None,
        })
    }};
}
