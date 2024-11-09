use crate::config::{ConfigSet, LarkConfig, NotificationConfigType, TelegramConfig};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame, Terminal,
};
use std::{
    error::Error,
    io::{self, Stdout},
};

#[derive(PartialEq)]
enum EditorMode {
    Normal,       // Browsing the main config items
    AddingConfig, // Selecting notification type to add
    Editing,      // Editing a value
    Notification, // Editing notification details
}

pub struct Editor {
    config: ConfigSet,
    selected_index: usize,
    editing_value: String,
    mode: EditorMode,
    notification_index: Option<usize>,
    notification_field_index: Option<usize>,
}

impl Editor {
    pub fn new(config: ConfigSet) -> Self {
        Self {
            config,
            selected_index: 0,
            editing_value: String::new(),
            mode: EditorMode::Normal,
            notification_index: None,
            notification_field_index: None,
        }
    }

    pub fn run(&mut self) -> Result<ConfigSet, Box<dyn Error>> {
        enable_raw_mode()?;
        io::stdout().execute(EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(io::stdout());
        let mut terminal = Terminal::new(backend)?;

        let result = self.run_app(&mut terminal);

        // restore terminal
        disable_raw_mode()?;
        io::stdout().execute(LeaveAlternateScreen)?;

        result
    }

    fn run_app(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    ) -> Result<ConfigSet, Box<dyn Error>> {
        loop {
            terminal.draw(|f| self.ui::<CrosstermBackend<Stdout>>(f))?;

            if let Event::Key(key) = event::read()? {
                if let Ok(true) = self.handle_input(key.code) {
                    return Ok(self.config.clone());
                }
            }
        }
    }

    fn ui<B>(&self, f: &mut Frame)
    where
        B: Backend,
    {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Min(3),    // Main content area
                    Constraint::Length(3), // Key hints
                ]
                .as_ref(),
            )
            .split(f.size());

        match self.mode {
            EditorMode::Normal => self.render_main_view::<B>(f, chunks[0]),
            EditorMode::AddingConfig => self.render_add_config_view(f, chunks[0]),
            EditorMode::Editing => self.render_editing_view::<B>(f, chunks[0]),
            EditorMode::Notification => self.render_notification_view::<B>(f, chunks[0]),
        }

        self.render_hints::<B>(f, chunks[1]);
    }

    fn render_main_view<B>(&self, f: &mut Frame, area: Rect)
    where
        B: Backend,
    {
        let items: Vec<ListItem> = self
            .get_main_items()
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let style = if i == self.selected_index {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };

                let indent = if item.starts_with("Notification:") {
                    "  "
                } else {
                    ""
                };

                ListItem::new(Line::from(vec![
                    Span::raw(indent),
                    Span::styled(item.clone(), style),
                ]))
            })
            .collect();

        let list = List::new(items).block(
            Block::default()
                .title("Configuration")
                .borders(Borders::ALL),
        );

        f.render_widget(list, area);
    }

    fn render_notification_view<B: Backend>(&self, f: &mut Frame, area: Rect) {
        if let Some(notif_idx) = self.notification_index {
            // Render notification details here
            // This would show all fields of the selected notification
            let notification = &self.config.notification_configs.configs[notif_idx];
            let items: Vec<ListItem> = self
                .get_notification_items(notification)
                .iter()
                .enumerate()
                .map(|(i, item)| {
                    let style = if Some(i) == self.notification_field_index {
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                    };
                    ListItem::new(Span::styled(item.clone(), style))
                })
                .collect();

            let list = List::new(items).block(
                Block::default()
                    .title("Notification Settings")
                    .borders(Borders::ALL),
            );

            f.render_widget(list, area);
        }
    }

    fn render_editing_view<B: Backend>(&self, f: &mut Frame, area: Rect) {
        let input = Paragraph::new(self.editing_value.as_str())
            .block(Block::default().title("Editing").borders(Borders::ALL))
            .wrap(Wrap { trim: true });
        f.render_widget(input, area);
    }

    fn render_hints<B: Backend>(&self, f: &mut Frame, area: Rect) {
        let hints = match self.mode {
            EditorMode::Normal => "↑↓: Navigate | Enter: Edit | q: Quit",
            EditorMode::AddingConfig => "↑↓: Navigate | Enter: Add | Esc: Cancel",
            EditorMode::Editing => "Enter: Save | Esc: Cancel",
            EditorMode::Notification => "↑↓: Navigate | Enter: Edit Field | Esc: Back | q: Quit",
        };

        let hints = Paragraph::new(hints).block(Block::default().borders(Borders::ALL));
        f.render_widget(hints, area);
    }

    fn get_main_items(&self) -> Vec<String> {
        let mut items = vec![];
        items.push("+ Add New Notification".to_string());
        items.push(format!("Config Name: {}", self.config.name));

        // Add notifications with indent
        for notification in self.config.notification_configs.configs.iter() {
            let type_str = match notification {
                NotificationConfigType::Telegram(_) => "Telegram",
                NotificationConfigType::Lark(_) => "Lark",
                // Add other types here
                _ => "Unknown",
            };
            items.push(format!("  Notification: {}", type_str));
        }

        items
    }

    fn get_notification_items(&self, notification: &NotificationConfigType) -> Vec<String> {
        // Convert notification fields to displayable items
        // This will depend on your NotificationConfigs structure
        match notification {
            // NotificationConfigType::Email(config) => {
            //     vec![
            //         format!("Type: Email"),
            //         format!("SMTP Server: {}", config.smtp.server),
            //         format!("From: {}", config.from),
            //         format!("To: {}", config.to),
            //         // ... other email fields
            //     ]
            // }
            NotificationConfigType::Telegram(config) => {
                vec![
                    format!("Type: Telegram"),
                    format!("Token: {}", config.token),
                    format!("Chat ID: {}", config.chat_id),
                ]
            }
            NotificationConfigType::Lark(config) => {
                vec![
                    format!("Type: Lark"),
                    format!("Webhook URL: {}", config.webhook_url),
                    format!("Sign Key: {}", config.sign_key),
                    format!("At: {}", config.at.clone().unwrap_or_default()),
                ]
            }
            // Add other notification types
            _ => vec![format!("Unsupported notification type")],
        }
    }

    fn start_editing(&mut self) {
        self.mode = EditorMode::Editing;
        self.editing_value = self.get_current_value();
    }

    fn get_current_value(&self) -> String {
        if let Some(notif_idx) = self.notification_index {
            let notification = &self.config.notification_configs.configs[notif_idx];
            if let Some(field_idx) = self.notification_field_index {
                match notification {
                    NotificationConfigType::Telegram(config) => match field_idx {
                        1 => config.token.clone(),
                        2 => config.chat_id.clone(),
                        _ => String::new(),
                    },
                    NotificationConfigType::Lark(config) => match field_idx {
                        1 => config.webhook_url.clone(),
                        2 => config.sign_key.clone(),
                        3 => config.at.clone().unwrap_or_default(),
                        _ => String::new(),
                    },
                    _ => String::new(),
                }
            } else {
                String::new()
            }
        } else {
            String::new()
        }
    }

    fn apply_edit(&mut self) {
        if let Some(notif_idx) = self.notification_index {
            if let Some(field_idx) = self.notification_field_index {
                let notification = &mut self.config.notification_configs.configs[notif_idx];
                match notification {
                    NotificationConfigType::Telegram(config) => match field_idx {
                        1 => config.token = self.editing_value.clone(),
                        2 => config.chat_id = self.editing_value.clone(),
                        _ => {}
                    },
                    NotificationConfigType::Lark(config) => match field_idx {
                        1 => config.webhook_url = self.editing_value.clone(),
                        2 => config.sign_key = self.editing_value.clone(),
                        3 => config.at = Some(self.editing_value.clone()),
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
        self.editing_value.clear();
    }

    fn handle_input(&mut self, key: KeyCode) -> Result<bool, Box<dyn Error>> {
        match self.mode {
            EditorMode::Normal => match key {
                KeyCode::Char('q') => return Ok(true),
                KeyCode::Up => self.selected_index = self.selected_index.saturating_sub(1),
                KeyCode::Down => {
                    self.selected_index =
                        (self.selected_index + 1).min(self.get_main_items().len() - 1)
                }
                KeyCode::Enter => {
                    match self.selected_index {
                        0 => {
                            // Add New Notification
                            self.mode = EditorMode::AddingConfig;
                            self.selected_index = 0;
                        }
                        1 => {} // Config name - do nothing as it's not editable
                        _ => {
                            // Notification editing
                            self.notification_index = Some(self.selected_index - 2); // Adjust for header items
                            self.notification_field_index = Some(0);
                            self.mode = EditorMode::Notification;
                        }
                    }
                }
                _ => {}
            },
            EditorMode::AddingConfig => match key {
                KeyCode::Esc => {
                    self.mode = EditorMode::Normal;
                    self.selected_index = 0;
                }
                KeyCode::Up => {
                    self.selected_index = self.selected_index.saturating_sub(1);
                }
                KeyCode::Down => {
                    self.selected_index = (self.selected_index + 1)
                        .min(self.get_available_notification_types().len() - 1);
                }
                KeyCode::Enter => {
                    let notification_type =
                        &self.get_available_notification_types()[self.selected_index];
                    self.create_new_notification(notification_type);
                    self.mode = EditorMode::Normal;
                    self.selected_index = self.get_main_items().len() - 1; // Select the newly added notification
                }
                _ => {}
            },
            EditorMode::Editing => match key {
                KeyCode::Enter => {
                    self.apply_edit();
                    self.mode = EditorMode::Notification;
                }
                KeyCode::Esc => {
                    self.editing_value.clear();
                    self.mode = EditorMode::Notification;
                }
                KeyCode::Backspace => {
                    self.editing_value.pop();
                }
                KeyCode::Char(c) => {
                    self.editing_value.push(c);
                }
                _ => {}
            },
            EditorMode::Notification => match key {
                KeyCode::Esc => {
                    self.mode = EditorMode::Normal;
                    self.notification_index = None;
                    self.notification_field_index = None;
                }
                KeyCode::Up => {
                    if let Some(idx) = self.notification_field_index {
                        self.notification_field_index = Some(idx.saturating_sub(1));
                    }
                }
                KeyCode::Down => {
                    if let Some(idx) = self.notification_field_index {
                        let max = self.get_notification_field_count() - 1;
                        self.notification_field_index = Some((idx + 1).min(max));
                    }
                }
                KeyCode::Enter => {
                    self.start_editing();
                }
                _ => {}
            },
        }
        Ok(false)
    }

    fn get_available_notification_types(&self) -> Vec<String> {
        vec![
            "Telegram".to_string(),
            "Lark".to_string(),
            "Email".to_string(),
            // Add other notification types here
        ]
    }

    fn create_new_notification(&mut self, notification_type: &str) {
        let new_config = match notification_type {
            "Telegram" => NotificationConfigType::Telegram(TelegramConfig::default()),
            "Lark" => NotificationConfigType::Lark(LarkConfig::default()),
            // Add other types here
            _ => return,
        };
        self.config.notification_configs.configs.push(new_config);
    }

    fn render_add_config_view(&self, f: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self
            .get_available_notification_types()
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let style = if i == self.selected_index {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                ListItem::new(Line::from(vec![Span::styled(item.clone(), style)]))
            })
            .collect();

        let list = List::new(items).block(
            Block::default()
                .title("Select Notification Type")
                .borders(Borders::ALL),
        );

        f.render_widget(list, area);
    }

    fn render_add_config_hints(&self, f: &mut Frame, area: Rect) {
        let hints = match self.mode {
            EditorMode::Normal => "↑↓: Navigate | Enter: Select | q: Quit",
            EditorMode::AddingConfig => "↑↓: Navigate | Enter: Add | Esc: Cancel",
            EditorMode::Editing => "Enter: Save | Esc: Cancel",
            EditorMode::Notification => "↑↓: Navigate | Enter: Edit Field | Esc: Back | q: Quit",
        };

        let hints = Paragraph::new(hints).block(Block::default().borders(Borders::ALL));
        f.render_widget(hints, area);
    }

    fn get_notification_field_count(&self) -> usize {
        if let Some(notif_idx) = self.notification_index {
            let notification = &self.config.notification_configs.configs[notif_idx];
            self.get_notification_items(notification).len()
        } else {
            0
        }
    }
}
