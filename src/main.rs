extern crate minifb;
use minifb::{Key, Window, WindowOptions};

mod objects;
use objects::{RectObject, RigidBody, Vector2};

const WINDOW_WIDTH: usize = 0xff * 4;
const WINDOW_HEIGHT: usize = 0xff * 3;

fn main() {
    // our window :)
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
    let mut window_buffer: Vec<u32> = vec![0; WINDOW_WIDTH * WINDOW_HEIGHT];

    // how long each frame takes (in seconds)
    let mut frame_time: f64 = 0.0;

    // our player
    let mut player = RigidBody {
        center: Vector2::new(255.0, 520.0),
        width: 20.0,
        height: 40.0,

        velocity: Vector2::new(0.0, 0.0),
        density: 0.0,
        static_friction: false,
    };

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // used to measure the frame time
        let now = std::time::Instant::now();

        // physics
        player.velocity.y -= frame_time / 40.0;
        player.move_by(&Vector2::multiply(&player.velocity, frame_time));

        // cache bounds
        let bounds = player.bounds();

        // render each pixel
        for x in 0..WINDOW_WIDTH {
            for y in 0..WINDOW_HEIGHT {
                window_buffer[y * WINDOW_WIDTH + x] =
                    match player.contains_point_cache_bounds(&Vector2::new(x as f64, (WINDOW_HEIGHT - y) as f64), &bounds) {
                        true => 0xff0000,
                        false => 0x0,
                    };
            }
        }

        // update our window with our pixel values
        window
            .update_with_buffer(&window_buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap_or_else(|error| {
                panic!("Error updating window: {}", error);
            });

        // update how long the frame took
        frame_time = now.elapsed().as_micros() as f64 / 10000.0;
    }
}
