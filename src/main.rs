use bevy::prelude::*;
mod terminal;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    bevy::app::App::new()
        .add_plugins(bevy::MinimalPlugins.set(runloop()))
        .add_systems(Update, handle_events)
        .insert_resource(terminal::Terminal::new()?)
        .run();

    Ok(())
}

fn runloop() -> bevy::app::ScheduleRunnerPlugin {
    // bevy::app::ScheduleRunnerPlugin::run_loop(std::time::Duration::from_secs_f32(1.0 / 60.0))
    bevy::app::ScheduleRunnerPlugin::run_once()
}

fn handle_events() {
    use crossterm::event::{self, Event, KeyCode};
    match event::poll(std::time::Duration::from_millis(50)) {
        Ok(true) => match event::read() {
            Ok(Event::Key(key)) => {
                if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    println!("quit please");
                }
            }
            _ => {}
        },
        _ => {}
    };
}
