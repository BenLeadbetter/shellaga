pub fn plugin(app: &mut bevy::app::App) {
    app.add_systems(bevy::app::PostUpdate, render);
}

#[derive(bevy::ecs::component::Component, Clone, Default)]
pub struct Sprite {
    pub buffer: String,
    pub size: bevy::math::i32::IVec2,
    // #[builder(default = "Default::default()")]
    // origin: bevy::math::i32::IVec2,
}

impl ratatui::widgets::Widget for Sprite {
    fn render(self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer)
    where
        Self: Sized,
    {
        use itertools::Itertools;
        for (row, col) in (0..self.size.x).cartesian_product(0..self.size.y) {
            if let Some(c) = self.buffer.chars().nth((row * self.size.x + col) as usize) {
                let x = col as u16 + area.x;
                let y = row as u16 + area.y;
                if buf.area().contains(ratatui::layout::Position{x, y}) {
                    buf.get_mut(x, y).set_char(c);
                }
            }
        }
    }
}

fn render(
    mut terminal: bevy::ecs::system::ResMut<crate::terminal::Terminal>,
    query: bevy::ecs::system::Query<(&Sprite, &bevy::transform::components::GlobalTransform)>,
) {
    terminal
        .draw(|frame| {
            use ratatui::widgets::{Block, Borders};

            let border = Block::default().borders(Borders::ALL);
            frame.render_widget(border.clone(), frame.size());

            use bevy::math::Vec3Swizzles;
            for (sprite, transform) in &query {
                let translation = transform.translation().xy();
                let mut area = border.inner(frame.size());
                area.x += translation.x as u16;
                area.y += translation.y as u16;
                area.x = area.x.clamp(0, area.width - 1);
                area.y = area.y.clamp(0, area.height - 1);
                area.width = area.width.saturating_sub(area.x);
                area.height = area.height.saturating_sub(area.y);
                frame.render_widget(sprite.clone(), area);
            }
        })
        .expect("frame rendered sucessfully");
}
