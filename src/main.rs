use color_eyre::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, BorderType, Borders, Paragraph},
    DefaultTerminal, Frame,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(draw)?;
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                break Ok(());
            }
        }
    }
}

fn draw(frame: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(8),
            Constraint::Percentage(8),
            Constraint::Percentage(8),
            Constraint::Percentage(54),
            Constraint::Percentage(24),
        ])
        .split(frame.area());
    render_block(frame, layout[0], "Translation", &Paragraph::new("Test"));
    render_block(frame, layout[1], "Book", &Paragraph::new("Test"));
    render_block(frame, layout[2], "Chapter", &Paragraph::new("Test"));
    render_block(frame, layout[3], "Read", &Paragraph::new("Test"));
    render_block(frame, layout[4], "Search", &Paragraph::new("Test"));
}

fn render_block(frame: &mut Frame, area: Rect, title: &str, paragraph: &Paragraph) {
    let block = Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(title);
    frame.render_widget(paragraph.clone().block(block), area);
}
