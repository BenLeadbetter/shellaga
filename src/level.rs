#[derive(bevy::ecs::component::Component)]
pub struct Level {
    pub length: f32,
}

#[derive(bevy::ecs::event::Event, std::cmp::PartialEq, std::cmp::Eq)]
pub enum LevelEvent {
    LevelStart,
    LevelEnd,
}

pub fn plugin(app: &mut bevy::app::App) {
    use bevy::ecs::schedule::{common_conditions::any_with_component, IntoSystemConfigs};

    app.add_event::<LevelEvent>();
    app.add_systems(
        bevy::app::Update,
        spawn.run_if(level_not_spawned).run_if(on_level_start_event),
    );
    app.add_systems(
        bevy::app::Update,
        teardown_level
            .run_if(any_with_component::<Level>)
            .run_if(on_level_end_event),
    );
}

fn level_not_spawned(query: bevy::ecs::system::Query<(), bevy::ecs::query::With<Level>>) -> bool {
    query.is_empty()
}

fn on_level_start_event(mut events: bevy::ecs::event::EventReader<LevelEvent>) -> bool {
    events.read().any(|e| *e == LevelEvent::LevelStart)
}

fn on_level_end_event(mut events: bevy::ecs::event::EventReader<LevelEvent>) -> bool {
    events.read().any(|e| *e == LevelEvent::LevelEnd)
}

fn spawn(mut commands: bevy::ecs::system::Commands) {
    log::info!("spawning level");
    commands.spawn((
        bevy::transform::TransformBundle::default(),
        Level { length: 1000.0 },
    ));
}

fn despawn_with_children(
    commands: &mut bevy::ecs::system::Commands,
    entity: bevy::ecs::entity::Entity,
    query: &bevy::ecs::system::Query<(bevy::ecs::entity::Entity, &bevy::hierarchy::Parent)>,
) {
    let mut children_to_despawn = Vec::new();
    for (child_entity, parent) in query.iter() {
        if parent.get() == entity {
            children_to_despawn.push(child_entity);
        }
    }

    for child in &children_to_despawn {
        despawn_with_children(commands, *child, query);
    }

    for child in &children_to_despawn {
        commands.entity(*child).despawn();
    }

    commands.entity(entity).despawn();
}

fn teardown_level(
    mut commands: bevy::ecs::system::Commands,
    entities_with_parent_query: bevy::ecs::system::Query<(
        bevy::ecs::entity::Entity,
        &bevy::hierarchy::Parent,
    )>,
    level_query: bevy::ecs::system::Query<bevy::ecs::entity::Entity, bevy::ecs::query::With<Level>>,
    mut app_event_sender: bevy::ecs::event::EventWriter<bevy::app::AppExit>,
) {
    log::info!("Teardown level");

    let Ok(entity) = level_query.get_single() else {
        log::error!("Couldn't get unique level instance");
        return;
    };

    despawn_with_children(&mut commands, entity, &entities_with_parent_query);
    app_event_sender.send(bevy::app::AppExit);
}
