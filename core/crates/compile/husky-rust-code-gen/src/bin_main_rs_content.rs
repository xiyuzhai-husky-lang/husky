use std::path::Path;

use husky_compile_dir::get_code_snapshot_dir;

use crate::*;

pub(crate) fn rust_bin_main_rs_content(
    db: &dyn RustCodeGenQueryGroup,
    main_file: FilePtr,
) -> Arc<String> {
    let pack = db.package(main_file).unwrap();
    Arc::new(format!(
        r#"use husky_lang_debugger::*;
use {}::__init__::link_entity_with_compiled;
use husky_compile_time::*;

#[tokio::main]
async fn main() {{
    Debugger::new(|compile_time| init_compile_time(compile_time))
        .serve("localhost:51617")
        .await
        .expect("")
}}

fn init_compile_time(compile_time: &mut HuskyCompileTime) {{
    compile_time.load_package({:?}.into());
    link_entity_with_compiled(compile_time)
}}
"#,
        &pack.ident,
        &get_code_snapshot_dir(&pack)
    ))
}
