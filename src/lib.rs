use anyhow::Result;
use spin_sdk::{
    http::{IntoResponse, Method, Params, Request, Response, Router},
    http_component,
};

/// Handle the GET request to the `/` endpoint, that returns a simple JSON response.
/// A Spin HTTP component that internally routes requests.
#[http_component]
async fn handle_route(req: Request) -> Response {
    let mut router = Router::new();
    router.any("/*", handle_healthcheck);
    router.get_async("/api/openai", handle_openai);
    router.handle(req)
}

fn handle_healthcheck(req: Request, _: Params) -> anyhow::Result<impl IntoResponse> {
    println!("Handling request to {:?}", req.header("spin-full-url"));

    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body("{\"message\": \"Healthy :-)\"}")
        .build())
}

async fn handle_openai(_req: Request, _: Params) -> Result<impl IntoResponse> {
    println!("Sending request to OpenAI API");

    // Create the outbound request object
    let req = Request::builder()
        .method(Method::Get)
        .uri("https://random-data-api.fermyon.app/animals/json")
        .build();

    // Send the request and await the response
    let res: Response = spin_sdk::http::send(req).await?;

    println!("{:?}", res); // log the response
    Ok(res)
}
