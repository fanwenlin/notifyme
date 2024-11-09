use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "notifyme")]
#[command(author = "Your Name <your.email@example.com>")]
#[command(version = "0.1.0")]
#[command(about = "A command-line tool to send notifications", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run a command and send notifications
    Run {
        /// Configuration set name
        #[arg(short, long, default_value = "default")]
        config_set: String,
        /// Command to execute
        #[arg()]
        cmd: String,
        /// Arguments for the command
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },
    /// List available configuration sets
    List,
    /// Create a new configuration set
    Create {
        /// Configuration set name
        name: String,
    },
    /// Edit a configuration set
    Edit {
        /// Configuration set name
        name: String,
    },
    /// Delete a configuration set
    Delete {
        /// Configuration set name
        name: String,
    },
    /// Test a configuration set
    Test {
        /// Configuration set name
        name: String,
    },
}
