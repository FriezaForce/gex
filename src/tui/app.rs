use crate::error::Result;
use crate::git::ConfigScope;
use crate::profile::manager::ProfileManager;
use crate::switcher::ProfileSwitcher;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, BorderType, List, ListItem, ListState, Paragraph, Clear},
    Frame, Terminal,
};
use std::io;

// Modern icons using Unicode
const ICON_PROFILE: &str = "👤";
const ICON_SWITCH: &str = "🔄";
const ICON_STATUS: &str = "📊";
const ICON_QUIT: &str = "🚪";
const ICON_GLOBAL: &str = "🌍";
const ICON_LOCAL: &str = "📁";
const ICON_EMAIL: &str = "📧";
const ICON_KEY: &str = "🔑";
const ICON_SUCCESS: &str = "✅";
const ICON_ERROR: &str = "❌";
const ICON_INFO: &str = "ℹ️";
const ICON_ARROW: &str = "➤";
const ICON_CHECK: &str = "✓";
const ICON_STAR: &str = "⭐";
const ICON_SEARCH: &str = "🔍";
const ICON_HELP: &str = "❓";

enum AppState {
    MainMenu,
    ListProfiles,
    SwitchProfile,
    Status,
    Message { text: String, is_error: bool },
    ConfirmSwitch { profile_index: usize, scope: ConfigScope },
}

pub struct TuiApp {
    profile_manager: ProfileManager,
    switcher: ProfileSwitcher,
    state: AppState,
    list_state: ListState,
    should_quit: bool,
    selected_menu_item: usize,
    selected_scope: ConfigScope,
}

impl TuiApp {
    pub fn new() -> Result<Self> {
        let profile_manager = ProfileManager::new()?;
        let switcher = ProfileSwitcher::new()?;
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Ok(Self {
            profile_manager,
            switcher,
            state: AppState::MainMenu,
            list_state,
            should_quit: false,
            selected_menu_item: 0,
            selected_scope: ConfigScope::Global,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // Run the app
        let res = self.run_app(&mut terminal);

        // Restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        if let Err(err) = res {
            println!("Error: {:?}", err);
        }

        Ok(())
    }

    fn run_app<B: ratatui::backend::Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        loop {
            terminal.draw(|f| self.ui(f))?;

            if let Event::Key(key) = event::read()? {
                match &self.state {
                    AppState::MainMenu => self.handle_main_menu_input(key.code, key.modifiers),
                    AppState::ListProfiles => self.handle_list_profiles_input(key.code),
                    AppState::SwitchProfile => self.handle_switch_profile_input(key.code),
                    AppState::Status => self.handle_status_input(key.code),
                    AppState::Message { .. } => self.handle_message_input(key.code),
                    AppState::ConfirmSwitch { .. } => self.handle_confirm_input(key.code),
                }
            }

            if self.should_quit {
                break;
            }
        }

        Ok(())
    }

    fn ui(&mut self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(5),  // Header
                Constraint::Min(0),     // Content
                Constraint::Length(3),  // Footer
            ])
            .split(f.size());

        // Render header
        self.render_header(f, chunks[0]);

        // Content based on state
        match &self.state {
            AppState::MainMenu => self.render_main_menu(f, chunks[1]),
            AppState::ListProfiles => self.render_list_profiles(f, chunks[1]),
            AppState::SwitchProfile => self.render_switch_profile(f, chunks[1]),
            AppState::Status => self.render_status(f, chunks[1]),
            AppState::Message { text, is_error } => self.render_message(f, chunks[1], text.clone(), *is_error),
            AppState::ConfirmSwitch { profile_index, scope } => {
                self.render_confirm_switch(f, chunks[1], *profile_index, scope.clone())
            }
        }

        // Render footer
        self.render_footer(f, chunks[2]);
    }

    fn render_header(&self, f: &mut Frame, area: Rect) {
        let header_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Length(2)])
            .split(area);

        // Title with gradient effect
        let title_text = vec![
            Line::from(vec![
                Span::styled("╔═══════════════════════════════════════════════════════════╗", 
                    Style::default().fg(Color::Cyan)),
            ]),
            Line::from(vec![
                Span::styled("║  ", Style::default().fg(Color::Cyan)),
                Span::styled("⚡ ", Style::default().fg(Color::Yellow)),
                Span::styled("GEX", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
                Span::styled(" - ", Style::default().fg(Color::White)),
                Span::styled("Git Profile Switcher", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::styled(" ⚡", Style::default().fg(Color::Yellow)),
                Span::styled("  ║", Style::default().fg(Color::Cyan)),
            ]),
            Line::from(vec![
                Span::styled("╚═══════════════════════════════════════════════════════════╝", 
                    Style::default().fg(Color::Cyan)),
            ]),
        ];

        let title = Paragraph::new(title_text)
            .alignment(Alignment::Center);
        f.render_widget(title, header_chunks[0]);

        // Status bar
        let status_text = match &self.state {
            AppState::MainMenu => format!("{} Main Menu", ICON_STAR),
            AppState::ListProfiles => format!("{} Profiles", ICON_PROFILE),
            AppState::SwitchProfile => format!("{} Switch Profile", ICON_SWITCH),
            AppState::Status => format!("{} Status", ICON_STATUS),
            AppState::Message { .. } => format!("{} Message", ICON_INFO),
            AppState::ConfirmSwitch { .. } => format!("{} Confirm", ICON_INFO),
        };

        let status_bar = Paragraph::new(status_text)
            .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center);
        f.render_widget(status_bar, header_chunks[1]);
    }

    fn render_footer(&self, f: &mut Frame, area: Rect) {
        let help_text = match &self.state {
            AppState::MainMenu => "↑↓: Navigate | Enter: Select | q/Esc: Quit",
            AppState::ListProfiles => "↑↓: Scroll | Esc: Back",
            AppState::SwitchProfile => "↑↓: Navigate | Enter: Confirm | g: Global | l: Local | Esc: Back",
            AppState::Status => "Esc: Back",
            AppState::Message { .. } => "Enter/Esc: Back",
            AppState::ConfirmSwitch { .. } => "y: Confirm | n/Esc: Cancel",
        };

        let footer = Paragraph::new(Line::from(vec![
            Span::styled(format!("{} ", ICON_HELP), Style::default().fg(Color::Yellow)),
            Span::styled(help_text, Style::default().fg(Color::Gray)),
        ]))
        .alignment(Alignment::Center)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(Color::DarkGray)));

        f.render_widget(footer, area);
    }

    fn render_main_menu(&mut self, f: &mut Frame, area: Rect) {
        let menu_options = vec![
            (ICON_PROFILE, "List Profiles", "View all configured profiles"),
            (ICON_SWITCH, "Switch Profile", "Change active profile"),
            (ICON_STATUS, "Show Status", "Display current configuration"),
            (ICON_QUIT, "Quit", "Exit application"),
        ];

        let items: Vec<ListItem> = menu_options
            .iter()
            .enumerate()
            .map(|(i, (icon, title, desc))| {
                let is_selected = i == self.selected_menu_item;
                let style = if is_selected {
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };

                let prefix = if is_selected { ICON_ARROW } else { " " };
                
                ListItem::new(vec![
                    Line::from(vec![
                        Span::styled(format!(" {} ", prefix), style),
                        Span::styled(format!("{} ", icon), style),
                        Span::styled(*title, style),
                    ]),
                    Line::from(vec![
                        Span::styled(format!("    {}", desc), 
                            if is_selected { 
                                Style::default().fg(Color::Black).bg(Color::Cyan)
                            } else { 
                                Style::default().fg(Color::DarkGray) 
                            }
                        ),
                    ]),
                    Line::from(""),
                ])
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .title(format!(" {} Main Menu ", ICON_STAR))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::Cyan))
            )
            .highlight_style(Style::default());

        f.render_stateful_widget(list, area, &mut self.list_state);
    }

    fn render_list_profiles(&mut self, f: &mut Frame, area: Rect) {
        let profiles = match self.profile_manager.get_all_profiles() {
            Ok(p) => p,
            Err(_) => vec![],
        };

        if profiles.is_empty() {
            let empty_msg = vec![
                Line::from(""),
                Line::from(Span::styled(
                    format!("  {} No profiles found", ICON_INFO),
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
                Line::from(Span::styled(
                    "  Add profiles using CLI:",
                    Style::default().fg(Color::Cyan),
                )),
                Line::from(""),
                Line::from(Span::styled(
                    "  gex add <name> --username <user> \\",
                    Style::default().fg(Color::Green),
                )),
                Line::from(Span::styled(
                    "              --email <email> \\",
                    Style::default().fg(Color::Green),
                )),
                Line::from(Span::styled(
                    "              --ssh-key <key>",
                    Style::default().fg(Color::Green),
                )),
                Line::from(""),
            ];

            let msg = Paragraph::new(empty_msg)
                .block(
                    Block::default()
                        .title(format!(" {} Profiles ", ICON_PROFILE))
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .border_style(Style::default().fg(Color::Yellow))
                )
                .alignment(Alignment::Left);
            f.render_widget(msg, area);
            return;
        }

        // Get current status to highlight active profile
        let current_global = self.switcher.get_current_status()
            .ok()
            .and_then(|s| s.global)
            .map(|p| p.name);

        let items: Vec<ListItem> = profiles
            .iter()
            .enumerate()
            .map(|(_i, p)| {
                let is_active = current_global.as_ref().map_or(false, |name| name == &p.name);
                let number_style = if is_active {
                    Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::Cyan)
                };

                let active_indicator = if is_active {
                    format!(" {} ", ICON_CHECK)
                } else {
                    "   ".to_string()
                };

                ListItem::new(vec![
                    Line::from(vec![
                        Span::styled(active_indicator, Style::default().fg(Color::Green)),
                        Span::styled(
                            format!("{} ", ICON_PROFILE),
                            number_style,
                        ),
                        Span::styled(
                            &p.name,
                            number_style.add_modifier(Modifier::BOLD),
                        ),
                        if is_active {
                            Span::styled(" (Active)", Style::default().fg(Color::Green))
                        } else {
                            Span::raw("")
                        },
                    ]),
                    Line::from(vec![
                        Span::raw("     "),
                        Span::styled(format!("👤 {}", p.username), Style::default().fg(Color::White)),
                    ]),
                    Line::from(vec![
                        Span::raw("     "),
                        Span::styled(format!("{} {}", ICON_EMAIL, p.email), Style::default().fg(Color::Gray)),
                    ]),
                    Line::from(vec![
                        Span::raw("     "),
                        Span::styled(format!("{} {}", ICON_KEY, p.ssh_key_name), Style::default().fg(Color::Gray)),
                    ]),
                    Line::from(""),
                ])
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .title(format!(" {} Profiles ({}) ", ICON_PROFILE, profiles.len()))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::Cyan))
            )
            .style(Style::default().fg(Color::White));

        f.render_stateful_widget(list, area, &mut self.list_state);
    }

    fn render_switch_profile(&mut self, f: &mut Frame, area: Rect) {
        let profiles = match self.profile_manager.get_all_profiles() {
            Ok(p) => p,
            Err(_) => vec![],
        };

        if profiles.is_empty() {
            let msg = Paragraph::new(vec![
                Line::from(""),
                Line::from(Span::styled(
                    format!("  {} No profiles available", ICON_INFO),
                    Style::default().fg(Color::Yellow),
                )),
                Line::from(""),
            ])
            .block(
                Block::default()
                    .title(format!(" {} Switch Profile ", ICON_SWITCH))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::Yellow))
            );
            f.render_widget(msg, area);
            return;
        }

        let selected = self.list_state.selected().unwrap_or(0);

        let items: Vec<ListItem> = profiles
            .iter()
            .enumerate()
            .map(|(i, p)| {
                let is_selected = i == selected;
                let style = if is_selected {
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };

                let prefix = if is_selected { ICON_ARROW } else { " " };

                ListItem::new(vec![
                    Line::from(vec![
                        Span::styled(format!(" {} ", prefix), style),
                        Span::styled(format!("{} ", ICON_PROFILE), style),
                        Span::styled(&p.name, style.add_modifier(Modifier::BOLD)),
                    ]),
                    Line::from(vec![
                        Span::styled(
                            format!("     {} {}", ICON_EMAIL, p.email),
                            if is_selected {
                                Style::default().fg(Color::Black).bg(Color::Cyan)
                            } else {
                                Style::default().fg(Color::Gray)
                            },
                        ),
                    ]),
                    Line::from(""),
                ])
            })
            .collect();

        let scope_indicator = match self.selected_scope {
            ConfigScope::Global => format!("{} Global", ICON_GLOBAL),
            ConfigScope::Local => format!("{} Local", ICON_LOCAL),
        };

        let list = List::new(items)
            .block(
                Block::default()
                    .title(format!(" {} Switch Profile - {} ", ICON_SWITCH, scope_indicator))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::Cyan))
            )
            .highlight_style(Style::default());

        f.render_stateful_widget(list, area, &mut self.list_state);
    }

    fn render_status(&mut self, f: &mut Frame, area: Rect) {
        let status = match self.switcher.get_current_status() {
            Ok(s) => s,
            Err(_) => {
                let msg = Paragraph::new(vec![
                    Line::from(""),
                    Line::from(Span::styled(
                        format!("  {} Failed to get status", ICON_ERROR),
                        Style::default().fg(Color::Red),
                    )),
                    Line::from(""),
                ])
                .block(
                    Block::default()
                        .title(format!(" {} Status ", ICON_STATUS))
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .border_style(Style::default().fg(Color::Red))
                );
                f.render_widget(msg, area);
                return;
            }
        };

        let mut lines = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("  ╔══════════════════════════════════════════════╗", 
                    Style::default().fg(Color::Cyan)),
            ]),
            Line::from(vec![
                Span::styled("  ║  ", Style::default().fg(Color::Cyan)),
                Span::styled(format!("{} GLOBAL PROFILE", ICON_GLOBAL),
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::styled("                      ║", Style::default().fg(Color::Cyan)),
            ]),
            Line::from(vec![
                Span::styled("  ╚══════════════════════════════════════════════╝", 
                    Style::default().fg(Color::Cyan)),
            ]),
            Line::from(""),
        ];

        if let Some(profile) = status.global {
            lines.push(Line::from(vec![
                Span::styled("    ", Style::default()),
                Span::styled(format!("{} ", ICON_PROFILE), Style::default().fg(Color::Green)),
                Span::styled("Profile: ", Style::default().fg(Color::Gray)),
                Span::styled(profile.name.clone(), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            ]));
            lines.push(Line::from(vec![
                Span::styled("    ", Style::default()),
                Span::styled("👤 ", Style::default().fg(Color::Green)),
                Span::styled("Username: ", Style::default().fg(Color::Gray)),
                Span::styled(profile.username.clone(), Style::default().fg(Color::White)),
            ]));
            lines.push(Line::from(vec![
                Span::styled("    ", Style::default()),
                Span::styled(format!("{} ", ICON_EMAIL), Style::default().fg(Color::Green)),
                Span::styled("Email: ", Style::default().fg(Color::Gray)),
                Span::styled(profile.email.clone(), Style::default().fg(Color::White)),
            ]));
            lines.push(Line::from(vec![
                Span::styled("    ", Style::default()),
                Span::styled(format!("{} ", ICON_KEY), Style::default().fg(Color::Green)),
                Span::styled("SSH Key: ", Style::default().fg(Color::Gray)),
                Span::styled(profile.ssh_key_name.clone(), Style::default().fg(Color::White)),
            ]));
        } else {
            lines.push(Line::from(vec![
                Span::styled("    ", Style::default()),
                Span::styled(format!("{} ", ICON_INFO), Style::default().fg(Color::Yellow)),
                Span::styled("No profile set", Style::default().fg(Color::DarkGray)),
            ]));
        }

        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::styled("  ╔══════════════════════════════════════════════╗", 
                Style::default().fg(Color::Magenta)),
        ]));
        lines.push(Line::from(vec![
            Span::styled("  ║  ", Style::default().fg(Color::Magenta)),
            Span::styled(format!("{} LOCAL PROFILE", ICON_LOCAL),
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled("                       ║", Style::default().fg(Color::Magenta)),
        ]));
        lines.push(Line::from(vec![
            Span::styled("  ╚══════════════════════════════════════════════╝", 
                Style::default().fg(Color::Magenta)),
        ]));
        lines.push(Line::from(""));

        if let Some(profile) = status.local {
            lines.push(Line::from(vec![
                Span::styled("    ", Style::default()),
                Span::styled(format!("{} ", ICON_PROFILE), Style::default().fg(Color::Magenta)),
                Span::styled("Profile: ", Style::default().fg(Color::Gray)),
                Span::styled(profile.name.clone(), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            ]));
            lines.push(Line::from(vec![
                Span::styled("    ", Style::default()),
                Span::styled("👤 ", Style::default().fg(Color::Magenta)),
                Span::styled("Username: ", Style::default().fg(Color::Gray)),
                Span::styled(profile.username.clone(), Style::default().fg(Color::White)),
            ]));
            lines.push(Line::from(vec![
                Span::styled("    ", Style::default()),
                Span::styled(format!("{} ", ICON_EMAIL), Style::default().fg(Color::Magenta)),
                Span::styled("Email: ", Style::default().fg(Color::Gray)),
                Span::styled(profile.email.clone(), Style::default().fg(Color::White)),
            ]));
            lines.push(Line::from(vec![
                Span::styled("    ", Style::default()),
                Span::styled(format!("{} ", ICON_KEY), Style::default().fg(Color::Magenta)),
                Span::styled("SSH Key: ", Style::default().fg(Color::Gray)),
                Span::styled(profile.ssh_key_name.clone(), Style::default().fg(Color::White)),
            ]));
        } else {
            lines.push(Line::from(vec![
                Span::styled("    ", Style::default()),
                Span::styled(format!("{} ", ICON_INFO), Style::default().fg(Color::Yellow)),
                Span::styled("No profile set or not in git repo", Style::default().fg(Color::DarkGray)),
            ]));
        }

        lines.push(Line::from(""));

        let paragraph = Paragraph::new(lines)
            .block(
                Block::default()
                    .title(format!(" {} Current Status ", ICON_STATUS))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::Cyan))
            );

        f.render_widget(paragraph, area);
    }

    fn render_message(&mut self, f: &mut Frame, area: Rect, msg: String, is_error: bool) {
        let (icon, color, title) = if is_error {
            (ICON_ERROR, Color::Red, "Error")
        } else {
            (ICON_SUCCESS, Color::Green, "Success")
        };

        let lines = vec![
            Line::from(""),
            Line::from(""),
            Line::from(vec![
                Span::styled(format!("  {} ", icon), Style::default().fg(color)),
                Span::styled(&msg, Style::default().fg(color).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(""),
        ];

        let paragraph = Paragraph::new(lines)
            .block(
                Block::default()
                    .title(format!(" {} {} ", icon, title))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(color))
            )
            .alignment(Alignment::Center);

        f.render_widget(paragraph, area);
    }

    fn render_confirm_switch(&mut self, f: &mut Frame, area: Rect, profile_index: usize, scope: ConfigScope) {
        let profiles = match self.profile_manager.get_all_profiles() {
            Ok(p) => p,
            Err(_) => {
                self.state = AppState::Message {
                    text: "Failed to load profiles".to_string(),
                    is_error: true,
                };
                return;
            }
        };

        if profile_index >= profiles.len() {
            self.state = AppState::MainMenu;
            return;
        }

        let profile = &profiles[profile_index];
        let scope_text = match scope {
            ConfigScope::Global => format!("{} Global", ICON_GLOBAL),
            ConfigScope::Local => format!("{} Local", ICON_LOCAL),
        };

        let lines = vec![
            Line::from(""),
            Line::from(Span::styled(
                "  Confirm Profile Switch",
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(vec![
                Span::styled("  Profile: ", Style::default().fg(Color::Gray)),
                Span::styled(&profile.name, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled("  Scope: ", Style::default().fg(Color::Gray)),
                Span::styled(&scope_text, Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("  👤 ", Style::default()),
                Span::styled(&profile.username, Style::default().fg(Color::White)),
            ]),
            Line::from(vec![
                Span::styled(format!("  {} ", ICON_EMAIL), Style::default()),
                Span::styled(&profile.email, Style::default().fg(Color::White)),
            ]),
            Line::from(vec![
                Span::styled(format!("  {} ", ICON_KEY), Style::default()),
                Span::styled(&profile.ssh_key_name, Style::default().fg(Color::White)),
            ]),
            Line::from(""),
            Line::from(""),
            Line::from(Span::styled(
                "  Press 'y' to confirm or 'n' to cancel",
                Style::default().fg(Color::DarkGray),
            )),
        ];

        let paragraph = Paragraph::new(lines)
            .block(
                Block::default()
                    .title(format!(" {} Confirm ", ICON_INFO))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::Yellow))
            )
            .alignment(Alignment::Left);

        // Center the dialog
        let dialog_area = centered_rect(60, 60, area);
        f.render_widget(Clear, dialog_area);
        f.render_widget(paragraph, dialog_area);
    }

    fn handle_main_menu_input(&mut self, key: KeyCode, modifiers: KeyModifiers) {
        match key {
            KeyCode::Up => {
                if self.selected_menu_item > 0 {
                    self.selected_menu_item -= 1;
                    self.list_state.select(Some(self.selected_menu_item));
                }
            }
            KeyCode::Down => {
                if self.selected_menu_item < 3 {
                    self.selected_menu_item += 1;
                    self.list_state.select(Some(self.selected_menu_item));
                }
            }
            KeyCode::Enter => {
                match self.selected_menu_item {
                    0 => {
                        self.state = AppState::ListProfiles;
                        self.list_state.select(Some(0));
                    }
                    1 => {
                        self.state = AppState::SwitchProfile;
                        self.list_state.select(Some(0));
                    }
                    2 => self.state = AppState::Status,
                    3 => self.should_quit = true,
                    _ => {}
                }
            }
            KeyCode::Char('1') => {
                self.state = AppState::ListProfiles;
                self.list_state.select(Some(0));
            }
            KeyCode::Char('2') => {
                self.state = AppState::SwitchProfile;
                self.list_state.select(Some(0));
            }
            KeyCode::Char('3') => self.state = AppState::Status,
            KeyCode::Char('4') | KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Esc => self.should_quit = true,
            KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => self.should_quit = true,
            _ => {}
        }
    }

    fn handle_list_profiles_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc => {
                self.state = AppState::MainMenu;
                self.list_state.select(Some(self.selected_menu_item));
            }
            KeyCode::Up => {
                let i = match self.list_state.selected() {
                    Some(i) => {
                        if i > 0 {
                            i - 1
                        } else {
                            i
                        }
                    }
                    None => 0,
                };
                self.list_state.select(Some(i));
            }
            KeyCode::Down => {
                let profiles_count = self.profile_manager.get_all_profiles().map(|p| p.len()).unwrap_or(0);
                let i = match self.list_state.selected() {
                    Some(i) => {
                        if i < profiles_count.saturating_sub(1) {
                            i + 1
                        } else {
                            i
                        }
                    }
                    None => 0,
                };
                self.list_state.select(Some(i));
            }
            _ => {}
        }
    }

    fn handle_switch_profile_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc => {
                self.state = AppState::MainMenu;
                self.list_state.select(Some(self.selected_menu_item));
            }
            KeyCode::Up => {
                let i = match self.list_state.selected() {
                    Some(i) => {
                        if i > 0 {
                            i - 1
                        } else {
                            i
                        }
                    }
                    None => 0,
                };
                self.list_state.select(Some(i));
            }
            KeyCode::Down => {
                let profiles_count = self.profile_manager.get_all_profiles().map(|p| p.len()).unwrap_or(0);
                let i = match self.list_state.selected() {
                    Some(i) => {
                        if i < profiles_count.saturating_sub(1) {
                            i + 1
                        } else {
                            i
                        }
                    }
                    None => 0,
                };
                self.list_state.select(Some(i));
            }
            KeyCode::Char('g') | KeyCode::Char('G') => {
                self.selected_scope = ConfigScope::Global;
            }
            KeyCode::Char('l') | KeyCode::Char('L') => {
                self.selected_scope = ConfigScope::Local;
            }
            KeyCode::Enter => {
                if let Some(index) = self.list_state.selected() {
                    self.state = AppState::ConfirmSwitch {
                        profile_index: index,
                        scope: self.selected_scope.clone(),
                    };
                }
            }
            _ => {}
        }
    }

    fn handle_status_input(&mut self, key: KeyCode) {
        if key == KeyCode::Esc {
            self.state = AppState::MainMenu;
            self.list_state.select(Some(self.selected_menu_item));
        }
    }

    fn handle_message_input(&mut self, key: KeyCode) {
        if key == KeyCode::Esc || key == KeyCode::Enter {
            self.state = AppState::MainMenu;
            self.list_state.select(Some(self.selected_menu_item));
        }
    }

    fn handle_confirm_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                if let AppState::ConfirmSwitch { profile_index, scope } = &self.state {
                    if let Ok(profiles) = self.profile_manager.get_all_profiles() {
                        if *profile_index < profiles.len() {
                            let profile_name = &profiles[*profile_index].name;
                            match self.switcher.switch_profile(profile_name, scope.clone()) {
                                Ok(_) => {
                                    let scope_text = match scope {
                                        ConfigScope::Global => "globally",
                                        ConfigScope::Local => "locally",
                                    };
                                    self.state = AppState::Message {
                                        text: format!("Successfully switched to '{}' {}", profile_name, scope_text),
                                        is_error: false,
                                    };
                                }
                                Err(e) => {
                                    self.state = AppState::Message {
                                        text: format!("Failed to switch profile: {}", e),
                                        is_error: true,
                                    };
                                }
                            }
                        }
                    }
                }
            }
            KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                self.state = AppState::SwitchProfile;
            }
            _ => {}
        }
    }
}

// Helper function to create centered rect
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
