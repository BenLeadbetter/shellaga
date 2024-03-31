pub fn plugin(app: &mut bevy::app::App) {
    app.add_systems(bevy::app::Update, handle_exit);
}

fn should_exit(event: &crate::terminal::TerminalEvent) -> bool {
    match event {
        crate::terminal::TerminalEvent::Key(crossterm::event::KeyEvent {
            kind: crossterm::event::KeyEventKind::Press,
            code: crossterm::event::KeyCode::Esc,
            modifiers: _,
            state: _,
        }) => true,
        _ => false,
    }
}

fn handle_exit(
    mut reader: bevy::ecs::event::EventReader<crate::terminal::TerminalEvent>,
    mut sender: bevy::ecs::event::EventWriter<bevy::app::AppExit>,
) {
    if reader.read().any(should_exit) {
        sender.send(bevy::app::AppExit);
    }
}

