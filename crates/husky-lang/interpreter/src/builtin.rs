use crate::*;

use std::ops::BitAnd;
use std::ops::Shl;
use std::ops::Shr;

macro_rules! def {
    (pure $func:ident($($args:ident:$tys:ty), *) $body:expr)  => {
        any_func!(pure $func($($args:$tys), *) $body);
    };
}

// i32
def!(pure add_i32(a: i32, b: i32) a + b);
def!(pure sub_i32(a: i32, b: i32) a - b);
def!(pure mult_i32(a: i32, b: i32) a * b);
def!(pure div_i32(a: i32, b: i32) a / b);
def!(pure greater_i32(a: i32, b: i32) a > b);
def!(pure geq_i32(a: i32, b: i32) a > b);
def!(pure less_i32(a: i32, b: i32) a < b);
def!(pure leq_i32(a: i32, b: i32) a < b);
def!(pure eq_i32(a: i32, b: i32)  a == b);
def!(pure neq_i32(a: i32, b: i32) a != b);

// f32
def!(pure add_f32(a: f32, b: f32) a + b);
def!(pure sub_f32(a: f32, b: f32) a - b);
def!(pure mult_f32(a: f32, b: f32) a * b);
def!(pure div_f32(a: f32, b: f32) a / b);
def!(pure greater_f32(a: f32, b: f32) a > b);
def!(pure geq_f32(a: f32, b: f32) a > b);
def!(pure less_f32(a: f32, b: f32) a < b);
def!(pure leq_f32(a: f32, b: f32) a < b);
def!(pure eq_f32(a: f32, b: f32) a == b);
def!(pure neq_f32(a: f32, b: f32) a != b);

// u32
def!(pure bitor_u32(a: u32, b: u32) a | b);
def!(pure bitxor_u32(a: u32, b: u32) a ^ b);
def!(pure bitand_u32(a: u32, b: u32) a.bitand(b));
def!(pure lshift_u32(a: u32, b: i32) a.shl(b));
def!(pure rshift_u32(a: u32, b: i32) a.shr(b));
