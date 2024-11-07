use crate::config::{
    CommandConfig, EmailConfig, HttpConfig, NotificationConfigType, PhoneCallConfig,
    TelegramConfig, TwilioSmsConfig,
};
use std::collections::HashMap;

pub mod email;
pub mod http_request;
pub mod lark;
pub mod phone_call_twilio;
pub mod sms_twilio;
pub mod telegram;

#[async_trait::async_trait]
pub trait NotificationSender: Send + Sync {
    async fn send(&self, message: &str) -> Result<(), Box<dyn std::error::Error>>;
}

pub fn create_notification_sender(
    config: &NotificationConfigType,
) -> Result<Box<dyn NotificationSender>, Box<dyn std::error::Error>> {
    match config {
        NotificationConfigType::Telegram(TelegramConfig { token, chat_id, .. }) => Ok(Box::new(
            telegram::TelegramNotifier::new(token.clone(), chat_id.clone()),
        )),
        NotificationConfigType::Email(_) => Err("Email notification not implemented yet".into()),
        NotificationConfigType::Http(_) => Err("HTTP notification not implemented yet".into()),
        NotificationConfigType::Cmd(_) => Err("Command notification not implemented yet".into()),
        NotificationConfigType::TwilioSms(_) => {
            Err("Twilio SMS notification not implemented yet".into())
        }
        NotificationConfigType::PhoneCall(_) => {
            Err("Phone call notification not implemented yet".into())
        }
        NotificationConfigType::Lark(config) => Ok(Box::new(lark::LarkNotifier::new(
            config.webhook_url.clone(),
            config.sign_key.clone(),
            config.at.clone(),
        ))),
    }
}
