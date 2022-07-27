#[tokio::main]
async fn main() {
    println!("Hello, world!");
    ff().await;
}

struct A {}

impl Drop for A {
    fn drop(&mut self) {
        todo!()
    }
}

async fn ff() {
    unsafe {
        f();
    }
    {
        let a = A {};
        todo!();
    }
    panic!()
}
#[no_mangle]
extern "C" fn g() {
    println!("Hello, world!");
}

extern "C" {
    fn f();
}
