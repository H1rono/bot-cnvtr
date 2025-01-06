use std::sync::Arc;

use http::{Request, Response};
use http_body::Body;
use tower::util::BoxCloneSyncService;

use domain::Infra;

#[must_use]
pub trait Bot<I: Infra> {
    fn build_service<B>(
        self,
        infra: Arc<I>,
    ) -> BoxCloneSyncService<Request<B>, Response<String>, domain::Error>
    where
        B: Body + Send + 'static,
        B::Data: Send + 'static,
        B::Error: Into<Box<dyn std::error::Error + Send + Sync + 'static>>;
}
