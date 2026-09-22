use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, Tabs},
    Terminal,
};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::ffi::CString;
use std::io;
use std::os::raw::c_char;

extern "C" {
    fn execute_system_command(command: *const c_char, output: *mut c_char, max_len: i32) -> i32;
}

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut current_tab = 0;
    let mut bg_color = Color::Black;
    
    let mut input_buffer = String::new();
    let mut console_history = vec![
        String::from("Welcome to TitaniumOS!"),
        String::from("Type 'help' for a list of commands."),
        String::from("")
    ];

     let ascii_art = r#"
  _________________________ 
  _______   _
 |__   __| |-|         / __ \ / ____|
    | |    |_|         | |  | | (___  
    | |    | |         | |  | |\___ \ 
    | |    | |         | |__| |____) |
    |_|    |_|          \____/|_____/ 
    
    
    
    
    
      "#;

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Min(1), Constraint::Length(3)].as_ref())
                .split(size);

            let main_style = Style::default().bg(bg_color).fg(Color::White);

            let desktop_block = Block::default()
                .title(" TitaniumOS v0.03 ")
                .borders(Borders::ALL)
                .style(main_style);

            match current_tab {
                0 => {
                    let content = format!(
                        "{}\n\n 📁 Documents   🎮 Games\n\n Welcome to your desktop!\n Use Left/Right arrows to navigate the Taskbar.",
                        ascii_art
                    );
                    f.render_widget(Paragraph::new(content).block(desktop_block), chunks[0]);
                }
                1 => {
                    let mut console_lines = console_history.clone();
                    console_lines.push(format!("root@titanium_os:~# {}", input_buffer));
                    let console_content = console_lines.join("\n");
                    f.render_widget(Paragraph::new(console_content).block(desktop_block), chunks[0]);
                }
                2 => {
                    let content = " 🛠️ Settings Menu\n\n Press keys to change Accent Background Color:\n 1 -> Deep Blue\n 2 -> Classic Black\n 3 -> Slate Gray\n\n System Info Summary:\n Host: Hybrid Linux Environment";
                    f.render_widget(Paragraph::new(content).block(desktop_block), chunks[0]);
                }
                _ => {}
            }

            let titles = vec!["🏠 Desktop", "📟 Console", "⚙️ Settings", "❌ Exit (Esc)"];
            let tabs = Tabs::new(titles)
                .block(Block::default().title(" Taskbar ").borders(Borders::ALL))
                .select(current_tab)
                .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
            f.render_widget(tabs, chunks[1]);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc => break,
                KeyCode::Right => {
                    current_tab = (current_tab + 1) % 4;
                    if current_tab == 3 { break; }
                }
                KeyCode::Left => {
                    if current_tab > 0 { current_tab -= 1; } else { current_tab = 2; }
                }
                _ => {
                    if current_tab == 1 {
                        match key.code {
                            KeyCode::Enter => {
                                if !input_buffer.trim().is_empty() {
                                    console_history.push(format!("root@titanium_os:~# {}", input_buffer));
                                    
                                    let c_command = CString::new(input_buffer.trim()).unwrap();
                                    let mut buffer = vec![0u8; 256];
                                    
                                    unsafe {
                                        let status = execute_system_command(
                                            c_command.as_ptr(),
                                            buffer.as_mut_ptr() as *mut c_char,
                                            256
                                        );
                                        
                                        if status == 2 {
                                            console_history.clear();
                                        } else {
                                            let c_str = std::ffi::CStr::from_ptr(buffer.as_ptr() as *const c_char);
                                            let response = c_str.to_string_lossy().into_owned();
                                            for line in response.lines() {
                                                console_history.push(line.to_string());
                                            }
                                        }
                                    }
                                    console_history.push(String::new());
                                    input_buffer.clear();
                                }
                            }
                            KeyCode::Backspace => {
                                input_buffer.pop();
                            }
                            KeyCode::Char(c) => {
                                input_buffer.push(c);
                            }
                            _ => {}
                        }
                    } else if current_tab == 2 {
                        match key.code {
                            KeyCode::Char('1') => bg_color = Color::Blue,
                            KeyCode::Char('2') => bg_color = Color::Black,
                            KeyCode::Char('3') => bg_color = Color::DarkGray,
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
