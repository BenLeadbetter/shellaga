#[derive(bevy::ecs::component::Component)]
pub struct Player;

#[derive(bevy::ecs::component::Component)]
pub struct PlayerState {
    speed: f32,
    state: u8,
}

const MOVING_LEFT: u8 = 0b0000_0001;
const MOVING_RIGHT: u8 = 0b0000_0010;
const MOVING_UP: u8 = 0b0000_0100;
const MOVING_DOWN: u8 = 0b0000_1000;

fn direction(state: u8) -> bevy::math::f32::Vec3 {
    let component = |flag: u8| -> f32 {
        if state & flag > 0 {
            1.0
        } else {
            0.0
        }
    };
    match bevy::math::f32::Vec3::new(
        component(MOVING_RIGHT) - component(MOVING_LEFT),
        component(MOVING_DOWN) - component(MOVING_UP),
        0.0,
    )
    .try_normalize()
    {
        Some(v) => v,
        None => bevy::math::f32::Vec3::default(),
    }
}

pub fn plugin(app: &mut bevy::app::App) {
    use bevy::ecs::schedule::{
        common_conditions::{any_with_component, not},
        IntoSystemConfigs,
    };

    app.add_systems(
        bevy::app::Update,
        spawn
            .run_if(not(any_with_component::<PlayerState>))
            .run_if(any_with_component::<crate::frame::Frame>),
    );
    app.add_systems(
        bevy::app::Update,
        update.run_if(any_with_component::<PlayerState>),
    );
}

fn spawn(
    mut commands: bevy::ecs::system::Commands,
    frame_query: bevy::ecs::system::Query<
        (bevy::ecs::entity::Entity, &crate::collider::Collider),
        bevy::ecs::query::With<crate::frame::Frame>,
    >,
) {
    let Ok((frame, frame_collider)) = frame_query.get_single() else {
        log::error!("Couldn't get a frame instance");
        return;
    };

    log::info!("spawning player");

    use bevy::hierarchy::BuildChildren;

    let weapon = commands
        .spawn((
            Player,
            crate::weapon::Weapon::new(0.3),
            bevy::transform::TransformBundle::from_transform(
                bevy::transform::components::Transform::from_translation(
                    bevy::math::f32::Vec3::new(3.0, 0.0, 0.0),
                ),
            ),
        ))
        .id();

    commands
        .spawn((
            Player,
            PlayerState {
                speed: 20.0,
                state: 0,
            },
            crate::sprite::Sprite {
                buffer: crate::buffer::Buffer(ndarray::array![[
                    crate::buffer::Cell {
                        character: Some(']'),
                        ..Default::default()
                    },
                    crate::buffer::Cell {
                        character: Some('o'),
                        ..Default::default()
                    },
                    crate::buffer::Cell {
                        character: Some('>'),
                        ..Default::default()
                    },
                ]]),
            },
            bevy::transform::TransformBundle::from_transform(
                bevy::transform::components::Transform::from_translation(
                    bevy::math::f32::Vec3::new(0.0, frame_collider.y / 2.0, 0.0),
                ),
            ),
            crate::collider::Collider::new(3.0, 1.0),
        ))
        .push_children(&[weapon])
        .set_parent(frame);
}

fn update(
    time: bevy::ecs::system::Res<bevy::time::Time>,
    mut reader: bevy::ecs::event::EventReader<crate::terminal::TerminalEvent>,
    mut weapon_query: bevy::ecs::system::Query<
        &mut crate::weapon::Weapon,
        (
            bevy::ecs::query::Without<crate::frame::Frame>,
            bevy::ecs::query::With<Player>,
        ),
    >,
    mut query: bevy::ecs::system::Query<
        (
            &mut bevy::transform::components::Transform,
            &mut PlayerState,
            &crate::collider::Collider,
        ),
        (
            bevy::ecs::query::Without<crate::frame::Frame>,
            bevy::ecs::query::With<Player>,
        ),
    >,
    frame_query: bevy::ecs::system::Query<
        &crate::collider::Collider,
        bevy::ecs::query::With<crate::frame::Frame>,
    >,
) {
    let Ok((mut transform, mut player_state, collider)) = query.get_single_mut() else {
        log::error!("Couldn't get a unique player instance.");
        return;
    };

    let Ok(mut weapon) = weapon_query.get_single_mut()  else {
        log::error!("Couldn't get a unique weapon instance.");
        return;
    };

    for event in reader.read() {
        let crate::terminal::TerminalEvent::Key(key) = event else {
            continue;
        };

        use crossterm::event::KeyCode::*;
        use crossterm::event::KeyEventKind::*;

        let mut update_move_state = |state: u8| match &key.kind {
            Press => {
                player_state.state |= state;
            }
            Release => {
                player_state.state &= !state;
            }
            _ => {}
        };

        let mut update_weapon_state = || match &key.kind {
            Press => {
                weapon.trigger(true);
            }
            Release => {
                weapon.trigger(false);
            }
            _ => {}
        };

        match &key.code {
            Char('w') => update_move_state(MOVING_UP),
            Char('a') => update_move_state(MOVING_LEFT),
            Char('s') => update_move_state(MOVING_DOWN),
            Char('d') => update_move_state(MOVING_RIGHT),
            Char(' ') => update_weapon_state(),
            _ => {}
        }
    }

    transform.translation +=
        player_state.speed * direction(player_state.state) * time.delta_seconds();

    let Ok(frame_collider) = frame_query.get_single() else {
        log::error!("Couldn't get a unique frame instance");
        return;
    };

    transform.translation.x = transform
        .translation
        .x
        .clamp(0.0, frame_collider.x - collider.x);
    transform.translation.y = transform
        .translation
        .y
        .clamp(0.0, frame_collider.y - collider.y);
}
