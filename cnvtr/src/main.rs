use std::error::Error;
use std::net::SocketAddr;
use std::time::Duration;

use axum::{extract::MatchedPath, http::Request, response::Response};
use tower_http::trace::TraceLayer;
use tracing::{info_span, Span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use ::bot::BotImpl;
use ::traq_client::ClientImpl;
use ::wh_handler::WebhookHandlerImpl;
use repository::RepositoryImpl;
use router::make_router;

pub mod app;
pub mod bot;
pub mod config;
pub mod infra;
pub mod repo;
pub mod traq_client;
pub mod wh_handler;

use config::ConfigComposite;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "example_tracing_aka_logging=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let ConfigComposite {
        bot_config,
        router_config,
        client_config,
        repo_config,
    } = ConfigComposite::from_env().or_else(|_| -> Result<_, Box<dyn Error>> {
        dotenvy::from_filename_override(".env")?;
        Ok(ConfigComposite::from_env()?)
    })?;
    let client = ClientImpl::from_config(client_config);
    let repo = RepositoryImpl::from_config(repo_config).await?;
    repo.migrate().await?;
    let infra = infra::InfraImpl::new_wrapped(repo, client);
    let bot = BotImpl::from_config(bot_config);
    let bot = bot::BotWrapper(bot);
    let wh = WebhookHandlerImpl::new();
    let wh = wh_handler::WHandlerWrapper(wh);
    let app = app::AppImpl(bot, wh);
    let router = make_router(router_config, infra, app)
        // `TraceLayer` is provided by tower-http so you have to add that as a dependency.
        // It provides good defaults but is also very customizable.
        //
        // See https://docs.rs/tower-http/0.1.1/tower_http/trace/index.html for more details.
        //
        // If you want to customize the behavior using closures here is how.
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    // Log the matched route's path (with placeholders not filled in).
                    // Use request.uri() or OriginalUri if you want the real path.
                    let matched_path = request
                        .extensions()
                        .get::<MatchedPath>()
                        .map(MatchedPath::as_str);

                    info_span!(
                        "http_request",
                        method = ?request.method(),
                        matched_path,
                        some_other_field = tracing::field::Empty,
                    )
                })
                .on_request(|_request: &Request<_>, _span: &Span| {
                    // You can use `_span.record("some_other_field", value)` in one of these
                    // closures to attach a value to the initially empty field in the info_span
                    // created above.
                })
                .on_response(|response: &Response, latency: Duration, span: &Span| {
                    span.record("latency", latency.as_secs_f32())
                        .record("status", response.status().as_str());
                }),
        );
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("listening on {} ...", addr);
    axum::serve(listener, router).await?;
    Ok(())
}
