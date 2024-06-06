#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Toml parse error: {0}")]
    Toml(#[from] toml::de::Error),

    #[error("WebDriver error: {0}")]
    WebDriver(#[from] thirtyfour::error::WebDriverError),

    #[error("WebDriver error: {0}")]
    Telegram(#[from] teloxide::RequestError),
}
