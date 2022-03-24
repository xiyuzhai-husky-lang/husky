use std::path::Path;

use crate::*;

pub(crate) fn rust_bin_main_rs_content(
    db: &dyn RustGenQueryGroup,
    main_file: FilePtr,
) -> Arc<String> {
    Arc::new(format!(
        r#"use husky_lang_debugger::*;
use package_name_todo::__init__::link_entity_with_compiled;
use compile_time_db::*;

#[tokio::main]
async fn main() {{
    Debugger::new(|compile_time| init_compile_time(compile_time))
        .serve("localhost:51617")
        .await
        .expect("")
}}

fn init_compile_time(compile_time: &mut HuskyLangCompileTime) {{
    compile_time.load_package({:?}.into());
    link_entity_with_compiled(compile_time)
}}
"#,
        &get_code_snapshot_dir(&main_file)
    ))
}

fn get_code_snapshot_dir(main_file_path: &Path) -> String {
    let package_dir = main_file_path.parent().unwrap();
    let rust_dir = package_dir.join(".rust");
    assert!(rust_dir.exists());
    let snapshot_dir = rust_dir.join("snapshot");
    assert!(snapshot_dir.exists());
    snapshot_dir.to_str().unwrap().into()
}
