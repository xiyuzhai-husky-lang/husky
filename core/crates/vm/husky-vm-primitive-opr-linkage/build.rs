use husky_bindgen_utils::simple_bindgen;
use husky_c_code_build::build_single_file_to_lib;
use husky_c_code_repr::*;
use husky_rust_code_repr::{BuildCodeGenStart, NonPrimitiveTypeRegistration};
use husky_word::RootIdentifier;
use husky_write_utils::w;
use std::io::Write;
use std::path::PathBuf;
use std::{env, fs::File};
use vm::PureBinaryOpr;

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
use vm::*;

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
        (I32, Eq, I32),
        (F32, Greater, F32),
        (F32, Geq, F32),
        (F32, Less, F32),
        (F32, Leq, F32),
    ];
    for (lopd_ty_ident, opr, ropd_ty_ident) in supported_pure_binary_opns {
        let lopd_ty_husky_name = lopd_ty_ident.as_str();
        let ropd_ty_husky_name = ropd_ty_ident.as_str();
        let opr_code = opr.code();
        let rust_trait_method_name = opr.rust_trait_method_name();
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
        _ => {{
            panic!("Assign operation {{:?}} is not supported in Husky", (lopd_ty, opr, ropd_ty))
        }}
    }}
}}
"#
    )?;

    static supported_assign_binary_opns: &'static [(
        RootIdentifier,
        std::option::Option<PureBinaryOpr>,
        RootIdentifier,
    )] = &[(I32, Some(Add), I32)];
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
            let rust_trait_method_name = opr.rust_trait_method_name();
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
            todo!()
        }
        // some {lopd_ty_husky_name}::{rust_trait_method_name}
    }
    write!(
        f,
        r#"
        _ => {{
            panic!("{{:?}} is not supported in Husky", (lopd_ty, opt_opr, ropd_ty))
        }}
    }}
}}
"#
    )
}
