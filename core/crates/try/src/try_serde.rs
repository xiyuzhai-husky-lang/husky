use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct A {
    x: i32,
}

#[derive(Serialize)]
pub enum B {
    Haha { x: i32 },
    Hehe,
}

#[test]
fn try_serialize_a() {
    use crate::*;
    let a = A { x: 3 };
    should_eq!(
        format!("{}", &serde_json::to_string(&a).unwrap()),
        "{\"x\":3}"
    );
    let b = B::Haha { x: 32 };
    let c = B::Hehe;
    p!(serde_json::to_string(&b).unwrap());
    p!(serde_json::to_string(&c).unwrap());
    p!(serde_json::to_string(&Ok::<i32, String>(1i32)).unwrap());
    p!(serde_json::to_string(&Err::<i32, String>("what".into())).unwrap());
}

fn try_serialize_hashmap() {}
