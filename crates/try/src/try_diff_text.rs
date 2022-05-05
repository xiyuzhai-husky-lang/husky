use text_diff::print_diff;
#[test]
fn it_work() {
    print_diff("Diffs are awesome", "Diffs are cool", "\n");
}
