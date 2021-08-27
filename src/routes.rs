use warp::Filter;

pub async fn server() {
    let hello_world = warp::path::end().map(|| "Hello, World at root!");
    let hi = warp::path("hi").map(|| "Hello, World!");
    let hello_from_warp = warp::path!("hello" / "from" / "warp").map(|| "Hello from warp!");
    let sum = warp::path!("sum" / u32 / u32).map(|a, b| format!("{} + {} = {}", a, b, a + b));

    let routes = warp::get().and(
        hello_world
            .or(hi)
            .or(hello_from_warp)
            .or(sum)
    );
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}