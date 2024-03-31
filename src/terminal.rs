use crossterm::ExecutableCommand;

pub struct Terminal(pub ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>);

impl std::ops::Drop for Terminal {
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

impl bevy::ecs::system::Resource for Terminal {}

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
