use lambda_http::{
    http::Response, run, service_fn, Error, IntoResponse, Request, RequestPayloadExt,
};
use tracing::error;
use shared::{Model, PostModel, ViewModel};
use shared::telemetry::setup_telemetry;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(handler)).await
}

#[tracing::instrument(
    name = "Creating a model",
    skip(event),
    fields(
        name = %event.payload::<PostModel>().unwrap_or(None).map(|m| m.name).unwrap_or("unknown".to_string())
    )
)]
async fn handler(event: Request) -> Result<impl IntoResponse, Error> {
    let tracer = setup_telemetry("starbooks".into(), "info".into(), std::io::stdout);
    
    let body = event.payload::<PostModel>();

    let resp = match body {
        Ok(item) => {
            match item {
                Some(i) => {
                    // convert to the model
                    let model: Model = i.into();
                    // do some work
                    // convert back to view model
                    let view_model: ViewModel = model.into();
                    let serde_model = serde_json::to_string(&view_model)?;
                    let resp = Response::builder()
                        .status(201)
                        .header("content-type", "text/json")
                        .body(serde_model)
                        .map_err(|e| {
                            error!("Failed to serialize the model: {:?}", e);
                            e
                        })?;
                    Ok(resp)
                }
                None => {
                    let resp = Response::builder()
                        .status(400)
                        .header("content-type", "text/json")
                        .body("".to_string())
                        .map_err(|e| {
                            error!("Failed to serialize the model: {:?}", e);
                            e
                        })?;
                    Ok(resp)
                }
            }
        }
        Err(e) => {
            let resp = Response::builder()
                .status(400)
                .header("content-type", "text/json")
                .body(e.to_string())
                .map_err(|e| {
                    error!("Failed to serialize the model: {:?}", e);
                    e
                })?;
            Ok(resp)
        }
    };
    
    tracer.force_flush();
    
    resp
}
