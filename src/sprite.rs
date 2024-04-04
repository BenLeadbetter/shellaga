pub fn plugin(app: &mut bevy::app::App) {
    app.add_systems(bevy::app::PostUpdate, render);
}

#[derive(bevy::ecs::component::Component, Clone, Default)]
pub struct Sprite {
    pub buffer: crate::buffer::Buffer,
    // #[builder(default = "Default::default()")]
    // origin: bevy::math::i32::IVec2,
}

fn render(
    mut buffer: bevy::ecs::system::ResMut<crate::buffer::Buffer>,
    query: bevy::ecs::system::Query<(&Sprite, &bevy::transform::components::GlobalTransform)>,
) {
    for (sprite, global_transform) in &query {
        render_to_buffer(sprite, &global_transform.compute_transform(), &mut *buffer);
    }
}

fn render_to_buffer(
    sprite: &Sprite,
    transform: &bevy::transform::components::Transform,
    buffer: &mut crate::buffer::Buffer,
) {
    use itertools::Itertools;
    for (row, col) in
        (0..sprite.buffer.0.shape()[0]).cartesian_product(0..sprite.buffer.0.shape()[1])
    {
        let translation = &transform.translation;
        let x = col + translation.x.round() as usize;
        let y = row + translation.y.round() as usize;
        let depth = translation.z;
        if let Some(cell) = buffer.0.get_mut([y, x]) {
            let sprite_cell = sprite.buffer.0[[row, col]];
            let sprite_cell_depth = depth + sprite_cell.depth;
            if sprite_cell_depth > cell.depth {
                continue;
            }
            *cell = crate::buffer::Cell {
                character: sprite_cell.character,
                fg: sprite_cell.fg,
                bg: sprite_cell.bg,
                depth: sprite_cell_depth,
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::buffer::Cell;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn render_to_buffer_test() {
        let mut buffer = crate::buffer::Buffer(ndarray::Array2::from_elem(
            (2, 3),
            crate::buffer::Cell::default(),
        ));
        let sprite = Sprite {
            buffer: crate::buffer::Buffer(ndarray::array![[crate::buffer::Cell {
                character: Some('x'),
                ..Default::default()
            }]]),
        };

        render_to_buffer(&sprite, &Default::default(), &mut buffer);

        assert_eq!(
            buffer,
            crate::buffer::Buffer(
                ndarray::array![[Some('x'), None, None], [None, None, None],].mapv(|c| Cell {
                    character: c,
                    ..Default::default()
                })
            )
        );
    }

    #[test]
    fn render_to_buffer_with_x_translation_test() {
        let mut buffer = crate::buffer::Buffer(ndarray::Array2::from_elem(
            (2, 3),
            crate::buffer::Cell::default(),
        ));
        let sprite = Sprite {
            buffer: crate::buffer::Buffer(ndarray::array![[crate::buffer::Cell {
                character: Some('x'),
                ..Default::default()
            }]]),
        };

        render_to_buffer(
            &sprite,
            &bevy::transform::components::Transform::from_translation(bevy::math::f32::Vec3::new(
                1.0, 0.0, 0.0,
            )),
            &mut buffer,
        );

        assert_eq!(
            buffer,
            crate::buffer::Buffer(
                ndarray::array![[None, Some('x'), None], [None, None, None],].mapv(|c| Cell {
                    character: c,
                    ..Default::default()
                })
            )
        );
    }

    #[test]
    fn render_to_buffer_with_y_translation_test() {
        let mut buffer = crate::buffer::Buffer(ndarray::Array2::from_elem(
            (2, 3),
            crate::buffer::Cell::default(),
        ));
        let sprite = Sprite {
            buffer: crate::buffer::Buffer(ndarray::array![[crate::buffer::Cell {
                character: Some('x'),
                ..Default::default()
            }]]),
        };

        render_to_buffer(
            &sprite,
            &bevy::transform::components::Transform::from_translation(bevy::math::f32::Vec3::new(
                0.0, 1.0, 0.0,
            )),
            &mut buffer,
        );

        assert_eq!(
            buffer,
            crate::buffer::Buffer(
                ndarray::array![[None, None, None], [Some('x'), None, None],].mapv(|c| Cell {
                    character: c,
                    ..Default::default()
                })
            )
        );
    }

    #[test]
    fn render_to_buffer_with_non_trivial_sprite_test() {
        let mut buffer = crate::buffer::Buffer(ndarray::Array2::from_elem(
            (2, 3),
            crate::buffer::Cell::default(),
        ));

        let sprite = Sprite {
            buffer: crate::buffer::Buffer(ndarray::array![
                [
                    crate::buffer::Cell {
                        character: Some('x'),
                        ..Default::default()
                    },
                    crate::buffer::Cell {
                        character: Some('x'),
                        ..Default::default()
                    }
                ],
                [
                    Default::default(),
                    crate::buffer::Cell {
                        character: Some('x'),
                        ..Default::default()
                    }
                ],
            ]),
        };

        render_to_buffer(&sprite, &Default::default(), &mut buffer);

        assert_eq!(
            buffer,
            crate::buffer::Buffer(
                ndarray::array![[Some('x'), Some('x'), None], [None, Some('x'), None],].mapv(|c| {
                    Cell {
                        character: c,
                        ..Default::default()
                    }
                })
            )
        );
    }

    #[test]
    fn render_to_buffer_with_existing_texel_in_front() {
        let mut buffer = crate::buffer::Buffer(ndarray::array![[crate::buffer::Cell {
            character: Some('a'),
            depth: -1.0,
            ..Default::default()
        }]]);

        let sprite = Sprite {
            buffer: crate::buffer::Buffer(ndarray::array![[crate::buffer::Cell {
                character: Some('x'),
                ..Default::default()
            },],]),
        };

        let unchanged = buffer.clone();

        render_to_buffer(&sprite, &Default::default(), &mut buffer);

        assert_eq!(buffer, unchanged);
    }

    #[test]
    fn render_to_buffer_with_existing_texel_behind() {
        let mut buffer = crate::buffer::Buffer(ndarray::array![[crate::buffer::Cell {
            character: Some('a'),
            depth: -1.0,
            ..Default::default()
        }]]);

        let sprite = Sprite {
            buffer: crate::buffer::Buffer(ndarray::array![[crate::buffer::Cell {
                character: Some('x'),
                ..Default::default()
            },],]),
        };

        render_to_buffer(
            &sprite,
            &bevy::transform::components::Transform::from_translation(bevy::math::f32::Vec3::new(
                0.0, 0.0, -2.0,
            )),
            &mut buffer,
        );

        assert_eq!(
            buffer,
            crate::buffer::Buffer(ndarray::array![[crate::buffer::Cell {
                character: Some('x'),
                depth: -2.0,
                ..Default::default()
            }]])
        );
    }
}
