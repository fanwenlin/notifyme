use crate::notifications::NotificationSender;
use log::{error, info};
use quick_xml::de::from_str;
use quick_xml::se::to_string;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;
use std::{collections::HashMap, io::Error};

const CONFIG_DIR: &str = ".config/notifyme/configs/";
const DEFAULT_CONFIG_NAME: &str = "default";

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "config-set")]
pub struct ConfigSet {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "notification-configs")]
    pub notification_configs: NotificationConfigs,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationConfigs {
    #[serde(default)]
    #[serde(rename = "$value")]
    pub configs: Vec<NotificationConfigType>,
}

#[derive(Debug, Serialize, Deserialize)]
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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TelegramConfig {
    pub token: String,
    pub chat_id: String,
    pub message: Option<String>,
    pub parse_mode: Option<String>,
    pub disable_web_page_preview: Option<bool>,
    pub disable_notification: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailConfig {
    pub to: String,
    pub from: String,
    pub subject: Option<String>,
    pub body: Option<String>,
    pub smtp: SmtpConfig,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpConfig {
    pub url: String,
    pub method: String,
    pub headers: Option<Vec<HttpHeader>>,
    pub body: Option<String>,
    pub timeout: Option<u32>,
    pub retry: Option<u32>,
    pub retry_delay: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpHeader {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandConfig {
    pub command: String,
    pub args: Option<String>,
    pub timeout: Option<u32>,
    pub retry: Option<u32>,
    pub retry_delay: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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
            let handler: Box<dyn NotificationSender> = match config {
                NotificationConfigType::Telegram(TelegramConfig { token, chat_id, .. }) => {
                    Box::new(crate::notifications::telegram::TelegramNotifier::new(
                        token.clone(),
                        chat_id.clone(),
                    ))
                }
                NotificationConfigType::Email(_) => {
                    return Err("Email notification not implemented yet".into())
                }
                NotificationConfigType::Http(_) => {
                    return Err("HTTP notification not implemented yet".into())
                }
                NotificationConfigType::Cmd(_) => {
                    return Err("Command notification not implemented yet".into())
                }
                NotificationConfigType::TwilioSms(_) => {
                    return Err("Twilio SMS notification not implemented yet".into())
                }
                NotificationConfigType::PhoneCall(_) => {
                    return Err("Phone call notification not implemented yet".into())
                }
            };

            handlers.push(handler);
        }
        Ok(handlers)
    }

    // unused temporarily, will be used for config write in the future
    pub fn add_notification_config(&mut self, config_type: &str, params: HashMap<String, String>) {
        // let value_params: HashMap<String, Value> = params
        //     .into_iter()
        //     .map(|(k, v)| (k, Value::String(v)))
        //     .collect();

        // self.notification_configs.configs.push(NotificationConfig {
        //     config_type: config_type.to_string(),
        //     config: match config_type.as_str() {
        //         "telegram" => NotificationConfigType::Telegram(TelegramConfig {
        //             token: params.get("token").unwrap().clone(),
        //             chat_id: params.get("chat_id").unwrap().clone(),
        //             message: params.get("message").map(|v| v.clone()),
        //             parse_mode: params.get("parse_mode").map(|v| v.clone()),
        //             disable_web_page_preview: params
        //                 .get("disable_web_page_preview")
        //                 .map(|v| v.parse::<bool>().unwrap()),
        //             disable_notification: params
        //                 .get("disable_notification")
        //                 .map(|v| v.parse::<bool>().unwrap()),
        //         }),
        //         "email" => NotificationConfigType::Email(EmailConfig {
        //             to: params.get("to").unwrap().clone(),
        //             from: params.get("from").unwrap().clone(),
        //             subject: params.get("subject").map(|v| v.clone()),
        //             body: params.get("body").map(|v| v.clone()),
        //             smtp: SmtpConfig {
        //                 host: params.get("smtp_host").unwrap().clone(),
        //                 port: params.get("smtp_port").unwrap().parse::<u16>().unwrap(),
        //                 username: params.get("smtp_username").unwrap().clone(),
        //                 password: params.get("smtp_password").unwrap().clone(),
        //                 encryption: params.get("smtp_encryption").map(|v| v.clone()),
        //                 auth: params.get("smtp_auth").map(|v| v.parse::<bool>().unwrap()),
        //                 debug: params.get("smtp_debug").map(|v| v.parse::<bool>().unwrap()),
        //                 timeout: params
        //                     .get("smtp_timeout")
        //                     .map(|v| v.parse::<u32>().unwrap()),
        //                 tls_verify: params
        //                     .get("smtp_tls_verify")
        //                     .map(|v| v.parse::<bool>().unwrap()),
        //                 tls_ca_certs: params.get("smtp_tls_ca_certs").map(|v| v.clone()),
        //                 tls_key: params.get("smtp_tls_key").map(|v| v.clone()),
        //                 tls_cert: params.get("smtp_tls_cert").map(|v| v.clone()),
        //                 tls_ciphers: params.get("smtp_tls_ciphers").map(|v| v.clone()),
        //             },
        //         }),
        //         "sms-twilio" => NotificationConfigType::TwilioSms(TwilioSmsConfig {
        //             account_sid: params.get("account_sid").unwrap().clone(),
        //             auth_token: params.get("auth_token").unwrap().clone(),
        //             from: params.get("from").unwrap().clone(),
        //             to: params.get("to").unwrap().clone(),
        //             body: params.get("body").unwrap().clone(),
        //             media_urls: params
        //                 .get("media_urls")
        //                 .map(|v| v.split(',').map(|s| s.trim().to_string()).collect()),
        //             mms: params.get("mms").map(|v| v.parse::<bool>().unwrap()),
        //             sender_id: params.get("sender_id").map(|v| v.clone()),
        //             carrier: params.get("carrier").map(|v| v.clone()),
        //             carrier_lookup: params
        //                 .get("carrier_lookup")
        //                 .map(|v| v.parse::<bool>().unwrap()),
        //             carrier_lookup_country_code: params
        //                 .get("carrier_lookup_country_code")
        //                 .map(|v| v.clone()),
        //         }),
        //         "phone-call" => NotificationConfigType::PhoneCall(PhoneCallConfig {
        //             account_sid: params.get("account_sid").unwrap().clone(),
        //             auth_token: params.get("auth_token").unwrap().clone(),
        //             from: params.get("from").unwrap().clone(),
        //             to: params.get("to").unwrap().clone(),
        //             url: params.get("url").unwrap().clone(),
        //             method: params.get("method").map(|v| v.clone()),
        //             timeout: params.get("timeout").map(|v| v.parse::<u32>().unwrap()),
        //             record: params.get("record").map(|v| v.parse::<bool>().unwrap()),
        //             status_callback: params.get("status_callback").map(|v| v.clone()),
        //             status_callback_method: params.get("status_callback_method").map(|v| v.clone()),
        //             machine_detection: params
        //                 .get("machine_detection")
        //                 .map(|v| v.parse::<bool>().unwrap()),
        //             machine_detection_timeout: params
        //                 .get("machine_detection_timeout")
        //                 .map(|v| v.parse::<u32>().unwrap()),
        //             machine_detection_url: params.get("machine_detection_url").map(|v| v.clone()),
        //             machine_detection_method: params
        //                 .get("machine_detection_method")
        //                 .map(|v| v.clone()),
        //         }),
        //         "cmd" => NotificationConfigType::Command(CommandConfig {
        //             command: params.get("command").unwrap().clone(),
        //             args: params.get("args").map(|v| v.clone()),
        //             timeout: params.get("timeout").map(|v| v.parse::<u32>().unwrap()),
        //             retry: params.get("retry").map(|v| v.parse::<u32>().unwrap()),
        //             retry_delay: params.get("retry_delay").map(|v| v.parse::<u32>().unwrap()),
        //         }),
        //         "http" => NotificationConfigType::Http(HttpConfig {
        //             url: params.get("url").unwrap().clone(),
        //             method: params.get("method").unwrap().clone(),
        //             headers: params.get("headers").map(|v| {
        //                 v.split(',')
        //                     .map(|s| HttpHeader {
        //                         key: s.split('=').next().unwrap().trim().to_string(),
        //                         value: s.split('=').last().unwrap().trim().to_string(),
        //                     })
        //                     .collect()
        //             }),
        //             body: params.get("body").map(|v| v.clone()),
        //             timeout: params.get("timeout").map(|v| v.parse::<u32>().unwrap()),
        //             retry: params.get("retry").map(|v| v.parse::<u32>().unwrap()),
        //             retry_delay: params.get("retry_delay").map(|v| v.parse::<u32>().unwrap()),
        //         }),
        //         _ => unreachable!(),
        //     },
        // });
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
