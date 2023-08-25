use lambda_http::{aws_lambda_events::encodings::Error, Response};
use serde_json::Value;

pub struct SuccessApiResponse;

impl SuccessApiResponse {
    pub fn init(body: Value) -> Result<Response<Value>, Error> {
        let response = Response::builder()
            .status(200)
            .header("content-type", "application/json")
            .body(body)
            .map_err(Box::new)?;

        Ok(response)
    }
}
