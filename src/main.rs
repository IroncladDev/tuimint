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
use wallet::Wallet;

#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
    Intro,
    Wallet,
    SendModal,
    ReceiveModal,
}

#[derive(Debug, Clone)]
pub struct App {
    state: AppState,
    invite_code: String,
    balance: u64,
    send_amount: String,
    receive_input: String,
    send_result: Option<String>,
    receive_result: Option<String>,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: AppState::Intro,
            invite_code: String::new(),
            balance: 0,
            send_amount: String::new(),
            receive_input: String::new(),
            send_result: None,
            receive_result: None,
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match self.state {
            AppState::Intro => {
                match key.code {
                    KeyCode::Char(c) => self.invite_code.push(c),
                    KeyCode::Backspace => { self.invite_code.pop(); }
                    KeyCode::Enter => {
                        if !self.invite_code.is_empty() {
                            self.join_federation();
                            self.state = AppState::Wallet;
                            self.balance = 1000; // Placeholder balance
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
                        self.send_result = None;
                    }
                    KeyCode::Char('r') => {
                        self.state = AppState::ReceiveModal;
                        self.receive_input.clear();
                        self.receive_result = None;
                    }
                    KeyCode::Char('e') => self.export_keys(),
                    _ => {}
                }
            }
            AppState::SendModal => {
                match key.code {
                    KeyCode::Char(c) => self.send_amount.push(c),
                    KeyCode::Backspace => { self.send_amount.pop(); }
                    KeyCode::Enter => {
                        if !self.send_amount.is_empty() {
                            self.send_ecash();
                            self.send_result = Some(format!("Sent {} satoshis", self.send_amount));
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
                            self.receive_ecash();
                            self.receive_result = Some("Ecash received successfully".to_string());
                            self.balance += 500; // Placeholder balance update
                        }
                    }
                    KeyCode::Esc => self.state = AppState::Wallet,
                    _ => {}
                }
            }
        }
    }

    fn join_federation(&mut self) {
        todo!("Implement clientJoin functionality");
    }

    fn send_ecash(&mut self) {
        todo!("Implement send functionality");
    }

    fn receive_ecash(&mut self) {
        todo!("Implement receive functionality");
    }

    fn export_keys(&mut self) {
        todo!("Implement export keys functionality");
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

    let balance_text = format!("Balance: {} satoshis", app.balance);
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

    let input_text = format!("Amount: {}", app.send_amount);
    let input = Paragraph::new(input_text)
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL).title("Enter Amount"))
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(input, chunks[1]);

    let help = Paragraph::new("Enter amount and press Enter | ESC to go back")
        .style(Style::default().fg(Color::Gray))
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(help, chunks[2]);

    if let Some(result) = &app.send_result {
        let result_area = centered_rect(50, 20, f.area());
        f.render_widget(Clear, result_area);
        let result_paragraph = Paragraph::new(result.as_str())
            .style(Style::default().fg(Color::Green))
            .block(Block::default().borders(Borders::ALL).title("Send Result"))
            .alignment(ratatui::layout::Alignment::Center);
        f.render_widget(result_paragraph, result_area);
    }
}

fn draw_receive_modal(f: &mut Frame, app: &App) {
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

    let title = Paragraph::new("Receive Ecash")
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL))
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(title, chunks[0]);

    let input_text = format!("Input: {}", app.receive_input);
    let input = Paragraph::new(input_text)
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL).title("Enter Ecash"))
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(input, chunks[1]);

    let help = Paragraph::new("Enter ecash and press Enter | ESC to go back")
        .style(Style::default().fg(Color::Gray))
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(help, chunks[2]);

    if let Some(result) = &app.receive_result {
        let result_area = centered_rect(50, 20, f.area());
        f.render_widget(Clear, result_area);
        let result_paragraph = Paragraph::new(result.as_str())
            .style(Style::default().fg(Color::Green))
            .block(Block::default().borders(Borders::ALL).title("Receive Result"))
            .alignment(ratatui::layout::Alignment::Center);
        f.render_widget(result_paragraph, result_area);
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

    loop {
        terminal.draw(|f| draw(f, &app))?;

        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                break;
            }
            app.handle_key(key);
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
