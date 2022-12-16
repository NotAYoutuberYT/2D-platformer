extern crate minifb;
use minifb::{Key, Window, WindowOptions};

mod objects;
use objects::{contains_point_cache_bounds, RectObject, RigidBody, StaticObject, Vector2};

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

    // grounds
    let ground = StaticObject {
        center: Vector2::new(510.0, 75.0),
        width: 1020.0,
        height: 150.0,
    };

    // cache object bounds
    let ground_bounds = ground.bounds();
    let mut player_bounds;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // recache player bounds
        player_bounds = player.bounds();

        // used to measure the frame time
        let now = std::time::Instant::now();

        // physics
        player.velocity.y -= frame_time / 20.0;

        // calculate how far the player moves
        let movement_vector = &Vector2::multiply(&player.velocity, frame_time);

        // do all moving and physics only if the window is active
        // so that physics don't break
        if window.is_active() {
            player.move_by(&movement_vector);

            if window.is_key_down(Key::A) {
                player.center.x -= 5.0 * frame_time;
            }
            if window.is_key_down(Key::D) {
                player.center.x += 5.0 * frame_time;
            }
        }

        let mut on_ground: bool = false;

        // if the player is on the ground, move it out of the ground
        if player.collides_with(&ground) {
            // if we collide with the ground, put ourselves on it
            player.center.y = ground_bounds[3] + player.height / 2.0;
            player.velocity = Vector2::new(0.0, 0.0);

            on_ground = true;
        }

        // jump if space is pressed and window is focused
        if on_ground && window.is_key_down(Key::Space) && window.is_active() {
            player.velocity.y = 2.5;
        }

        // render each pixel
        for x in 0..WINDOW_WIDTH {
            for y in 0..WINDOW_HEIGHT {
                let rgb: u32;
                let point = Vector2::new(x as f64, (WINDOW_HEIGHT - y) as f64);

                if contains_point_cache_bounds(&point, &player_bounds) {
                    rgb = 0xff0000;
                } else if contains_point_cache_bounds(&point, &ground_bounds) {
                    rgb = 0xff;
                } else {
                    rgb = 0x200020;
                }

                window_buffer[y * WINDOW_WIDTH + x] = rgb;
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
