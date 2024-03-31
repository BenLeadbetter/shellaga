use bevy::app::PluginGroup;

mod app;
mod level;
mod logging;
mod player;
mod sprite;
mod terminal;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    logging::init()?;

    bevy::app::App::new()
        .add_plugins(bevy::MinimalPlugins.set(runloop()))
        .add_plugins(bevy::transform::TransformPlugin)
        .add_plugins(terminal::plugin)
        .add_plugins(app::plugin)
        .add_plugins(sprite::plugin)
        .add_plugins(level::plugin)
        .add_plugins(player::plugin)
        .add_systems(bevy::app::Startup, startup)
        .run();

    Ok(())
}

fn runloop() -> bevy::app::ScheduleRunnerPlugin {
    bevy::app::ScheduleRunnerPlugin::run_loop(std::time::Duration::from_secs_f32(1.0 / 60.0))
}

fn startup(mut writer: bevy::ecs::event::EventWriter<level::LevelEvent>) {
    log::info!("startup");
    writer.send(level::LevelEvent::LevelStart);
}
