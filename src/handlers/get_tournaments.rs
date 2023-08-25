use heat1_lambda::{domain::success_api_response::SuccessApiResponse, service::tournament_service::TournamentService};
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use serde_json::{json, Value};

async fn function_handler(_event: Request) -> Result<Value, Error> {
    let tournaments = TournamentService::get_tournaments().await?;
    
    let body = json!(tournaments);
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
