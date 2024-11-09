mod app;
mod cli;
mod config;
mod editor;
mod error;
mod executor;
mod notifications;

use chrono::Local;
use clap::Parser;
use cli::{Cli, Commands};
use env_logger::Builder;
use log::LevelFilter;
use log::{error, info};
use std::io::Write;

fn main() {
    // Initialize logger with custom format

    Builder::new()
        .filter_level(LevelFilter::Info)
        .format(|buf, record| {
            writeln!(
                buf,
                "[{} {} {}:{:?}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .init();

    info!("Starting NotifyMe application");

    let cli = Cli::parse();

    match cli.command {
        Commands::Run {
            config_set,
            cmd,
            args,
        } => {
            let rt = tokio::runtime::Runtime::new().unwrap();
            if let Err(e) = rt.block_on(app::run_command(&config_set, &cmd, &args)) {
                error!("Error running command: {}", e);
                std::process::exit(1);
            }
        }

        Commands::List => {
            if let Err(e) = app::list_configs() {
                eprintln!("Error listing configs: {}", e);
                std::process::exit(1);
            }
        }

        Commands::Create { name } => {
            if let Err(e) = app::create_config(&name) {
                eprintln!("Error creating config '{}': {}", name, e);
                std::process::exit(1);
            }
        }

        Commands::Edit { name } => {
            if let Err(e) = app::edit_config(&name) {
                eprintln!("Error editing config '{}': {}", name, e);
                std::process::exit(1);
            }
        }

        Commands::Delete { name } => {
            if let Err(e) = app::delete_config(&name) {
                eprintln!("Error deleting config '{}': {}", name, e);
                std::process::exit(1);
            }
        }
        Commands::Test { name: _ } => {
            // TODO: Implement test logic
        }
    }
}
