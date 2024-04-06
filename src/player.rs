#[derive(bevy::ecs::component::Component)]
pub struct Player {
    speed: f32,
    moving_state: u8,
}

const MOVING_LEFT: u8 = 0b0000_0001;
const MOVING_RIGHT: u8 = 0b0000_0010;
const MOVING_UP: u8 = 0b0000_0100;
const MOVING_DOWN: u8 = 0b0000_1000;

fn direction(moving_state: u8) -> bevy::math::f32::Vec3 {
    let component = |flag: u8| -> f32 {
        if moving_state & flag > 0 {
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
        common_conditions::{any_with_component, not, on_event},
        IntoSystemConfigs,
    };

    app.add_systems(
        bevy::app::Update,
        spawn
            .run_if(not(any_with_component::<Player>))
            .run_if(on_event::<crate::level::LevelEvent>()),
    );
    app.add_systems(
        bevy::app::Update,
        update.run_if(any_with_component::<Player>),
    );
}

fn spawn(
    mut commands: bevy::ecs::system::Commands,
    mut reader: bevy::ecs::event::EventReader<crate::level::LevelEvent>,
) {
    let Some(root) = reader.read().find_map(|e| {
        match e {
            crate::level::LevelEvent::RootSpawned(root) => {
                Some(root)
            },
            _  => {
                None
            },
        }
    }) else {
        return;
    };

    log::info!("spawning player");
    use bevy::hierarchy::BuildChildren;
    commands
        .spawn((
            Player {
                speed: 0.5,
                moving_state: 0,
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
                    bevy::math::f32::Vec3::new(0.0, 6.0, 0.0),
                ),
            ),
            crate::collider::Collider::new(3.0, 1.0),
        ))
        .set_parent(*root);
}

fn update(
    mut reader: bevy::ecs::event::EventReader<crate::terminal::TerminalEvent>,
    mut query: bevy::ecs::system::Query<(
        &mut bevy::transform::components::Transform,
        &mut Player,
    ), bevy::ecs::query::Without<crate::frame::Frame>>,
    mut frame_query: bevy::ecs::system::Query<
        (
            &crate::collider::Collider,
            &bevy::transform::components::Transform,
        ),
        bevy::ecs::query::With<crate::frame::Frame>,
    >,
) {
    let Ok((mut transform, mut player)) = query.get_single_mut() else {
        log::error!("More that one player spawn at one time");
        return;
    };

    for event in reader.read() {
        let crate::terminal::TerminalEvent::Key(key) = event else {
                continue;
            };
        use crossterm::event::KeyCode::*;
        use crossterm::event::KeyEventKind::*;
        match &key.code {
            Char('w') => match &key.kind {
                Press => {
                    player.moving_state |= MOVING_UP;
                }
                Release => {
                    player.moving_state &= !MOVING_UP;
                }
                _ => {}
            },
            Char('a') => match &key.kind {
                Press => {
                    player.moving_state |= MOVING_LEFT;
                }
                Release => {
                    player.moving_state &= !MOVING_LEFT;
                }
                _ => {}
            },
            Char('s') => match &key.kind {
                Press => {
                    player.moving_state |= MOVING_DOWN;
                }
                Release => {
                    player.moving_state &= !MOVING_DOWN;
                }
                _ => {}
            },
            Char('d') => match &key.kind {
                Press => {
                    player.moving_state |= MOVING_RIGHT;
                }
                Release => {
                    player.moving_state &= !MOVING_RIGHT;
                }
                _ => {}
            },
            _ => {}
        }
    }

    transform.translation += player.speed * direction(player.moving_state);

    let Ok((frame_collider, frame_global_transform)) = frame_query.get_single_mut() else {
        log::error!("More that one player spawn at one time");
        return;
    };

    use bevy::math::Vec3Swizzles;
    let frame_top_left = frame_global_transform
        .transform_point(bevy::math::f32::Vec3::default())
        .xy();
    let frame_bottom_right = frame_global_transform
        .transform_point(bevy::math::f32::Vec3::new(
            frame_collider.x,
            frame_collider.y,
            0.0,
        ))
        .xy();

    transform.translation.x = transform.translation.x.clamp(frame_top_left.x, frame_bottom_right.x);
    transform.translation.y = transform.translation.y.clamp(frame_top_left.y, frame_bottom_right.y);
}
