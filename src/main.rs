extern crate minifb;
use minifb::{Key, Window, WindowOptions};

mod objects;

const WINDOW_WIDTH: usize = 0xff * 4;
const WINDOW_HEIGHT: usize = 0xff * 3;

fn main() {
    let mut window = Window::new(
        "Platformer - ESC to exit",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|error| {
        panic!("Error opening window: {}", error);
    });

    // limit fps to 60
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    // this will be where we write out pixel values
    let mut buffer: Vec<u32> = vec![0; WINDOW_WIDTH * WINDOW_HEIGHT];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for x in 0..WINDOW_WIDTH {
            for y in 0..WINDOW_HEIGHT {
                buffer[y * WINDOW_WIDTH + x] = (x / 4 * 0x10000 + y / 3 + (WINDOW_WIDTH - x) / 4 * 0x100) as u32;
            }
        }

        // update our window with our pixel values
        window
            .update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap_or_else(|error| {
                panic!("Error updating window: {}", error);
            });
    }
}
