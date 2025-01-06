use warp::{Filter, Reply};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Message {
    content: String,
}

struct WarpServer {
    host: [u8; 4],
    port: u16,
}

// https://doc.rust-lang.org/book/ch03-02-data-types.html < self reference.

impl WarpServer {
    fn new(host: [u8; 4], port: u16) -> Self {
        WarpServer { host, port }
    }

    fn get_routes(&self) -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
        let hello = warp::path!("hello" / String)
            .and(warp::get())
            .map(|name| format!("Hello, {}!", name));

        let default = warp::path::end()
            .and(warp::get())
            .map(|| "Welcome to my Warp server!");

        hello.or(default)
    }

    fn post_routes(&self) -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
        let post_message = warp::path("message")
            .and(warp::post())
            .and(warp::body::json())
            .map(|message: Message| {
                format!("Received message: {}", message.content)
            });

        post_message
    }

    fn routes(&self) -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
        self.get_routes().or(self.post_routes())
    }

    async fn run(&self) {
        let routes = self.routes();
        warp::serve(routes)
            .run((self.host, self.port))
            .await;
    }
}

#[tokio::main]
async fn main() {
    let server = WarpServer::new([127, 0, 0, 1], 3030);
    server.run().await;
}