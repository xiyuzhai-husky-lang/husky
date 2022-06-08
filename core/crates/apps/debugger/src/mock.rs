use compile_time_db::*;

pub fn init_compile_time1(compile_time: &mut HuskyCompileTime) {
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

pub fn init_compile_time2(compile_time: &mut HuskyCompileTime) {
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

pub fn init_compile_time3(compile_time: &mut HuskyCompileTime) {
    compile_time.set_live_file_text(
        "haha/main.hsk".into(),
        r#"
func f() -> i32:
    1

dataset:
    synthetic::trivial::real1d::dataset1()

main:
    a = 1
    b = f()
    assert a == b
    if true:
        1
    else:
        2
"#
        .into(),
    );
}

pub fn init_compile_time4(compile_time: &mut HuskyCompileTime) {
    compile_time.set_live_file_text(
        "haha/main.hsk".into(),
        r#"
func f() -> i32:
    1 + 1

dataset:
    synthetic::trivial::real1d::dataset1()

main:
    a = 1
    b = f()
    assert a == b
    if true:
        1
    else:
        2
"#
        .into(),
    );
}
