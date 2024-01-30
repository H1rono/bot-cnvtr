pub struct ConfigComposite {
    pub bot_config: ::bot::Config,
    pub router_config: router::Config,
    pub client_config: traq_client::Config,
    pub repo_config: repository::Config,
}

impl ConfigComposite {
    pub fn from_env() -> envy::Result<Self> {
        Ok(Self {
            bot_config: ::bot::Config::from_env()?,
            router_config: router::Config::from_env()?,
            client_config: traq_client::Config::from_env()?,
            repo_config: repository::Config::from_env()?,
        })
    }
}