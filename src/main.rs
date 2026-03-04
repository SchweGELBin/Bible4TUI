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
    columns: (String, String, String, String),
}

impl App {
    pub fn new() -> Self {
        let running = true;
        let _ = logic::initialize();
        let selection = logic::get_selection().unwrap();
        let columns = App::get_columns(&selection.0, selection.1, selection.2).unwrap();
        Self {
            running,
            selection,
            columns,
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
            (_, KeyCode::Left) => {
                let _ = logic::turn_chapter(false);
                let selection = logic::get_selection().unwrap();
                self.update(&selection.0, selection.1, selection.2);
            }
            (_, KeyCode::Right) => {
                let _ = logic::turn_chapter(true);
                let selection = logic::get_selection().unwrap();
                self.update(&selection.0, selection.1, selection.2);
            }
            _ => {}
        }
    }

    fn quit(&mut self) {
        self.running = false;
    }

    fn update(&mut self, translation: &str, book: usize, chapter: usize) {
        self.selection = (translation.to_string(), book, chapter);
        self.columns = App::get_columns(translation, book, chapter).unwrap();
    }

    fn get_columns(
        translation: &str,
        book: usize,
        chapter: usize,
    ) -> Result<(String, String, String, String)> {
        let col_translation = logic::get_translation_list().unwrap();
        let col_book = logic::get_book_list(&translation).unwrap();
        let col_chapter = logic::get_chapter_list(&translation, book).unwrap();
        let col_read = logic::get_chapter(&translation, book, chapter).unwrap();
        Ok((col_translation, col_book, col_chapter, col_read))
    }
}
