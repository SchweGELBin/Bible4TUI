use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

pub fn draw(frame: &mut Frame, app: &crate::App) {
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
    render_block(
        frame,
        layout[0],
        "Translation",
        &Paragraph::new(&*app.col_translation),
    );
    render_block(frame, layout[1], "Book", &Paragraph::new("Test"));
    render_block(frame, layout[2], "Chapter", &Paragraph::new("Test"));
    render_block(
        frame,
        layout[3],
        "Read",
        &Paragraph::new(&*app.col_read).wrap(Wrap { trim: true }),
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
