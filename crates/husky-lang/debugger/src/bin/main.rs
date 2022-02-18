use husky_lang_debugger::*;

#[tokio::main]
async fn main() {
    if let Err(e) = Debugger::new(mock::init_compile_time3)
        .serve("localhost:51617")
        .await
    {
        eprintln!("{}", e);
        todo!()
    }
}
