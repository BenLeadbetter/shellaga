pub const WIDTH: usize = 128;
pub const HEIGHT: usize = 32;

#[derive(bevy::ecs::component::Component)]
pub struct Frame;

#[derive(bevy::ecs::system::Resource)]
struct FrameProgressTimer(bevy::time::Timer);

impl std::ops::Deref for FrameProgressTimer {
    type Target = bevy::time::Timer;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for FrameProgressTimer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn plugin(app: &mut bevy::app::App) {
    use bevy::ecs::schedule::{
        common_conditions::{any_with_component, not},
        IntoSystemConfigs,
    };

    app.insert_resource(FrameProgressTimer(bevy::time::Timer::new(
        std::time::Duration::from_secs(5),
        bevy::time::TimerMode::Repeating,
    )));
    app.add_systems(
        bevy::app::Update,
        spawn
            .run_if(not(any_with_component::<Frame>))
            .run_if(any_with_component::<crate::level::Level>),
    );
    app.add_systems(
        bevy::app::Update,
        move_frame.run_if(any_with_component::<Frame>),
    );
    app.add_systems(
        bevy::app::Update,
        end_level
            .run_if(any_with_component::<Frame>)
            .run_if(any_with_component::<crate::level::Level>),
    );
    app.add_systems(
        bevy::app::Update,
        log_level_progress
            .run_if(any_with_component::<Frame>)
            .run_if(any_with_component::<crate::level::Level>)
            .run_if(should_log_level_progress),
    );
    app.add_systems(
        bevy::app::Update,
        update_log_level_timer,
    );
}

// for now moves at constant speed
// todo: move along a path defined by the level
fn move_frame(
    mut query: bevy::ecs::system::Query<
        &mut bevy::transform::components::Transform,
        bevy::ecs::query::With<Frame>,
    >,
) {
    let Ok(mut transform) = query.get_single_mut() else {
        log::error!("More that one frame spawn at one time");
        return;
    };

    transform.translation.x += 0.1;
}

fn update_log_level_timer(
    mut timer: bevy::ecs::system::ResMut<FrameProgressTimer>,
    time: bevy::ecs::system::Res<bevy::time::Time>,
) {
    timer.tick(time.delta());
}

fn should_log_level_progress(timer: bevy::ecs::system::Res<FrameProgressTimer>) -> bool {
    timer.just_finished()
}

fn log_level_progress(
    frame_query: bevy::ecs::system::Query<
        (
            &bevy::transform::components::GlobalTransform,
            &crate::collider::Collider,
        ),
        bevy::ecs::query::With<Frame>,
    >,
    level_query: bevy::ecs::system::Query<&crate::level::Level>,
) {
    let Ok((frame_transform, frame_collider)) = frame_query.get_single() else {
        log::error!("Couldn't get unique frame instance");
        return;
    };

    let Ok(level) = level_query.get_single() else {
        log::error!("Couldn't get unique level instance");
        return;
    };

    let bevy::math::f32::Vec3 { x, .. } =
        frame_transform.transform_point(frame_collider.extend(0.0));

    log::info!(
        "Level progress: {:.1}%",
        (x - frame_collider.x) * 100.0 / (level.length - frame_collider.x)
    );
}

fn end_level(
    frame_query: bevy::ecs::system::Query<
        (
            &bevy::transform::components::GlobalTransform,
            &crate::collider::Collider,
        ),
        bevy::ecs::query::With<Frame>,
    >,
    level_query: bevy::ecs::system::Query<&crate::level::Level>,
    mut write_level_events: bevy::ecs::event::EventWriter<crate::level::LevelEvent>,
) {
    let Ok((frame_transform, frame_collider)) = frame_query.get_single() else {
        log::error!("Couldn't get unique frame instance");
        return;
    };

    let Ok(level) = level_query.get_single() else {
        log::error!("Couldn't get unique level instance");
        return;
    };

    let bevy::math::f32::Vec3 { x, .. } =
        frame_transform.transform_point(frame_collider.extend(0.0));

    if x > level.length {
        write_level_events.send(crate::level::LevelEvent::LevelEnd);
    }
}

fn spawn(
    mut commands: bevy::ecs::system::Commands,
    query: bevy::ecs::system::Query<
        bevy::ecs::entity::Entity,
        bevy::ecs::query::With<crate::level::Level>,
    >,
) {
    let Ok(level) = query.get_single() else {
        log::error!("Couldn't get a level instance");
        return;
    };

    log::info!("spawning Frame");
    use bevy::hierarchy::BuildChildren;
    commands
        .spawn((
            Frame,
            bevy::transform::TransformBundle::default(),
            crate::collider::Collider::new(WIDTH as f32, HEIGHT as f32),
        ))
        .set_parent(level);
}
