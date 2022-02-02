use std::io;
use termion::{
    event::Key, input::MouseTerminal, raw::IntoRawMode, raw::RawTerminal, screen::AlternateScreen,
};
use tui::{backend::TermionBackend, Terminal};

use crate::app::App;
use crate::util::event::{Event, Events};

// ----- [ End of use_module phase ] ----- //

type BackendTerm =
    Terminal<TermionBackend<AlternateScreen<MouseTerminal<RawTerminal<io::Stdout>>>>>;

pub struct Tui {
    events: Events,
    // --- [ Tui Backend ] ---
    terminal: BackendTerm,
}

impl Tui {
    pub fn new() -> Self {
        let stdout = io::stdout().into_raw_mode().unwrap();
        let stdout = MouseTerminal::from(stdout);
        let stdout = AlternateScreen::from(stdout);
        let backend = TermionBackend::new(stdout);
        let terminal = Terminal::new(backend).unwrap();
        let events = Events::new();

        Tui { terminal, events }
    }

    pub fn render(&mut self, app: &mut App) -> Result<(), ()> {
        if app.stop_render {
            return Err(());
        }

        match self.terminal.draw(|frame| app.render(frame)) {
            Ok(_x) => {}
            Err(_x) => {
                return Err(());
            }
        };

        Ok(())
    }

    pub fn event(&mut self, app: &mut App) -> Result<(), ()> {
        match self.events.next() {
            Ok(x) => {
                if let Event::Input(key) = x {
                    if key == Key::Esc {
                        return Err(());
                    }
                    app.handle_key(&key);
                }

                return Ok(());
            }
            _ => (),
        };

        Ok(())
    }
}
