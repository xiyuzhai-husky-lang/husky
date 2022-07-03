#[macro_export]
macro_rules! lazy_entity_route_from_text {
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

#[macro_export]
macro_rules! __lazy_entity_route_from_text {
    ($text: expr) => {{
        static mut MEMOIZED: Option<__EntityRoutePtr> = None;
        unsafe {
            match MEMOIZED {
                Some(v) => v,
                None => {
                    let v = __parse_entity_route($text);
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

#[macro_export]
macro_rules! lazy_entity_route {
    ($v: expr) => {{
        static mut MEMOIZED: Option<EntityRoutePtr> = None;
        unsafe {
            match MEMOIZED {
                Some(v) => v,
                None => {
                    let v = $v;
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
