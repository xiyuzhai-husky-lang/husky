#[macro_export]
macro_rules! lazy_entity_route {
    ($text: expr) => {{
        static mut MEMOIZED_VALUE: Option<EntityRoutePtr> = None;
        unsafe {
            match MEMOIZED_VALUE {
                Some(v) => v,
                None => {
                    let v = parse_entity_route($text);
                    // no need to worry about asynchronous access
                    // because the result would be the same
                    // I believe
                    MEMOIZED_VALUE = Some(v);
                    v
                }
            }
        }
    }};
}
