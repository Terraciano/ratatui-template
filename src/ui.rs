use crate::App;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Stylize},
    widgets::{block::Title, Block, Borders, Paragraph},
    Frame,
};

pub fn ui(f: &mut Frame, app: &App) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(f.size());

    let right_panel_block = Block::default()
        .borders(Borders::ALL)
        .add_modifier(Modifier::BOLD)
        .title(Title::from(app.app_title.clone()).alignment(Alignment::Center));

    let current_input = Paragraph::new(app.current_input.clone()).block(right_panel_block);

    f.render_widget(current_input, main_layout[0]);
}
