use super::{
    camera::{Camera, Rgb},
    constants::{
        BACKGROUND_COLOR, CHECKPOINT_COLOR, FRICTION_AIR, FRICTION_GROUND, GRAVITY_MOVING_DOWN,
        GRAVITY_MOVING_UP, JUMP_BUFFER_HUNDREDTH_SECONDS, JUMP_FORCE, MOVING_OBJECT_COLOR,
        MOVING_PLATFORM_INDICATOR_COLOR, PLAYER_AIR_ACCELERATION_RATIO, PLAYER_COLOR,
        PLAYER_WALKING_ACCEL, STATIC_OBJECT_COLOR, STUCK_PLATFORM_VELOCITY_ADD_MODIFIER,
        VERTICAL_VELOCITY_ON_OR_UNDER_OBJECT, VOID_COLOR, VOID_TRANSITION_SIZE, WINDOW_HEIGHT,
        WINDOW_WIDTH,
    },
    map::Map,
    objects::{CollisionTypes, MovingObject, RectObject, Vector2},
};

use crate::constants::COYOTE_TIME_HUNDREDTH_SECONDS;
use minifb::{Key, KeyRepeat, Window};

// this is the function we use to render the game
fn render_game(world_point: Vector2, map: &Map) -> Rgb {
    let rgb: Rgb;

    // determine collision with player
    let player_collision = map.player.contains_point(&world_point);

    // determine collision with static objects
    let static_object_collision = map
        .static_objects
        .iter()
        .any(|object| object.bounds().contains_point(&world_point));

    // determine collision with moving objects
    let moving_object_collision = map
        .moving_objects
        .iter()
        .any(|object| object.bounds().contains_point(&world_point));

    // determine if there should be any rendering of circles
    let mut circle_color: Option<Rgb> = None;
    if map
        .moving_object_indicators
        .iter()
        .any(|circle| circle.contains_point(&world_point))
    {
        circle_color = Some(MOVING_PLATFORM_INDICATOR_COLOR)
    }

    if map
        .checkpoints
        .iter()
        .any(|checkpoint| checkpoint.indicator.contains_point(&world_point))
    {
        circle_color = Some(CHECKPOINT_COLOR);
    }

    if map.goal.contains_point(&world_point) {
        circle_color = Some(map.goal.color);
    }

    // find the proper rgb value
    if player_collision {
        rgb = PLAYER_COLOR;
    } else if moving_object_collision {
        rgb = MOVING_OBJECT_COLOR;
    } else if static_object_collision {
        rgb = STATIC_OBJECT_COLOR;
    } else if let Some(color) = circle_color {
        rgb = color;
    } else if world_point.y > map.lowest_point + VOID_TRANSITION_SIZE / 2.0 {
        rgb = BACKGROUND_COLOR;
    } else if world_point.y < map.lowest_point - VOID_TRANSITION_SIZE / 2.0 {
        rgb = VOID_COLOR;
    } else {
        let distance_in = map.lowest_point + VOID_TRANSITION_SIZE / 2.0 - world_point.y;
        let blend_amount = distance_in / VOID_TRANSITION_SIZE;
        rgb = BACKGROUND_COLOR.blend(blend_amount, VOID_COLOR);
    }

    rgb
}

/**
  plays a game with a supplied map and window
* function will end when the player beats the level or presses escape
* returns if the user tried to terminate the entire program
*/
pub fn play_game(map: &mut Map, window: &mut Window) -> bool {
    // this will be where we write out pixel values
    let mut window_buffer: Vec<u32> = vec![0; WINDOW_WIDTH * WINDOW_HEIGHT];

    // create our camera
    let mut camera = Camera::new(map.player.center.x - WINDOW_WIDTH as f64 / 2.0, 0.0);

    // how long each frame takes (in hundreds of a seconds)
    let mut frame_time: f64 = 0.0;

    // jump buffers make movement feel a little better by jumping even if the
    // player clicks the jump button just before they land on the ground
    let mut jump_buffer: f64 = 0.0;

    // coyote time makes movement feel a little better by jumping
    // even if the player jumps just after leaving the ground
    let mut coyote_time: f64 = 0.0;

    // this is where we'll store the player's active collision
    let mut collision: Vec<CollisionTypes> = Vec::new();

    // when a player is on a moving platform, we "stick" them to
    // the platform to stop them from bouncing on it as it moves
    let mut stuck_platform: Option<MovingObject> = None;

    while window.is_open()
        && (!window.is_key_pressed(Key::Key1, KeyRepeat::No)
            && !window.is_key_pressed(Key::Escape, KeyRepeat::No))
    {
        // used to measure the frame time
        let frame_start = std::time::Instant::now();

        //
        // player movement and velocity
        //

        // this is where the player's acceleration is stored
        let mut player_acceleration_vector: Vector2 = Vector2::new(0.0, 0.0);

        // configure vertical acceleration (gravity)
        player_acceleration_vector.y = match map.player.velocity.y <= 0.0 {
            false => GRAVITY_MOVING_UP,
            true => GRAVITY_MOVING_DOWN,
        };

        // configure horizontal acceleration (movement)
        let mut current_x_acceleration = PLAYER_WALKING_ACCEL;
        if !collision.contains(&CollisionTypes::Top) {
            current_x_acceleration *= PLAYER_AIR_ACCELERATION_RATIO;
        }

        if window.is_key_down(Key::D) || window.is_key_down(Key::Right) {
            player_acceleration_vector.x += current_x_acceleration;
        }
        if window.is_key_down(Key::A) || window.is_key_down(Key::Left) {
            player_acceleration_vector.x -= current_x_acceleration;
        }

        // find horizontal acceleration
        let current_friction = f64::min(
            map.player.velocity.x.abs(),
            match collision.contains(&CollisionTypes::Top) {
                true => FRICTION_GROUND * map.player.velocity.x.abs(),
                false => FRICTION_AIR * map.player.velocity.x.abs(),
            },
        );

        // apply friction
        player_acceleration_vector.x += match map.player.velocity.x < 0.0 {
            true => current_friction,
            false => -current_friction,
        };

        // move the player (we integrate the player's movement to make
        // the physics continuous and therefore framerate-independent)
        let movement_vector = Vector2::add(
            // accel * t^2 / 2
            &Vector2::multiply(&player_acceleration_vector, frame_time * frame_time / 2.0),
            // vel * t
            &Vector2::multiply(&map.player.velocity, frame_time),
            // c is already stored in the player's position
            // and will be included when we add this movement
            // vector to the player's current position
        );

        // apply the movement vector we calculated (adds c)
        map.player.move_by(&movement_vector);

        // update velocity (no integrating is needed as accel * t is exactly the growth in velocity)
        Vector2::multiply(&player_acceleration_vector, frame_time).add_to(&mut map.player.velocity);

        //
        // moving platform stuff
        //

        // update the position of moving platforms
        for moving_object in &mut map.moving_objects {
            moving_object.update(frame_time);
        }

        // move with the platform we're stuck to
        if let Some(mut stuck_obj) = stuck_platform {
            // only keep the player stuck if they're still on the platform
            if map.player.collides_with_y(&stuck_obj) {
                stuck_obj.update(frame_time);
                map.player.center.x += stuck_obj.prev_move().x;
                // move the player slightly into the platform to keep them stuck
                map.player.center.y = stuck_obj.bounds().top + map.player.height / 2.0 - 0.01;
            }
        }

        stuck_platform = None;

        //
        // collision handling
        //

        // reset collision
        collision = Vec::new();

        // handle collisions with moving objects, and if we're stuck to
        // an object, update the stuck_platform variable
        if let Some(index) = map
            .player
            .handle_collisions(&map.moving_objects, &mut collision)
        {
            stuck_platform = Some(map.moving_objects[index].clone());
        }

        // handle collisions with static objects
        map.player
            .handle_collisions(&map.static_objects, &mut collision);

        //
        // final physics before rendering graphics
        //

        // decrease our jump buffer
        jump_buffer -= frame_time;

        // handle coyote time
        if collision.contains(&CollisionTypes::Top) {
            coyote_time = COYOTE_TIME_HUNDREDTH_SECONDS;
        } else if coyote_time > 0.0 {
            coyote_time -= frame_time;
        }

        // reset the player's velocity if they're
        // on the side of an object
        if collision.contains(&CollisionTypes::Left) || collision.contains(&CollisionTypes::Right) {
            map.player.velocity.x = 0.0;
        }

        // if any of the space keys are pressed, start jump buffer
        let jump_pressed = window.is_key_pressed(Key::Space, KeyRepeat::No)
            || window.is_key_pressed(Key::W, KeyRepeat::No)
            || window.is_key_pressed(Key::Up, KeyRepeat::No);

        if jump_pressed {
            jump_buffer = JUMP_BUFFER_HUNDREDTH_SECONDS;
        }

        // handle jumping
        if coyote_time > 0.0 && jump_buffer > 0.0 {
            // reset coyote time
            coyote_time = 0.0;

            // if the player is stuck to a platform, add that object's
            // velocity multiplied by a constant to the player's velocity
            let mut additional_velocity = Vector2::new(0.0, 0.0);
            if let Some(obj) = stuck_platform {
                additional_velocity =
                    Vector2::multiply(&obj.prev_move(), STUCK_PLATFORM_VELOCITY_ADD_MODIFIER);
            }

            // set the correct vertical velocity
            map.player.velocity.y = JUMP_FORCE;
            additional_velocity.add_to(&mut map.player.velocity);

            // reset the jump buffer
            jump_buffer = 0.0;

            // unstick the player from the platform
            stuck_platform = None;
        }
        // if the player is on the top of or the bottom of an
        // object, reset the player's vertical velocity
        else if collision.contains(&CollisionTypes::Top)
            || collision.contains(&CollisionTypes::Bottom)
        {
            map.player.velocity.y = VERTICAL_VELOCITY_ON_OR_UNDER_OBJECT;
        }

        // handle checkpoints
        for checkpoint in &map.checkpoints {
            if checkpoint.indicator.intersects_rigidbody(&map.player) {
                map.player_respawn = checkpoint.respawn;
            }
        }

        // respawn if the player is too low or is being squished
        if map.player.center.y < map.lowest_point
            || (collision.contains(&CollisionTypes::Top)
                && collision.contains(&CollisionTypes::Bottom))
            || (collision.contains(&CollisionTypes::Left)
                && collision.contains(&CollisionTypes::Right))
        {
            map.player = map.player_respawn;
        }

        // keep camera centered on player
        camera.keep_centered_on_player(&mut map.player, frame_time);

        //
        // graphics
        //

        // render our graphics
        camera.render_frame(&render_game, map, &mut window_buffer);

        // update our window with our pixel values
        window
            .update_with_buffer(&window_buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap_or_else(|error| {
                panic!("Error updating window: {}", error);
            });

        // update how long the frame took
        frame_time = frame_start.elapsed().as_micros() as f64 / 10000.0;

        // go to the next level if the goal was reached
        if map.goal.intersects_rigidbody(&map.player) {
            break;
        }
    }

    // update window with the last rendered frame so that
    // any keys pressed last frame don't count as pressed
    // next time they're read with key-repeat true
    window
        .update_with_buffer(&window_buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
        .unwrap_or_else(|error| {
            panic!("Error updating window: {}", error);
        });

    // returns if the entire program was set to be terminated
    !window.is_open() || window.is_key_down(Key::Escape)
}
