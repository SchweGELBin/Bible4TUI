use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

pub fn draw(frame: &mut Frame) {
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
    render_block(
        frame,
        layout[3],
        "Read",
        &Paragraph::new(get_read_text()).wrap(Wrap { trim: true }),
    );
    render_block(frame, layout[4], "Search", &Paragraph::new("Test"));
}

fn render_block(frame: &mut Frame, area: Rect, title: &str, paragraph: &Paragraph) {
    let block = Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(title);
    frame.render_widget(paragraph.clone().block(block), area);
}

fn get_read_text() -> String {
    let selection = crate::logic::get_selection().unwrap();
    crate::logic::get_chapter(&selection.0, selection.1, selection.2).unwrap()
}
