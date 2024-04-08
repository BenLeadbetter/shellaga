use crossterm::ExecutableCommand;

#[derive(bevy::ecs::event::Event, Debug)]
pub enum TerminalEvent {
    Key(crossterm::event::KeyEvent),
    Resize(u16, u16),
}

pub fn plugin(app: &mut bevy::app::App) {
    app.add_event::<TerminalEvent>();
    app.insert_resource(Terminal::new().expect("error initialising terminal"));
    app.add_systems(bevy::app::PreUpdate, handle_events);
    app.add_systems(bevy::app::Last, render);
}

#[derive(bevy::ecs::system::Resource)]
pub struct Terminal(ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>);

impl Drop for Terminal {
    fn drop(&mut self) {
        crossterm::execute!(
            std::io::stdout(),
            crossterm::event::PopKeyboardEnhancementFlags
        )
        .expect("pop keyboard extentions");
        std::io::stdout()
            .execute(crossterm::terminal::LeaveAlternateScreen)
            .expect("leave terminal alternate screen");
        crossterm::terminal::disable_raw_mode().expect("leave terminal raw mode");
    }
}

impl Terminal {
    pub fn new() -> Result<Self, std::boxed::Box<dyn std::error::Error>> {
        crossterm::terminal::enable_raw_mode()?;
        std::io::stdout().execute(crossterm::terminal::EnterAlternateScreen)?;
        crossterm::execute!(
            std::io::stdout(),
            crossterm::event::PushKeyboardEnhancementFlags(
                crossterm::event::KeyboardEnhancementFlags::REPORT_EVENT_TYPES
                    | crossterm::event::KeyboardEnhancementFlags::REPORT_ALTERNATE_KEYS
                    | crossterm::event::KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES
                    | crossterm::event::KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
            )
        )?;
        let backend = ratatui::backend::CrosstermBackend::new(std::io::stdout());
        Ok(Self(ratatui::Terminal::new(backend)?))
    }
}

struct LevelWidget<'a>(&'a crate::buffer::Buffer);

impl<'a> ratatui::widgets::Widget for LevelWidget<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let shape = self.0 .0.shape();
        use itertools::Itertools;
        for (row, col) in (0..shape[0]).cartesian_product(0..shape[1]) {
            let cell = self.0 .0[[row, col]];
            if col >= area.width.into() || row >= area.height.into() {
                continue;
            }
            buf.content[buf.area.width as usize * (row + area.y as usize) + col + area.x as usize] = {
                let mut rat_cell = ratatui::buffer::Cell::default();
                if let Some(c) = cell.character {
                    rat_cell.set_char(c);
                }
                rat_cell
            }
        }
    }
}

fn fallible_render(terminal: &mut Terminal, buffer: &crate::buffer::Buffer) -> std::io::Result<()> {
    terminal.0.draw(|frame| {
        let frame_size = ratatui::layout::Rect::new(
            0,
            0,
            crate::frame::WIDTH as u16,
            crate::frame::HEIGHT as u16,
        );
        let main_layout_vertical = ratatui::layout::Layout::new(
            ratatui::layout::Direction::Vertical,
            [
                ratatui::layout::Constraint::Fill(1),
                ratatui::layout::Constraint::Min(frame_size.y),
                ratatui::layout::Constraint::Fill(1),
            ],
        )
        .split(frame.size());
        let main_layout = ratatui::layout::Layout::new(
            ratatui::layout::Direction::Horizontal,
            [
                ratatui::layout::Constraint::Fill(1),
                ratatui::layout::Constraint::Min(frame_size.x),
                ratatui::layout::Constraint::Fill(1),
            ],
        )
        .split(main_layout_vertical[1]);
        let block = ratatui::widgets::Block::default()
            .title("Shellaga")
            .borders(ratatui::widgets::Borders::ALL);
        let inner_size = block.inner(main_layout[1]);
        frame.render_widget(block, main_layout[1]);
        frame.render_widget(LevelWidget(&*buffer), inner_size);
    })?;
    Ok(())
}

fn render(
    mut terminal: bevy::ecs::system::ResMut<Terminal>,
    buffer: bevy::ecs::system::Res<crate::buffer::Buffer>,
) {
    if let Err(_) = fallible_render(&mut *terminal, &*buffer) {
        log::error!("Failed to render frame");
    }
}

fn handle_events(mut event_sender: bevy::ecs::event::EventWriter<TerminalEvent>) {
    while let Ok(true) = crossterm::event::poll(std::time::Duration::from_millis(0)) {
        match crossterm::event::read() {
            Ok(e) => {
                log::trace!("crossterm event {:?}", e);
                match e {
                    // forward crossterm events into bevy
                    crossterm::event::Event::Key(key_event) => {
                        event_sender.send(TerminalEvent::Key(key_event));
                    }
                    crossterm::event::Event::Resize(w, h) => {
                        event_sender.send(TerminalEvent::Resize(w, h));
                    }
                    // ignore these for now
                    crossterm::event::Event::FocusGained => {}
                    crossterm::event::Event::FocusLost => {}
                    crossterm::event::Event::Mouse(_) => {}
                    crossterm::event::Event::Paste(_) => {}
                }
            }
            _ => {}
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn render_top_left() {
        let buffer = crate::buffer::Buffer(ndarray::array![['x']].map(|c| crate::buffer::Cell {
            character: Some(*c),
            ..Default::default()
        }));
        let widget = LevelWidget(&buffer);
        let mut terminal_buffer = ratatui::buffer::Buffer {
            area: ratatui::layout::Rect {
                x: 0,
                y: 0,
                width: 2,
                height: 2,
            },
            content: vec![Default::default(); 4],
        };

        let mut expected = terminal_buffer.clone();
        expected.content[0].set_char('x');

        use ratatui::widgets::Widget;
        widget.render(
            ratatui::layout::Rect {
                x: 0,
                y: 0,
                width: 1,
                height: 1,
            },
            &mut terminal_buffer,
        );

        assert_eq!(terminal_buffer, expected);
    }

    #[test]
    fn render_translated() {
        let buffer = crate::buffer::Buffer(ndarray::array![['x']].map(|c| crate::buffer::Cell {
            character: Some(*c),
            ..Default::default()
        }));
        let widget = LevelWidget(&buffer);
        let mut terminal_buffer = ratatui::buffer::Buffer {
            area: ratatui::layout::Rect {
                x: 0,
                y: 0,
                width: 2,
                height: 2,
            },
            content: vec![Default::default(); 4],
        };

        let mut expected = terminal_buffer.clone();
        expected.content[3].set_char('x');

        use ratatui::widgets::Widget;
        widget.render(
            ratatui::layout::Rect {
                x: 1,
                y: 1,
                width: 1,
                height: 1,
            },
            &mut terminal_buffer,
        );

        assert_eq!(terminal_buffer, expected);
    }

    #[test]
    fn render_2d_translated() {
        let buffer = crate::buffer::Buffer(
            ndarray::array![[Some('x'), Some('x')], [None, Some('x')]].map(|c| {
                crate::buffer::Cell {
                    character: *c,
                    ..Default::default()
                }
            }),
        );
        let widget = LevelWidget(&buffer);
        let mut terminal_buffer = ratatui::buffer::Buffer {
            area: ratatui::layout::Rect {
                x: 0,
                y: 0,
                width: 4,
                height: 4,
            },
            content: vec![Default::default(); 16],
        };

        let target = ratatui::layout::Rect {
            x: 1,
            y: 1,
            width: 2,
            height: 2,
        };

        // 0 0 0 0
        // 0 x x 0
        // 0 0 x 0
        // 0 0 0 0
        let mut expected = terminal_buffer.clone();
        expected.content[5].set_char('x');
        expected.content[6].set_char('x');
        expected.content[10].set_char('x');

        use ratatui::widgets::Widget;
        widget.render(target, &mut terminal_buffer);

        assert_eq!(terminal_buffer, expected);
    }

    #[test]
    fn render_2d_translated_x() {
        let buffer = crate::buffer::Buffer(
            ndarray::array![[Some('x'), Some('x')], [None, Some('x')]].map(|c| {
                crate::buffer::Cell {
                    character: *c,
                    ..Default::default()
                }
            }),
        );
        let widget = LevelWidget(&buffer);
        let mut terminal_buffer = ratatui::buffer::Buffer {
            area: ratatui::layout::Rect {
                x: 0,
                y: 0,
                width: 4,
                height: 4,
            },
            content: vec![Default::default(); 16],
        };

        let target = ratatui::layout::Rect {
            x: 2,
            y: 0,
            width: 2,
            height: 2,
        };

        // 0 0 x x
        // 0 0 0 x
        // 0 0 0 0
        // 0 0 0 0
        let mut expected = terminal_buffer.clone();
        expected.content[2].set_char('x');
        expected.content[3].set_char('x');
        expected.content[7].set_char('x');

        use ratatui::widgets::Widget;
        widget.render(target, &mut terminal_buffer);

        assert_eq!(terminal_buffer, expected);
    }

    #[test]
    fn render_2d_translated_y() {
        let buffer = crate::buffer::Buffer(
            ndarray::array![[Some('x'), Some('x')], [None, Some('x')]].map(|c| {
                crate::buffer::Cell {
                    character: *c,
                    ..Default::default()
                }
            }),
        );
        let widget = LevelWidget(&buffer);
        let mut terminal_buffer = ratatui::buffer::Buffer {
            area: ratatui::layout::Rect {
                x: 0,
                y: 0,
                width: 4,
                height: 4,
            },
            content: vec![Default::default(); 16],
        };

        let target = ratatui::layout::Rect {
            x: 0,
            y: 2,
            width: 2,
            height: 2,
        };

        // 0 0 0 0
        // 0 0 0 0
        // x x 0 0
        // 0 x 0 0
        let mut expected = terminal_buffer.clone();
        expected.content[8].set_char('x');
        expected.content[9].set_char('x');
        expected.content[13].set_char('x');

        use ratatui::widgets::Widget;
        widget.render(target, &mut terminal_buffer);

        assert_eq!(terminal_buffer, expected);
    }
}
