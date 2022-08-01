use husky_bindgen_utils::simple_bindgen;
use husky_c_code_build::build_single_file_to_lib;
use husky_c_code_repr::*;
use husky_opn_syntax::*;
use husky_rust_code_repr::{registration::NonPrimitiveTypeRegistration, BuildCodeGenStart};
use husky_word::RootIdentifier;
use husky_write_utils::w;
use std::io::Write;
use std::path::PathBuf;
use std::{env, fs::File};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    // let husky_dir = std::env::var("HUSKY_DIR").expect("HUSKY_DIR is not set");
    let husky_dir = "/home/xiyuzhai/Documents/husky";
    let crate_dir = format!("{husky_dir}/core/crates/vm/husky-vm-primitive-opr-linkage/src");
    let binary_mod_path = format!("{crate_dir}/binary.rs");
    gen_binary(&binary_mod_path).unwrap();
}

fn gen_binary(binary_mod_path: &str) -> std::io::Result<()> {
    let mut f = File::create(binary_mod_path).unwrap();
    write!(
        f,
        r#"
use husky_print_utils::p;
use husky_word::RootIdentifier;
use husky_vm_interface::*;
use husky_opn_syntax::*;

pub fn resolve_primitive_pure_binary_opr_linkage(
    lopd_ty: RootIdentifier,
    opr: PureBinaryOpr,
    ropd_ty: RootIdentifier,
) -> __Linkage {{
    use PureBinaryOpr::*;
    use RootIdentifier::*;
    type b32 = u32;
    type b64 = u64;

    match (lopd_ty, opr, ropd_ty) {{"#
    )?;
    use PureBinaryOpr::*;
    use RootIdentifier::*;
    static supported_pure_binary_opns: &'static [(
        RootIdentifier,
        PureBinaryOpr,
        RootIdentifier,
    )] = &[
        // bool
        (Bool, And, Bool),
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
    ];
    for (lopd_ty_ident, opr, ropd_ty_ident) in supported_pure_binary_opns {
        let lopd_ty_husky_name = lopd_ty_ident.as_str();
        let ropd_ty_husky_name = ropd_ty_ident.as_str();
        let opr_code = opr.code();
        // let rust_trait_method_name = opr.rust_trait_method_name();
        write!(
            f,
            r#"
        ({lopd_ty_ident:?}, {opr:?}, {ropd_ty_ident:?}) => transfer_linkage!(
            |_,arguments| unsafe {{
                (arguments[0].downcast_{lopd_ty_husky_name}() {opr_code} arguments[1].downcast_{ropd_ty_husky_name}()).to_register()
            }},
            none
        ),"#
        )?
        // some {lopd_ty_husky_name}::{rust_trait_method_name}
    }
    write!(
        f,
        r#"
        (I32, Power, I32) => transfer_linkage!(
            |_,arguments| unsafe {{
                num::pow(arguments[0].downcast_i32(), arguments[1].downcast_i32() as usize).to_register()
            }},
            none
        ),
        (I32, RemEuclid, I32) => transfer_linkage!(
            |_, arguments| unsafe {{
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

    static supported_assign_binary_opns: &'static [(
        RootIdentifier,
        std::option::Option<PureBinaryOpr>,
        RootIdentifier,
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
        (F32, None, F32),
    ];
    write!(
        f,
        r#"

pub fn resolve_primitive_assign_binary_opr_linkage(
    lopd_ty: RootIdentifier,
    opt_opr: Option<PureBinaryOpr>,
    ropd_ty: RootIdentifier,
) -> __Linkage {{
    use PureBinaryOpr::*;
    use RootIdentifier::*;
    type b32 = u32;
    type b64 = u64;

    match (lopd_ty, opt_opr, ropd_ty) {{"#
    )?;
    for (lopd_ty_ident, opt_opr, ropd_ty_ident) in supported_assign_binary_opns {
        let lopd_ty_husky_name = lopd_ty_ident.as_str();
        let ropd_ty_husky_name = ropd_ty_ident.as_str();
        if let Some(opr) = opt_opr {
            let opr_code = opr.code();
            // let rust_trait_method_name = opr.rust_trait_method_name();
            write!(
                f,
                r#"
            ({lopd_ty_ident:?}, Some({opr:?}), {ropd_ty_ident:?}) => transfer_linkage!(
                |_,arguments| unsafe {{
                    let new_value: {lopd_ty_husky_name} = (arguments[0].downcast_{lopd_ty_husky_name}() {opr_code} arguments[1].downcast_{ropd_ty_husky_name}());
                    *arguments[0].downcast_temp_mut::<{lopd_ty_husky_name}>() = new_value;
                    __Register::new_void()
                }},
                none
            ),"#
            )?
        } else {
            write!(
                f,
                r#"
            ({lopd_ty_ident:?}, None, {ropd_ty_ident:?}) => transfer_linkage!(
                |_,arguments| unsafe {{
                    *arguments[0].downcast_temp_mut::<{lopd_ty_husky_name}>() = arguments[1].downcast_{ropd_ty_husky_name}();
                    __Register::new_void()
                }},
                none
            ),"#
            )?
        }
        // some {lopd_ty_husky_name}::{rust_trait_method_name}
    }
    write!(
        f,
        r#"
        _ => {{
            panic!("Assign operation {{:?}} is not supported in Husky", (lopd_ty, opt_opr, ropd_ty))
        }}
    }}
}}
"#
    )
}
