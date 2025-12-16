use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame, Terminal,
};
use std::io;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

mod wallet;
use wallet::{Wallet, WalletError};

use fedimint_core::Amount;

#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
    Intro,
    Wallet,
    SendModal,
    ReceiveModal,
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    Info(String),
    Success(String),
    Error(String),
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
        // Clear messages on any key press except ESC
        if key.code != KeyCode::Esc {
            self.message = None;
        }

        match self.state {
            AppState::Intro => {
                match key.code {
                    KeyCode::Char(c) => self.invite_code.push(c),
                    KeyCode::Backspace => { self.invite_code.pop(); }
                    KeyCode::Enter => {
                        if !self.invite_code.is_empty() {
                            self.join_federation().await;
                        }
                    }
                    _ => {}
                }
            }
            AppState::Wallet => {
                match key.code {
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
                    _ => {}
                }
            }
            AppState::SendModal => {
                match key.code {
                    KeyCode::Char(c) => {
                        if c.is_ascii_digit() {
                            self.send_amount.push(c);
                        }
                    }
                    KeyCode::Backspace => { self.send_amount.pop(); }
                    KeyCode::Enter => {
                        if !self.send_amount.is_empty() {
                            self.send_ecash().await;
                        }
                    }
                    KeyCode::Esc => self.state = AppState::Wallet,
                    _ => {}
                }
            }
            AppState::ReceiveModal => {
                match key.code {
                    KeyCode::Char(c) => self.receive_input.push(c),
                    KeyCode::Backspace => { self.receive_input.pop(); }
                    KeyCode::Enter => {
                        if !self.receive_input.is_empty() {
                            self.receive_ecash().await;
                        }
                    }
                    KeyCode::Esc => self.state = AppState::Wallet,
                    _ => {}
                }
            }
        }
    }

    async fn join_federation(&mut self) {
        let invite_code = self.invite_code.clone();
        
        match self.wallet.join(&invite_code).await {
            Ok(_) => {
                match self.wallet.balance().await {
                    Ok(balance) => {
                        self.balance = balance;
                        self.state = AppState::Wallet;
                        self.wallet_loaded = true;
                        self.message = Some(AppMessage::Success("Successfully joined federation!".to_string()));
                    }
                    Err(_) => {
                        self.message = Some(AppMessage::Error("Failed to get balance after joining".to_string()));
                    }
                }
            }
            Err(_) => {
                self.message = Some(AppMessage::Error("Failed to join federation".to_string()));
            }
        }
    }

    async fn send_ecash(&mut self) {
        if let Ok(amount) = self.send_amount.parse::<u64>() {
            let sats = Amount::from_sats(amount);
            
            match self.wallet.spend_ecash(sats).await {
                Ok((_, _notes)) => {
                    self.message = Some(AppMessage::Success(format!("Sent {} SATS successfully!", amount)));
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
        
        match self.wallet.receive_ecash(&notes).await {
            Ok(amount) => {
                match self.wallet.balance().await {
                    Ok(new_balance) => {
                        self.balance = new_balance;
                        self.message = Some(AppMessage::Success(format!("Received {} sats successfully!", amount.sats_f64() as u64)));
                    }
                    Err(_) => {
                        self.message = Some(AppMessage::Error("Failed to update balance after receiving".to_string()));
                    }
                }
            }
            Err(_) => {
                self.message = Some(AppMessage::Error("Failed to receive ecash".to_string()));
            }
        }
    }

    async fn export_keys(&mut self) {
        todo!("Implement export keys functionality - needs load_or_generate_mnemonic to be public");
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
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL))
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(title, chunks[0]);

    let input_text = format!("Invite Code: {}", app.invite_code);
    let input = Paragraph::new(input_text)
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Enter Invite Code"))
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
        .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL).title("Wallet"))
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(balance, chunks[0]);

    let actions = Paragraph::new("Press 's' to Send | Press 'r' to Receive | Press 'e' to Export Keys")
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

    let title = Paragraph::new("Send Ecash")
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL))
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(title, chunks[0]);

    let input_text = format!("Amount (sats): {}", app.send_amount);
    let input = Paragraph::new(input_text)
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL).title("Enter SATS Amount"))
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(input, chunks[1]);

    let help = Paragraph::new("Enter amount in SATS and press Enter | ESC to go back")
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
            .block(Block::default().borders(Borders::ALL).title("Send Status"))
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
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL))
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(title, chunks[0]);

    // Truncate display for long ecash notes
    let display_input = if app.receive_input.len() > 50 {
        format!("{}...{}", &app.receive_input[..25], &app.receive_input[app.receive_input.len()-25..])
    } else {
        app.receive_input.clone()
    };
    
    let input_text = format!("Ecash notes: {}", display_input);
    let input = Paragraph::new(input_text)
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL).title("Paste Ecash Notes"))
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
            .block(Block::default().borders(Borders::ALL).title("Receive Status"))
            .alignment(ratatui::layout::Alignment::Center);
        f.render_widget(message_paragraph, message_area);
    }
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

impl std::fmt::Display for AppMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppMessage::Info(msg) => write!(f, "{}", msg),
            AppMessage::Success(msg) => write!(f, "{}", msg),
            AppMessage::Error(msg) => write!(f, "{}", msg),
        }
    }
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
        // Update balance periodically
        if let Err(_) = app.update_balance().await {
            // Handle balance update error if needed
        }

        terminal.draw(|f| draw(f, &app))?;

        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                break;
            }
            app.handle_key(key).await;
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
