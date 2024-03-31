use crossterm::ExecutableCommand;

#[derive(bevy::ecs::event::Event, Debug)]
pub enum TerminalEvent {
    Key(crossterm::event::KeyEvent),
    Resize(u16, u16),
}

pub fn plugin(app: &mut bevy::app::App) {
    app.add_event::<TerminalEvent>();
    app.insert_resource(Terminal::new().expect("error initialising terminal"));
    app.add_systems(bevy::app::PreUpdate, handle_events);
}

#[derive(bevy::ecs::system::Resource)]
pub struct Terminal(ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>);

impl Drop for Terminal {
    fn drop(&mut self) {
        crossterm::execute!(
            std::io::stdout(),
            crossterm::event::PopKeyboardEnhancementFlags
        )
        .expect("pop keyboard extentions");
        std::io::stdout()
            .execute(crossterm::terminal::LeaveAlternateScreen)
            .expect("leave terminal alternate screen");
        crossterm::terminal::disable_raw_mode().expect("leave terminal raw mode");
    }
}

impl Terminal {
    pub fn new() -> Result<Self, std::boxed::Box<dyn std::error::Error>> {
        crossterm::terminal::enable_raw_mode()?;
        std::io::stdout().execute(crossterm::terminal::EnterAlternateScreen)?;
        crossterm::execute!(
            std::io::stdout(),
            crossterm::event::PushKeyboardEnhancementFlags(
                crossterm::event::KeyboardEnhancementFlags::REPORT_EVENT_TYPES
                    | crossterm::event::KeyboardEnhancementFlags::REPORT_ALTERNATE_KEYS
                    | crossterm::event::KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES
                    | crossterm::event::KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
            )
        )?;
        let backend = ratatui::backend::CrosstermBackend::new(std::io::stdout());
        Ok(Self(ratatui::Terminal::new(backend)?))
    }

    pub fn draw<F>(&mut self, f: F) -> std::io::Result<ratatui::CompletedFrame<'_>>
    where
        F: FnOnce(&mut ratatui::Frame<'_>),
    {
        self.0.draw(f)
    }
}

fn handle_events(mut event_sender: bevy::ecs::event::EventWriter<TerminalEvent>) {
    while let Ok(true) = crossterm::event::poll(std::time::Duration::from_millis(0)) {
        match crossterm::event::read() {
            Ok(e) => {
                log::trace!("crossterm event {:?}", e);
                match e {
                    // forward crossterm events into bevy
                    crossterm::event::Event::Key(key_event) => {
                        event_sender.send(TerminalEvent::Key(key_event));
                    }
                    crossterm::event::Event::Resize(w, h) => {
                        event_sender.send(TerminalEvent::Resize(w, h));
                    }
                    // ignore these for now
                    crossterm::event::Event::FocusGained => {}
                    crossterm::event::Event::FocusLost => {}
                    crossterm::event::Event::Mouse(_) => {}
                    crossterm::event::Event::Paste(_) => {}
                }
            }
            _ => {}
        };
    }
}
