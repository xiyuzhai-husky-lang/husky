use std::{
    any::Any,
    panic::{catch_unwind, UnwindSafe},
};

pub fn catch_unwind_with_message<F, R>(f: F) -> Result<R, String>
where
    F: FnOnce() -> R + UnwindSafe,
{
    catch_unwind(f).map_err(|e| resolve_error(e))
}

fn resolve_error(e: Box<dyn Any + Send>) -> String {
    if let Some(_s) = e.downcast_ref::<String>() {
        return *(unsafe { e.downcast().unwrap_unchecked() });
    }
    if let Some(s) = e.downcast_ref::<&str>() {
        return s.to_string();
    }
    todo!()
}
