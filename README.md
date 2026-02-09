# NotifyMe

<p align="center">
  A powerful CLI tool for monitoring long-running commands and sending notifications through multiple channels
</p>

<p align="center">
  <a href="#features">Features</a> •
  <a href="#installation">Installation</a> •
  <a href="#quick-start">Quick Start</a> •
  <a href="#configuration">Configuration</a> •
  <a href="#development-status">Status</a>
</p>

> **Note**: This project is under active development. Some features mentioned in the documentation may not be fully implemented yet. See [Development Status](#development-status) for details.

## Features

- 🚀 Monitor long-running commands and get notified upon completion
- 📱 Multiple notification channels:
  - Telegram
  - Lark (Feishu)
  - Email (coming soon)
  - SMS via Twilio (coming soon)
  - Phone calls via Twilio (coming soon)
- ⚙️ Customizable configuration system
- 🔧 Interactive configuration editor
- 🔄 Retry mechanisms and error handling (coming soon)

## Use Cases

- Monitor long-running builds, deployments, or data processing tasks
- You want to monitor the status of a long-running command and send notifications when it's finished.
- You are about to go outside after starting a command and want to know if you need get back early to fix something.
- You're switching to video games and want to get noticed when it's time to turn back to your job.

## Installation

### Prerequisites
- Rust 1.70 or higher
- Linux or macOS (Windows support coming soon)

### From Source

```bash
cargo install --git https://github.com/fanwenlin/notifyme
```

## Quick Start

1. Create a default configuration:
```bash
notifyme create default
```

2. Edit the configuration to add your notification preferences:
```bash
notifyme edit default
```

3. Run a command with notifications:
```bash
# Using the delimiter
notifyme run --config myconfig -- ping -c 5 google.com

# With Default config
notifyme run -- ping -c 5 google.com
```

4. Send a notification directly (for integration):
```bash
notifyme send "Critical issue: Database is down!"
```

## Skill Integration (for Agents)

NotifyMe can be used as a "Skill" for AI Agents (like Gemini CLI, Cursor, or specialized GPTs) to allow them to grab your attention when they hit a roadblock.

### Installing for Gemini CLI

Add the following to your Gemini CLI configuration or skill directory:

1. Copy the `SKILL.md` file to your skills folder.
2. Ensure the `notifyme` binary is in your `PATH`.
3. The Agent will now be able to use the `notifyme send` command whenever it needs to notify you about critical events or manual intervention.

### Integration Principle

The `notifyme send` command is designed to be a "fire-and-forget" high-priority alert. When an Agent executes this command:
- It uses your pre-configured channels (Lark, Telegram, etc.).
- It automatically handles user mentions (@) and notification priority.
- It provides a standardized `[NotifyMe]` prefix for easy filtering.

## Configuration

Configurations are stored in XML format at `~/.config/notifyme/configs/`. Each configuration set can include multiple notification methods.

For detailed configuration options, see [Configuration Guide](docs/configuration.md) (coming soon).

## Development Status

### Currently Implemented
- ✅ Basic CLI framework
- ✅ Configuration management system
- ✅ Interactive configuration editor
- ✅ Telegram notifications
- ✅ Lark (Feishu) notifications
- ✅ Command execution and monitoring

### In Progress
- 🔄 Email notification support
- 🔄 SMS notifications via Twilio
- 🔄 Phone call notifications
- 🔄 HTTP webhook support
- 🔄 Configuration validation
- 🔄 Error handling improvements


## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Uses [clap](https://github.com/clap-rs/clap) for CLI parsing
- Uses [ratatui](https://github.com/ratatui-org/ratatui) for terminal UI