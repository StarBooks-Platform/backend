use lambda_http::{
    run, service_fn, Error, IntoResponse, Request, RequestExt, RequestPayloadExt, Response,
};
use shared::{Model, PutModel, ViewModel};
use tracing::info;
use shared::telemetry::setup_telemetry;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(handler)).await
}

#[tracing::instrument(
    name = "Updating a model by id",
    skip(event),
    fields(
        id = %event.path_parameters_ref().and_then(|params| params.first("id")).unwrap_or("unknown"),
        name = %event.payload::<PutModel>().unwrap_or(None).map(|m| m.name).unwrap_or("unknown".to_string())
    )
)]
async fn handler(event: Request) -> Result<impl IntoResponse, Error> {
    let tracer = setup_telemetry("starbooks".into(), "info".into(), std::io::stdout);
    
    // get the path parameter
    let _id = event
        .path_parameters_ref()
        .and_then(|params| params.first("id"))
        .unwrap();

    let body = event.payload::<PutModel>();

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
                        .status(200)
                        .header("content-type", "text/json")
                        .body(serde_model)
                        .map_err(|e| {
                            info!("Failed to serialize the model: {:?}", e);
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
                            info!("Failed to serialize the model: {:?}", e);
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
                    info!("Failed to serialize the model: {:?}", e);
                    e
                })?;
            Ok(resp)
        }
    };
    
    tracer.force_flush();
    
    resp
}
