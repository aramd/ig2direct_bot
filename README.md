# Rust Telegram Bot for Instagram Reels

This is an example project written in Rust that demonstrates how to create a simple Telegram bot to retrieve direct reel URLs from Instagram using Selenium. Essentially, it's a Telegram bot for interacting with Instagram, with the first example focusing on obtaining direct links from video reels.

## Features

- Send a reel's URL to the Telegram bot, and it will provide the direct video URL.

## Setup

### Prerequisites

- You need Docker and Docker Compose installed on your host machine.

### Configuration

1. **Create a Telegram Bot**:
    - Follow the instructions to create a bot using [BotFather](https://core.telegram.org/bots#botfather) on Telegram.
    - Obtain the bot token from BotFather.

2. **Set Up `Secrets.toml`**:
    - Create (or Update) a `Secrets.toml` file in the root directory of the project.
    - Add the following information to `Secrets.toml`:

      ```toml
      tg_token = "YOUR_TELEGRAM_BOT_TOKEN"
      ig_username = "YOUR_INSTAGRAM_USERNAME"
      ig_password = "YOUR_INSTAGRAM_PASSWORD"
      ```

### Running the Bot

1. **Build and Launch the Bot**:
    - Use Docker Compose to build and run the bot:

      ```sh
      docker-compose up
      ```

   This command will build the Docker image and start the Telegram bot.

## Usage

- Send a reel's URL to your Telegram bot.
- The bot will respond with the direct video URL.

## Notes

- Ensure that your `Secrets.toml` file contains the correct Telegram bot token and Instagram credentials.
- Docker Compose is required to run the bot. Make sure Docker Compose is installed on your host machine.

## Example `Secrets.toml`

```toml
tg_token = "123456789:ABCDEF1234567890abcdef1234567890abcdef"
ig_username = "your_instagram_username"
ig_password = "your_instagram_password"
```
