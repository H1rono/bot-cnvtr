use axum::{
    body::Bytes,
    extract::State,
    http::{HeaderMap, StatusCode},
};

use super::AppState;

pub(super) async fn event(
    State(st): State<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> StatusCode {
    match st.parser.parse(headers, &body) {
        Ok(event) => match st.bot.handle_event(st.db.as_ref(), event).await {
            Ok(_) => StatusCode::NO_CONTENT,
            Err(err) => {
                eprintln!("ERROR: {err}");
                eprintln!("{err:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
        },
        Err(err) => {
            eprintln!("ERROR: {err}");
            eprintln!("{err:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
