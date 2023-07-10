pub fn dash_to_snake(dash_name: &str) -> String {
    if dash_name.len() == 0 {
        panic!()
    }
    let mut chars = dash_name.chars();
    let c_start = chars.next().unwrap();
    assert!(c_start.is_alphabetic() && c_start.is_lowercase());
    let mut snake_name: String = [c_start].into_iter().collect();
    for c in chars {
        if c.is_alphabetic() {
            assert!(c.is_lowercase());
        }
        if c.is_alphanumeric() {
            snake_name.push(c);
        } else if c == '-' {
            snake_name.push('_')
        } else {
            todo!()
        }
    }
    snake_name
}

pub fn snake_to_dash(snake_name: &str) -> String {
    if snake_name.len() == 0 {
        panic!()
    }
    let mut chars = snake_name.chars();
    let c_start = chars.next().unwrap();
    assert!(c_start.is_alphabetic() && c_start.is_lowercase());
    let mut snake_name: String = [c_start].into_iter().collect();
    for c in chars {
        if c.is_alphabetic() {
            assert!(c.is_lowercase());
        }
        if c.is_alphanumeric() {
            snake_name.push(c);
        } else if c == '_' {
            snake_name.push('-')
        } else {
            todo!()
        }
    }
    snake_name
}
