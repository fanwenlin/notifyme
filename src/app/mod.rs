use crate::config::{ConfigManager, ConfigSet, NotificationConfigs};
use crate::executor::CommandExecutor;
use log::{error, info};
use std::error::Error;

pub struct App {
    config_manager: ConfigManager,
}

impl App {
    pub fn new() -> Self {
        Self {
            config_manager: ConfigManager::new(),
        }
    }

    pub fn list_configs(&self) -> Result<(), Box<dyn Error>> {
        let configs = self.config_manager.list_configs()?;

        println!("Available configuration sets:");
        for config_name in configs {
            println!("- {}", config_name);
        }

        Ok(())
    }

    pub fn create_config(&self, name: &str) -> Result<(), Box<dyn Error>> {
        let config_set = ConfigSet::new(name.to_string());
        self.config_manager.write_config(&config_set)?;
        println!("Config set '{}' created.", name);
        Ok(())
    }

    pub async fn run_command(
        &self,
        config_set_name: &str,
        cmd: &str,
        args: &[String],
    ) -> Result<(), Box<dyn Error>> {
        info!("Running command with config set: {}", config_set_name);

        // 1. Read the config set
        let config_set = match self.config_manager.read_config(config_set_name) {
            Ok(config) => config,
            Err(e) => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("Failed to read config set '{}': {}", config_set_name, e),
                )))
            }
        };

        // 2. Get notification handlers
        let handlers = match config_set.get_notification_handlers() {
            Ok(h) => h,
            Err(e) => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to get notification handlers: {}", e),
                )))
            }
        };

        // 3. Execute the command
        let mut executor = CommandExecutor::new(cmd.to_string(), args.to_vec());
        executor.execute().await?;

        // 4. Get the output and prepare notification message
        let message = match executor.get_output() {
            Some(output) => format!("Command output:\n{}", output),
            None => "Command executed but produced no output".to_string(),
        };

        // 5. Send notifications through all handlers
        for handler in handlers {
            if let Err(e) = handler.send(&message).await {
                error!("Failed to send notification: {}", e);
                // Optionally: return Err(e) if you want to fail on first error
            }
        }

        info!("Command executed and notifications sent successfully");
        Ok(())
    }
}

// Keep these for backward compatibility
pub fn list_configs() -> Result<(), Box<dyn Error>> {
    App::new().list_configs()
}

pub fn create_config(name: &str) -> Result<(), Box<dyn Error>> {
    App::new().create_config(name)
}

pub async fn run_command(
    config_set_name: &str,
    cmd: &str,
    args: &[String],
) -> Result<(), Box<dyn Error>> {
    App::new().run_command(config_set_name, cmd, args).await
}