use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, Tabs},
    Terminal,
};
use crossterm::{
    event::{self, Event, KeyCode, MouseEventKind, MouseButton},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::ffi::CString;
use std::io;
use std::os::raw::c_char;
use std::time::{Duration, Instant};

extern "C" {
    fn execute_system_command(command: *const c_char, output: *mut c_char, max_len: i32) -> i32;
}

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout, 
        EnterAlternateScreen, 
        crossterm::event::EnableMouseCapture
    )?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut current_tab = 0;
    let mut bg_color = Color::Black;
    
    let mut input_buffer = String::new();
    let mut console_history = vec![
        String::from("Welcome to TitaniumOS Terminal!"),
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

    let mut fps = 0;
    let mut frame_count = 0;
    let mut last_fps_update = Instant::now();
    let mut target_fps: u32 = 60;

    loop {
        let frame_start = Instant::now();

        frame_count += 1;
        if last_fps_update.elapsed() >= Duration::from_secs(1) {
            fps = frame_count;
            frame_count = 0;
            last_fps_update = Instant::now();
        }

        let mut taskbar_areas = Vec::new();

        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Min(1), Constraint::Length(3)].as_ref())
                .split(size);

            let main_style = Style::default().bg(bg_color).fg(Color::White);

            let desktop_block = Block::default()
                .title(" 💻 TitaniumOS Desktop v0.04 ")
                .borders(Borders::ALL)
                .style(main_style);

            match current_tab {
                0 => {
                    let content = format!(
                        "{}\n\n 📁 Documents   🎮 Games\n\n Welcome to your desktop!\n Use Left/Right arrows or click the Taskbar to navigate.",
                        ascii_art
                    );
                    f.render_widget(Paragraph::new(content).block(desktop_block), chunks[0]);
                }
                1 => {
                    let mut console_lines = console_history.clone();
                    console_lines.push(format!("guest@titanium_os:~# {}", input_buffer));
                    let console_content = console_lines.join("\n");
                    f.render_widget(Paragraph::new(console_content).block(desktop_block), chunks[0]);
                }
                2 => {
                    let hz_status = if target_fps == 9999 { String::from("UNLIMITED") } else { format!("{} Hz", target_fps) };
                    let content = format!(
                        " 🛠️ Settings Menu\n\n \
                         [Background Color] Press:\n \
                         1 -> Deep Blue\n \
                         2 -> Classic Black\n \
                         3 -> Slate Gray\n\n \
                         [Frame Rate Limiter / Display Hz] Press:\n \
                         4 -> Set to 30 Hz (Power Save)\n \
                         5 -> Set to 60 Hz (Standard)\n \
                         6 -> Set to 144 Hz (Gaming)\n \
                         7 -> Uncapped (Max Performance)\n\n \
                         Current Limiter Status: {}\n\n \
                         System Info Summary:\n \
                         Host: Hybrid Linux Environment",
                        hz_status
                    );
                    f.render_widget(Paragraph::new(content).block(desktop_block), chunks[0]);
                }
                _ => {}
            }

            let taskbar_title = format!(" Taskbar | FPS: {} | Limit: {}Hz ", fps, if target_fps == 9999 { "None".to_string() } else { target_fps.to_string() });
            let titles = vec!["🏠 Desktop", "📟 Console", "⚙️ Settings", "❌ Exit (Esc)"];
            
            let tabs = Tabs::new(titles)
                .block(Block::default().title(taskbar_title).borders(Borders::ALL))
                .select(current_tab)
                .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
            
            f.render_widget(tabs, chunks[1]);

            let start_x = chunks[1].x + 2; 
            let y = chunks[1].y + 1;
            let tab_widths = vec![12, 12, 13, 13]; 
            let mut current_x = start_x;
            
            for width in tab_widths {
                taskbar_areas.push((current_x, current_x + width, y));
                current_x += width + 1; 
            }
        })?;

        if event::poll(Duration::from_millis(1))? {
            match event::read()? {
                Event::Key(key) => {
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
                                            console_history.push(format!("guest@titanium_os:~# {}", input_buffer));
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
                                    KeyCode::Char('4') => target_fps = 30,
                                    KeyCode::Char('5') => target_fps = 60,
                                    KeyCode::Char('6') => target_fps = 144,
                                    KeyCode::Char('7') => target_fps = 9999,
                                    _ => {}
                                }
                            }
                        }
                    }
                }
                Event::Mouse(mouse_event) => {
                    if mouse_event.kind == MouseEventKind::Down(MouseButton::Left) {
                        for (index, &(start_x, end_x, y)) in taskbar_areas.iter().enumerate() {
                            if mouse_event.row == y && mouse_event.column >= start_x && mouse_event.column < end_x {
                                if index == 3 {
                                    disable_raw_mode()?;
                                    execute!(terminal.backend_mut(), LeaveAlternateScreen, crossterm::event::DisableMouseCapture)?;
                                    return Ok(());
                                }
                                current_tab = index;
                            }
                        }
                    }
                }
                _ => {}}}if target_fps != 9999 {
                    let target_frame_time = Duration::from_secs_f64(1.0 / target_fps as f64);
                    let elapsed = frame_start.elapsed();
                    if elapsed < target_frame_time {std::thread::sleep(target_frame_time - elapsed);
                    }
                }
            }
            disable_raw_mode()?;
            execute!
            (terminal.backend_mut(),
            LeaveAlternateScreen,
            crossterm::event::DisableMouseCapture)?;
            terminal.show_cursor()?;Ok(())
        }
