use serde_json::Value;

use usecases::WebhookHandler;

#[derive(Clone)]
pub struct WHandlerWrapper<W: WebhookHandler>(pub W);

impl<I, W> WebhookHandler<I> for WHandlerWrapper<W>
where
    I: Infra<Error = domain::Error>,
    W: WebhookHandler<I>,
    domain::Error: From<W::Error>,
{
    type Error = domain::Error;

    fn github_webhook<'a, H, K, V>(
        &'a self,
        infra: &'a I,
        webhook: Webhook,
        headers: H,
        payload: Value,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send
    where
        H: Iterator<Item = (&'a K, &'a V)> + Send,
        K: AsRef<[u8]> + Send + ?Sized + 'static,
        V: AsRef<[u8]> + Send + ?Sized + 'static,
    {
        Ok(self
            .0
            .github_webhook(infra, webhook, headers, payload)
            .await?)
    }

    fn gitea_webhook<'a, H, K, V>(
        &'a self,
        infra: &'a I,
        webhook: Webhook,
        headers: H,
        payload: Value,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send
    where
        H: Iterator<Item = (&'a K, &'a V)> + Send,
        K: AsRef<[u8]> + Send + ?Sized + 'static,
        V: AsRef<[u8]> + Send + ?Sized + 'static,
    {
        Ok(self
            .0
            .gitea_webhook(infra, webhook, headers, payload)
            .await?)
    }

    fn clickup_webhook<'a, H, K, V>(
        &'a self,
        infra: &'a I,
        webhook: Webhook,
        headers: H,
        payload: Value,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send
    where
        H: Iterator<Item = (&'a K, &'a V)> + Send,
        K: AsRef<[u8]> + Send + ?Sized + 'static,
        V: AsRef<[u8]> + Send + ?Sized + 'static,
    {
        Ok(self
            .0
            .clickup_webhook(infra, webhook, headers, payload)
            .await?)
    }
}
