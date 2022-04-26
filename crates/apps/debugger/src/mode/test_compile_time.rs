use std::io::{stdin, stdout, Write};

use super::*;
use compile_time_db::*;
use serde::Serialize;
use token::AbsSemanticToken;

pub(super) async fn test_compile_time(dir: PathBuf) {
    let pack_paths = collect_pack_dirs(dir);
    println!(
        "\n{}Running{} {} tests on compile time:",
        print_utils::CYAN,
        print_utils::RESET,
        pack_paths.len()
    );

    for pack_path in pack_paths {
        let mut compile_time = HuskyLangCompileTime::default();
        init_compile_time_from_dir(&mut compile_time, pack_path.to_path_buf());
        println!(
            "\n{}test{} {}",
            print_utils::CYAN,
            print_utils::RESET,
            pack_path.as_os_str().to_str().unwrap(),
        );
        test_semantic_tokens(&pack_path, &compile_time).await;
        test_diagnostics(&pack_path, &compile_time).await;
        println!(
            "    {}result{}: {}ok{}",
            print_utils::CYAN,
            print_utils::RESET,
            print_utils::GREEN,
            print_utils::RESET,
        )
    }
}

async fn test_semantic_tokens(pack_path: &Path, compile_time: &HuskyLangCompileTime) {
    type SemanticTokensTable = HashMap<String, Vec<AbsSemanticToken>>;

    let modules = compile_time.all_modules();
    let mut highlights_table = HashMap::<String, Vec<AbsSemanticToken>>::new();
    for module in modules {
        let file = compile_time.module_file(module).unwrap();
        let ast_text = compile_time.ast_text(file).unwrap();
        let semantic_tokens = ast_text.semantic_tokens.clone();
        assert!(highlights_table
            .insert(module.to_str(), semantic_tokens)
            .is_none());
    }
    compare_semantic_tokens_tables(highlights_table, pack_path);

    fn compare_semantic_tokens_tables(semantic_tokens_table: SemanticTokensTable, path: &Path) {
        let semantic_tokens_table_path = path.join("semantic_tokens_table.json");
        let semantic_tokens_table_on_disk: SemanticTokensTable =
            if !semantic_tokens_table_path.exists() {
                Default::default()
            } else {
                let text = fs::read_to_string(&semantic_tokens_table_path).unwrap();
                let v: serde_json::Value = serde_json::from_str(&text).unwrap();
                match serde_json::from_value(v) {
                    Ok(v) => v,
                    Err(e) => {
                        notify_deserialize_error(
                            semantic_tokens_table,
                            &text,
                            &e,
                            &semantic_tokens_table_path,
                        );
                        return;
                    }
                }
            };
        if semantic_tokens_table_on_disk != semantic_tokens_table {
            notify_change(
                semantic_tokens_table,
                semantic_tokens_table_on_disk,
                &semantic_tokens_table_path,
            )
        }
    }
}

async fn test_diagnostics(pack_path: &Path, compile_time: &HuskyLangCompileTime) {
    type DiagnosticsTable = HashMap<String, Vec<Diagnostic>>;

    let modules = compile_time.all_modules();
    let mut diagnostics_table = HashMap::<String, Vec<Diagnostic>>::new();
    for module in modules {
        compile_time
            .diagnostic_reserve(module)
            .release(|diagnostics| {
                if diagnostics.len() > 0 {
                    assert!(diagnostics_table
                        .insert(module.to_str(), diagnostics.clone())
                        .is_none());
                }
            });
    }
    compare_diagnostics_tables(diagnostics_table, pack_path);

    fn compare_diagnostics_tables(diagnostics_table: DiagnosticsTable, path: &Path) {
        let diagnostics_table_path = path.join("diagnostics_table.json");
        let diagnostics_table_on_disk: DiagnosticsTable = if !diagnostics_table_path.exists() {
            Default::default()
        } else {
            let text = fs::read_to_string(&diagnostics_table_path).unwrap();
            let v: serde_json::Value = serde_json::from_str(&text).unwrap();
            match serde_json::from_value(v) {
                Ok(v) => v,
                Err(e) => {
                    notify_deserialize_error(diagnostics_table, &text, &e, &diagnostics_table_path);
                    return;
                }
            }
        };
        if diagnostics_table_on_disk != diagnostics_table {
            p!(diagnostics_table);
            p!(diagnostics_table_on_disk);
            todo!()
        }
    }
}

fn notify_change<T>(new: T, old: T, save_path: &Path)
where
    T: std::fmt::Debug + Serialize,
{
    // notify the difference between the old and the new
    // ask whether to update the old
    println!(
        "{}Change in saved data{} for file {}{:?}{},",
        print_utils::MAGENTA,
        print_utils::RESET,
        print_utils::GREEN,
        save_path.as_os_str(),
        print_utils::RESET,
    );
    print!("old = \n  {:?}\n", &old);
    print!("new = \n  {:?}\n", &new);
    let accept: bool = loop {
        print!("Do you want to accept change in saved data (y/n)? ");
        let mut s = String::new();
        let _ = stdout().flush();
        stdin()
            .read_line(&mut s)
            .expect("Did not enter a correct string");
        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }
        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }
        match &s as &str {
            "y" => break true,
            "n" => break false,
            _ => println!("Invalid answer: {}", s),
        }
    };
    if accept {
        fs::write(save_path, serde_json::to_string(&new).unwrap()).expect("Error writing");
    } else {
        panic!("Change in saved data not accepted")
    }
}

fn notify_deserialize_error<T>(
    new: T,
    old_text: &str,
    e: &serde_json::error::Error,
    save_path: &Path,
) where
    T: std::fmt::Debug + Serialize,
{
    // notify the difference between the old and the new
    // ask whether to update the old
    println!(
        "{}Unable to deserialize saved data{} for file {}{:?}{},\n{}error{}:\n  {:?}",
        print_utils::RED,
        print_utils::RESET,
        print_utils::GREEN,
        save_path.as_os_str(),
        print_utils::RESET,
        print_utils::RED,
        print_utils::RESET,
        e
    );
    print!(
        "{}old text{} = \n  {:?}\n",
        print_utils::CYAN,
        print_utils::RESET,
        &old_text
    );
    print!(
        "{}new{} = \n  {:?}\n",
        print_utils::CYAN,
        print_utils::RESET,
        &new
    );
    let accept: bool = loop {
        print!("Do you want to accept change in saved data (y/n)? ");
        let mut s = String::new();
        let _ = stdout().flush();
        stdin()
            .read_line(&mut s)
            .expect("Did not enter a correct string");
        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }
        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }
        match &s as &str {
            "y" => break true,
            "n" => break false,
            _ => println!("Invalid answer: {}", s),
        }
    };
    if accept {
        fs::write(save_path, serde_json::to_string(&new).unwrap()).expect("Error writing");
    } else {
        panic!("Change in saved data not accepted")
    }
}
