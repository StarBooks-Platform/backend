use opentelemetry::trace::TracerProvider as _;
use opentelemetry::{global, KeyValue};
use opentelemetry_sdk::propagation::TraceContextPropagator;
use opentelemetry_sdk::runtime::Tokio;
use opentelemetry_sdk::trace::{Config, TracerProvider};
use opentelemetry_sdk::Resource;
use opentelemetry_semantic_conventions::resource::SERVICE_NAME;
use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::EnvFilter;

fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
    tracer_provider: &TracerProvider,
) -> impl Subscriber + Send + Sync
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let filter_layer =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    let fmt_layer = BunyanFormattingLayer::new(name.clone(), sink);
    let otel_layer = tracing_opentelemetry::layer().with_tracer(tracer_provider.tracer(name));

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(JsonStorageLayer)
        .with(fmt_layer)
        .with(otel_layer)
}

fn init_tracer(name: String) -> TracerProvider {
    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_trace_config(
            Config::default().with_resource(Resource::new(vec![KeyValue::new(
                SERVICE_NAME,
                name.clone(),
            )])),
        )
        .with_exporter(opentelemetry_otlp::new_exporter().http())
        .install_batch(Tokio)
        .expect("Failed to install otel pipeline")
}

fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    let _ = LogTracer::init();
    global::set_text_map_propagator(TraceContextPropagator::new());
    let _ = tracing::subscriber::set_global_default(subscriber);
}

pub fn setup_telemetry<Sink>(name: String, env_filter: String, sink: Sink) -> TracerProvider
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let tracer_provider = init_tracer(name.clone());
    let subscriber = get_subscriber(name, env_filter, sink, &tracer_provider);
    init_subscriber(subscriber);

    tracer_provider
}
