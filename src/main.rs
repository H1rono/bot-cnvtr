use std::net::SocketAddr;

use axum::{
    body::Bytes,
    extract::State,
    http::{HeaderMap, StatusCode},
    routing::post,
    Router,
};

use traq_bot_http::{Event, RequestParser};

use bot_cnvtr::Config;

#[tokio::main]
async fn main() {
    let Config(bot_config, _db_config) = Config::from_env().unwrap();
    let parser = RequestParser::new(&bot_config.verification_token);
    let app = Router::new().route("/", post(handler)).with_state(parser);
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler(
    State(parser): State<RequestParser>,
    headers: HeaderMap,
    body: Bytes,
) -> StatusCode {
    use Event::*;
    match parser.parse(headers, &body) {
        Ok(Joined(payload)) => {
            println!("チャンネル {} に参加しました。", payload.channel.name);
            StatusCode::NO_CONTENT
        }
        Ok(Left(payload)) => {
            println!("チャンネル {} から退出しました。", payload.channel.name);
            StatusCode::NO_CONTENT
        }
        Ok(MessageCreated(payload)) => {
            print!(
                "{}さんがメッセージを投稿しました。\n内容: {}\n",
                payload.message.user.display_name, payload.message.text
            );
            StatusCode::NO_CONTENT
        }
        Ok(_) => StatusCode::NO_CONTENT,
        Err(err) => {
            eprintln!("ERROR: {err}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
