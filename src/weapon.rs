use bevy::hierarchy::BuildChildren;

pub fn plugin(app: &mut bevy::app::App) {
    use bevy::ecs::schedule::{common_conditions::any_with_component, IntoSystemConfigs};

    app.add_systems(
        bevy::app::Update,
        reload_weapons_system
            .run_if(any_with_component::<Weapon>)
            .run_if(any_with_component::<crate::frame::Frame>),
    );
    app.add_systems(
        bevy::app::Update,
        update_shots.run_if(any_with_component::<Shot>),
    );
    app.add_systems(
        bevy::app::Update,
        despawn_shots
            .run_if(any_with_component::<Shot>)
            .run_if(any_with_component::<crate::frame::Frame>),
    );
}

#[derive(bevy::ecs::component::Component)]
pub struct Weapon {
    reload_timer: bevy::time::Timer,
    trigger: bool,
}

impl Weapon {
    pub fn ready(&self) -> bool {
        self.reload_timer.finished()
    }

    pub fn trigger(&mut self, pull: bool) {
        self.trigger = pull;
    }

    pub fn new(reload_duration: f32) -> Self {
        Self {
            reload_timer: bevy::time::Timer::from_seconds(
                reload_duration,
                bevy::time::TimerMode::Once,
            ),
            trigger: false,
        }
    }
}

#[derive(bevy::ecs::component::Component)]
pub struct LazerShot;

#[derive(bevy::ecs::component::Component)]
pub struct Shot {
    speed: f32,
}

fn reload_weapons_system(
    mut commands: bevy::ecs::system::Commands,
    time: bevy::ecs::system::Res<bevy::time::Time>,
    mut query: bevy::ecs::system::Query<(
        &mut Weapon,
        &bevy::transform::components::GlobalTransform,
    )>,
    mut frame_query: bevy::ecs::system::Query<
        (
            bevy::ecs::entity::Entity,
            &bevy::transform::components::GlobalTransform,
        ),
        bevy::ecs::query::With<crate::frame::Frame>,
    >,
) {
    let Ok((frame_entity, frame_transform)) = frame_query.get_single_mut() else {
        log::error!("Couldn't get reference to unique frame");
        return;
    };

    let frame_inverse = frame_transform.compute_matrix().inverse();

    for (mut weapon, weapon_transform) in query.iter_mut() {
        weapon.reload_timer.tick(time.delta());

        if weapon.reload_timer.just_finished() {
            log::trace!("Weapon ready");
        }

        if !weapon.ready() || !weapon.trigger {
            continue;
        }

        log::trace!("Firing weapon");
        shoot_lazer(
            &mut commands,
            frame_entity,
            // shots are parented to the frame but begin at the position
            // of the weaon so we must map the transforms from weapon space
            // to frame space
            bevy::transform::components::Transform::from_matrix(
                weapon_transform.compute_matrix() * frame_inverse,
            ),
        );
        weapon.reload_timer.reset();
    }
}

fn update_shots(
    time: bevy::ecs::system::Res<bevy::time::Time>,
    mut query: bevy::ecs::system::Query<(&Shot, &mut bevy::transform::components::Transform)>,
) {
    for (shot, mut transform) in &mut query {
        transform.translation +=
            bevy::math::f32::Vec3::new(1.0, 0.0, 0.0) * shot.speed * time.delta_seconds();
    }
}

fn despawn_shots(
    mut commands: bevy::ecs::system::Commands,
    mut shot_query: bevy::ecs::system::Query<
        (
            bevy::ecs::entity::Entity,
            &crate::collider::Collider,
            &bevy::transform::components::GlobalTransform,
        ),
        bevy::ecs::query::With<Shot>,
    >,
    mut frame_query: bevy::ecs::system::Query<
        (
            &bevy::transform::components::GlobalTransform,
            &crate::collider::Collider,
        ),
        bevy::ecs::query::With<crate::frame::Frame>,
    >,
) {
    let Ok((frame_transform, frame_collider)) = frame_query.get_single_mut() else {
        log::error!("Couldn't get reference to unique frame");
        return;
    };

    let frame_top_left = frame_transform.transform_point(bevy::math::f32::Vec3::default());
    let frame_bottom_right = frame_transform.transform_point(bevy::math::f32::Vec3::new(
        frame_collider.x,
        frame_collider.y,
        0.0,
    ));

    for (entity, shot_collider, shot_transform) in &mut shot_query {
        let shot_top_left = shot_transform.transform_point(bevy::math::f32::Vec3::default());
        let shot_bottom_right = shot_transform.transform_point(bevy::math::f32::Vec3::new(
            shot_collider.x,
            shot_collider.y,
            0.0,
        ));

        let out_right = shot_top_left.x > frame_bottom_right.x;
        let out_left = shot_bottom_right.x < frame_top_left.x;
        let out_up = shot_bottom_right.y < frame_top_left.y;
        let out_down = shot_top_left.y > frame_bottom_right.y;
        if out_right || out_left || out_up || out_down {
            log::trace!("Despawning shot");
            commands.entity(entity).despawn();
        }
    }
}

fn shoot_lazer(
    commands: &mut bevy::ecs::system::Commands,
    frame: bevy::ecs::entity::Entity,
    transform: bevy::transform::components::Transform,
) {
    commands
        .spawn((
            LazerShot,
            Shot { speed: 40.0 },
            crate::sprite::Sprite {
                buffer: crate::buffer::Buffer(ndarray::array![[crate::buffer::Cell {
                    character: Some('-'),
                    depth: 1.0, //  behind player
                    ..Default::default()
                },]]),
            },
            crate::collider::Collider::new(1.0, 1.0),
            bevy::transform::TransformBundle::from_transform(transform),
        ))
        .set_parent(frame);
}
