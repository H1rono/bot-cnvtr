use serde_json::Value;

use usecases::WebhookHandler;

#[derive(Clone)]
pub struct WHandlerWrapper<W: WebhookHandler>(pub W);

impl<W> WebhookHandler for WHandlerWrapper<W>
where
    W: WebhookHandler,
    usecases::Error: From<W::Error>,
{
    type Error = usecases::Error;

    fn github_webhook<'a, H, K, V>(
        &self,
        headers: H,
        payload: Value,
    ) -> Result<Option<String>, Self::Error>
    where
        H: Iterator<Item = (&'a K, &'a V)>,
        K: AsRef<[u8]> + ?Sized + 'static,
        V: AsRef<[u8]> + ?Sized + 'static,
    {
        Ok(self.0.github_webhook(headers, payload)?)
    }

    fn gitea_webhook<'a, H, K, V>(
        &self,
        headers: H,
        payload: Value,
    ) -> Result<Option<String>, Self::Error>
    where
        H: Iterator<Item = (&'a K, &'a V)>,
        K: AsRef<[u8]> + ?Sized + 'static,
        V: AsRef<[u8]> + ?Sized + 'static,
    {
        Ok(self.0.gitea_webhook(headers, payload)?)
    }

    fn clickup_webhook<'a, H, K, V>(
        &self,
        headers: H,
        payload: Value,
    ) -> Result<Option<String>, Self::Error>
    where
        H: Iterator<Item = (&'a K, &'a V)>,
        K: AsRef<[u8]> + ?Sized + 'static,
        V: AsRef<[u8]> + ?Sized + 'static,
    {
        Ok(self.0.clickup_webhook(headers, payload)?)
    }
}
