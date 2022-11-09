use toml_edit::{value, Document};

#[test]
fn toml_edit_should_work() {
    let toml = r#"
"hello" = 'toml!' # comment
['a'.b]
    "#;
    let mut doc = toml.parse::<Document>().expect("invalid doc");
    assert_eq!(doc.to_string(), toml);
    // let's add a new key/value pair inside a.b: c = {d = "hello"}
    doc["a"]["b"]["c"]["d"] = value("hello");
    // autoformat inline table a.b.c: { d = "hello" }
    doc["a"]["b"]["c"].as_inline_table_mut().map(|t| t.fmt());
    let expected = r#"
"hello" = 'toml!' # comment
['a'.b]
c = { d = "hello" }
    "#;
    assert_eq!(doc.to_string(), expected);
}
