use husky_rust_code_repr::{registration::*, *};
use husky_write_utils::w;
use std::io::prelude::*;
use std::path::PathBuf;
use std::{env, fs::File};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    write_rust_code("src/__rust_code_gen__.rs").unwrap();
}

pub fn write_rust_code(rust_path: &str) -> std::io::Result<()> {
    let mut f = File::create(rust_path).expect(&format!("rust path {rust_path} doesn't exist"));
    w!(f; BuildCodeGenStart);
    for ty in PRIMITIVE_TYPES {
        w!(f; RootPrimitiveTypeRegistration { ty })
    }
    for ty in NONPRIMITIVE_BUILTIN_TYPES {
        w!(f; NonPrimitiveTypeRegistration { ty })
    }
    for nargs in 0..10 {
        w!(f; ImplFp { nargs })
    }
    Ok(())
}

pub static PRIMITIVE_TYPES: &'static [&'static str] =
    &["void", "bool", "i32", "i64", "b32", "b64", "f32", "f64"];

pub static NONPRIMITIVE_BUILTIN_TYPES: &'static [&'static str] =
    &["__VirtualFunction", "__VirtualEnum"];

pub struct ImplFp {
    nargs: usize,
}

impl std::fmt::Display for ImplFp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use itertools::Itertools;

        let arg_types = (0..self.nargs)
            .into_iter()
            .map(|i| -> String { format!("A{i}") })
            .join(", ");
        let opt_comma = if self.nargs > 0 { ", " } else { "" };
        f.write_fmt(format_args!(
            r#"

#[rustfmt::skip]
impl<'eval, {arg_types}{opt_comma}Output> ThinFp<'eval>
    for fn({arg_types}) -> Output {{}}

impl<'eval, {arg_types}{opt_comma}Output> ThinFp<'eval>
    for fn(&dyn __EvalContext<'eval>, {arg_types}) -> Output {{}}

#[rustfmt::skip]
impl<'eval, {arg_types}{opt_comma}Output> BaseFp<'eval>
    for fn({arg_types}) -> Output
{{
    type WithContext = fn(&dyn __EvalContext<'eval>, {arg_types}) -> Output;
}}"#,
        ))
    }
}
