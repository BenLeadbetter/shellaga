pub const WIDTH: usize = 128;
pub const HEIGHT: usize = 75;

#[derive(bevy::ecs::component::Component)]
struct Frame;

pub fn plugin(app: &mut bevy::app::App) {
    use bevy::ecs::schedule::{
        common_conditions::{any_with_component, not, on_event},
        IntoSystemConfigs,
    };

    app.add_systems(
        bevy::app::Update,
        spawn
            .run_if(not(any_with_component::<Frame>))
            .run_if(on_event::<crate::level::LevelEvent>()),
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

    log::info!("spawning Frame");
    use bevy::hierarchy::BuildChildren;
    commands
        .spawn((
            Frame,
            bevy::transform::TransformBundle::default(),
            crate::collider::Collider {
                size: bevy::math::f32::Vec2::new(WIDTH as f32, HEIGHT as f32),
                ..Default::default()
            }
        ))
        .set_parent(*root);
}
