#[derive(bevy::ecs::component::Component)]
pub struct Level {
    pub length: f32,
}

const BACKGROUND_DEPTH: f32 = 10.0;
const BACKGROUND_DENSITY: f32 = 0.97;

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
    let length = 1000.0;
    let level = commands
        .spawn((
            bevy::transform::TransformBundle::default(),
            Level { length },
        ))
        .id();
    spawn_background(&mut commands, level, length);
}

fn spawn_background(
    commands: &mut bevy::ecs::system::Commands,
    parent: bevy::ecs::entity::Entity,
    length: f32,
) {
    use bevy::hierarchy::BuildChildren;
    use itertools::Itertools;
    for (row, col) in (0..length as usize).cartesian_product(0..crate::frame::HEIGHT) {

        if rand::random::<f32>() < BACKGROUND_DENSITY {
            continue;
        }

        commands
            .spawn((
                bevy::transform::TransformBundle::from_transform(
                    bevy::transform::components::Transform::from_translation(
                        bevy::math::f32::Vec3::new(row as f32, col as f32, BACKGROUND_DEPTH),
                    ),
                ),
                crate::sprite::Sprite {
                    buffer: crate::buffer::Buffer(ndarray::array![[crate::buffer::Cell {
                        character: Some('*'),
                        ..Default::default()
                    }]]),
                },
            ))
            .set_parent(parent);
    }
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
