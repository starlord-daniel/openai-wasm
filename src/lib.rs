mod handle_methods;
extern crate serde_json;

use spin_sdk::{
    http::{Params, Request, Response, Router},
    http_component,
};

use crate::handle_methods::{handle_healthcheck, handle_openai, handle_search};

/// Handle the GET request to the `/` endpoint, that returns a simple JSON response.
/// A Spin HTTP component that internally routes requests.
#[http_component]
async fn handle_route(req: Request) -> Response {
    let mut router = Router::new();
    router.get("/", serve_webpage);
    router.post_async("/api/openai", handle_openai);
    router.post_async("api/search", handle_search);
    router.any("/*", handle_healthcheck);
    router.handle(req)
}

fn serve_webpage(_: Request, _: Params) -> Response {
    Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(include_str!("../main.html"))
        .build()
}
