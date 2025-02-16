use domain::{Infra, TraqClient};

use super::BotImplInner;
use crate::cli::help::CompleteHelp;
use crate::error::Error;

impl BotImplInner {
    pub(super) async fn handle_help_command<I>(
        &self,
        infra: &I,
        help: CompleteHelp,
    ) -> Result<(), Error>
    where
        I: Infra,
    {
        let message = crate::HELP_TEMPLATE.replace("BOT_cnvtr", &self.name);
        let client = infra.traq_client();
        match help {
            CompleteHelp::Channel(channel_id) => {
                client.send_message(&channel_id, &message, false).await?;
            }
            CompleteHelp::Dm(user_id) => {
                client
                    .send_direct_message(&user_id, &message, false)
                    .await?;
            }
        }
        Ok(())
    }
}
