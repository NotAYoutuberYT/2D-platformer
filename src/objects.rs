// basic point struct
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Point { x: x, y: y }
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
    fn points(&self) -> Vec<Point>;

    // returns a vector containing the leftmost
    // x value, rightmost x value, lowest y
    // value, and uppermost y value respectively
    fn bounds(&self) -> Vec<f64>;

    // detects collision with the other object
    // (colliding edges counts as collision)
    fn collides_with(&self, other: &dyn RectObject) -> bool {
        let self_bounds: Vec<f64> = self.bounds();
        let other_bounds: Vec<f64> = other.bounds();

        let mut collide_y: bool = false;
        let mut collide_x: bool = false;

        if sandwitch(other_bounds[0], self_bounds[0], other_bounds[1]) {
            // is self's leftmost side in other?
            collide_x = true;
        } else if sandwitch(other_bounds[0], self_bounds[1], other_bounds[1]) {
            // is self's rightmost side in other?
            collide_x = true;
        } else if self_bounds[0] <= other_bounds[0] && other_bounds[1] < self_bounds[1] {
            // do we completely contatain other on x axis?
            collide_x = true;
        }

        if sandwitch(other_bounds[2], self_bounds[2], other_bounds[3]) {
            // is self's bottom side in other?
            collide_y = true;
        } else if sandwitch(other_bounds[2], self_bounds[3], other_bounds[3]) {
            // is self's top side in other?
            collide_y = true;
        } else if self_bounds[2] <= other_bounds[2] && other_bounds[3] < self_bounds[3] {
            // do we completely contatain other on y axis?
            collide_y = true;
        }

        collide_x && collide_y
    }
}
