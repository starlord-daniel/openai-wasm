use anyhow::Result;
use spin_sdk::{
    http::{IntoResponse, Method, Params, Request, Response},
    variables,
};

pub fn handle_healthcheck(req: Request, _: Params) -> anyhow::Result<impl IntoResponse> {
    println!("Handling request to {:?}", req.header("spin-full-url"));

    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body("{\"message\": \"Healthy :-)\"}")
        .build())
}

pub async fn handle_openai(_req: Request, _: Params) -> Result<impl IntoResponse> {
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

    // Get the body as json of the incoming request
    let incoming_request_body = _req.body();
    let body_json: serde_json::Value = serde_json::from_slice(incoming_request_body)?;

    // Create a mutable messages array to build the request for the OpenAI API
    let mut messages_array = Vec::new();

    // Add a system message to the messages array
    messages_array.push(serde_json::json!({
        "role": "system",
        "content": "You are a helpful assistant."
    }));

    // Access the messages in the json
    let messages = body_json["messages"].as_array().unwrap();

    // Add the incoming messages to the new_responses vector
    for message in messages {
        messages_array.push(message.clone());
    }

    // Create a json object with the messages array
    let messages_json = serde_json::json!({
        "messages": messages_array
    });
    println!("Messages to Open AI API: {}", messages_json);

    let req = Request::builder()
        .method(Method::Post)
        .uri(url)
        .header("Content-Type", "application/json")
        .header("API-Key", api_key)
        .body(messages_json.to_string())
        .build();

    // Send the request and await the response
    let res: Response = spin_sdk::http::send(req).await?;

    // From the choices/0/message/content save the answer
    let res_body = res.body();
    let res_json: serde_json::Value = serde_json::from_slice(res_body)?;
    let answer = res_json["choices"][0]["message"]["content"]
        .as_str()
        .unwrap();

    println!("Answer from Open AI API: {}", answer);

    Ok(res)
}

pub async fn handle_search(_req: Request, _: Params) -> Result<impl IntoResponse> {
    println!("Handling request to {:?}", _req.header("spin-full-url"));

    let search_endpoint = variables::get("search_endpoint")?;
    let index_name = variables::get("search_index_name")?;
    let api_key = variables::get("search_api_key")?;

    // Build request to the search API
    let search_url = format!(
        "{search_endpoint}/indexes('{index_name}')/docs/search.post.search?api-version=2023-11-01"
    );

    let incoming_request_body = _req.body();
    let body_json: serde_json::Value = serde_json::from_slice(incoming_request_body)?;

    // Define search parameters
    let query = body_json["query"].as_str().unwrap();

    // TODO: Update search fields
    let search_filelds = "name,description";

    // TODO: Perform embedding of the question
    let question_embedding = [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];

    // define json body
    let search_body = serde_json::json!(
    {
        "count": true,
        "highlight": "description",
        "orderby": "rating desc",
        "queryType": "semantic",
        "search": query,
        "searchFields": search_filelds,
        "searchMode": "any",
        "skip": 0,
        "top": 1,
        "vectorQueries": [
        {
            "kind": "vector",
            "vector": question_embedding,
            "fields": "descriptionEmbedding",
            "k": 5,
            "exhaustive": true
        }
        ],
        "vectorFilterMode": "preFilter"
    });

    let req = Request::builder()
        .method(Method::Post)
        .uri(search_url)
        .header("Content-Type", "application/json")
        .header("api-key", api_key)
        .body(search_body.to_string())
        .build();

    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body("{\"message\": \"Search results\"}")
        .build())
}
