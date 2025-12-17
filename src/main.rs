use crate::types::Wallets;
use color_eyre::Result;
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind,
        KeyModifiers,
    },
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use futures::StreamExt;
use ratatui::{
    Frame, Terminal,
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Paragraph, Wrap},
};
use std::{
    io,
    str::FromStr,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::sync::mpsc;

mod database;
mod types;

use fedimint_core::{Amount, config::FederationId};

#[derive(Debug)]
enum Screen {
    Join,
    Main,
}

#[derive(Debug)]
enum Modal {
    Send {
        amount_input: String,
        token: Option<String>,
        error: Option<String>,
        sending: bool,
        copied: bool,
    },
    Receive {
        token_input: String,
        received: Option<Amount>,
        error: Option<String>,
        receiving: bool,
    },
    Mnemonic {
        words: Vec<String>,
    },
}

#[derive(Debug)]
enum AppMessage {
    SendToken(String),
    SendError(String),
    ReceiveAmount(Amount),
    ReceiveError(String),
}

#[derive(Debug)]
struct App {
    wallets: Arc<Mutex<Wallets>>,
    current_screen: Screen,
    active_federation: Option<FederationId>,
    sidebar_index: usize,
    focus: Focus,
    modal: Option<Modal>,
    balances: std::collections::BTreeMap<FederationId, Amount>,
    join_input: String,
    join_loading: bool,
    join_error: Option<String>,
    join_pending: Option<String>,
    tx: mpsc::UnboundedSender<AppMessage>,
    rx: mpsc::UnboundedReceiver<AppMessage>,
}

#[derive(Debug, PartialEq)]
enum Focus {
    Sidebar,
    Main,
}

impl App {
    async fn new() -> Result<Self> {
        let wallets = Arc::new(Mutex::new(Wallets::new().await?));
        wallets.lock().unwrap().load_configs().await?;
        let current_screen = if wallets.lock().unwrap().get_clients().unwrap().is_empty() {
            Screen::Join
        } else {
            Screen::Main
        };
        let active_federation = wallets
            .lock()
            .unwrap()
            .get_clients()
            .unwrap()
            .keys()
            .next()
            .cloned();
        let balances = std::collections::BTreeMap::new();
        let (tx, rx) = mpsc::unbounded_channel();
        Ok(App {
            wallets,
            current_screen,
            active_federation,
            sidebar_index: 0,
            focus: Focus::Main,
            modal: None,
            balances,
            join_input: String::new(),
            join_loading: false,
            join_error: None,
            join_pending: None,
            tx,
            rx,
        })
    }

    async fn handle_key(&mut self, key: KeyEvent) -> bool {
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            return true;
        }
        if let Some(modal) = &mut self.modal {
            match modal {
                Modal::Send {
                    amount_input,
                    token,
                    sending,
                    copied,
                    ..
                } => {
                    if key.code == KeyCode::Esc {
                        self.modal = None;
                    } else if token.is_some() && key.code == KeyCode::Char('c') {
                        if let Some(t) = token {
                            let _ = tokio::process::Command::new("wlcopy").arg(t).status().await;
                            *copied = true;
                        }
                    } else if !*sending && token.is_none() {
                        match key.code {
                            KeyCode::Char(c) if c.is_ascii_digit() => amount_input.push(c),
                            KeyCode::Backspace => {
                                amount_input.pop();
                            }
                            KeyCode::Enter if !amount_input.trim().is_empty() => {
                                if let Ok(amount_msats) = amount_input.trim().parse::<u64>() {
                                    let amount = Amount::from_sats(amount_msats);
                                    if let Some(fid) = self.active_federation {
                                        *sending = true;
                                        let wallets = self.wallets.clone();
                                        let tx = self.tx.clone();
                                        tokio::spawn(async move {
                                            let client = wallets
                                                .lock()
                                                .unwrap()
                                                .get_client_by_id(fid)
                                                .unwrap();
                                            if let Ok(mint) = client.get_first_module::<fedimint_mint_client::MintClientModule>() {
                                                let result = mint.spend_notes_with_selector(
                                                    &fedimint_mint_client::SelectNotesWithAtleastAmount,
                                                    amount,
                                                    std::time::Duration::from_secs(60 * 60 * 24),
                                                    true,
                                                    NoMeta {},
                                                ).await;
                                                match result {
                                                    Ok((_, notes)) => {
                                                        let _ = tx.send(AppMessage::SendToken(notes.to_string()));
                                                    }
                                                    Err(_) => {
                                                        let _ = tx.send(AppMessage::SendError("Spend failed".to_string()));
                                                    }
                                                }
                                            }
                                        });
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
                Modal::Receive {
                    token_input,
                    received,
                    receiving,
                    ..
                } => {
                    if key.code == KeyCode::Esc {
                        self.modal = None;
                    } else if !*receiving && received.is_none() {
                        match key.code {
                            KeyCode::Char(c) => token_input.push(c),
                            KeyCode::Backspace => {
                                token_input.pop();
                            }
                            KeyCode::Enter if !token_input.trim().is_empty() => {
                                if let Some(fid) = self.active_federation {
                                    *receiving = true;
                                    let wallets = self.wallets.clone();
                                    let tx = self.tx.clone();
                                    let token = token_input.trim().to_string();
                                    tokio::spawn(async move {
                                        let client =
                                            wallets.lock().unwrap().get_client_by_id(fid).unwrap();
                                        if let Ok(mint) = client.get_first_module::<fedimint_mint_client::MintClientModule>() {
                                            if let Ok(oob_notes) = fedimint_mint_client::OOBNotes::from_str(&token) {
                                                if let Ok(operation_id) = mint.reissue_external_notes(oob_notes.clone(), NoMeta {}).await {
                                                    let mut updates = mint.subscribe_reissue_external_notes(operation_id).await.unwrap().into_stream();
                                                    while let Some(update) = updates.next().await {
                                                        if let fedimint_mint_client::ReissueExternalNotesState::Failed(_) = update {
                                                            let _ = tx.send(AppMessage::ReceiveError("Receive failed".to_string()));
                                                            return;
                                                        }
                                                    }
                                                    let amount = oob_notes.total_amount();
                                                    let _ = tx.send(AppMessage::ReceiveAmount(amount));
                                                } else {
                                                    let _ = tx.send(AppMessage::ReceiveError("Invalid token".to_string()));
                                                }
                                            } else {
                                                let _ = tx.send(AppMessage::ReceiveError("Invalid token".to_string()));
                                            }
                                        } else {
                                            let _ = tx.send(AppMessage::ReceiveError("Module error".to_string()));
                                        }
                                    });
                                }
                            }
                            _ => {}
                        }
                    }
                }
                Modal::Mnemonic { .. } => {
                    if key.code == KeyCode::Esc {
                        self.modal = None;
                    }
                }
            }
        } else {
            match self.current_screen {
                Screen::Join => {
                    if self.join_loading {
                        // Do nothing while loading
                    } else {
                        match key.code {
                            KeyCode::Char(c) => self.join_input.push(c),
                            KeyCode::Backspace => {
                                self.join_input.pop();
                            }
                            KeyCode::Esc => {
                                self.current_screen = Screen::Main;
                                self.join_input.clear();
                                self.join_error = None;
                            }
                            KeyCode::Enter => {
                                let invite = self.join_input.trim().to_string();
                                if !invite.is_empty() {
                                    self.join_pending = Some(invite);
                                    self.join_loading = true;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                Screen::Main => match key.code {
                    KeyCode::Char('a') => {
                        self.current_screen = Screen::Join;
                        self.join_input.clear();
                        self.join_error = None;
                    }
                    KeyCode::Char('w') => self.focus = Focus::Sidebar,
                    KeyCode::Char('s') => {
                        if self.active_federation.is_some() {
                            self.modal = Some(Modal::Send {
                                amount_input: String::new(),
                                token: None,
                                error: None,
                                sending: false,
                                copied: false,
                            });
                        }
                    }
                    KeyCode::Char('r') => {
                        if self.active_federation.is_some() {
                            self.modal = Some(Modal::Receive {
                                token_input: String::new(),
                                received: None,
                                error: None,
                                receiving: false,
                            });
                        }
                    }
                    KeyCode::Char('e') => {
                        let wallets = self.wallets.clone();
                        if let Ok(words) = wallets.lock().unwrap().show_mnemonic().await {
                            self.modal = Some(Modal::Mnemonic { words });
                        }
                    }
                    KeyCode::Up if self.focus == Focus::Sidebar => {
                        if self.sidebar_index > 0 {
                            self.sidebar_index -= 1;
                        }
                    }
                    KeyCode::Down if self.focus == Focus::Sidebar => {
                        let len = self.wallets.lock().unwrap().get_clients().unwrap().len();
                        if self.sidebar_index < len.saturating_sub(1) {
                            self.sidebar_index += 1;
                        }
                    }
                    KeyCode::Enter if self.focus == Focus::Sidebar => {
                        if let Some(fid) = self
                            .wallets
                            .lock()
                            .unwrap()
                            .get_clients()
                            .unwrap()
                            .keys()
                            .nth(self.sidebar_index)
                        {
                            self.active_federation = Some(*fid);
                        }
                    }
                    _ => {}
                },
            }
        }
        false
    }

    async fn update(&mut self) {
        // Handle pending join
        if let Some(invite) = self.join_pending.take() {
            let result = self.wallets.lock().unwrap().join(&invite).await;
            match result {
                Ok(_) => {
                    self.join_loading = false;
                    self.current_screen = Screen::Main;
                    self.active_federation = self
                        .wallets
                        .lock()
                        .unwrap()
                        .get_clients()
                        .unwrap()
                        .keys()
                        .next()
                        .cloned();
                }
                Err(_) => {
                    self.join_loading = false;
                    self.join_error = Some("Join failed".to_string());
                }
            }
        }
        // Handle messages
        while let Ok(msg) = self.rx.try_recv() {
            match msg {
                AppMessage::SendToken(token) => {
                    if let Some(Modal::Send {
                        token: t, sending, ..
                    }) = &mut self.modal
                    {
                        *t = Some(token);
                        *sending = false;
                    }
                }
                AppMessage::SendError(err) => {
                    if let Some(Modal::Send {
                        error: e, sending, ..
                    }) = &mut self.modal
                    {
                        *e = Some(err);
                        *sending = false;
                    }
                }
                AppMessage::ReceiveAmount(amount) => {
                    if let Some(Modal::Receive {
                        received: r,
                        receiving,
                        ..
                    }) = &mut self.modal
                    {
                        *r = Some(amount);
                        *receiving = false;
                    }
                    // Update balance
                    if let Some(fid) = self.active_federation {
                        self.balances
                            .entry(fid)
                            .and_modify(|b| *b = *b + amount)
                            .or_insert(amount);
                    }
                }
                AppMessage::ReceiveError(err) => {
                    if let Some(Modal::Receive {
                        error: e,
                        receiving,
                        ..
                    }) = &mut self.modal
                    {
                        *e = Some(err);
                        *receiving = false;
                    }
                }
            }
        }
        // Update balances
        if let Ok(clients) = self.wallets.lock().unwrap().get_clients() {
            for (fid, client) in clients.iter() {
                if let Some(balance) = client.get_balance().await {
                    self.balances.insert(*fid, balance);
                }
            }
        }
    }
}

#[derive(serde::Serialize)]
struct NoMeta {}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = setup_terminal()?;
    let app = App::new().await?;
    run_app(&mut terminal, app).await?;
    restore_terminal(&mut terminal)?;
    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

async fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;
        if crossterm::event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if app.handle_key(key).await {
                        break;
                    }
                }
            }
        }
        app.update().await;
    }
    Ok(())
}

fn ui(f: &mut Frame, app: &App) {
    let size = f.area();
    match app.current_screen {
        Screen::Join => draw_join_screen(f, app, size),
        Screen::Main => draw_main_screen(f, app, size),
    }
    if let Some(modal) = &app.modal {
        draw_modal(f, app, modal, size);
    }
}

fn draw_join_screen(f: &mut Frame, app: &App, size: Rect) {
    let block = Block::default()
        .title("Join Federation")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().add_modifier(Modifier::DIM));
    let inner = block.inner(size);
    f.render_widget(block, size);
    let text = if app.join_loading {
        vec![Line::from("Joining federation...")]
    } else if let Some(err) = &app.join_error {
        vec![
            Line::from("Error:"),
            Line::from(err.clone()),
            Line::from("Press ESC to go back"),
        ]
    } else {
        vec![
            Line::from("Paste federation invite code:"),
            Line::from(""),
            Line::from(app.join_input.clone()),
        ]
    };
    let para = Paragraph::new(text)
        .block(Block::default())
        .alignment(Alignment::Center);
    f.render_widget(para, inner);
}

fn draw_main_screen(f: &mut Frame, app: &App, size: Rect) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(size);

    // Sidebar
    let block = Block::default()
        .title("Federations")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().add_modifier(Modifier::DIM).fg(
            if app.focus == Focus::Sidebar {
                Color::Blue
            } else {
                Color::White
            },
        ));
    let items: Vec<ListItem> = app
        .wallets
        .lock()
        .unwrap()
        .get_clients()
        .unwrap()
        .keys()
        .enumerate()
        .map(|(i, fid)| {
            let balance = app.balances.get(fid).cloned().unwrap_or(Amount::ZERO);
            let fid_str = fid.to_string();
            let mut spans = Vec::new();
            if fid_str.len() >= 6 {
                let color_str = &fid_str[0..6];
                if let Ok(color_val) = u32::from_str_radix(color_str, 16) {
                    let r = ((color_val >> 16) & 0xFF) as u8;
                    let g = ((color_val >> 8) & 0xFF) as u8;
                    let b = (color_val & 0xFF) as u8;
                    let color = Color::Rgb(r, g, b);
                    spans.push(Span::styled(
                        fid_str[0..6].to_string(),
                        Style::default().fg(color),
                    ));
                    spans.push(Span::styled(
                        fid_str[6..].to_string(),
                        Style::default().add_modifier(Modifier::DIM),
                    ));
                } else {
                    spans.push(Span::raw(fid_str));
                }
            } else {
                spans.push(Span::raw(fid_str));
            }
            spans.push(Span::raw(format!(": {} sats", balance.msats / 1000)));
            let mut style = Style::default();
            if Some(*fid) == app.active_federation {
                style = style.fg(Color::Green);
            }
            if app.focus == Focus::Sidebar && i == app.sidebar_index {
                style = style.add_modifier(Modifier::REVERSED);
            }
            ListItem::new(Line::from(spans)).style(style)
        })
        .collect();
    let list = List::new(items).block(block);
    f.render_widget(list, layout[0]);

    // Main area
    let mut title = "Actions".to_string();
    if let Some(fid) = app.active_federation {
        if let Some(balance) = app.balances.get(&fid) {
            let balance_sats = balance.msats / 1000;
            title = format!("Actions - Balance: {} sats", balance_sats);
        }
    }
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().add_modifier(Modifier::DIM).fg(
            if app.focus == Focus::Main {
                Color::Blue
            } else {
                Color::White
            },
        ));
    let text = vec![
        Line::from("Press 's' to Send"),
        Line::from("Press 'r' to Receive"),
        Line::from("Press 'e' to show Mnemonic"),
        Line::from("Press 'a' to Add federation"),
        Line::from("Press 'w' to focus sidebar"),
    ];
    let para = Paragraph::new(text).block(block);
    f.render_widget(para, layout[1]);
}

fn draw_modal(f: &mut Frame, app: &App, modal: &Modal, size: Rect) {
    let modal_size = Rect::new(
        size.width / 4,
        size.height / 4,
        size.width / 2,
        size.height / 2,
    );
    f.render_widget(Clear, modal_size);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().add_modifier(Modifier::DIM));
    let inner = block.inner(modal_size);
    f.render_widget(block, modal_size);
    match modal {
        Modal::Send {
            amount_input,
            token,
            error,
            sending,
            copied,
        } => {
            let title = if token.is_some() {
                "Send Ecash - Token"
            } else {
                "Send Ecash - Amount"
            };
            let block = Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().add_modifier(Modifier::DIM));
            let mut text = if let Some(err) = error {
                vec![Line::from("Error:"), Line::from(err.clone())]
            } else if *copied {
                vec![Line::from("Copied to clipboard!")]
            } else if *sending {
                vec![Line::from("Sending...")]
            } else if let Some(t) = token {
                vec![
                    Line::from("Token:"),
                    Line::from(t.clone()),
                    Line::from("Press 'c' to copy"),
                ]
            } else {
                vec![
                    Line::from("Enter amount in sats:"),
                    Line::from(amount_input.clone()),
                ]
            };
            text.push(Line::from(""));
            text.push(Line::from("Press ESC to close"));
            let para = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
            f.render_widget(para, inner);
        }
        Modal::Receive {
            token_input,
            received,
            error,
            receiving,
        } => {
            let title = if received.is_some() {
                "Receive Ecash - Success"
            } else {
                "Receive Ecash - Token"
            };
            let block = Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().add_modifier(Modifier::DIM));
            let mut text = if let Some(err) = error {
                vec![Line::from("Error:"), Line::from(err.clone())]
            } else if *receiving {
                vec![Line::from("Receiving...")]
            } else if let Some(amount) = received {
                vec![Line::from(format!("Received {} sats", amount.msats / 1000))]
            } else {
                vec![
                    Line::from("Paste ecash token:"),
                    Line::from(token_input.clone()),
                ]
            };
            text.push(Line::from(""));
            text.push(Line::from("Press ESC to close"));
            let para = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
            f.render_widget(para, inner);
        }
        Modal::Mnemonic { words } => {
            let block = Block::default()
                .title("Mnemonic")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().add_modifier(Modifier::DIM));
            let mut text: Vec<Line> = words
                .chunks(3)
                .map(|chunk| {
                    let spans: Vec<Span> = chunk
                        .iter()
                        .enumerate()
                        .flat_map(|(i, word)| {
                            if i > 0 {
                                vec![Span::raw(" "), Span::raw(word)]
                            } else {
                                vec![Span::raw(word)]
                            }
                        })
                        .collect();
                    Line::from(spans)
                })
                .collect();
            text.push(Line::from(""));
            text.push(Line::from("Press ESC to close"));
            let para = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
            f.render_widget(para, inner);
        }
    }
}
