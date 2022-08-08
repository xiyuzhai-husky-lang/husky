fn main() {
    call_dynamic().unwrap();
}
fn call_dynamic() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let lib = libloading::Library::new("../dylib0/target/debug/libdylib0.so")?;
        let func: libloading::Symbol<unsafe fn()> = lib.get(b"say_hello")?;
        Ok(func())
    }
}
