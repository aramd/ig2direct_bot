use crate::error::Error;
use crate::secrets;

use regex::Regex;
use secrets::Secrets;
use teloxide::dispatching::UpdateFilterExt;
use teloxide::dptree;
use teloxide::prelude::*;
use teloxide::types::{Message, Update};
use teloxide::utils::command::BotCommands;
use thirtyfour::prelude::*;
use tokio::time::{sleep, Duration};

pub struct TgBot {
    pub bot: Bot,
    pub driver: WebDriver,
}

impl TgBot {
    pub async fn new(secrets: Secrets) -> Result<Self, Error> {
        let caps = DesiredCapabilities::chrome();

        let driver = WebDriver::new("http://selenium:4444", caps).await?;
        tracing::debug!("WebDriver is created successfully.");
        driver
            .goto("https://www.instagram.com/accounts/login/")
            .await?;
        sleep(Duration::from_millis(1500)).await; // TODO better way

        let username = driver.find(By::Css("input[name='username']")).await?;
        username.send_keys(secrets.ig_username).await?;

        let password = driver.find(By::Css("input[name='password']")).await?;
        password.send_keys(secrets.ig_password).await?;

        let login_button = driver.find(By::XPath("//button[@type='submit']")).await?;
        login_button.click().await?;

        tracing::debug!("Successfully logged in");
        Ok(TgBot {
            bot: Bot::new(secrets.tg_token),
            driver,
        })
    }
    pub async fn start(&self) {
        let handler = Update::filter_message()
            .branch(
                dptree::entry()
                    .filter_command::<Commands>()
                    .endpoint(start_commands_handler),
            )
            .branch(
                dptree::filter(|msg: Message| msg.chat.is_chat()).endpoint(direct_chat_handler),
            );

        Dispatcher::builder(self.bot.clone(), handler)
            .dependencies(dptree::deps![self.driver.clone()])
            .default_handler(|upd| async move {
                tracing::warn!("Unhandled update: {:?}", upd);
            })
            .error_handler(LoggingErrorHandler::with_custom_text(
                "An error has occurred in the dispatcher",
            ))
            .enable_ctrlc_handler()
            .build()
            .dispatch()
            .await;
    }
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
enum Commands {
    Start,
}

async fn direct_chat_handler(bot: Bot, driver: WebDriver, msg: Message) -> Result<(), Error> {
    tracing::debug!("Received a message from bot: {:?}", msg);

    let chat_id = msg.chat.id;
    let instagram_link = msg.text().unwrap_or("No text");

    driver.get(instagram_link).await?;
    sleep(Duration::from_millis(1500)).await; // TODO better way
    let source = driver.source().await?;

    let re = Regex::new(r"BaseURL>(https:\\/\\/scontent\.cdninstagram\.com.*?)\\u003C\\/BaseURL>")
        .unwrap();

    if let Some(captures) = re.captures(&source) {
        let url = captures
            .get(1)
            .expect("Could not get regex group from matched data")
            .as_str();
        let url = url.replace("\\/", "/").replace("&amp;", "&");
        tracing::debug!("Found URL: {:?}", url);
        let mut message = bot.send_message(chat_id, url);
        message.disable_web_page_preview = Some(false);
        message.await?;
    } else {
        tracing::debug!("Could not find video URL in HTML: {:?}", source);
        bot.send_message(chat_id, "Please provide a valid Instagram video link. ☠️")
            .await?;
    }

    Ok(())
}

async fn start_commands_handler(bot: Bot, msg: Message, cmd: Commands) -> Result<(), Error> {
    let text = match cmd {
        Commands::Start => {
            format!("👋 Hello, {}!\n Send me an Instagram video or reels post, and I will provide the direct link. 🎉🌟🎉\nNote: This is an experimental bot, and the provided video may be without audio. 🚧", msg.from().unwrap().first_name)
        }
    };

    bot.send_message(msg.chat.id, text).await?;
    Ok(())
}
