use crate::notifications::NotificationSender;
use reqwest::Client;
use serde_json::json;
use std::collections::HashMap;
use std::error::Error;

pub struct TelegramNotifier {
    bot_token: String,
    chat_id: String,
    client: Client,
}

impl TelegramNotifier {
    pub fn new(bot_token: String, chat_id: String) -> Self {
        Self {
            bot_token,
            chat_id,
            client: Client::new(),
        }
    }

    pub fn create(
        params: HashMap<String, String>,
    ) -> Result<Box<dyn NotificationSender>, Box<dyn Error>> {
        let bot_token = params
            .get("bot_token")
            .ok_or("Telegram bot token not configured")?
            .clone();

        let chat_id = params
            .get("chat_id")
            .ok_or("Telegram chat ID not configured")?
            .clone();

        Ok(Box::new(TelegramNotifier::new(bot_token, chat_id)))
    }
}

#[async_trait::async_trait]
impl NotificationSender for TelegramNotifier {
    async fn send(&self, message: &str) -> Result<(), Box<dyn Error>> {
        let url = format!("https://api.telegram.org/bot{}/sendMessage", self.bot_token);

        let body = json!({
            "chat_id": self.chat_id,
            "text": message,
        });

        let response = self.client.post(&url).json(&body).send().await?;

        if response.status().is_success() {
            Ok(())
        } else {
            let status = response.status();
            let error_text = response.text().await?;
            Err(format!(
                "Failed to send Telegram message: {} - {}",
                status, error_text
            )
            .into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use once_cell::sync::Lazy;
    use serde::Deserialize;
    use std::fs;
    use std::path::PathBuf;

    #[derive(Deserialize, Debug)]
    struct TestConfig {
        telegram: TelegramTestConfig,
    }

    #[derive(Deserialize, Debug)]
    struct TelegramTestConfig {
        bot_token: String,
        chat_id: String,
    }

    static TEST_CREDENTIALS: Lazy<TelegramTestConfig> = Lazy::new(|| {
        // Try to load from test-config.toml first (for local development)
        let config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("test-config.toml");

        if let Ok(content) = fs::read_to_string(config_path) {
            if let Ok(config) = toml::from_str::<TestConfig>(&content) {
                return config.telegram;
            }
        }

        // Fallback to environment variables
        TelegramTestConfig {
            bot_token: std::env::var("TEST_TELEGRAM_BOT_TOKEN")
                .unwrap_or_else(|_| "dummy_token".to_string()),
            chat_id: std::env::var("TEST_TELEGRAM_CHAT_ID")
                .unwrap_or_else(|_| "dummy_chat_id".to_string()),
        }
    });

    #[test]
    fn test_telegram_notifier_creation() {
        let mut params = HashMap::new();
        params.insert("bot_token".to_string(), TEST_CREDENTIALS.bot_token.clone());
        params.insert("chat_id".to_string(), TEST_CREDENTIALS.chat_id.clone());

        let result = TelegramNotifier::create(params);
        assert!(result.is_ok());
    }

    #[test]
    fn test_telegram_notifier_missing_params() {
        let params = HashMap::new();
        let result = TelegramNotifier::create(params);
        assert!(result.is_err());
    }

    #[test]
    #[ignore] // Only run when credentials are configured
    fn test_telegram_send() {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let notifier = TelegramNotifier::new(
            TEST_CREDENTIALS.bot_token.clone(),
            TEST_CREDENTIALS.chat_id.clone(),
        );

        runtime.block_on(async {
            match notifier.send("Test message from Rust").await {
                Ok(_) => println!("Message sent successfully"),
                Err(e) => println!("Failed to send message: {}", e),
            }
        });
    }
}
