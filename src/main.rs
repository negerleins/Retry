use serde::{Deserialize, Serialize};
use warp::{Filter, Reply};
use warp::reply::json;
use serde_json::json;

#[derive(Deserialize, Serialize)]
struct Message {
    content: String,
}

struct WarpServer {
    host: [u8; 4],
    port: u16,
}

// https://doc.rust-lang.org/book/ch03-02-data-types.html < self reference.
// https://doc.rust-lang.org/book/ch17-01-what-is-oo.html < self reference.
// https://docs.rs/warp/latest/warp/ < warp documentation.

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
            .map(|message: Message| format!("Received message: {}", message.content));

        let post_message_json = warp::path("json")
            .and(warp::post())
            .and(warp::body::json())
            .and(warp::header("user-agent"))
            .map(|message: Message, agent: String| {
                let response = json!({
                    "status": "success",
                    "message": message.content,
                    "agent": agent
                });
                json(&response)
            });

        post_message.or(post_message_json)
    }

    fn routes(&self) -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
        self.get_routes().or(self.post_routes())
    }

    async fn run(&self) {
        let routes = self.routes();
        warp::serve(routes).run((self.host, self.port)).await;
    }
}

#[tokio::main]
async fn main() {
    let server = WarpServer::new([127, 0, 0, 1], 3030);
    server.run().await;
}
