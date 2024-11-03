use crate::notifications::NotificationSender;
use quick_xml::de::from_str;
use quick_xml::se::to_string;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;

const CONFIG_DIR: &str = ".config/notifyme/configs/";
const DEFAULT_CONFIG_NAME: &str = "default";

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigSet {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "notification-configs")]
    pub notification_configs: NotificationConfigs,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationConfigs {
    #[serde(rename = "config")]
    pub configs: Vec<NotificationConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationConfig {
    #[serde(rename = "@type")]
    pub config_type: String,
    pub to: Option<String>,
    pub from: Option<String>,
    pub subject: Option<String>,
    pub body: Option<String>,
    pub smtp: Option<SmtpConfig>,
    pub token: Option<String>,
    pub chat_id: Option<String>,
    pub message: Option<String>,
    pub parse_mode: Option<String>,
    pub disable_web_page_preview: Option<bool>,
    pub disable_notification: Option<bool>,
    pub command: Option<String>,
    pub args: Option<String>,
    pub timeout: Option<u32>,
    pub retry: Option<u32>,
    pub retry_delay: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
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
            let handler: Box<dyn NotificationSender> = match config.config_type.as_str() {
                "telegram" => {
                    let token = config.token.as_ref().ok_or("Missing telegram token")?;
                    let chat_id = config.chat_id.as_ref().ok_or("Missing telegram chat_id")?;
                    Box::new(crate::notifications::telegram::TelegramNotifier::new(
                        token.clone(),
                        chat_id.clone(),
                    ))
                }
                // Add other notification types here
                _ => {
                    return Err(
                        format!("Unsupported notification type: {}", config.config_type).into(),
                    )
                }
            };

            handlers.push(handler);
        }
        Ok(handlers)
    }

    pub fn add_notification_config(&mut self, config_type: &str, params: HashMap<String, String>) {
        let value_params: HashMap<String, Value> = params
            .into_iter()
            .map(|(k, v)| (k, Value::String(v)))
            .collect();

        self.notification_configs.configs.push(NotificationConfig {
            config_type: config_type.to_string(),
            to: None,
            from: None,
            subject: None,
            body: None,
            smtp: None,
            token: None,
            chat_id: None,
            message: None,
            parse_mode: None,
            disable_web_page_preview: None,
            disable_notification: None,
            command: None,
            args: None,
            timeout: None,
            retry: None,
            retry_delay: None,
        });
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        write_config(self)
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
        let content = to_string(&config_set)?;
        fs::write(config_path, content)?;
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
}

// Keep these helper functions for backward compatibility
pub fn get_config_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(CONFIG_DIR)
}

pub fn read_config(name: &str) -> Result<ConfigSet, Box<dyn std::error::Error>> {
    ConfigManager::new().read_config(name)
}

pub fn write_config(config_set: &ConfigSet) -> Result<(), Box<dyn std::error::Error>> {
    ConfigManager::new().write_config(config_set)
}
