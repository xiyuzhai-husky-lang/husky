use husky_opn_syntax::*;
use husky_word::RootBuiltinIdentifier;
use std::fmt::Write;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let path: PathBuf = "src/__rust_code_gen__.rs".into();
    husky_io_utils::diff_write(&path, &gen_rust_code().unwrap(), true);
}

pub fn gen_rust_code() -> Result<String, std::fmt::Error> {
    let mut code = String::new();
    write!(
        code,
        r#"use super::*;
use husky_word::RootBuiltinIdentifier;
use husky_vm_interface::*;
use husky_opn_syntax::*;

pub fn resolve_primitive_pure_binary_opr_linkage(
    lopd_ty: RootBuiltinIdentifier,
    opr: PureBinaryOpr,
    ropd_ty: RootBuiltinIdentifier,
) -> __Linkage {{
    use PureBinaryOpr::*;
    use RootBuiltinIdentifier::*;
    type b32 = u32;
    type b64 = u64;

    match (lopd_ty, opr, ropd_ty) {{"#
    )?;
    use PureBinaryOpr::*;
    use RootBuiltinIdentifier::*;
    static SUPPORTED_PURE_BINARY_OPNS: &'static [(
        RootBuiltinIdentifier,
        PureBinaryOpr,
        RootBuiltinIdentifier,
    )] = &[
        // bool is not supported due to short-circuiting
        // i32
        (I32, Add, I32),
        (I32, Div, I32),
        (I32, Eq, I32),
        (I32, Greater, I32),
        (I32, Geq, I32),
        (I32, Less, I32),
        (I32, Leq, I32),
        (I32, Mul, I32),
        (I32, Neq, I32),
        (I32, Sub, I32),
        // b32
        (B32, BitAnd, B32),
        (B32, BitOr, B32),
        (B32, Eq, B32),
        (B32, Neq, B32),
        (B32, Shl, I32),
        (B32, Shr, I32),
        // f32
        (F32, Add, F32),
        (F32, Div, F32),
        (F32, Eq, F32),
        (F32, Greater, F32),
        (F32, Geq, F32),
        (F32, Less, F32),
        (F32, Leq, F32),
        (F32, Mul, F32),
        (F32, Sub, F32),
        // bool
        (Bool, And, Bool),
    ];
    for (lopd_ty_ident, opr, ropd_ty_ident) in SUPPORTED_PURE_BINARY_OPNS {
        let lopd_ty_husky_name = lopd_ty_ident.as_str();
        let ropd_ty_husky_name = ropd_ty_ident.as_str();
        let opr_code = opr.husky_code();
        // let rust_trait_method_name = opr.rust_trait_method_name();
        write!(
            code,
            r#"
        ({lopd_ty_ident:?}, {opr:?}, {ropd_ty_ident:?}) => transfer_linkage!(
            |arguments, _| (arguments[0].downcast_{lopd_ty_husky_name}() {opr_code} arguments[1].downcast_{ropd_ty_husky_name}()).to_register(),
            none
        ),"#
        )?
    }
    write!(
        code,
        r#"
        (I32, Power, I32) => transfer_linkage!(
            |arguments, _| {{
                num::pow(arguments[0].downcast_i32(), arguments[1].downcast_i32() as usize).to_register()
            }},
            none
        ),
        (I32, RemEuclid, I32) => transfer_linkage!(
            |arguments, _| {{
                let dividend = arguments[0].downcast_i32();
                let divisor = arguments[1].downcast_i32();
                dividend.rem_euclid(divisor).to_register()
            }},
            none
        ),
        _ => {{
            panic!("Binary operation {{:?}} is not supported in Husky", (lopd_ty, opr, ropd_ty))
        }}
    }}
}}
"#
    )?;

    static SUPPORTED_ASSIGN_BINARY_OPNS: &'static [(
        RootBuiltinIdentifier,
        std::option::Option<PureBinaryOpr>,
        RootBuiltinIdentifier,
    )] = &[
        // bool
        (Bool, None, Bool),
        // i32
        (I32, None, I32),
        (I32, Some(Add), I32),
        (I32, Some(Sub), I32),
        // b32
        (B32, None, B32),
        (B32, Some(BitAnd), B32),
        (B32, Some(BitOr), B32),
        // f32
        (F32, Some(Add), F32),
        (F32, Some(Sub), F32),
        (F32, None, F32),
    ];
    write!(
        code,
        r#"

pub fn resolve_primitive_assign_binary_opr_linkage(
    lopd_ty: RootBuiltinIdentifier,
    opt_opr: Option<PureBinaryOpr>,
    ropd_ty: RootBuiltinIdentifier,
) -> __Linkage {{
    use PureBinaryOpr::*;
    use RootBuiltinIdentifier::*;
    type b32 = u32;
    type b64 = u64;

    match (lopd_ty, opt_opr, ropd_ty) {{"#
    )?;
    for (lopd_ty_ident, opt_opr, ropd_ty_ident) in SUPPORTED_ASSIGN_BINARY_OPNS {
        let lopd_ty_husky_name = lopd_ty_ident.as_str();
        let ropd_ty_husky_name = ropd_ty_ident.as_str();
        let upper_lopd_ty_husky_name = lopd_ty_husky_name.to_uppercase();
        if let Some(opr) = opt_opr {
            let opr_code = opr.husky_code();
            // let rust_trait_method_name = opr.rust_trait_method_name();
            write!(
                code,
                r#"
            ({lopd_ty_ident:?}, Some({opr:?}), {ropd_ty_ident:?}) => transfer_linkage!(
                |arguments, _| unsafe {{
                    let new_value: {lopd_ty_husky_name} = arguments[0].downcast_{lopd_ty_husky_name}() {opr_code} arguments[1].downcast_{ropd_ty_husky_name}();
                    *arguments[0].downcast_temp_mut::<{lopd_ty_husky_name}>(&__{upper_lopd_ty_husky_name}_VTABLE) = new_value;
                    __Register::new_void()
                }},
                none
            ),"#
            )?
        } else {
            write!(
                code,
                r#"
            ({lopd_ty_ident:?}, None, {ropd_ty_ident:?}) => transfer_linkage!(
                |arguments, _| unsafe {{
                    *arguments[0].downcast_temp_mut::<{lopd_ty_husky_name}>(&__{upper_lopd_ty_husky_name}_VTABLE) = arguments[1].downcast_{ropd_ty_husky_name}();
                    __Register::new_void()
                }},
                none
            ),"#
            )?
        }
        // some {lopd_ty_husky_name}::{rust_trait_method_name}
    }
    write!(
        code,
        r#"
        _ => {{
            panic!("Assign operation {{:?}} is not supported in Husky", (lopd_ty, opt_opr, ropd_ty))
        }}
    }}
}}
"#
    )?;
    Ok(code)
}
