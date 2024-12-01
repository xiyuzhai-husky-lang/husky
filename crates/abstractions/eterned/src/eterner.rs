use crate::*;
use std::sync::Mutex;

pub struct Interner {
    pool: Mutex<Pool<String, 1024>>,
}
