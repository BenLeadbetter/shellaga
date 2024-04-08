pub const WIDTH: usize = 128;
pub const HEIGHT: usize = 32;

#[derive(bevy::ecs::component::Component)]
pub struct Frame;

pub fn plugin(app: &mut bevy::app::App) {
    use bevy::ecs::schedule::{
        common_conditions::{any_with_component, not},
        IntoSystemConfigs,
    };

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

fn spawn(
    mut commands: bevy::ecs::system::Commands,
    query: bevy::ecs::system::Query<bevy::ecs::entity::Entity, bevy::ecs::query::With<crate::level::Level>>
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
