use std::time::Duration;

use crossterm::event::{self, Event};
use ratatui::{
    Frame, Terminal,
    layout::{Constraint, Direction, Layout},
    prelude::Backend,
    style::{Color, Style, Styled},
    widgets::{Block, BorderType, Borders, Paragraph, Row, Table},
};

use crate::{app::App, output::format_memory};
use anyhow::Result;

pub fn run_tui<B: Backend>(terminal: &mut Terminal<B>) -> Result<()> {
    let mut app = App::new();
    loop {
        terminal.draw(|f| ui(f, &mut app))?;
        if app.should_quit {
            break;
        }
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                app.handle_key(key.code);
            }
        }
        app.update_processes()?;
    }
    Ok(())
}

pub fn ui(f: &mut Frame, app: &mut App) {
    let size = f.area();

    let header = Row::new(vec!["PID", "USER", "NAME", "CPU%", "MEM(KB)", "STATE"])
        .style(Style::default().fg(Color::Yellow))
        .height(1);
    let widths = vec![
        Constraint::Percentage(16),
        Constraint::Percentage(16),
        Constraint::Percentage(16),
        Constraint::Percentage(16),
        Constraint::Percentage(16),
        Constraint::Percentage(16),
    ];

    let rows = app
        .processes
        .iter()
        .map(|process| {
            Row::new(vec![
                process.pid.to_string(),
                process.user.clone(),
                process.name.clone(),
                format!("{:1}", process.cpu_percent),
                format_memory(process.memory_kb),
                format!("{:?}", process.state),
            ])
            .style(Style::default().fg(Color::LightCyan))
        })
        .collect::<Vec<Row>>();
    let table = Table::new(rows, widths)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::all())
                .border_type(BorderType::Double),
        )
        .style(Style::default().fg(Color::Yellow))
        .row_highlight_style(Color::Cyan)
        .highlight_symbol(">>");

    let menu = Paragraph::new("[Q]uit | [C]pu | [M]em | [P]ID | [N]ame")
        .block(Block::default().borders(Borders::ALL).title("Menu"))
        .style(Style::default().fg(Color::Yellow));

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(5), Constraint::Length(3)])
        .split(size);

    f.render_stateful_widget(table, chunks[0], &mut app.table_state);
    f.render_widget(menu, chunks[1]);
}
