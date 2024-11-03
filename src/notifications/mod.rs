use std::collections::HashMap;

pub mod email;
pub mod http_request;
pub mod phone_call_twilio;
pub mod sms_twilio;
pub mod telegram;

#[async_trait::async_trait]
pub trait NotificationSender: Send + Sync {
    async fn send(&self, message: &str) -> Result<(), Box<dyn std::error::Error>>;
    // Optional: Add method for validation
    // fn validate(&self) -> Result<(), Box<dyn std::error::Error>> {
    //     Ok(())
    // }
}

// Registry for notification types (optional but recommended)
pub type NotificationFactory =
    fn(HashMap<String, String>) -> Result<Box<dyn NotificationSender>, Box<dyn std::error::Error>>;

use lazy_static::lazy_static;

lazy_static! {
    static ref NOTIFICATION_REGISTRY: HashMap<&'static str, NotificationFactory> = {
        let mut m = HashMap::new();
        m.insert("telegram", telegram::TelegramNotifier::create as NotificationFactory);
        // Add other notification types here
        m
    };
}

pub fn create_notification_sender(
    notification_type: &str,
    params: HashMap<String, String>,
) -> Result<Box<dyn NotificationSender>, Box<dyn std::error::Error>> {
    if let Some(factory) = NOTIFICATION_REGISTRY.get(notification_type) {
        factory(params)
    } else {
        Err(format!("Unsupported notification type: {}", notification_type).into())
    }
}
