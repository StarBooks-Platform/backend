use lambda_http::{Error, IntoResponse, Request, RequestExt, Response, run, service_fn};
use tracing::error;
use shared::{Model, ViewModel};
use shared::telemetry::setup_telemetry;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(handler)).await
}

#[tracing::instrument(
    name = "Retrieving a model by id",
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
    // fetch the model from the DB
    let model = Model::new("New Model".to_string());
    // convert to a view model
    let view_model: ViewModel = model.into();
    let view_mode_serde = serde_json::to_string(&view_model)?;

    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/json")
        .body(view_mode_serde)
        .map_err(|e| {
            error!("Failed to serialize the model: {:?}", e);
            e
        })?;

    tracer.force_flush();

    Ok(resp)
}
