use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, Paragraph},
};
use std::{fmt::Display, io};

mod wallet;
use wallet::{Wallet, WalletError};

use fedimint_core::Amount;

#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
    Intro,
    Wallet,
    SendModal,
    ReceiveModal,
    ReceivingModal,
    MnemonicModal,
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    Info(String),
    Success(String),
    Error(String),
}

impl Display for AppMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppMessage::Info(msg) => write!(f, "Info: {}", msg),
            AppMessage::Success(msg) => write!(f, "Success: {}", msg),
            AppMessage::Error(msg) => write!(f, "Error: {}", msg),
        }
    }
}

#[derive(Debug, Clone)]
pub struct App {
    state: AppState,
    wallet: Wallet,
    invite_code: String,
    balance: Amount,
    send_amount: String,
    receive_input: String,
    message: Option<AppMessage>,
    wallet_loaded: bool,
    sent_notes: Option<String>,
    show_sent_modal: bool,
    mnemonic_phrase: Option<String>,
    receiving_status: Option<String>,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: AppState::Intro,
            wallet: Wallet::new(),
            invite_code: String::new(),
            balance: Amount::from_sats(0),
            send_amount: String::new(),
            receive_input: String::new(),
            message: None,
            wallet_loaded: false,
            sent_notes: None,
            show_sent_modal: false,
            mnemonic_phrase: None,
            receiving_status: None,
        }
    }

    pub async fn init(&mut self) -> Result<(), WalletError> {
        match self.wallet.open().await {
            Ok(_) => {
                self.balance = self.wallet.balance().await?;
                self.state = AppState::Wallet;
                self.wallet_loaded = true;
            }
            Err(WalletError::OpenError) => {
                self.state = AppState::Intro;
            }
            Err(e) => {
                return Err(e);
            }
        }
        Ok(())
    }

    pub async fn handle_key(&mut self, key: KeyEvent) {
        // Clear messages on any key press except ESC and 'c' (for copy)
        if key.code != KeyCode::Esc && !(key.code == KeyCode::Char('c') && self.show_sent_modal) {
            self.message = None;
        }

        match self.state {
            AppState::Intro => match key.code {
                KeyCode::Char(c) => self.invite_code.push(c),
                KeyCode::Backspace => {
                    self.invite_code.pop();
                }
                KeyCode::Enter => {
                    if !self.invite_code.is_empty() {
                        self.join_federation().await;
                    }
                }
                _ => {}
            },
            AppState::Wallet => match key.code {
                KeyCode::Char('s') => {
                    self.state = AppState::SendModal;
                    self.send_amount.clear();
                    self.message = None;
                }
                KeyCode::Char('r') => {
                    self.state = AppState::ReceiveModal;
                    self.receive_input.clear();
                    self.message = None;
                }
                KeyCode::Char('e') => self.export_keys().await,
                KeyCode::Char('b') => {
                    if let Ok(new_balance) = self.wallet.balance().await {
                        self.balance = new_balance;
                        self.message = Some(AppMessage::Success("Balance refreshed!".to_string()));
                    } else {
                        self.message =
                            Some(AppMessage::Error("Failed to refresh balance".to_string()));
                    }
                }
                KeyCode::Char('q') => std::process::exit(0),
                _ => {}
            },
            AppState::SendModal => {
                if self.show_sent_modal {
                    match key.code {
                        KeyCode::Esc => {
                            self.show_sent_modal = false;
                            self.sent_notes = None;
                            self.state = AppState::Wallet;
                            self.message = None;
                        }
                        KeyCode::Char('c') => {
                            if let Some(notes) = self.sent_notes.clone() {
                                self.wl_copy(&notes);
                                self.message = Some(AppMessage::Success(
                                    "Ecash notes copied to clipboard!".to_string(),
                                ));
                            }
                        }
                        _ => {}
                    }
                } else {
                    match key.code {
                        KeyCode::Char(c) => {
                            if c.is_ascii_digit() {
                                self.send_amount.push(c);
                            }
                        }
                        KeyCode::Backspace => {
                            self.send_amount.pop();
                        }
                        KeyCode::Enter => {
                            if !self.send_amount.is_empty() {
                                self.send_ecash().await;
                            }
                        }
                        KeyCode::Esc => self.state = AppState::Wallet,
                        _ => {}
                    }
                }
            }
            AppState::ReceiveModal => match key.code {
                KeyCode::Char(c) => self.receive_input.push(c),
                KeyCode::Backspace => {
                    self.receive_input.pop();
                }
                KeyCode::Enter => {
                    if !self.receive_input.is_empty() {
                        self.receive_ecash().await;
                    }
                }
                KeyCode::Esc => self.state = AppState::Wallet,
                _ => {}
            },
            AppState::MnemonicModal => match key.code {
                KeyCode::Esc => {
                    self.state = AppState::Wallet;
                    self.mnemonic_phrase = None;
                }
                _ => {}
            },
            AppState::ReceivingModal => match key.code {
                KeyCode::Esc => {
                    self.state = AppState::Wallet;
                    self.receiving_status = None;
                }
                _ => {}
            },
        }
    }

    async fn join_federation(&mut self) {
        let invite_code = self.invite_code.clone();

        match self.wallet.join(&invite_code).await {
            Ok(_) => match self.wallet.balance().await {
                Ok(balance) => {
                    self.balance = balance;
                    self.state = AppState::Wallet;
                    self.wallet_loaded = true;
                    self.message = Some(AppMessage::Success(
                        "Successfully joined federation!".to_string(),
                    ));
                }
                Err(_) => {
                    self.message = Some(AppMessage::Error(
                        "Failed to get balance after joining".to_string(),
                    ));
                }
            },
            Err(_) => {
                self.message = Some(AppMessage::Error("Failed to join federation".to_string()));
            }
        }
    }

    async fn send_ecash(&mut self) {
        if let Ok(amount) = self.send_amount.parse::<u64>() {
            let sats = Amount::from_sats(amount);

            match self.wallet.spend_ecash(sats).await {
                Ok((_, notes)) => {
                    // Update balance after successful send
                    if let Ok(new_balance) = self.wallet.balance().await {
                        self.balance = new_balance;
                    }
                    self.sent_notes = Some(notes.to_string());
                    self.show_sent_modal = true;
                    self.message = Some(AppMessage::Success(format!(
                        "Generated {} SATS ecash notes!",
                        amount
                    )));
                }
                Err(_) => {
                    self.message = Some(AppMessage::Error("Failed to send ecash".to_string()));
                }
            }
        } else {
            self.message = Some(AppMessage::Error("Invalid amount".to_string()));
        }
    }

    async fn receive_ecash(&mut self) {
        let notes = self.receive_input.clone();

        // Clear input and show receiving modal
        self.receive_input.clear();
        self.state = AppState::ReceivingModal;
        self.receiving_status = Some("Receiving ecash notes...".to_string());

        match self.wallet.receive_ecash(&notes).await {
            Ok(amount) => {
                // Update balance after successful receive
                if let Ok(new_balance) = self.wallet.balance().await {
                    self.balance = new_balance;
                }
                self.receiving_status = Some(format!(
                    "Successfully received {} sats!",
                    amount.sats_f64() as u64
                ));
            }
            Err(e) => {
                self.receiving_status = Some(format!("Failed to receive ecash: {:?}", e));
            }
        }
    }

    async fn export_keys(&mut self) {
        if self.wallet_loaded {
            match self.wallet.show_mnemonic().await {
                Ok(words) => {
                    let mnemonic = words.join(" ");
                    self.mnemonic_phrase = Some(mnemonic);
                    self.state = AppState::MnemonicModal;
                }
                Err(_) => {
                    self.message = Some(AppMessage::Error("Failed to get seed phrase".to_string()));
                }
            }
        } else {
            self.message = Some(AppMessage::Error("No wallet loaded".to_string()));
        }
    }

    fn wl_copy(&self, text: &str) {
        use std::process::Command;

        // Fire and forget - don't care if it works
        let _ = Command::new("wl-copy").arg(text).spawn();
    }

    pub async fn update_balance(&mut self) -> Result<(), WalletError> {
        if self.wallet_loaded {
            self.balance = self.wallet.balance().await?;
        }
        Ok(())
    }
}

fn draw_intro(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .split(f.area());

    let title = Paragraph::new("TUImint - Fedimint TUI Wallet")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::ALL))
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(title, chunks[0]);

    let input_text = format!("Invite Code: {}", app.invite_code);
    let input = Paragraph::new(input_text)
        .style(Style::default().fg(Color::Yellow))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Enter Invite Code"),
        )
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(input, chunks[1]);

    let help = Paragraph::new("Type invite code and press Enter")
        .style(Style::default().fg(Color::Gray))
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(help, chunks[2]);

    if let Some(message) = &app.message {
        let message_area = centered_rect(60, 20, f.area());
        f.render_widget(Clear, message_area);

        let style = match message {
            AppMessage::Info(_) => Style::default().fg(Color::Blue),
            AppMessage::Success(_) => Style::default().fg(Color::Green),
            AppMessage::Error(_) => Style::default().fg(Color::Red),
        };

        let message_paragraph = Paragraph::new(message.to_string())
            .style(style)
            .block(Block::default().borders(Borders::ALL).title("Status"))
            .alignment(ratatui::layout::Alignment::Center);
        f.render_widget(message_paragraph, message_area);
    }
}

fn draw_wallet(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(5),
            Constraint::Length(3),
        ])
        .split(f.area());

    let balance_text = format!("Balance: {} sats", app.balance.sats_f64() as u64);
    let balance = Paragraph::new(balance_text)
        .style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::ALL).title("Wallet"))
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(balance, chunks[0]);

    let actions = Paragraph::new("Press 's' to Send | Press 'r' to Receive | Press 'b' to Refresh Balance | Press 'e' to Export Keys")
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL).title("Actions"))
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(actions, chunks[1]);

    let help = Paragraph::new("Press ESC to quit")
        .style(Style::default().fg(Color::Gray))
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(help, chunks[2]);
}

fn draw_send_modal(f: &mut Frame, app: &App) {
    let area = centered_rect(80, 60, f.area());
    f.render_widget(Clear, area);

    if app.show_sent_modal {
        // Show ecash notes modal
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(5),
            ])
            .margin(1)
            .split(area);

        let title = Paragraph::new("Ecash Notes Generated")
            .style(
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            )
            .block(Block::default().borders(Borders::ALL))
            .alignment(ratatui::layout::Alignment::Center);
        f.render_widget(title, chunks[0]);

        if let Some(notes) = &app.sent_notes {
            let notes_display = Paragraph::new(notes.as_str())
                .style(Style::default().fg(Color::White))
                .block(Block::default().borders(Borders::ALL).title("OOB Notes"))
                .wrap(ratatui::widgets::Wrap { trim: true })
                .alignment(ratatui::layout::Alignment::Left);
            f.render_widget(notes_display, chunks[1]);
        }

        let help = Paragraph::new("Press 'c' to copy to clipboard | ESC to go back")
            .style(Style::default().fg(Color::Gray))
            .alignment(ratatui::layout::Alignment::Center);
        f.render_widget(help, chunks[2]);
    } else {
        // Show amount input modal
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
            ])
            .margin(1)
            .split(area);

        let title = Paragraph::new("Send Ecash")
            .style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
            .block(Block::default().borders(Borders::ALL))
            .alignment(ratatui::layout::Alignment::Center);
        f.render_widget(title, chunks[0]);

        let input_text = format!("Amount (sats): {}", app.send_amount);
        let input = Paragraph::new(input_text)
            .style(Style::default().fg(Color::White))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Enter SATS Amount"),
            )
            .alignment(ratatui::layout::Alignment::Center);
        f.render_widget(input, chunks[1]);

        let help = Paragraph::new("Enter amount in SATS and press Enter | ESC to go back")
            .style(Style::default().fg(Color::Gray))
            .alignment(ratatui::layout::Alignment::Center);
        f.render_widget(help, chunks[2]);
    }

    if let Some(message) = &app.message {
        let message_area = centered_rect(50, 20, f.area());
        f.render_widget(Clear, message_area);

        let style = match message {
            AppMessage::Info(_) => Style::default().fg(Color::Blue),
            AppMessage::Success(_) => Style::default().fg(Color::Green),
            AppMessage::Error(_) => Style::default().fg(Color::Red),
        };

        let message_paragraph = Paragraph::new(message.to_string())
            .style(style)
            .block(Block::default().borders(Borders::ALL).title("Status"))
            .alignment(ratatui::layout::Alignment::Center);
        f.render_widget(message_paragraph, message_area);
    }
}

fn draw_receive_modal(f: &mut Frame, app: &App) {
    let area = centered_rect(80, 40, f.area());
    f.render_widget(Clear, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(5),
            Constraint::Length(3),
        ])
        .margin(1)
        .split(area);

    let title = Paragraph::new("Receive Ecash")
        .style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::ALL))
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(title, chunks[0]);

    // Truncate display for long ecash notes
    let display_input = if app.receive_input.len() > 50 {
        format!(
            "{}...{}",
            &app.receive_input[..25],
            &app.receive_input[app.receive_input.len() - 25..]
        )
    } else {
        app.receive_input.clone()
    };

    let input_text = format!("Ecash notes: {}", display_input);
    let input = Paragraph::new(input_text)
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Paste Ecash Notes"),
        )
        .alignment(ratatui::layout::Alignment::Center)
        .wrap(ratatui::widgets::Wrap { trim: true });
    f.render_widget(input, chunks[1]);

    let help = Paragraph::new("Paste ecash notes and press Enter | ESC to go back")
        .style(Style::default().fg(Color::Gray))
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(help, chunks[2]);

    if let Some(message) = &app.message {
        let message_area = centered_rect(50, 20, f.area());
        f.render_widget(Clear, message_area);

        let style = match message {
            AppMessage::Info(_) => Style::default().fg(Color::Blue),
            AppMessage::Success(_) => Style::default().fg(Color::Green),
            AppMessage::Error(_) => Style::default().fg(Color::Red),
        };

        let message_paragraph = Paragraph::new(message.to_string())
            .style(style)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Receive Status"),
            )
            .alignment(ratatui::layout::Alignment::Center);
        f.render_widget(message_paragraph, message_area);
    }
}

fn draw_mnemonic_modal(f: &mut Frame, app: &App) {
    let area = centered_rect(80, 60, f.area());
    f.render_widget(Clear, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(3),
        ])
        .margin(1)
        .split(area);

    let title = Paragraph::new("Seed Phrase")
        .style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::ALL))
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(title, chunks[0]);

    if let Some(phrase) = &app.mnemonic_phrase {
        let phrase_display = Paragraph::new(phrase.as_str())
            .style(
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("12 Word Mnemonic"),
            )
            .alignment(ratatui::layout::Alignment::Center)
            .wrap(ratatui::widgets::Wrap { trim: true });
        f.render_widget(phrase_display, chunks[1]);
    }

    let help = Paragraph::new("IMPORTANT: Write this down! Store it safely. ESC to go back")
        .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(help, chunks[2]);
}

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

fn draw_receiving_modal(f: &mut Frame, app: &App) {
    let area = centered_rect(60, 40, f.area());
    f.render_widget(Clear, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .margin(1)
        .split(area);

    let title = Paragraph::new("Receiving Ecash")
        .style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::ALL))
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(title, chunks[0]);

    if let Some(status) = &app.receiving_status {
        let status_style = if status.contains("Successfully") {
            Style::default().fg(Color::Green)
        } else if status.contains("Failed") {
            Style::default().fg(Color::Red)
        } else {
            Style::default().fg(Color::Blue)
        };

        let status_paragraph = Paragraph::new(status.to_string())
            .style(status_style)
            .block(Block::default().borders(Borders::ALL).title("Status"))
            .alignment(ratatui::layout::Alignment::Center);
        f.render_widget(status_paragraph, chunks[1]);
    }

    let help = Paragraph::new("ESC to go back")
        .style(Style::default().fg(Color::Gray))
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(help, chunks[2]);
}

fn draw(f: &mut Frame, app: &App) {
    match app.state {
        AppState::Intro => draw_intro(f, app),
        AppState::Wallet => draw_wallet(f, app),
        AppState::SendModal => {
            draw_wallet(f, app);
            draw_send_modal(f, app);
        }
        AppState::ReceiveModal => {
            draw_wallet(f, app);
            draw_receive_modal(f, app);
        }
        AppState::MnemonicModal => {
            draw_wallet(f, app);
            draw_mnemonic_modal(f, app);
        }
        AppState::ReceivingModal => {
            draw_wallet(f, app);
            draw_receiving_modal(f, app);
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    if let Err(e) = app.init().await {
        eprintln!("Failed to initialize app: {:?}", e);
        return Err("Initialization failed".into());
    }

    loop {
        terminal.draw(|f| draw(f, &app))?;

        match event::read() {
            Ok(Event::Key(key)) => {
                if key.code == KeyCode::Char('c')
                    && key
                        .modifiers
                        .contains(crossterm::event::KeyModifiers::CONTROL)
                {
                    break;
                }
                app.handle_key(key).await;
            }
            Ok(_) => {
                // Ignore other events
            }
            Err(e) => {
                // Log error but continue running
                eprintln!("Event read error: {:?}", e);
                continue;
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
