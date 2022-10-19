use toml::Value;

fn main() {
    println!("Hello, world!");
}

#[test]
fn test() {
    let value = "foo = 'bar'".parse::<Value>().unwrap();
    println!("{:?}", value);
    assert_eq!(value["foo"].as_str(), Some("bar"));
}
