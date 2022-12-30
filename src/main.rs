use std::{error::Error, io};

use crossterm::{terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, execute, event::{EnableMouseCapture, DisableMouseCapture, self, KeyCode, Event}};
use tui::{backend::{CrosstermBackend, Backend}, Terminal, Frame, layout::{Layout, Direction, Constraint}, widgets::{Paragraph, Block}, style::Style};


fn main() -> Result<(), Box<dyn Error>> {

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        format!("{err}");
    }
    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f))?;

        if let Event::Key(key) = event::read()? {
            match key.code {

                KeyCode::Char(_) => {
                    return Ok(());
                }
                _ => {}
            }
        }
    }

}

fn ui<B: Backend>(f: &mut Frame<B>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Min(1),
            ].as_ref()
            )
        .split(f.size());

    let text = Paragraph::new("Test string")
        .style(Style::default())
        .block(Block::default());

    f.render_widget(text, chunks[0]);

}
