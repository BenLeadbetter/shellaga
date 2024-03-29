use crossterm::ExecutableCommand;

pub struct Terminal(pub ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>);

impl std::ops::Drop for Terminal {
    fn drop(&mut self) {
        crossterm::terminal::disable_raw_mode().expect("leave terminal raw mode");
        std::io::stdout().execute(crossterm::terminal::LeaveAlternateScreen).expect("leave terminal alternate screen");
    }
}

impl bevy::ecs::system::Resource for Terminal {}

impl Terminal {
    pub fn new() -> Result<Self, std::boxed::Box<dyn std::error::Error>> {
        crossterm::terminal::enable_raw_mode()?;
        std::io::stdout().execute(crossterm::terminal::EnterAlternateScreen)?;

        let backend = ratatui::backend::CrosstermBackend::new(std::io::stdout());
        Ok(Self(ratatui::Terminal::new(backend)?))
    }
}
