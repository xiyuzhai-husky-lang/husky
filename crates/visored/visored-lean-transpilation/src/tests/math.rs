use super::*;
use latex_prelude::mode::LxMode;

fn t(input: &str, expect: &Expect) {
    let db = &DB::default();
    let example = VdLeanTranspilationExample::new(input, LxMode::Math, &[], &[], db);
    expect.assert_eq(&example.show_display_tree(db));
}

#[test]
fn basic_visored_to_lean_works() {
    t("1 + 1", &expect![[r#"
        binary: `+`
        ├─ literal: `1`
        └─ literal: `1`
    "#]]);
}
