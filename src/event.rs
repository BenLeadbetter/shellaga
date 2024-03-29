#[derive(bevy::ecs::event::Event)]
pub enum TerminalEvent {
    Key(crossterm::event::KeyEvent),
    Resize(u16, u16),
}
