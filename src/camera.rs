use super::{
    constants::{
        CAMERA_MOVING_EASING_X, CAMERA_MOVING_EASING_Y, MAX_X_FROM_CAMERA, MAX_Y_FROM_CAMERA,
        MIN_X_FROM_CAMERA, MIN_Y_FROM_CAMERA,
    },
    objects::{RigidBody, Vector2},
};

pub struct Camera {
    pub bottom_left: Vector2,
}

impl Camera {
    /// creates a new camera with the x and why coordinates
    /// of the bottom left corner of the rendering area
    pub fn new(x: f64, y: f64) -> Camera {
        Camera {
            bottom_left: Vector2::new(x, y),
        }
    }

    /// returns the actual point in the game
    /// that world the pixel point represents
    pub fn get_game_position(&self, point: Vector2) -> Vector2 {
        Vector2::new(
            self.bottom_left.x + point.x as f64,
            self.bottom_left.y + (super::constants::WINDOW_HEIGHT - point.y as usize) as f64,
        )
    }

    /// keeps the camera centered on a player
    pub fn keep_centered_on_player(&mut self, player: &mut RigidBody, frame_time: f64) {
        if player.center.x - self.bottom_left.x < MIN_X_FROM_CAMERA {
            self.bottom_left.x -= (player.center.x - self.bottom_left.x - MIN_X_FROM_CAMERA)
                * (player.center.x - self.bottom_left.x - MIN_X_FROM_CAMERA)
                * frame_time
                * CAMERA_MOVING_EASING_X;

            // avoid over-correcting the camera
            if player.center.x - self.bottom_left.x > MIN_X_FROM_CAMERA {
                self.bottom_left.x = player.center.x - MIN_X_FROM_CAMERA;
            }
        } else if player.center.x - self.bottom_left.x > MAX_X_FROM_CAMERA {
            self.bottom_left.x += (player.center.x - self.bottom_left.x - MAX_X_FROM_CAMERA)
                * (player.center.x - self.bottom_left.x - MAX_X_FROM_CAMERA)
                * frame_time
                * CAMERA_MOVING_EASING_X;

            // avoid over-correcting the camera
            if player.center.x - self.bottom_left.x < MAX_X_FROM_CAMERA {
                self.bottom_left.x = player.center.x - MAX_X_FROM_CAMERA;
            }
        }

        if player.center.y - self.bottom_left.y < MIN_Y_FROM_CAMERA {
            self.bottom_left.y -= (player.center.y - self.bottom_left.y - MIN_Y_FROM_CAMERA)
                * (player.center.y - self.bottom_left.y - MIN_Y_FROM_CAMERA)
                * frame_time
                * CAMERA_MOVING_EASING_Y;

            // avoid over-correcting the camera
            if player.center.y - self.bottom_left.y > MIN_Y_FROM_CAMERA {
                self.bottom_left.y = player.center.y - MIN_Y_FROM_CAMERA;
            }
        } else if player.center.y - self.bottom_left.y > MAX_Y_FROM_CAMERA {
            self.bottom_left.y += (player.center.y - self.bottom_left.y - MAX_Y_FROM_CAMERA)
                * (player.center.y - self.bottom_left.y - MAX_Y_FROM_CAMERA)
                * frame_time
                * CAMERA_MOVING_EASING_Y;

            // avoid over-correcting the camera
            if player.center.y - self.bottom_left.y < MAX_Y_FROM_CAMERA {
                self.bottom_left.y = player.center.y - MAX_Y_FROM_CAMERA;
            }
        }
    }

    // renders the camera, using the inputted function to convert
    // pixels in the gamespace into rgb values

    // the function should take the following inputs:
    // the game point, the player bounds, if the player is sprinting,
    // the static object bounds, the moving object bounds
    pub fn render_frame(
        &self,
        func: &dyn Fn(
            Vector2,
            &(f64, f64, f64, f64),
            &[(f64, f64, f64, f64)],
            &Vec<(f64, f64, f64, f64)>,
        ) -> u32,
        player_bounds: &(f64, f64, f64, f64),
        static_object_bounds: &[(f64, f64, f64, f64)],
        moving_object_bounds: &Vec<(f64, f64, f64, f64)>,
        buffer: &mut Vec<u32>,
    ) {
        for x in 0..super::WINDOW_WIDTH {
            for y in 0..super::WINDOW_HEIGHT {
                // the coordinate in the world that this pixel is
                let world_point = self.get_game_position(Vector2::new(x as f64, y as f64));

                buffer[y * super::WINDOW_WIDTH + x] = func(
                    world_point,
                    player_bounds,
                    static_object_bounds,
                    moving_object_bounds,
                );
            }
        }
    }
}
