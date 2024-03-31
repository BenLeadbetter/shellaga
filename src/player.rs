pub struct Player;

#[derive(bevy::ecs::system::Resource, Default)]
pub struct PlayerEntity(Option<bevy::ecs::entity::Entity>);

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

impl Player {
    pub fn spawn(
        mut commands: bevy::ecs::system::Commands,
        mut entity: bevy::ecs::system::ResMut<PlayerEntity>,
    ) {
        let id = commands
            .spawn((
                crate::sprite::Sprite::builder()
                    .buffer("xxxx".to_string())
                    .size(bevy::math::i32::IVec2::new(2, 2))
                    .build()
                    .unwrap(),
                bevy::transform::TransformBundle::from_transform(
                    bevy::transform::components::Transform::from_translation(
                        bevy::math::f32::Vec3::new(0.0, 6.0, 0.0),
                    ),
                ),
                PlayerMovingState::default(),
                PlayerSpeed(0.5),
            ))
            .id();
        *entity = PlayerEntity(Some(id));
    }

    pub fn update(
        mut reader: bevy::ecs::event::EventReader<crate::event::TerminalEvent>,
        mut query: bevy::ecs::system::Query<(
            &mut bevy::transform::components::Transform,
            &mut PlayerMovingState,
            &PlayerSpeed,
        )>,
        entity: bevy::ecs::system::Res<PlayerEntity>,
    ) {
        let Some(id) = entity.0 else {
            return;
        };
        let Ok((mut transform, mut moving_state, speed)) = query.get_mut(id) else {
            return;
        };

        for event in reader.read() {
            let crate::event::TerminalEvent::Key(key) = event else {
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
}
