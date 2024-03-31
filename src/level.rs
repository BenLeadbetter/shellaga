#[derive(bevy::ecs::component::Component)]
struct Level;

#[derive(bevy::ecs::event::Event, std::cmp::PartialEq, std::cmp::Eq)]
pub enum LevelEvent {
    LevelStart,
    RootSpawned(bevy::ecs::entity::Entity),
}

pub fn plugin(app: &mut bevy::app::App) {
    app.add_event::<LevelEvent>();
    app.add_systems(bevy::app::Update, spawn);
}

fn spawn(
    mut events: bevy::ecs::system::ResMut<bevy::ecs::event::Events<LevelEvent>>,
    query: bevy::ecs::system::Query<&Level>,
    mut commands: bevy::ecs::system::Commands,
) {
    let level_started = events
        .get_reader()
        .read(&events)
        .any(|e| *e == LevelEvent::LevelStart);
    // we send one start event but this system handles 
    // it three times. why??
    let level_spawned = !query.is_empty(); 
    if level_started && !level_spawned {
        log::info!("spawning level");
        let id = commands
            .spawn((bevy::transform::TransformBundle::default(), Level))
            .id();
        events.send(LevelEvent::RootSpawned(id));
    }
}
