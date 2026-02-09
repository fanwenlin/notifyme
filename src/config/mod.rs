use crate::notifications::NotificationSender;
use log::error;
use quick_xml::de::from_str;
use quick_xml::se::to_string;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const CONFIG_DIR: &str = ".config/notifyme/configs/";

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename = "config-set")]
pub struct ConfigSet {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "notification-configs")]
    pub notification_configs: NotificationConfigs,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NotificationConfigs {
    #[serde(default)]
    #[serde(rename = "$value")]
    pub configs: Vec<NotificationConfigType>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum NotificationConfigType {
    Telegram(TelegramConfig),
    Email(EmailConfig),
    Http(HttpConfig),
    Cmd(CommandConfig),
    #[serde(rename = "sms-twilio")]
    TwilioSms(TwilioSmsConfig),
    #[serde(rename = "phone-call")]
    PhoneCall(PhoneCallConfig),
    #[serde(rename = "lark")]
    Lark(LarkConfig),
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TelegramConfig {
    pub token: String,
    pub chat_id: String,
    #[serde(default)]
    pub at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct LarkConfig {
    pub webhook_url: String,
    pub sign_key: String,
    pub at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct EmailConfig {
    pub to: String,
    pub from: String,
    pub subject: Option<String>,
    pub body: Option<String>,
    pub smtp: SmtpConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub encryption: Option<String>,
    pub auth: Option<bool>,
    pub debug: Option<bool>,
    pub timeout: Option<u32>,
    pub tls_verify: Option<bool>,
    pub tls_ca_certs: Option<String>,
    pub tls_key: Option<String>,
    pub tls_cert: Option<String>,
    pub tls_ciphers: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct HttpConfig {
    pub url: String,
    pub method: String,
    pub headers: Option<Vec<HttpHeader>>,
    pub body: Option<String>,
    pub timeout: Option<u32>,
    pub retry: Option<u32>,
    pub retry_delay: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct HttpHeader {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CommandConfig {
    pub command: String,
    pub args: Option<String>,
    pub timeout: Option<u32>,
    pub retry: Option<u32>,
    pub retry_delay: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TwilioSmsConfig {
    pub account_sid: String,
    pub auth_token: String,
    pub from: String,
    pub to: String,
    pub body: String,
    pub media_urls: Option<Vec<String>>,
    pub mms: Option<bool>,
    pub sender_id: Option<String>,
    pub carrier: Option<String>,
    pub carrier_lookup: Option<bool>,
    pub carrier_lookup_country_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PhoneCallConfig {
    pub account_sid: String,
    pub auth_token: String,
    pub from: String,
    pub to: String,
    pub url: String,
    pub method: Option<String>,
    pub timeout: Option<u32>,
    pub record: Option<bool>,
    pub status_callback: Option<String>,
    pub status_callback_method: Option<String>,
    pub machine_detection: Option<bool>,
    pub machine_detection_timeout: Option<u32>,
    pub machine_detection_url: Option<String>,
    pub machine_detection_method: Option<String>,
}

impl ConfigSet {
    pub fn new(name: String) -> Self {
        Self {
            name,
            notification_configs: NotificationConfigs {
                configs: Vec::new(),
            },
        }
    }

    pub fn get_notification_handlers(
        &self,
    ) -> Result<Vec<Box<dyn NotificationSender>>, Box<dyn std::error::Error>> {
        let mut handlers = Vec::new();

        for config in &self.notification_configs.configs {
            let handler = crate::notifications::create_notification_sender(config)?;
            handlers.push(handler);
        }
        Ok(handlers)
    }
}

pub struct ConfigManager {
    config_dir: PathBuf,
}

impl ConfigManager {
    pub fn new() -> Self {
        Self {
            config_dir: get_config_dir(),
        }
    }

    pub fn read_config(&self, name: &str) -> Result<ConfigSet, Box<dyn std::error::Error>> {
        let config_path = self.config_dir.join(format!("{}.xml", name));
        let content = match fs::read_to_string(&config_path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!(
                    "Error reading config set '{}', path '{}' : {}",
                    name,
                    config_path.display(),
                    e
                );
                return Err(Box::new(e));
            }
        };
        let config_set: ConfigSet = from_str(&content)?;
        Ok(config_set)
    }

    pub fn write_config(&self, config_set: &ConfigSet) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = self.config_dir.join(format!("{}.xml", config_set.name));
        let content = match to_string(&config_set) {
            Ok(content) => content,
            Err(e) => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("convert config_set to xml failed: {}", e),
                )))
            }
        };

        // check if path exists
        if !self.config_dir.exists() {
            match fs::create_dir_all(self.config_dir.clone()) {
                Ok(_) => {}
                Err(e) => {
                    error!(
                        "Failed to create config dir {}, error {}",
                        self.config_dir.display(),
                        e
                    );
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Failed to create config dir: {}", e),
                    )));
                }
            }
        }

        match fs::write(config_path.clone(), content) {
            Ok(_) => {}
            Err(e) => {
                error!(
                    "Failed to write config {}, error {}",
                    config_path.display(),
                    e
                );
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to write config : {}", e),
                )));
            }
        }
        Ok(())
    }

    pub fn list_configs(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let entries = fs::read_dir(&self.config_dir)?;
        let mut configs = Vec::new();

        for entry in entries {
            let entry = entry?;
            if let Some(name) = entry.file_name().to_str() {
                if name.ends_with(".xml") {
                    configs.push(name.trim_end_matches(".xml").to_string());
                }
            }
        }

        Ok(configs)
    }

    pub fn delete_config(&self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = self.config_dir.join(format!("{}.xml", name));
        if config_path.exists() {
            fs::remove_file(config_path.clone()).map_err(|e| {
                error!(
                    "Failed to delete config {}, error: {}",
                    config_path.display(),
                    e
                );
                e
            })?;
        } else {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Config set '{}' not found", name),
            )));
        }
        Ok(())
    }
}

// Keep these helper functions for backward compatibility
pub fn get_config_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(CONFIG_DIR)
}
