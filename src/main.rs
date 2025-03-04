use std::io::Result;

use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Direction, Flex, Layout, Rect},
    style::{Style, Stylize},
    symbols,
    widgets::{Block, Borders, List, Widget},
    DefaultTerminal, Frame,
};

#[derive(Debug, Default)]
struct App {
    exit: bool,
}

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|f| self.render(f))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn render(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key_ev) => {
                self.handle_key_ev(key_ev);
            }
            _ => {}
        }

        Ok(())
    }

    fn handle_key_ev(&mut self, key_ev: KeyEvent) {
        match key_ev.code {
            KeyCode::Char('q') => {
                self.exit = true;
            }
            KeyCode::Up => {
                todo!("TODO");
            }
            _ => {}
        }
    }
}

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Fill(1), Constraint::Fill(2)])
            .split(area);

        let [new_area] = Layout::horizontal([Constraint::Length(2)])
            .flex(Flex::Center)
            .areas(layout[0]);

        Block::new()
            .borders(Borders::ALL)
            .border_style(Style::default().red())
            .border_set(symbols::border::ROUNDED)
            .render(layout[0], buf);

        Block::new()
            .borders(Borders::ALL)
            .border_style(Style::default().red())
            .border_set(symbols::border::ROUNDED)
            .render(layout[1], buf);

        let l_list = List::new(vec!["Testing", "Testing", "Testing"]);
        let r_list = List::new(vec!["Testing", "Testing", "Testing"]);

        l_list.render(new_area, buf);
        r_list.render(layout[1], buf);
    }
}

fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    App::default().run(&mut terminal)
}
