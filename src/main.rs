use bevy::prelude::*;

mod event;
mod player;
mod sprite;
mod terminal;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    bevy::app::App::new()
        .add_event::<event::TerminalEvent>()
        .add_plugins(bevy::MinimalPlugins.set(runloop()))
        .add_systems(Startup, spawn_player)
        .add_systems(Update, (handle_terminal_events, handle_exit))
        .add_systems(PostUpdate, render)
        .insert_resource(terminal::Terminal::new()?)
        .run();

    Ok(())
}

fn runloop() -> bevy::app::ScheduleRunnerPlugin {
    bevy::app::ScheduleRunnerPlugin::run_loop(std::time::Duration::from_secs_f32(1.0 / 60.0))
}

fn handle_terminal_events(mut event_sender: EventWriter<event::TerminalEvent>) {
    while let Ok(true) = crossterm::event::poll(std::time::Duration::from_millis(0)) {
        match crossterm::event::read() {
            Ok(e) => {
                match e {
                    // forward crossterm events into bevy
                    crossterm::event::Event::Key(key_event) => {
                        event_sender.send(event::TerminalEvent::Key(key_event));
                    }
                    crossterm::event::Event::Resize(w, h) => {
                        event_sender.send(event::TerminalEvent::Resize(w, h));
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

fn should_exit(event: &event::TerminalEvent) -> bool {
    match event {
        event::TerminalEvent::Key(crossterm::event::KeyEvent {
            kind: crossterm::event::KeyEventKind::Press,
            code: crossterm::event::KeyCode::Esc,
            modifiers: _,
            state: _,
        }) => true,
        _ => false,
    }
}

fn handle_exit(
    mut reader: EventReader<event::TerminalEvent>,
    mut sender: EventWriter<bevy::app::AppExit>,
) {
    if reader.read().any(should_exit) {
        sender.send(bevy::app::AppExit);
    }
}

fn render(mut terminal: ResMut<terminal::Terminal>, query: Query<&sprite::Sprite>) {
    terminal
        .draw(|frame| {
            use ratatui::widgets::{Block, Borders};

            let border = Block::default().borders(Borders::ALL);
            frame.render_widget(border.clone(), frame.size());

            for sprite in &query {
                frame.render_widget(sprite.clone(), border.inner(frame.size()));
            }
        })
        .expect("frame rendered sucessfully");
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        player::Player,
        sprite::Sprite::builder()
            .buffer("xxxx".to_string())
            .size(IVec2::new(2, 2))
            .build()
            .unwrap(),
    ));
}
