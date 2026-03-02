mod logic;
mod ui;

use color_eyre::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    DefaultTerminal,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}

#[derive(Debug, Default)]
pub struct App {
    running: bool,
    selection: (String, usize, usize),
    col_read: String,
    col_translation: String,
}

impl App {
    pub fn new() -> Self {
        let running = true;
        let selection = logic::get_selection().unwrap();
        let col_read = logic::get_chapter(&selection.0, selection.1, selection.2).unwrap();
        let col_translation = logic::get_translation_list().unwrap();
        Self {
            running,
            selection,
            col_read,
            col_translation,
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| ui::draw(frame, &self))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            _ => {}
        }
    }

    fn quit(&mut self) {
        self.running = false;
    }
}
