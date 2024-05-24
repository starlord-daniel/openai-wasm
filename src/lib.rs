use anyhow::Result;
use spin_sdk::{
    http::{IntoResponse, Method, Params, Request, Response, Router},
    http_component, variables,
};
extern crate serde_json;

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

    // Load the AZURE_OPENAI_API_KEY from an .env file
    let api_key = variables::get("openai_api_key")?;

    // Load the AZURE_OPENAI_ENDPOINT from an .env file
    let endpoint = variables::get("openai_endpoint")?;

    // Load the deployment_name from an .env file
    let deployment_name = variables::get("openai_deployment_name")?;

    let api_version = "2024-02-01";

    // Create the chat completions url
    let url = format!(
        "{}/openai/deployments/{}/chat/completions?api-version={}",
        endpoint, deployment_name, api_version
    );

    // Create the request body
    let body = serde_json::json!({
        "messages": [
            {
                "role": "system",
                "content": "You are a helpful assistant."
            },
            {
                "role": "user",
                "content": "If you'd have to choose, what is your favourite movie from the MCU?"
            }
        ]
    });

    let req = Request::builder()
        .method(Method::Post)
        .uri(url)
        .header("Content-Type", "application/json")
        .header("API-Key", api_key)
        .body(body.to_string())
        .build();

    // Send the request and await the response
    let res: Response = spin_sdk::http::send(req).await?;

    println!("{:?}", res); // log the response
    Ok(res)
}
