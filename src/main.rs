use std::fmt;
// An attribute to hide warnings for unused code.
// #![allow(dead_code)]

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn distance_from_origin(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
// let p1 = Point::new(4, 4);
// let d = p1.distance_from_origin();
// println!("The distance between point {} and origin is {d}.", p1);
fn main() {
    let point1 = Point { x: 1, y: 2 };
    let point2 = Point::new(4, 4);

    let d = distance_between_points(&point1, &point2);

    let x = point2.distance_from_origin();
    println!(
        "---------------------------------------------------------------------------------------"
    );

    println!("This is {:.3} something.", x);

    println!("The distance between {} and {} is {:.3}", point1, point2, d);
    println!(
        "---------------------------------------------------------------------------------------"
    );
}

fn distance_between_points(a: &Point, b: &Point) -> f64 {
    println!("THIS IS: {:?} AND : {:?}", a, b);
    let x_distance = (b.x - a.x).pow(2);
    let y_distance = (b.y - a.y).pow(2);
    let distance = ((x_distance + y_distance) as f64).sqrt();

    return distance;
}
