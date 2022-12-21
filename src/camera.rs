use super::objects::Vector2;

pub struct Camera {
    pub bottom_left: Vector2,
}

impl Camera {
    pub fn new(x: f64, y: f64) -> Camera {
        Camera {
            bottom_left: Vector2::new(x, y),
        }
    }

    // returns the actual point in the game
    // that world the pixel point represents
    pub fn get_game_position(&self, point: Vector2) -> Vector2 {
        Vector2::new(
            self.bottom_left.x + point.x as f64,
            self.bottom_left.y + (super::constants::WINDOW_HEIGHT - point.y as usize) as f64,
        )
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
            bool,
            &[(f64, f64, f64, f64)],
            &Vec<(f64, f64, f64, f64)>,
        ) -> u32,
        player_bounds: &(f64, f64, f64, f64),
        is_sprinting: bool,
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
                    is_sprinting,
                    static_object_bounds,
                    moving_object_bounds,
                );
            }
        }
    }
}
