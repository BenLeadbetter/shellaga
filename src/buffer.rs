#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    Gray,
    DarkGray,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    White,
}

impl Default for Color {
    fn default() -> Self {
        Self::Black
    }
}

#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub struct Cell {
    pub fg: Color,
    pub bg: Color,
    pub depth: f32,
    pub character: Option<char>,
}

#[derive(bevy::ecs::system::Resource, PartialEq, Debug, Clone, Default)]
pub struct Buffer(pub ndarray::Array2<Cell>);

pub fn plugin(app: &mut bevy::app::App) {
    app.insert_resource::<Buffer>(Buffer(ndarray::Array2::from_elem((75, 128), Cell::default())));
    app.add_systems(bevy::app::First, clear_buffer);
}

pub fn clear_buffer(mut buffer: bevy::ecs::system::ResMut<Buffer>) {
    buffer.0 = buffer.0.map(|_| Cell {
        depth: f32::MAX,
        ..Default::default()
    });
}
