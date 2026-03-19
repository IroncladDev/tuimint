use crate::state::Screen;
use crate::ui::prelude::*;
use ratatui::prelude::*;
use ratatui::widgets::*;
use ratatui_macros::{line, span};

pub struct TutorialScreen {
    step: u32,
}

impl TutorialScreen {
    pub fn new() -> Self {
        Self { step: 0 }
    }

    fn get_content(&self) -> Vec<Line<'_>> {
        match self.step {
            0 => vec![
                line![
                    span!(" TUImint").blue().bold(),
                    span!(" is a wallet client for "),
                    span!(" Fedimint").light_cyan().bold(),
                ],
                line!("                                    🬇🬃   ").light_cyan(),
                line![
                    span!(" ▟▀█▀▙").blue(),
                    span!("                           "),
                    span!("🬁🬀🬖🬂🬂🬢🬁🬀").light_cyan(),
                ],
                line![
                    span!(" ▜ ▀ ▛").blue(),
                    span!("                           "),
                    span!("🬋 🬈🬭🬭🬅 🬋").light_cyan(),
                ],
                line!("                                   🬋  🬋  ").light_cyan(),
            ],
            1 => vec![
                line![
                    span!("Multiple "),
                    span!("🬋 Guardian Servers").magenta().bold(),
                    span!(" are linked together"),
                ],
                line!(""),
                line!(span!("   🬇🬃   ").magenta()),
                line![
                    span!("🬁🬀").magenta(),
                    span!("🬖🬂🬂🬢").light_cyan(),
                    span!("🬁🬀").magenta(),
                ],
                line![
                    span!("🬋 ").magenta(),
                    span!("🬈🬭🬭🬅").light_cyan(),
                    span!(" 🬋").magenta(),
                ],
                line!(span!("  🬋  🬋  ")).magenta(),
                line!(""),
                line![
                    span!("to form a single "),
                    span!(" Federation").light_cyan().bold(),
                ],
            ],
            2 => vec![
                line![
                    span!("If some "),
                    span!("🬋 Guardians").magenta().bold(),
                    span!(" go "),
                    span!("offline").red().bold(),
                    span!(","),
                ],
                line![
                    span!("the "),
                    span!(" Federation").light_cyan().bold(),
                    span!(" can still function"),
                ],
                line!(""),
                line!("   🬇🬃   ").red(),
                line![
                    span!("🬁🬀").magenta(),
                    span!("🬖🬂🬂🬢").light_cyan(),
                    span!("🬁🬀").magenta(),
                ],
                line![
                    span!("🬋 ").magenta(),
                    span!("🬈🬭🬭🬅").light_cyan(),
                    span!(" 🬋").magenta(),
                ],
                line![span!("  🬋 ").red(), span!(" 🬋  ").magenta(),],
                line!(""),
                line!("This improves reliability and uptime"),
            ],
            3 => vec![
                line!("To make an authoritative decision such as"),
                line![
                    span!("updating the "),
                    span!(" Fedimint").light_cyan().bold(),
                    span!(" software or shutting down,"),
                ],
                line![
                    span!("🬋 Guardians").magenta().bold(),
                    span!(" must vote and reach consensus"),
                ],
                line!(""),
                line![
                    span!("🬋").green(),
                    span!(" = yes, "),
                    span!("🬋").red(),
                    span!(" = no"),
                ],
                line!(""),
                line!("   🬇🬃   ").green(),
                line![
                    span!("🬁🬀").green(),
                    span!("🬖🬂🬂🬢").light_cyan(),
                    span!("🬁🬀").green(),
                ],
                line![
                    span!("🬋 ").green(),
                    span!("🬈🬭🬭🬅").light_cyan(),
                    span!(" 🬋").green(),
                ],
                line!("  🬋  🬋  ").red(),
                line!(""),
                line!("This reduces the risk of individual bad actors"),
            ],
            4 => vec![
                line!("Under the Gold Standard, people could go"),
                line![
                    span!("to the bank and exchange "),
                    span!("cash").green().bold(),
                    span!(" for "),
                    span!("gold").yellow().bold(),
                ],
                line!(""),
                line!("▄▄▄"),
                line![
                    span!("╭─── $10 of ").blue(),
                    span!("cash").green(),
                    span!(" ───▶").blue(),
                    span!(" ▄█████▄ "),
                    span!("◀─── $5 of ").magenta(),
                    span!("gold").yellow(),
                    span!(" ───╮").magenta(),
                ],
                line![
                    span!("  ◀─ $10 of ").blue(),
                    span!("gold").yellow(),
                    span!(" ──────").blue(),
                    span!("█ █ █"),
                    span!("────── $5 of ").magenta(),
                    span!("cash").green(),
                    span!(" ─▶  ").magenta(),
                ],
                line![
                    span!("Bob").blue(),
                    span!("                    ▀▀▀▀▀▀▀                  "),
                    span!("Alice").magenta(),
                ],
            ],
            5 => vec![
                line![
                    span!(" Fedimint").light_cyan().bold(),
                    span!(" allows you to exchange "),
                    span!(" Bitcoin").light_yellow().bold(),
                ],
                line![
                    span!(" for "),
                    span!(" Electronic Cash").light_green().bold(),
                    span!(" (Ecash)").green(),
                    span!(" and vice versa")
                ],
                line!(""),
                line!("🬇🬃").light_cyan(),
                line![
                    span!("╭─── 0.1 ").blue(),
                    span!("BTC").light_yellow().bold(),
                    span!(" ─────────▶").blue(),
                    span!(" 🬁🬀🬖🬂🬂🬢🬁🬀").light_cyan(),
                    span!(" ◀── 0.2 ").magenta(),
                    span!("BTC").light_yellow().bold(),
                    span!(" of ").magenta(),
                    span!("Ecash").green(),
                    span!(" ─╮").magenta(),
                ],
                line![
                    span!(" ◀─ 0.1 ").blue(),
                    span!("BTC").light_yellow().bold(),
                    span!(" of ").blue(),
                    span!("Ecash").green(),
                    span!(" ──").blue(),
                    span!("🬋 🬈🬭🬭🬅 🬋").light_cyan(),
                    span!("──── 0.2 ").magenta(),
                    span!("BTC").light_yellow().bold(),
                    span!(" ────────▶ ").magenta(),
                ],
                line![
                    span!("Bob ").blue(),
                    span!("                      🬋  🬋                     ").light_cyan(),
                    span!("Alice").magenta(),
                ],
                line!(""),
                line![
                    span!("All "),
                    span!(" Ecash").light_green().bold(),
                    span!(" is backed by "),
                    span!(" Bitcoin").light_yellow().bold(),
                    span!(", making it impossible"),
                ],
                line!("to mint fraudulent notes backed by nothing"),
            ],
            _ => vec![line!("your mom")],
        }
    }
}

impl Component for TutorialScreen {
    fn render(&mut self, frame: &mut Frame, state: &AppStateMutex) {
        let state = state.lock().unwrap();

        if state.screen != Screen::Tutorial {
            return;
        }

        let lines = self.get_content();
        let number_of_lines = lines.len() as u16;
        let text = Text::from(lines).style(Style::default());
        let paragraph = Paragraph::new(text).centered();

        let block = Block::bordered()
            .padding(Padding::vertical(1))
            .title_bottom(span!(" [ESC] home ").into_left_aligned_line())
            .title_bottom(span!(" [p] prev, [n] next ").into_right_aligned_line());
        frame.render_widget(
            block,
            frame
                .area()
                .centered(Constraint::Max(60), Constraint::Max(18)),
        );
        frame.render_widget(
            paragraph,
            frame
                .area()
                .centered(Constraint::Max(60), Constraint::Length(number_of_lines)),
        );
    }

    fn on_key_event(
        &mut self,
        event: KeyEvent,
        state: &AppStateMutex,
        _tx: TxSender,
    ) -> anyhow::Result<()> {
        let mut state = state.lock().unwrap();

        if KeyCode::Char('n') == event.code {
            self.step += 1;
            // TODO: add constraint
        }

        if KeyCode::Char('p') == event.code && self.step > 0 {
            self.step -= 1;
        }

        if KeyCode::Esc == event.code {
            self.step = 0;
            state.navigate(Screen::Splash);
        }

        Ok(())
    }
}
