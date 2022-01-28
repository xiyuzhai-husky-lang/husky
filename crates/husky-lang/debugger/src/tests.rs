use warp::Filter;

#[tokio::test]
async fn test1() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}\n!", name));

    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}
