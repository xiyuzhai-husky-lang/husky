#[macro_export]
macro_rules! transfer_linkage {
    ($wrapper: expr, some $raw_fp: expr) => {{
        __Linkage::Transfer(linkage_fp!($wrapper, some $raw_fp))
    }};
    ($wrapper: expr, none) => {{
        __Linkage::Transfer(linkage_fp!($wrapper, none))
    }};
}
