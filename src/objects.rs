#![allow(dead_code)]

// basic vector2 struct
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Self {
        Vector2 { x: x, y: y }
    }

    // returns a copy of self
    pub fn copy(&self) -> Vector2 {
        Vector2::new(self.x, self.y)
    }

    // makes set equal to passed vector
    pub fn set(&mut self, other: &Vector2) {
        self.x = other.x;
        self.y = other.y;
    }

    // returns the sum of self and other
    pub fn add(vector1: &Vector2, vector2: &Vector2) -> Vector2 {
        Vector2::new(vector1.x + vector2.x, vector1.y + vector2.y)
    }

    pub fn multiply(vector1: &Vector2, scalar: f64) -> Vector2 {
        Vector2::new(vector1.x * scalar, vector1.y * scalar)
    }

    // linearly interpolates the two vectors
    // t should be between 0 and one
    pub fn lerp(vector1: &Vector2, vector2: &Vector2, t: f64) -> Vector2 {
        Vector2::add(
            &Vector2::multiply(&vector1, 1.0 - t),
            &Vector2::multiply(&vector2, t),
        )
    }

    // adds self to other
    pub fn add_to(&self, other: &mut Vector2) {
        other.set(&Vector2::add(&self, &other));
    }
}

// ensures f2 is beteen f1 and f3
// (f1 must be below f3)
fn sandwitch(f1: f64, f2: f64, f3: f64) -> bool {
    f1 <= f2 && f2 <= f3
}

// a rectangle object that has two
// flat and two vertical lines
pub trait RectObject {
    // returns the object's 4 points
    pub fn points(&self) -> Vec<Vector2>;

    // returns a vector containing the leftmost
    // x value, rightmost x value, lowest y
    // value, and uppermost y value respectively
    pub fn bounds(&self) -> Vec<f64>;

    // detects if the there is collion on
    // the horizontal axis
    pub fn collides_with_y(&self, other: &dyn RectObject) -> bool {
        let self_bounds: Vec<f64> = self.bounds();
        let other_bounds: Vec<f64> = other.bounds();

        let mut collides: bool = false;

        if sandwitch(other_bounds[0], self_bounds[0], other_bounds[1]) {
            // is self's leftmost side in other?
            collides = true;
        } else if sandwitch(other_bounds[0], self_bounds[1], other_bounds[1]) {
            // is self's rightmost side in other?
            collides = true;
        } else if self_bounds[0] <= other_bounds[0] && other_bounds[1] < self_bounds[1] {
            // do we completely contatain other on x axis?
            collides = true;
        }

        collides
    }

    // detects if the there is collion on
    // the vertical axis
    pub fn collides_with_x(&self, other: &dyn RectObject) -> bool {
        let self_bounds: Vec<f64> = self.bounds();
        let other_bounds: Vec<f64> = other.bounds();

        let mut collides: bool = false;

        if sandwitch(other_bounds[2], self_bounds[2], other_bounds[3]) {
            // is self's bottom side in other?
            collides = true;
        } else if sandwitch(other_bounds[2], self_bounds[3], other_bounds[3]) {
            // is self's top side in other?
            collides = true;
        } else if self_bounds[2] <= other_bounds[2] && other_bounds[3] < self_bounds[3] {
            // do we completely contatain other on y axis?
            collides = true;
        }

        collides
    }

    // detects collision with the other object
    // (colliding edges counts as collision)
    pub fn collides_with(&self, other: &dyn RectObject) -> bool {
        self.collides_with_x(other) && self.collides_with_y(other)
    }
}

// RigidBody code

pub struct RigidBody {
    pub center: Vector2,
    pub velocity: Vector2,
    pub width: f64,
    pub height: f64,
    pub density: f64,
}

impl RigidBody {
    // movement with no physics
    pub fn move_by(&mut self, movement: &Vector2) {
        movement.add_to(&mut self.center);
    }
}

impl RectObject for RigidBody {
    fn points(&self) -> Vec<Vector2> {
        let mut points: Vec<Vector2> = vec![];

        let half_width: f64 = self.width / 2.0;
        let half_height: f64 = self.height / 2.0;

        // add points to vector
        points.push(Vector2::new(
            self.center.x - half_width,
            self.center.y + half_height,
        ));
        points.push(Vector2::new(
            self.center.x + half_width,
            self.center.y + half_height,
        ));
        points.push(Vector2::new(
            self.center.x + half_width,
            self.center.y - half_height,
        ));
        points.push(Vector2::new(
            self.center.x - half_width,
            self.center.y - half_height,
        ));

        points
    }

    fn bounds(&self) -> Vec<f64> {
        let mut points: Vec<f64> = vec![];

        // push left x, right x
        let half_width: f64 = self.width / 2.0;
        points.push(self.center.x - half_width);
        points.push(self.center.x + half_width);

        // push bottom y, top y
        let half_height: f64 = self.width / 2.0;
        points.push(self.center.y - half_height);
        points.push(self.center.y + half_height);

        points
    }
}

// MovingObject code

#[derive(PartialEq, Eq)]
pub enum MovingObjectDirections {
    Leaving,
    Returning,
    Standstil,
}

pub struct MovingObject {
    pub start_pos: Vector2,
    pub end_pos: Vector2,
    pub width: f64,
    pub height: f64,
    pub fallthrough: bool,

    moving_time: f64, // percent of time until destination point reached
    pub direction: MovingObjectDirections,
    pub center: Vector2,
}

impl MovingObject {
    // creates a new leaving moving platform at startpoing
    pub fn new(
        start_pos: Vector2,
        end_pos: Vector2,
        width: f64,
        height: f64,
        fallthrough: bool,
    ) -> Self {
        let center: Vector2 = start_pos.copy();

        MovingObject {
            start_pos,
            end_pos,
            width: width,
            height: height,
            fallthrough: fallthrough,

            moving_time: 0.0,
            direction: MovingObjectDirections::Leaving,
            center: center,
        }
    }

    // moves the object across its path by time (0 is start, 1 is end, 2 is returned)
    // will automatically change direction
    // if ammount is zero, will set direction to standstil
    // will panic on negative ammount values
    pub fn update(&mut self, ammount: f64) {
        // check if the object stopped moving, and do a little
        // error handling while we're at it
        if ammount < 0.0 {
            panic!("Error: attempted to move moving platform negative ammount");
        } else if ammount == 0.0 {
            self.direction = MovingObjectDirections::Standstil;
            return;
        }

        self.moving_time += ammount;

        // this prevents overflow
        while self.moving_time >= 2.0 {
            self.moving_time -= 2.0;
        }

        // sets moving direction to be correct
        if self.moving_time <= 1.0 {
            self.direction = MovingObjectDirections::Leaving;
        } else {
            self.direction = MovingObjectDirections::Returning;
        }

        // configures the lerp ammount
        let lerp_ammount: f64;
        if self.direction == MovingObjectDirections::Leaving {
            lerp_ammount = self.moving_time;
        } else {
            lerp_ammount = 2.0 - self.moving_time;
        }

        // lerp between the two points to determine the center of the moving platform
        self.center
            .set(&Vector2::lerp(&self.start_pos, &self.end_pos, lerp_ammount));
    }
}

impl RectObject for MovingObject {
    fn points(&self) -> Vec<Vector2> {
        let mut points: Vec<Vector2> = vec![];

        let half_width: f64 = self.width / 2.0;
        let half_height: f64 = self.height / 2.0;

        // add points to vector
        points.push(Vector2::new(
            self.center.x - half_width,
            self.center.y + half_height,
        ));
        points.push(Vector2::new(
            self.center.x + half_width,
            self.center.y + half_height,
        ));
        points.push(Vector2::new(
            self.center.x + half_width,
            self.center.y - half_height,
        ));
        points.push(Vector2::new(
            self.center.x - half_width,
            self.center.y - half_height,
        ));

        points
    }

    fn bounds(&self) -> Vec<f64> {
        let mut points: Vec<f64> = vec![];

        // push left x, right x
        let half_width: f64 = self.width / 2.0;
        points.push(self.center.x - half_width);
        points.push(self.center.x + half_width);

        // push bottom y, top y
        let half_height: f64 = self.width / 2.0;
        points.push(self.center.y - half_height);
        points.push(self.center.y + half_height);

        points
    }
}

// StaticObject code

pub struct StaticObject {
    pub center: Vector2,
    pub width: f64,
    pub height: f64,
}

impl RectObject for StaticObject {
    fn points(&self) -> Vec<Vector2> {
        let mut points: Vec<Vector2> = vec![];

        let half_width: f64 = self.width / 2.0;
        let half_height: f64 = self.height / 2.0;

        // add points to vector
        points.push(Vector2::new(
            self.center.x - half_width,
            self.center.y + half_height,
        ));
        points.push(Vector2::new(
            self.center.x + half_width,
            self.center.y + half_height,
        ));
        points.push(Vector2::new(
            self.center.x + half_width,
            self.center.y - half_height,
        ));
        points.push(Vector2::new(
            self.center.x - half_width,
            self.center.y - half_height,
        ));

        points
    }

    fn bounds(&self) -> Vec<f64> {
        let mut points: Vec<f64> = vec![];

        // push left x, right x
        let half_width: f64 = self.width / 2.0;
        points.push(self.center.x - half_width);
        points.push(self.center.x + half_width);

        // push bottom y, top y
        let half_height: f64 = self.width / 2.0;
        points.push(self.center.y - half_height);
        points.push(self.center.y + half_height);

        points
    }
}