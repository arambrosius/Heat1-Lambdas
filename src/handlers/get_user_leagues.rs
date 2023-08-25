use heat1_lambda::{data::models::app_user::AppUser, service::user_service::UserService};
use lambda_http::{run, service_fn, Error, Request, RequestExt, Response};
use serde_json::{json, Value};

async fn function_handler(event: Request) -> Result<Value, Error> {
    let firebase_id = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("firebase_id"))
        .unwrap_or_default();
    let user: AppUser;

    if firebase_id.is_empty() {
        let user_id = event
            .query_string_parameters_ref()
            .and_then(|params| params.first("user_id"))
            .unwrap_or_default();

        user = UserService::get_user_by_user_id(user_id.parse().unwrap()).await?;
    } else {
        user = UserService::get_user_by_firebase_id(firebase_id.to_string()).await?;
    }

    let body = json!(user);
    Ok(body)

    // SuccessApiResponse::init(body)
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
