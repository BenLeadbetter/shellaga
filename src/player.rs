#[derive(bevy::ecs::component::Component)]
pub struct Player;

pub fn plugin(app: &mut bevy::app::App) {
    app.add_systems(bevy::app::Update, spawn);
    app.add_systems(bevy::app::Update, update);
}

#[derive(bevy::ecs::component::Component, Default)]
pub struct PlayerMovingState {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

#[derive(bevy::ecs::component::Component)]
pub struct PlayerSpeed(f32);

trait ToFloat {
    fn to_float(&self) -> f32;
}

impl ToFloat for bool {
    fn to_float(&self) -> f32 {
        if *self {
            1.0
        } else {
            0.0
        }
    }
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
            Player,
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
            PlayerMovingState::default(),
            PlayerSpeed(0.5),
        ))
        .set_parent(*root);
}

fn update(
    mut reader: bevy::ecs::event::EventReader<crate::terminal::TerminalEvent>,
    mut query: bevy::ecs::system::Query<
        (
            &mut bevy::transform::components::Transform,
            &mut PlayerMovingState,
            &PlayerSpeed,
        ),
        bevy::ecs::query::With<Player>,
    >,
) {
    let Ok((mut transform, mut moving_state, speed)) = query.get_single_mut() else {
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
                    moving_state.up = true;
                }
                Release => {
                    moving_state.up = false;
                }
                _ => {}
            },
            Char('a') => match &key.kind {
                Press => {
                    moving_state.left = true;
                }
                Release => {
                    moving_state.left = false;
                }
                _ => {}
            },
            Char('s') => match &key.kind {
                Press => {
                    moving_state.down = true;
                }
                Release => {
                    moving_state.down = false;
                }
                _ => {}
            },
            Char('d') => match &key.kind {
                Press => {
                    moving_state.right = true;
                }
                Release => {
                    moving_state.right = false;
                }
                _ => {}
            },
            _ => {}
        }
    }

    if let Some(v) = bevy::math::f32::Vec3::new(
        moving_state.right.to_float() - moving_state.left.to_float(),
        moving_state.down.to_float() - moving_state.up.to_float(),
        0.0,
    )
    .try_normalize()
    {
        transform.translation += speed.0 * v
    }
}
