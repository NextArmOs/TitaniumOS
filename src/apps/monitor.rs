use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use sysinfo::{Pid, System};

pub struct SystemMonitor {
    sys: System,
    pid: Pid,
}

impl SystemMonitor {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        let pid = Pid::from(std::process::id() as usize);

        Self { sys, pid }
    }

  pub fn update(&mut self) {
        self.sys.refresh_cpu_usage();
        self.sys.refresh_processes();
    }

    pub fn draw(&self, f: &mut Frame, area: Rect) {
        let global_cpu = self.sys.global_cpu_info().cpu_usage();
        
        let process_memory = if let Some(process) = self.sys.process(self.pid) {
            process.memory() as f64 / 1024.0 / 1024.0
        } else {
            0.0
        };

        let content = format!(
            "\n   ⚙️ TitaniumOS System Resource Monitor\n\n   \
             [Host CPU Load]    {:.1} %\n   \
             [OS Memory Usage]  {:.2} MB\n\n   \
             Status: Operational (Tracking via real-time host sys calls)",
            global_cpu, process_memory
        );

        let block = Block::default()
            .title(" 📊 Task Manager / Monitor ")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Magenta));

        f.render_widget(Paragraph::new(content).block(block), area);
    }
}
