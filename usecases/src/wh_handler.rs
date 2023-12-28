use serde_json::Value;

pub trait WebhookHandler: Clone + Send + Sync + 'static {
    type Error: Send + Sync + 'static;

    fn github_webhook<'a, H, K, V>(
        &self,
        headers: H,
        payload: Value,
    ) -> Result<Option<String>, Self::Error>
    where
        H: Iterator<Item = (&'a K, &'a V)>,
        K: AsRef<[u8]> + ?Sized + 'static,
        V: AsRef<[u8]> + ?Sized + 'static;

    fn gitea_webhook<'a, H, K, V>(
        &self,
        headers: H,
        payload: Value,
    ) -> Result<Option<String>, Self::Error>
    where
        H: Iterator<Item = (&'a K, &'a V)>,
        K: AsRef<[u8]> + ?Sized + 'static,
        V: AsRef<[u8]> + ?Sized + 'static;

    fn clickup_webhook<'a, H, K, V>(
        &self,
        headers: H,
        payload: Value,
    ) -> Result<Option<String>, Self::Error>
    where
        H: Iterator<Item = (&'a K, &'a V)>,
        K: AsRef<[u8]> + ?Sized + 'static,
        V: AsRef<[u8]> + ?Sized + 'static;
}
