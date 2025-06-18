use std::fmt::Debug;

use color_eyre::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Position},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, List, ListItem, Paragraph},
    DefaultTerminal, Frame,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::new().run(terminal);
    ratatui::restore();
    app_result
}
pub struct App{
    input:      String,
    char_index: usize,
    todos:      Vec<String>,
    exit:       bool,
    mode:       MODE,  
}
impl App {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            char_index: 0,
            todos: Vec::new(),
            exit: false,
            mode: MODE::CMD,
        }
    }
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop{
            terminal.draw(|frame| self.draw(frame))?;
            
            if let Event::Key(key) = event::read()? {
                match self.mode {
                    MODE::CMD => match key.code {
                        KeyCode::Char('q') => {
                            self.exit = true;
                            return Ok(());
                        },
                        KeyCode::Char('a') => {
                            self.mode = MODE::CREATING;
                        },
                        KeyCode::Char('s') => {},
                        _ => {},
                    }
                    MODE::CREATING => match key.code {
                        KeyCode::Esc => {
                            self.mode = MODE::CMD;
                        },
                        KeyCode::Char(_to_insert) => {},
                        KeyCode::Backspace => {},
                        KeyCode::Enter => {},
                        KeyCode::Left => {},
                        KeyCode::Right => {},
                        _ => {},
                    }
                }
            }
        }
    }
    
    fn draw(& self, frame: &mut Frame){
        let vertical = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Min(1),
        ]);
        let [help_area, input_area, messages_area] = vertical.areas(frame.area());
        let (msg, style) = match self.mode {
            MODE::CMD => (
                vec![
                    "Press ".into(),
                    "q".bold(),
                    " to exit, ".into(),
                    "s".bold(),
                    " to save, ".into(),
                    "a".bold(),
                    " to starting editing.".into(),
                ],
                Style::default().add_modifier(Modifier::SLOW_BLINK),
            ),
            MODE::CREATING => (
                vec![
                    "Press ".into(),
                    "Esc".bold(),
                    " to stop editing, ".into(),
                    "Enter".bold(),
                    " to record the message".into(),
                ],
                Style::default(),
            ),
        };
        
        let text = Text::from(Line::from(msg)).patch_style(style);
        let help_message = Paragraph::new(text);
        frame.render_widget(help_message, help_area);

        let input = Paragraph::new(self.input.as_str())
            .style(match self.mode {
                MODE::CMD => Style::default(),
                MODE::CREATING => Style::default().fg(Color::Yellow),
            })
            .block(Block::bordered().title("Input"));
        frame.render_widget(input, input_area);
        match self.mode {
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            MODE::CMD => {}

            // Make the cursor visible and ask ratatui to put it at the specified coordinates after
            // rendering
            #[allow(clippy::cast_possible_truncation)]
            MODE::CREATING => frame.set_cursor_position(Position::new(
                // Draw the cursor at the current position in the input field.
                // This position is can be controlled via the left and right arrow key
                input_area.x + self.char_index as u16 + 1,
                // Move one line down, from the border to the input line
                input_area.y + 1,
            )),
        }

        let messages: Vec<ListItem> = self
            .todos
            .iter()
            .enumerate()
            .map(|(i, m)| {
                let content = Line::from(Span::raw(format!("{i}: {m}")));
                ListItem::new(content)
            })
            .collect();
        let messages = List::new(messages).block(Block::bordered().title("Messages"));
        frame.render_widget(messages, messages_area);
    }
}



enum MODE{
    CREATING,
    CMD,
}
impl Default for MODE{
    fn default() -> Self {MODE::CMD}
}
impl Debug for MODE {
    fn fmt(& self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self{
            MODE::CREATING => write!(f,"CREATING"),
            MODE::CMD => write!(f, "CMD"),
        }
    }
}