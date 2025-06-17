use crate::tools::{Tool, ToolRunner};
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};
use std::path::Path;
use tokio::sync::mpsc;

pub struct App {
    pub tools: Vec<Tool>,
    pub selected_tool: usize,
    pub results: Vec<(String, String)>, // (tool_name, result)
    pub should_quit: bool,
    pub directory: std::path::PathBuf,
    tools_state: ListState,
    results_state: ListState,
    result_sender: mpsc::Sender<(String, String)>,
    result_receiver: mpsc::Receiver<(String, String)>,
    show_help: bool,
}

impl App {
    pub fn new(directory: &Path) -> Self {
        let tools = crate::tools::detect_tools(directory);
        let mut tools_state = ListState::default();
        tools_state.select(Some(0));
        
        let (result_sender, result_receiver) = mpsc::channel(100);
        
        Self {
            tools,
            selected_tool: 0,
            results: Vec::new(),
            should_quit: false,
            directory: directory.to_path_buf(),
            tools_state,
            results_state: ListState::default(),
            result_sender,
            result_receiver,
            show_help: false,
        }
    }

    pub fn handle_events(&mut self) -> Result<()> {
        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => self.should_quit = true,
                        KeyCode::Char('?') => self.show_help = !self.show_help,
                        KeyCode::Char('r') => {
                            let sender = self.result_sender.clone();
                            let tool = self.tools[self.selected_tool].clone();
                            let directory = self.directory.clone();
                            tokio::spawn(async move {
                                if let Ok(result) = tool.run(&directory).await {
                                    let _ = sender.send((tool.name, result)).await;
                                }
                            });
                        }
                        KeyCode::Char('a') => {
                            let sender = self.result_sender.clone();
                            let tools = self.tools.clone();
                            let directory = self.directory.clone();
                            tokio::spawn(async move {
                                for tool in tools {
                                    if let Ok(result) = tool.run(&directory).await {
                                        let _ = sender.send((tool.name, result)).await;
                                    }
                                }
                            });
                        }
                        KeyCode::Up => {
                            if self.selected_tool > 0 {
                                self.selected_tool -= 1;
                                self.tools_state.select(Some(self.selected_tool));
                            }
                        }
                        KeyCode::Down => {
                            if self.selected_tool < self.tools.len().saturating_sub(1) {
                                self.selected_tool += 1;
                                self.tools_state.select(Some(self.selected_tool));
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        // Check for new results
        while let Ok((tool_name, result)) = self.result_receiver.try_recv() {
            self.results.push((tool_name, result));
        }

        Ok(())
    }

    pub fn draw(&mut self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(f.size());

        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(70),
            ])
            .split(chunks[0]);

        // Tools list
        let tools: Vec<ListItem> = self
            .tools
            .iter()
            .enumerate()
            .map(|(i, tool)| {
                let style = if i == self.selected_tool {
                    Style::default().add_modifier(Modifier::REVERSED)
                } else {
                    Style::default()
                };
                ListItem::new(tool.name.clone()).style(style)
            })
            .collect();

        let tools_list = List::new(tools)
            .block(Block::default().title("Tools").borders(Borders::ALL));
        f.render_stateful_widget(tools_list, main_chunks[0], &mut self.tools_state);

        // Results
        let results: Vec<ListItem> = self
            .results
            .iter()
            .map(|(tool, result)| {
                ListItem::new(format!("[{}]\n{}", tool, result))
                    .style(Style::default().fg(Color::Green))
            })
            .collect();

        let results_list = List::new(results)
            .block(Block::default().title("Results").borders(Borders::ALL));
        f.render_stateful_widget(results_list, main_chunks[1], &mut self.results_state);

        // Help menu
        if self.show_help {
            // Calculate the size of the help window
            let help_text = vec![
                "Key Bindings:".to_string(),
                "  ↑/↓    - Navigate tools".to_string(),
                "  r      - Run selected tool".to_string(),
                "  a      - Run all tools".to_string(),
                "  ?      - Toggle this help menu".to_string(),
                "  q      - Quit".to_string(),
            ];
            let help_height = help_text.len() as u16 + 2; // +2 for borders
            let help_width = help_text.iter().map(|s| s.len()).max().unwrap_or(0) as u16 + 4; // +4 for borders

            // Center the help window
            let help_area = centered_rect(help_width, help_height, f.size());
            
            // Create shadow effect by drawing a slightly offset dark rectangle
            let shadow_area = Rect {
                x: help_area.x + 1,
                y: help_area.y + 1,
                width: help_area.width,
                height: help_area.height,
            };
            let shadow = Block::default()
                .style(Style::default().bg(Color::Rgb(0, 0, 0)));
            f.render_widget(shadow, shadow_area);
            
            // Create the help window with a semi-transparent background
            let help = Paragraph::new(help_text.join("\n"))
                .block(
                    Block::default()
                        .title("Help")
                        .borders(Borders::ALL)
                        .style(Style::default().bg(Color::Rgb(32, 32, 32)).fg(Color::White))
                )
                .style(Style::default().fg(Color::Yellow));
            
            f.render_widget(help, help_area);
        }

        // Status bar with key hints
        let status_text = if self.show_help {
            "Press ? to hide help"
        } else {
            "Press ? for help | ↑/↓: Navigate | r: Run | a: Run All | q: Quit"
        };
        let status = Paragraph::new(status_text)
            .block(Block::default().borders(Borders::TOP))
            .style(Style::default().fg(Color::Gray));
        f.render_widget(status, chunks[1]);
    }
}

impl Clone for App {
    fn clone(&self) -> Self {
        let (result_sender, result_receiver) = mpsc::channel(100);
        Self {
            tools: self.tools.clone(),
            selected_tool: self.selected_tool,
            results: self.results.clone(),
            should_quit: self.should_quit,
            directory: self.directory.clone(),
            tools_state: ListState::default(),
            results_state: ListState::default(),
            result_sender,
            result_receiver,
            show_help: self.show_help,
        }
    }
}

/// Helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(width: u16, height: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(((100 - height as u16 * 100 / r.height) / 2) as u16),
            Constraint::Length(height),
            Constraint::Percentage(((100 - height as u16 * 100 / r.height) / 2) as u16),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(((100 - width as u16 * 100 / r.width) / 2) as u16),
            Constraint::Length(width),
            Constraint::Percentage(((100 - width as u16 * 100 / r.width) / 2) as u16),
        ])
        .split(popup_layout[1])[1]
} 