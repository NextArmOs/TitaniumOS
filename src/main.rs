use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph, Tabs},
    Terminal,
};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut current_tab = 0;

    loop {
        terminal.draw(|f| {
            let size = f.size();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Min(1), Constraint::Length(3)].as_ref())
                .split(size);

            let desktop_content = match current_tab {
                0 => " 📁 Documents   ⚙️ Settings   🎮 Games\n\n Select a tab below to switch applications.",
                1 => " [OS Console]\n guest@terminal_os:~# _",
                2 => " 🎮 Launch Tetris or Snake! (Games coming soon)",
                _ => "",
            };

            let desktop = Block::default()
                .title(" TitaniumOS v0.2 ")
                .borders(Borders::ALL);
            let paragraph = Paragraph::new(desktop_content).block(desktop);
            f.render_widget(paragraph, chunks);

            let titles = vec!["🏠 Desktop", "📟 Console", "🎮 Game Center", "❌ Exit (Esc)"];
            let tabs = Tabs::new(titles)
                .block(Block::default().title(" Taskbar ").borders(Borders::ALL))
                .select(current_tab);
            f.render_widget(tabs, chunks);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc => break,
                KeyCode::Right => {
                    current_tab = (current_tab + 1) % 4;
                    if current_tab == 3 { break; }
                }
                KeyCode::Left => {
                    if current_tab > 0 {
                        current_tab -= 1;
                    } else {
                        current_tab = 2;
                    }
                }
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
