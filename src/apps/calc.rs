use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct Calculator {
    pub input: String,
    pub result: String,
}

impl Calculator {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            result: String::from("0"),
        }
    }

    pub fn handle_input(&mut self, key: crossterm::event::KeyCode) {
        use crossterm::event::KeyCode;
        match key {
            KeyCode::Char(c) => {
                if "0123456789+-*/. ".contains(c) {
                    self.input.push(c);
                }
            }
            KeyCode::Backspace => {
                self.input.pop();
            }
            KeyCode::Enter => {
                self.calculate();
            }
            KeyCode::Esc => {
                self.input.clear();
                self.result = String::from("0");
            }
            _ => {}
        }
    }

    fn calculate(&mut self) {
        let trimmed = self.input.replace(" ", "");
        let operators = ['+', '-', '*', '/'];
        let mut found_op = None;

        for op in operators {
            if let Some(idx) = trimmed.find(op) {
                if idx == 0 && op == '-' { continue; }
                found_op = Some((op, idx));
                break;
            }
        }

        if let Some((op, idx)) = found_op {
            let left_str = &trimmed[..idx];
            let right_str = &trimmed[idx + 1..];

            let left = left_str.parse::<f64>().unwrap_or(0.0);
            let right = right_str.parse::<f64>().unwrap_or(0.0);

            let res = match op {
                '+' => left + right,
                '-' => left - right,
                '*' => left * right,
                '/' => {
                    if right == 0.0 {
                        self.result = String::from("Error: Div by 0");
                        return;
                    }
                    left / right
                }
                _ => 0.0,
            };
            self.result = res.to_string();
        } else {
            self.result = trimmed.parse::<f64>().unwrap_or(0.0).to_string();
        }
    }

    pub fn draw(&self, f: &mut Frame, area: Rect) {
        let content = format!(
            "\n   📊 Expressive TUI Calculator\n\n   \
             Input:  {}\n   \
             Result: {}\n\n   \
             Controls: Type math (e.g., 5 * 15), press Enter to evaluate.\n   \
             Press Esc inside this tab to clear.",
            self.input, self.result
        );

        let block = Block::default()
            .title(" 🧮 Calculator App ")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Cyan));

        f.render_widget(Paragraph::new(content).block(block), area);
    }
}
