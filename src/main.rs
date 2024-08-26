use std::{
    collections::HashMap,
    io::{self, stdout},
    time::Duration,
};

use serde::{Deserialize, Serialize};

use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event, execute,
        style::Stylize,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    widgets::{Block, List},
    Frame, Terminal,
};
use reqwest::blocking::ClientBuilder;
use serde_json::Value;

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
    internal: Box<dyn Screen>,
}

impl App {
    fn run<B: Backend>(mut self, t: &mut Terminal<B>) -> io::Result<()> {
        while !self._exit {
            t.draw(|f| self.internal.draw(f))?;
            self._poll_events();
        }
        Ok(())
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
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            }
        }
    }
}

impl Default for App {
    fn default() -> Self {
        App {
            internal: Box::new(Main {}),
            state: State::Main,
            _exit: false,
        }
    }
}

// #[derive(Serialize, Deserialize)]
// struct

#[derive(Serialize, Deserialize)]
struct Album {
    #[serde(rename(deserialize = "#text"))]
    text: String,
    mbid: String,
}

#[derive(Serialize, Deserialize)]
struct Track {
    album: Album,

    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

fn main() -> io::Result<()> {
    // enable_raw_mode()?;
    // execute!(stdout(), EnterAlternateScreen)?;

    if let Ok(client) = ClientBuilder::new().build() {
        let res = client
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

        let a: Vec<Track> = serde_json::from_value(test).unwrap();

        println!("{}", a[0].album.text);
    } else {
        panic!("bruh")
    }

    // let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    // App::default().run(&mut terminal)?;

    // execute!(stdout(), LeaveAlternateScreen)?;
    // disable_raw_mode()?;
    Ok(())
}
