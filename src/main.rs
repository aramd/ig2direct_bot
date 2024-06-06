mod bot;
mod error;
mod secrets;

use bot::TgBot;
use error::Error;
use secrets::Secrets;

use tracing::error;

// TODO
// https://stackoverflow.com/questions/70794535/selenium-unable-to-find-session-with-id-after-a-few-minutes-of-idling
// 1. keep alive, keep pinging Selenium, to keep session open
// 2. sschace for fast build

async fn run() -> Result<(), Error> {
    let secrets = Secrets::new()?;
    let tg_bot = TgBot::new(secrets).await?;
    tg_bot.start().await;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    if let Err(e) = run().await {
        error!("{}", e);
        return Err(e);
    }

    Ok(())
}
