use warp::Filter;

#[tokio::main]
async fn main() {
    // Route for /hello/{name}
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    // Default route for /
    let default = warp::path::end()
        .map(|| "Welcome to my Warp server!");

    // Combine the routes
    let routes = default.or(hello);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}