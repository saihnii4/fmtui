use std::{
    io::{self, stdout},
    time::Duration,
};

mod fmtui;

use fmtui::structs::*;

use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event, execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    symbols,
    widgets::{Block, List, Padding},
    Frame, Terminal,
};
use reqwest::blocking::{Client, ClientBuilder};

trait Screen {
    fn draw(&self, f: &mut Frame);
}

struct Main {}

impl Screen for Main {
    fn draw(&self, f: &mut Frame) {
        let block = Block::bordered().title_top("nablasleep");
        let list = List::new(["testing", "testing2"])
            .highlight_symbol(">")
            .highlight_spacing(ratatui::widgets::HighlightSpacing::Always)
            .block(block);
        f.render_widget(list, f.area());
    }
}

enum State {
    Main,
}

struct App {
    _exit: bool,
    state: State,
    client: Client,
    tracks: Vec<Track>,
    internal: Box<dyn Screen>,
}

impl App {
    fn run<B: Backend>(mut self, t: &mut Terminal<B>) -> io::Result<()> {
        while !self._exit {
            // TODO: multi-screen API
            // t.draw(|f| self.internal.draw(f))?;
            t.draw(|f| self._draw(f));
            self._poll_events();
        }
        Ok(())
    }

    fn _draw(&self, f: &mut Frame) {
        let block = Block::bordered()
            .border_set(symbols::border::DOUBLE)
            .title_bottom("fmtui")
            .padding(Padding::new(5, 5, 5, 5));
        let list = List::new(
            self.tracks
                .iter()
                .map(|t| format!("{} — {}", t.name, t.artist.text)),
        )
        .block(block);
        f.render_widget(list, f.area());
    }

    fn _poll_events(&mut self) {
        if let Ok(ready) = event::poll(Duration::from_millis(100)) {
            if !ready {
                return;
            }

            match event::read().unwrap() {
                event::Event::Key(key_ev) => match key_ev.kind {
                    event::KeyEventKind::Press => match key_ev.code {
                        event::KeyCode::Char('q') => {
                            self._exit = true;
                        }
                        event::KeyCode::Char('r') => {
                            self.refresh();
                        }
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            }
        }
    }

    fn refresh(&mut self) {

        let res = self
            .client
            .execute(req)
            .unwrap()
            .json::<serde_json::Value>()
            .unwrap();

        let test = res
            .as_object()
            .unwrap()
            .to_owned()
            .get("recenttracks")
            .unwrap()
            .to_owned()
            .get("track")
            .unwrap()
            .to_owned();

        self.tracks = serde_json::from_value(test).unwrap();
    }
}

impl Default for App {
    fn default() -> Self {
        let client = ClientBuilder::new()
            .build()
            .unwrap_or_else(|_err| panic!("bruh"));
        App {
            internal: Box::new(Main {}),
            state: State::Main,
            _exit: false,
            tracks: vec![],
            client: client,
        }
    }
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    App::default().run(&mut terminal)?;

    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
