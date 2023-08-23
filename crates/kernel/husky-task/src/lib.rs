use std::panic::{RefUnwindSafe, UnwindSafe};

pub trait Task: RefUnwindSafe + UnwindSafe {}
