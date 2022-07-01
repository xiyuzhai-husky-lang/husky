#[macro_export]
macro_rules! lazy_entity_route {
    ($text: expr) => {{
        static mut MEMOIZED: Option<EntityRoutePtr> = None;
        unsafe {
            match MEMOIZED {
                Some(v) => v,
                None => {
                    let v = husky_eval_time::parse_entity_route($text);
                    // no need to worry about asynchronous access
                    // because the result would be the same
                    // I believe
                    MEMOIZED = Some(v);
                    v
                }
            }
        }
    }};
}
