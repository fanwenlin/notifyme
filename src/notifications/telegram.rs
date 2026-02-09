use crate::notifications::{NotificationOptions, NotificationSender};
use reqwest::Client;
use serde_json::{json, Value};
use std::error::Error;

fn build_client() -> Client {
    std::panic::catch_unwind(|| Client::new()).unwrap_or_else(|_| {
        Client::builder()
            .no_proxy()
            .build()
            .expect("Failed to build reqwest client")
    })
}

pub struct TelegramNotifier {
    bot_token: String,
    chat_id: String,
    at: Option<String>,
    client: Client,
}

impl TelegramNotifier {
    pub fn new(bot_token: String, chat_id: String, at: Option<String>) -> Self {
        Self {
            bot_token,
            chat_id,
            at,
            client: build_client(),
        }
    }

    fn format_message(&self, message: &str, mention_user: bool) -> String {
        if !mention_user {
            return message.to_string();
        }

        let Some(at) = self.at.as_ref() else {
            return message.to_string();
        };
        let at = at.trim();
        if at.is_empty() {
            return message.to_string();
        }

        format!("{} {}", at, message)
    }

    async fn send_inner(
        &self,
        message: &str,
        options: &NotificationOptions,
    ) -> Result<(), Box<dyn Error>> {
        let url = format!("https://api.telegram.org/bot{}/sendMessage", self.bot_token);
        let text = self.format_message(message, options.mention_user);

        let mut body = json!({
            "chat_id": self.chat_id,
            "text": text,
        });

        if options.silent {
            body["disable_notification"] = Value::Bool(true);
        }

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

#[async_trait::async_trait]
impl NotificationSender for TelegramNotifier {
    async fn send(&self, message: &str) -> Result<(), Box<dyn Error>> {
        self.send_with_options(message, &NotificationOptions::default())
            .await
    }

    async fn send_with_options(
        &self,
        message: &str,
        options: &NotificationOptions,
    ) -> Result<(), Box<dyn Error>> {
        self.send_inner(message, options).await
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
            None,
        );

        runtime.block_on(async {
            match notifier.send("Test message from Rust").await {
                Ok(_) => println!("Message sent successfully"),
                Err(e) => println!("Failed to send message: {}", e),
            }
        });
    }
}
