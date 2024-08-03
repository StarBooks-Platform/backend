use lambda_http::{run, service_fn, Error, IntoResponse, Request, RequestExt, Response};
use tracing::error;
use shared::telemetry::setup_telemetry;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(handler)).await
}

#[tracing::instrument(
    name = "Deleting a model by id",
    skip(event),
    fields(
        id = %event.path_parameters_ref().and_then(|params| params.first("id")).unwrap_or("unknown")
    )
)]
async fn handler(event: Request) -> Result<impl IntoResponse, Error> {
    let tracer = setup_telemetry("starbooks".into(), "info".into(), std::io::stdout);
    
    let _id = event
        .path_parameters_ref()
        .and_then(|params| params.first("id"))
        .unwrap();

    // execute the deletion in the DB

    let resp = Response::builder()
        .status(204)
        .header("content-type", "text/json")
        .body("".to_string())
        .map_err(|e| {
            error!("Failed to serialize the model: {:?}", e);
            e
        })?;
    
    tracer.force_flush();
    
    Ok(resp)
}
