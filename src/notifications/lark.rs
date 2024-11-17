use crate::notifications::NotificationSender;
use base64::{engine::general_purpose::STANDARD, Engine};
use hmac::{Hmac, Mac};
use log::{debug, error, info};
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use sha2::Sha256;
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct LarkNotifier {
    webhook_url: String,
    sign_key: String,
    at_user_id: Option<String>,
    client: Client,
}

impl LarkNotifier {
    pub fn new(webhook_url: String, sign_key: String, at_user_id: Option<String>) -> Self {
        Self {
            webhook_url,
            sign_key,
            at_user_id,
            client: Client::new(),
        }
    }

    fn generate_sign(&self, timestamp: u64) -> String {
        let string_to_sign = format!("{}\n{}", timestamp, self.sign_key);
        let mut mac = Hmac::<Sha256>::new_from_slice(string_to_sign.as_bytes())
            .expect("HMAC can take key of any size");
        let result = mac.finalize();
        STANDARD.encode(result.into_bytes())
    }

    fn format_message(&self, message: &str) -> String {
        if let Some(user_id) = &self.at_user_id {
            format!("<at user_id=\"{}\"></at>\n{}", user_id, message)
        } else {
            message.to_string()
        }
    }
}

#[derive(Deserialize, Debug)]
struct LarkResponse {
    code: i32,
    msg: String,
}

#[async_trait::async_trait]
impl NotificationSender for LarkNotifier {
    async fn send(&self, message: &str) -> Result<(), Box<dyn Error>> {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        let sign = self.generate_sign(timestamp);
        let formatted_message = self.format_message(message);

        let body = json!({
            "timestamp": timestamp,
            "sign": sign,
            "msg_type": "text",
            "content": {
                "text": formatted_message
            }
        });

        let response = self
            .client
            .post(&self.webhook_url)
            .json(&body)
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;
        let response_body: LarkResponse = match serde_json::from_str(&text) {
            Ok(body) => body,
            Err(e) => {
                error!("Failed to parse Lark response: {}, body: {}", e, text);
                return Err(e.into());
            }
        };

        if status.is_success() && response_body.code == 0 {
            info!("Lark message sent successfully");
            Ok(())
        } else {
            error!(
                "Failed to send Lark message\nStatus: {}\nCode: {}\nMsg: {}",
                status, response_body.code, response_body.msg
            );
            Err(format!(
                "Failed to send Lark message: status={}, code={}, msg={}",
                status, response_body.code, response_body.msg
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
        lark: LarkTestConfig,
    }

    #[derive(Deserialize, Debug)]
    struct LarkTestConfig {
        webhook_url: String,
        sign_key: String,
        at_user_id: String,
    }

    static TEST_CREDENTIALS: Lazy<LarkTestConfig> = Lazy::new(|| {
        // Try to load from test-config.toml first (for local development)
        let config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("test-config.toml");

        if let Ok(content) = fs::read_to_string(config_path) {
            if let Ok(config) = toml::from_str::<TestConfig>(&content) {
                return config.lark;
            }
        }

        // Fallback to environment variables
        LarkTestConfig {
            webhook_url: std::env::var("TEST_LARK_WEBHOOK_URL")
                .unwrap_or_else(|_| "dummy_webhook_url".to_string()),
            sign_key: std::env::var("TEST_LARK_SIGN_KEY")
                .unwrap_or_else(|_| "dummy_sign_key".to_string()),
            at_user_id: std::env::var("TEST_LARK_AT_USER_ID")
                .unwrap_or_else(|_| "dummy_at_user_id".to_string()),
        }
    });

    #[test]
    fn test_generate_sign() {
        let notifier = LarkNotifier::new(
            "dummy_url".to_string(),
            "tYIt547EbBsGp121BKiRtfzw0DcbnDALcKGOBxVW+e8=".to_string(),
            None,
        );

        let sign = notifier.generate_sign(1730999346);
        assert_eq!(sign, "DfgglLrO1biOt0KM7goV43WDrRiM7N2YHczNXLUtV0U=");
    }

    #[tokio::test]
    #[ignore] // Ignore by default as it sends real messages
    async fn test_lark_notifier_with_at() {
        let notifier = LarkNotifier::new(
            TEST_CREDENTIALS.webhook_url.clone(),
            TEST_CREDENTIALS.sign_key.clone(),
            Some(TEST_CREDENTIALS.at_user_id.clone()),
        );

        // Test message formatting with @mention
        let formatted = notifier.format_message("Hello world");
        assert!(formatted.contains(&format!(
            "<at user_id=\"{}\"></at>",
            TEST_CREDENTIALS.at_user_id
        )));

        // Test actual message sending
        let result = notifier.send("Test message with @mention from Rust").await;
        assert!(result.is_ok(), "Failed to send message: {:?}", result.err());
    }

    #[tokio::test]
    #[ignore] // Ignore by default as it sends real messages
    async fn test_lark_notifier_without_at() {
        let notifier = LarkNotifier::new(
            TEST_CREDENTIALS.webhook_url.clone(),
            TEST_CREDENTIALS.sign_key.clone(),
            None,
        );

        // Test message formatting without @mention
        let formatted = notifier.format_message("Hello world");
        assert_eq!(formatted, "Hello world");
        assert!(!formatted.contains("<at"));

        // Test actual message sending
        let result = notifier
            .send("Test message without @mention from Rust")
            .await;
        assert!(result.is_ok(), "Failed to send message: {:?}", result.err());
    }

    #[test]
    fn test_format_message() {
        let notifier = LarkNotifier::new(
            "dummy_url".to_string(),
            "dummy_key".to_string(),
            Some(TEST_CREDENTIALS.at_user_id.clone()),
        );

        // Test with @mention
        let formatted = notifier.format_message("Hello world");
        assert_eq!(
            formatted,
            format!(
                "<at user_id=\"{}\"></at>\nHello world",
                TEST_CREDENTIALS.at_user_id
            )
        );

        // Test without @mention
        let notifier_without_at =
            LarkNotifier::new("dummy_url".to_string(), "dummy_key".to_string(), None);
        let formatted = notifier_without_at.format_message("Hello world");
        assert_eq!(formatted, "Hello world");
    }
}
