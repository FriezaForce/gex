use crate::error::Result;
use crate::git::ConfigScope;
use crate::profile::manager::ProfileManager;
use crate::profile::Profile;
use crate::switcher::ProfileSwitcher;
use crate::utils::validator::Validator;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};
use std::io;

enum AppState {
    MainMenu,
    ListProfiles,
    AddProfile,
    SwitchProfile,
    DeleteProfile,
    Status,
    Message(String),
}

pub struct TuiApp {
    profile_manager: ProfileManager,
    switcher: ProfileSwitcher,
    state: AppState,
    list_state: ListState,
    should_quit: bool,
}

impl TuiApp {
    pub fn new() -> Result<Self> {
        let profile_manager = ProfileManager::new()?;
        let switcher = ProfileSwitcher::new()?;

        Ok(Self {
            profile_manager,
            switcher,
            state: AppState::MainMenu,
            list_state: ListState::default(),
            should_quit: false,
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
                    AppState::MainMenu => self.handle_main_menu_input(key.code),
                    AppState::ListProfiles => self.handle_list_profiles_input(key.code),
                    AppState::SwitchProfile => self.handle_switch_profile_input(key.code),
                    AppState::Status => self.handle_status_input(key.code),
                    AppState::Message(_) => self.handle_message_input(key.code),
                    _ => {
                        if key.code == KeyCode::Esc {
                            self.state = AppState::MainMenu;
                        }
                    }
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
            .margin(2)
            .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
            .split(f.size());

        // Title
        let title = Paragraph::new("gex - Git Profile Switcher")
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(title, chunks[0]);

        // Content based on state
        match &self.state {
            AppState::MainMenu => self.render_main_menu(f, chunks[1]),
            AppState::ListProfiles => self.render_list_profiles(f, chunks[1]),
            AppState::SwitchProfile => self.render_switch_profile(f, chunks[1]),
            AppState::Status => self.render_status(f, chunks[1]),
            AppState::Message(msg) => self.render_message(f, chunks[1], msg.clone()),
            _ => {}
        }
    }

    fn render_main_menu(&mut self, f: &mut Frame, area: ratatui::layout::Rect) {
        let menu_items = vec![
            "1. List Profiles",
            "2. Switch Profile",
            "3. Show Status",
            "4. Quit",
            "",
            "Press number to select or ESC to quit",
        ];

        let items: Vec<ListItem> = menu_items
            .iter()
            .map(|item| ListItem::new(*item))
            .collect();

        let list = List::new(items)
            .block(Block::default().title("Main Menu").borders(Borders::ALL))
            .style(Style::default().fg(Color::White));

        f.render_widget(list, area);
    }

    fn render_list_profiles(&mut self, f: &mut Frame, area: ratatui::layout::Rect) {
        let profiles = match self.profile_manager.get_all_profiles() {
            Ok(p) => p,
            Err(_) => vec![],
        };

        if profiles.is_empty() {
            let msg = Paragraph::new("No profiles found.\n\nUse CLI to add profiles:\ngex add <name> --username <user> --email <email> --ssh-key <key>\n\nPress ESC to return")
                .block(Block::default().title("Profiles").borders(Borders::ALL));
            f.render_widget(msg, area);
            return;
        }

        let items: Vec<ListItem> = profiles
            .iter()
            .map(|p| {
                ListItem::new(vec![
                    Line::from(Span::styled(
                        format!("● {}", p.name),
                        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
                    )),
                    Line::from(format!("  Username: {}", p.username)),
                    Line::from(format!("  Email: {}", p.email)),
                    Line::from(format!("  SSH Key: {}", p.ssh_key_name)),
                    Line::from(""),
                ])
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .title("Profiles (Press ESC to return)")
                    .borders(Borders::ALL),
            )
            .style(Style::default().fg(Color::White));

        f.render_widget(list, area);
    }

    fn render_switch_profile(&mut self, f: &mut Frame, area: ratatui::layout::Rect) {
        let profiles = match self.profile_manager.get_all_profiles() {
            Ok(p) => p,
            Err(_) => vec![],
        };

        if profiles.is_empty() {
            let msg = Paragraph::new("No profiles found.\n\nPress ESC to return")
                .block(Block::default().title("Switch Profile").borders(Borders::ALL));
            f.render_widget(msg, area);
            return;
        }

        let mut items: Vec<ListItem> = profiles
            .iter()
            .enumerate()
            .map(|(i, p)| {
                ListItem::new(format!("{}. {} ({})", i + 1, p.name, p.email))
            })
            .collect();

        items.push(ListItem::new(""));
        items.push(ListItem::new("Press number to switch (G for global, L for local)"));
        items.push(ListItem::new("Press ESC to return"));

        let list = List::new(items)
            .block(
                Block::default()
                    .title("Switch Profile")
                    .borders(Borders::ALL),
            )
            .style(Style::default().fg(Color::White));

        f.render_widget(list, area);
    }

    fn render_status(&mut self, f: &mut Frame, area: ratatui::layout::Rect) {
        let status = match self.switcher.get_current_status() {
            Ok(s) => s,
            Err(_) => {
                let msg = Paragraph::new("Failed to get status\n\nPress ESC to return")
                    .block(Block::default().title("Status").borders(Borders::ALL));
                f.render_widget(msg, area);
                return;
            }
        };

        let mut lines = vec![
            Line::from(Span::styled(
                "Global Profile:",
                Style::default().add_modifier(Modifier::BOLD),
            )),
        ];

        if let Some(profile) = status.global {
            lines.push(Line::from(format!("  Profile: {}", profile.name)));
            lines.push(Line::from(format!("  Username: {}", profile.username)));
            lines.push(Line::from(format!("  Email: {}", profile.email)));
        } else {
            lines.push(Line::from("  No profile set"));
        }

        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "Local Profile:",
            Style::default().add_modifier(Modifier::BOLD),
        )));

        if let Some(profile) = status.local {
            lines.push(Line::from(format!("  Profile: {}", profile.name)));
            lines.push(Line::from(format!("  Username: {}", profile.username)));
            lines.push(Line::from(format!("  Email: {}", profile.email)));
        } else {
            lines.push(Line::from("  No profile set or not in git repo"));
        }

        lines.push(Line::from(""));
        lines.push(Line::from("Press ESC to return"));

        let paragraph = Paragraph::new(lines)
            .block(Block::default().title("Status").borders(Borders::ALL));

        f.render_widget(paragraph, area);
    }

    fn render_message(&mut self, f: &mut Frame, area: ratatui::layout::Rect, msg: String) {
        let paragraph = Paragraph::new(format!("{}\n\nPress ESC to return", msg))
            .block(Block::default().title("Message").borders(Borders::ALL))
            .style(Style::default().fg(Color::Yellow));

        f.render_widget(paragraph, area);
    }

    fn handle_main_menu_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('1') => self.state = AppState::ListProfiles,
            KeyCode::Char('2') => self.state = AppState::SwitchProfile,
            KeyCode::Char('3') => self.state = AppState::Status,
            KeyCode::Char('4') | KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
            _ => {}
        }
    }

    fn handle_list_profiles_input(&mut self, key: KeyCode) {
        if key == KeyCode::Esc {
            self.state = AppState::MainMenu;
        }
    }

    fn handle_switch_profile_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc => self.state = AppState::MainMenu,
            KeyCode::Char(c) if c.is_ascii_digit() => {
                let index = c.to_digit(10).unwrap() as usize - 1;
                if let Ok(profiles) = self.profile_manager.get_all_profiles() {
                    if index < profiles.len() {
                        // For simplicity, default to global scope in TUI
                        // User can use CLI for local scope
                        let profile_name = &profiles[index].name;
                        match self.switcher.switch_profile(profile_name, ConfigScope::Global) {
                            Ok(_) => {
                                self.state = AppState::Message(format!(
                                    "Successfully switched to profile '{}'",
                                    profile_name
                                ));
                            }
                            Err(e) => {
                                self.state = AppState::Message(format!("Error: {}", e));
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn handle_status_input(&mut self, key: KeyCode) {
        if key == KeyCode::Esc {
            self.state = AppState::MainMenu;
        }
    }

    fn handle_message_input(&mut self, key: KeyCode) {
        if key == KeyCode::Esc {
            self.state = AppState::MainMenu;
        }
    }
}
