use crate::config::{
    NotificationConfigType, TelegramConfig,
};

pub mod email;
pub mod http_request;
pub mod lark;
pub mod phone_call_twilio;
pub mod sms_twilio;
pub mod telegram;

#[derive(Debug, Default, Clone)]
pub struct NotificationOptions {
    pub silent: bool,
    pub mention_user: bool,
}

#[async_trait::async_trait]
pub trait NotificationSender: Send + Sync {
    async fn send(&self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.send_with_options(message, &NotificationOptions::default()).await
    }
    
    async fn send_with_options(&self, message: &str, options: &NotificationOptions) -> Result<(), Box<dyn std::error::Error>>;
}

pub fn create_notification_sender(
    config: &NotificationConfigType,
) -> Result<Box<dyn NotificationSender>, Box<dyn std::error::Error>> {
    match config {
        NotificationConfigType::Telegram(TelegramConfig { token, chat_id, at }) => Ok(Box::new(
            telegram::TelegramNotifier::new(token.clone(), chat_id.clone(), at.clone()),
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
