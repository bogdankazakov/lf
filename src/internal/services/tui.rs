pub mod help;
pub mod key_inputs;
pub mod logs;
pub mod search_input;
pub mod stats;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    Event,
    internal::{
        domain::records::RecordsError,
        services::tui::{help::Help, logs::Logs, stats::Stats},
    },
};

use ratatui::layout::{Position, Rect};
use search_input::SearchInput;

#[derive(thiserror::Error, Debug)]
pub enum TuiError {
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    Records(#[from] RecordsError),
    #[error("receiver error: {0}")]
    Channel(#[from] std::sync::mpsc::RecvError),
}

pub struct App {
    exit: bool,
    logs: Logs,
    input: SearchInput,
    stats: Stats,
    help: Help,
    show_help: bool,
    show_scrollbar: bool,
    show_input: bool,
    rx: std::sync::mpsc::Receiver<Event>,
}

impl App {
    pub fn new(rx: std::sync::mpsc::Receiver<Event>) -> App {
        Self {
            exit: false,
            logs: Logs::default(),
            input: SearchInput::new(),
            stats: Stats::default(),
            help: Help::default(),
            show_help: false,
            show_scrollbar: true,
            show_input: true,
            rx,
        }
    }

    pub fn run(&mut self, terminal: &mut ratatui::DefaultTerminal) -> Result<(), TuiError> {
        // draw before any event happen
        terminal.draw(|frame| self.draw(frame))?;

        while !self.exit {
            match self.rx.recv()? {
                Event::StdIn(i) => {
                    self.logs.records_mut().add(i);
                }
                Event::KeyInput(event) => {
                    self.handle_key_input(event);
                }
            }
            terminal.draw(|frame| self.draw(frame))?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut ratatui::Frame) {
        // Render help
        if self.show_help {
            let help_v_layout = ratatui::layout::Layout::vertical([
                ratatui::layout::Constraint::Percentage(25),
                ratatui::layout::Constraint::Min(3),
                ratatui::layout::Constraint::Percentage(25),
            ])
            .split(frame.area())[1];
            let help_area = ratatui::layout::Layout::horizontal([
                ratatui::layout::Constraint::Percentage(30),
                ratatui::layout::Constraint::Min(3),
                ratatui::layout::Constraint::Percentage(30),
            ])
            .split(help_v_layout)[1];
            frame.render_widget(&mut self.help, help_area);
            return;
        }

        //Build layout
        let mut search_area = Rect::new(0, 0, 0, 0);
        let mut logs_area = Rect::new(0, 0, 0, 0);
        if self.show_input {
            let vertical_layout = ratatui::layout::Layout::vertical([
                ratatui::layout::Constraint::Min(3),
                ratatui::layout::Constraint::Percentage(100),
            ]);
            [search_area, logs_area] = vertical_layout.areas(frame.area());
        } else {
            let vertical_layout =
                ratatui::layout::Layout::vertical([ratatui::layout::Constraint::Percentage(100)]);
            [logs_area] = vertical_layout.areas(frame.area());
        }

        let mut input_area = Rect::new(0, 0, 0, 0);
        let mut stats_area = Rect::new(0, 0, 0, 0);
        if self.show_input {
            let search_layout = ratatui::layout::Layout::horizontal([
                ratatui::layout::Constraint::Percentage(100),
                ratatui::layout::Constraint::Min(14),
            ]);
            [input_area, stats_area] = search_layout.areas(search_area);
        }

        // Render logs
        self.logs.set_show_scrollbar(self.show_scrollbar);
        frame.render_widget(&mut self.logs, logs_area);

        if self.show_input {
            self.stats.set(
                self.logs.records().len() as u32,
                self.logs.records().len_filtered() as u32,
            );

            // Render input widget
            frame.render_widget(&mut self.input, search_area);
            frame.set_cursor_position(Position::new(
                input_area.x + self.input.character_index as u16 + 1,
                input_area.y + 1,
            ));

            // Render stats
            frame.render_widget(&mut self.stats, stats_area);
        }
    }

    fn handle_key_input(&mut self, key: KeyEvent) {
        match key {
            _ if key.code == KeyCode::Up => {
                self.logs.scroll_up();
            }
            _ if key.code == KeyCode::Down => {
                self.logs.scroll_down();
            }
            _ if key.code == KeyCode::Char('c') && key.modifiers == KeyModifiers::CONTROL => {
                self.exit = true;
            }
            _ if key.code == KeyCode::Char('q') && key.modifiers == KeyModifiers::CONTROL => {
                self.exit = true;
            }
            _ if key.code == KeyCode::Char('d') && key.modifiers == KeyModifiers::CONTROL => {
                self.logs.scroll_down();
            }
            _ if key.code == KeyCode::Char('u') && key.modifiers == KeyModifiers::CONTROL => {
                self.logs.scroll_up();
            }
            _ if key.code == KeyCode::Char('n') && key.modifiers == KeyModifiers::CONTROL => {
                self.logs.scroll_down_many();
            }
            _ if key.code == KeyCode::Char('p') && key.modifiers == KeyModifiers::CONTROL => {
                self.logs.scroll_up_many();
            }
            _ if key.code == KeyCode::Char('a') && key.modifiers == KeyModifiers::CONTROL => {
                self.logs.set_auto_scroll();
            }
            _ if key.code == KeyCode::Char('t') && key.modifiers == KeyModifiers::CONTROL => {
                self.logs.scroll_to_top();
            }
            _ if key.code == KeyCode::Char('b') && key.modifiers == KeyModifiers::CONTROL => {
                self.show_scrollbar = !self.show_scrollbar;
            }
            _ if key.code == KeyCode::Char('s') && key.modifiers == KeyModifiers::CONTROL => {
                self.show_input = !self.show_input;
                if !self.show_input {
                    self.input.clear();
                    self.logs.records_mut().set_filter_key("".into());
                }
            }
            _ if key.code == KeyCode::Char('h') && key.modifiers == KeyModifiers::CONTROL => {
                self.show_help = !self.show_help;
            }
            _ if key.code == KeyCode::Esc && self.show_help => {
                self.show_help = false;
            }
            _ => {
                if self.show_input {
                    self.input.process_input(key);
                    self.logs
                        .records_mut()
                        .set_filter_key(self.input.value.clone().into());
                    self.logs.scroll_to_top();
                    self.logs.set_auto_scroll();
                }
            }
        }
    }
}
