#[derive(bevy::ecs::component::Component)]
pub struct Level;

#[derive(bevy::ecs::event::Event, std::cmp::PartialEq, std::cmp::Eq)]
pub enum LevelEvent {
    LevelStart,
}

pub fn plugin(app: &mut bevy::app::App) {
    use bevy::ecs::schedule::IntoSystemConfigs;

    app.add_event::<LevelEvent>();
    app.add_systems(
        bevy::app::Update,
        spawn.run_if(level_not_spawned).run_if(on_level_start_event),
    );
}

fn level_not_spawned(query: bevy::ecs::system::Query<(), bevy::ecs::query::With<Level>>) -> bool {
    query.is_empty()
}

fn on_level_start_event(mut events: bevy::ecs::event::EventReader<LevelEvent>) -> bool {
    events.read().any(|e| *e == LevelEvent::LevelStart)
}

fn spawn(
    mut commands: bevy::ecs::system::Commands,
) {
    log::info!("spawning level");
    commands.spawn((bevy::transform::TransformBundle::default(), Level));
}
