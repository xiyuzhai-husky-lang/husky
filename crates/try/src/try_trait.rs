trait A1 {
    fn a(&self);
}
trait A2 {
    fn a();
}

struct A {}
impl A {
    // fn a(&self) {
    //     println!("A");
    // }
}

impl A1 for A {
    fn a(&self) {
        println!("A1");
    }
}

impl A2 for A {
    fn a() {}
}

#[test]
fn test_conflict() {
    let a = A {};
}
