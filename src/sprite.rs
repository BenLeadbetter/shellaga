#[derive(bevy::ecs::component::Component, derive_builder::Builder, derive_getters::Getters, Clone)]
pub struct Sprite {
    #[builder(default = "Default::default()")]
    buffer: String,
    #[builder(default = "Default::default()")]
    size: bevy::math::i32::IVec2,
    // #[builder(default = "Default::default()")]
    // origin: bevy::math::i32::IVec2,
}

impl Sprite {
    pub fn builder() -> SpriteBuilder {
        SpriteBuilder::default()
    }
}

impl ratatui::widgets::Widget for Sprite {
    fn render(self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer)
    where
        Self: Sized,
    {
        use itertools::Itertools;
        for (row, col) in (0..self.size().x).cartesian_product(0..self.size().y) {
            if let Some(c) = self.buffer().chars().nth((row * self.size().x + col) as usize) {
                buf.get_mut(row as u16 + area.x, col as u16 + area.y).set_char(c);
            }
        }
    }
}
