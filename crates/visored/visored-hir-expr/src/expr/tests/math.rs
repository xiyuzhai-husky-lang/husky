use super::*;

fn t(input: &str, expect: &Expect) {
    let db = &DB::default();
    let example = VdHirExprExample::new(input, LxMode::Math, &[], &[], db);
    expect.assert_eq(&example.show_display_tree());
}

#[test]
fn basic_to_vd_hir_works() {
    t("1 + 1", &expect![[""]]);
}
