#[derive(bevy::ecs::component::Component)]
pub struct Enemy;

pub type EnemyBundle = (Enemy, crate::sprite::Sprite, crate::collider::Collider);

const ENEMY_SPEED: f32 = 30.0;

impl Enemy {
    pub fn bundle() -> EnemyBundle {
        (
            Enemy,
            crate::sprite::Sprite {
                buffer: crate::buffer::Buffer(
                    ndarray::array![
                        [Some('/'), Some('/')],
                        [Some('/'), Some('/')],
                        [None, Some('o')],
                        [Some('\\'), Some('\\')],
                        [Some('\\'), Some('\\')],
                    ]
                    .map(|c| crate::buffer::Cell {
                        character: *c,
                        ..Default::default()
                    }),
                ),
            },
            crate::collider::Collider::new(2.0, 5.0),
        )
    }
}

pub fn plugin(app: &mut bevy::app::App) {
    use bevy::ecs::schedule::{common_conditions::any_with_component, IntoSystemConfigs};
    app.add_systems(
        bevy::app::Update,
        update.run_if(any_with_component::<Enemy>),
    );
    app.add_systems(
        bevy::app::Update,
        handle_player_enemy_collisions
            .run_if(any_with_component::<Enemy>)
            .run_if(any_with_component::<crate::player::Player>),
    );
    app.add_systems(
        bevy::app::Update,
        handle_enemy_shot
            .run_if(any_with_component::<Enemy>)
            .run_if(any_with_component::<crate::weapon::Shot>),
    );
}

fn update(
    time: bevy::ecs::system::Res<bevy::time::Time>,
    mut enemy_query: bevy::ecs::system::Query<
        (
            &mut bevy::transform::components::Transform,
            &bevy::transform::components::GlobalTransform,
        ),
        bevy::ecs::query::With<Enemy>,
    >,
    frame_query: bevy::ecs::system::Query<
        (
            &crate::collider::Collider,
            &bevy::transform::components::GlobalTransform,
        ),
        bevy::ecs::query::With<crate::frame::Frame>,
    >,
) {
    let Ok((frame_collider, frame_global_transform)) = frame_query.get_single() else {
        log::error!("Couldn't get a frame instance");
        return;
    };

    for (mut enemy_transform, enemy_global_transform) in &mut enemy_query {
        let global_translation = enemy_global_transform.translation();
        let enemy_in_frame = global_translation.x
            < frame_global_transform.translation().x + frame_collider.x
            && global_translation.x > frame_global_transform.translation().x;
        if enemy_in_frame {
            enemy_transform.translation.x -= ENEMY_SPEED * time.delta_seconds();
        }
    }
}

fn handle_player_enemy_collisions(
    enemy_query: bevy::ecs::system::Query<
        (
            &crate::collider::Collider,
            &bevy::transform::components::GlobalTransform,
        ),
        bevy::ecs::query::With<Enemy>,
    >,
    player_query: bevy::ecs::system::Query<
        (
            &crate::collider::Collider,
            &bevy::transform::components::GlobalTransform,
        ),
        bevy::ecs::query::With<crate::player::Player>,
    >,
    mut app_event_sender: bevy::ecs::event::EventWriter<bevy::app::AppExit>,
) {
    let Ok(player) = player_query.get_single() else {
        log::error!("Couldn't unique get player instance.");
        return;
    };

    let mut collision = false;
    for enemy in &enemy_query {
        if crate::collider::collide(player, enemy) {
            collision = true;
        }
    }

    if collision {
        app_event_sender.send(bevy::app::AppExit);
    }
}

fn handle_enemy_shot(
    enemy_query: bevy::ecs::system::Query<
        (
            bevy::ecs::entity::Entity,
            &crate::collider::Collider,
            &bevy::transform::components::GlobalTransform,
        ),
        bevy::ecs::query::With<Enemy>,
    >,
    shot_query: bevy::ecs::system::Query<
        (
            bevy::ecs::entity::Entity,
            &crate::collider::Collider,
            &bevy::transform::components::GlobalTransform,
        ),
        bevy::ecs::query::With<crate::weapon::Shot>,
    >,
    mut commands: bevy::ecs::system::Commands,
) {
    for (enemy, enemy_collider, enemy_transform) in &enemy_query {
        for (shot, shot_collider, shot_transform) in &shot_query {
            if crate::collider::collide(
                (shot_collider, shot_transform),
                (enemy_collider, enemy_transform),
            ) {
                commands.entity(enemy).despawn();
                commands.entity(shot).despawn();
            }
        }
    }
}
