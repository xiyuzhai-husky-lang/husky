use husky_lang_compile_time::*;
use husky_lang_debugger::*;

#[tokio::main]
async fn main() {
    if let Err(e) = Debugger::new(init_compile_time2)
        .serve("localhost:51617")
        .await
    {
        eprintln!("{}", e);
        todo!()
    }
}

fn init_compile_time1(compile_time: &mut HuskyLangCompileTime) {
    compile_time.set_live_file_text(
        "haha/main.hsk".into(),
        r#"
struct A:
    a: i32

dataset:
    synthetic::trivial::real1d::dataset1()

main:
    a = 1
    b = 1
    assert a == b
    a
"#
        .into(),
    );
}

fn init_compile_time2(compile_time: &mut HuskyLangCompileTime) {
    compile_time.set_live_file_text(
        "haha/main.hsk".into(),
        r#"
struct A:
    a: i32

dataset:
    synthetic::trivial::real1d::dataset1()

main:
    a = 1
    b = 1
    assert a == b
    if true:
        1
    else:
        2
"#
        .into(),
    );
}
