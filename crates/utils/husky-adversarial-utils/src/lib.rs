pub fn repeat_line(input: &str, idx: usize) -> String {
    let mut s = String::new();
    for (i, line) in input.lines().enumerate() {
        s += line;
        if (i < input.lines().count() - 1) {
            s += "\n";
        }

        if (i == idx) {
            s += line;
            if (i < input.lines().count() - 1) {
                s += "\n";
            }
        }
    }
    return s;
}

#[test]
fn test() {
    let new_data = repeat_line(
        r#"a+b=c;
                x+y=z;
                b+c=w;
             "#,
        2,
    );
    assert_eq!(
        new_data,
        r#"a+b=c;
                x+y=z;
                b+c=w;
                b+c=w;
             "#
    )
}
