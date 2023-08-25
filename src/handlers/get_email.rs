use heat1_lambda::{domain::success_api_response::SuccessApiResponse, service::user_service::UserService};
use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
use serde_json::{json, Value};


async fn function_handler(event: Request) -> Result<Value, Error> {
    let username = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("username"))
        .unwrap_or_default();

    let email = UserService::get_email_by_username(username.to_string()).await?;

    let body = json!({ "email": email  });

    // SuccessApiResponse::init(body)
    Ok(body)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
