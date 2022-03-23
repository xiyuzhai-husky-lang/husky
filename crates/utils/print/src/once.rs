use std::sync::Once;

static ONCE: Once = Once::new();

pub fn do_once(f: impl FnOnce()) {
    ONCE.call_once(f)
}
