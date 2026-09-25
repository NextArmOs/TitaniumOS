use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use std::collections::VecDeque;

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct SnakeGame {
    pub snake: VecDeque<(u16, u16)>,
    pub direction: Direction,
    pub food: (u16, u16),
    pub score: u32,
    pub is_game_over: bool,
    pub width: u16,
    pub height: u16,
}

impl SnakeGame {
    pub fn new() -> Self {
        let mut snake = VecDeque::new();
        snake.push_back((10, 5));
        snake.push_back((9, 5));
        snake.push_back((8, 5));

        Self {
            snake,
            direction: Direction::Right,
            food: (15, 4),
            score: 0,
            is_game_over: false,
            width: 30,
            height: 10,
        }
    }

    pub fn handle_input(&mut self, key: crossterm::event::KeyCode) {
        use crossterm::event::KeyCode;
        if self.is_game_over {
            if let KeyCode::Char('r') | KeyCode::Char('R') = key {
                *self = Self::new();
            }
            return;
        }

        match key {
            KeyCode::Char('w') | KeyCode::Char('W') if self.direction != Direction::Down => self.direction = Direction::Up,
            KeyCode::Char('s') | KeyCode::Char('S') if self.direction != Direction::Up => self.direction = Direction::Down,
            KeyCode::Char('a') | KeyCode::Char('A') if self.direction != Direction::Right => self.direction = Direction::Left,
            KeyCode::Char('d') | KeyCode::Char('D') if self.direction != Direction::Left => self.direction = Direction::Right,
            _ => {}
        }
    }

    pub fn update(&mut self) {
        if self.is_game_over {
            return;
        }

        let head = self.snake.front().copied().unwrap();
        let mut new_head = head;

        match self.direction {
            Direction::Up => new_head.1 = new_head.1.checked_sub(1).unwrap_or(self.height - 1),
            Direction::Down => new_head.1 = (new_head.1 + 1) % self.height,
            Direction::Left => new_head.0 = new_head.0.checked_sub(1).unwrap_or(self.width - 1),
            Direction::Right => new_head.0 = (new_head.0 + 1) % self.width,
        }

        if self.snake.contains(&new_head) {
            self.is_game_over = true;
            return;
        }

        self.snake.push_front(new_head);

        if new_head == self.food {
            self.score += 10;
            self.spawn_food();
        } else {
            self.snake.pop_back();
        }
    }

    fn spawn_food(&mut self) {
        use std::time::SystemTime;
        let nanos = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos();
        let next_x = (nanos % (self.width as u128)) as u16;
        let next_y = ((nanos >> 2) % (self.height as u128)) as u16;
        self.food = (next_x, next_y);
    }

    pub fn draw(&self, f: &mut Frame, area: Rect) {
        if self.is_game_over {
            let over_text = format!("\n   GAME OVER!\n   Score: {}\n   Press 'R' to Restart", self.score);
            let block = Block::default().title(" 🎮 Snake Game ").borders(Borders::ALL).style(Style::default().fg(Color::Red));
            f.render_widget(Paragraph::new(over_text).block(block), area);
            return;
        }

        let mut grid = vec![vec![" "; self.width as usize]; self.height as usize];

        let (fx, fy) = self.food;
        if (fy as usize) < grid.len() && (fx as usize) < grid[0].len() {
            grid[fy as usize][fx as usize] = "🍎";
        }

        for &(sx, sy) in &self.snake {
            if (sy as usize) < grid.len() && (sx as usize) < grid[0].len() {
                grid[sy as usize][sx as usize] = "■";
            }
        }

        let mut display_string = format!(" Score: {}\n\n", self.score);
        for row in grid {
            display_string.push_str("  ");
            for cell in row {
                display_string.push_str(cell);
            }
            display_string.push('\n');
        }

        let block = Block::default()
            .title(" 🎮 Snake Game (Controls: W, A, S, D) ")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Green));
            
        f.render_widget(Paragraph::new(display_string).block(block), area);
    }
}

