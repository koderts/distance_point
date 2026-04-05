use std::fmt;
use std::str::FromStr;
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

// Implements FromStr to help parse string into a type
#[derive(Debug)]
struct ParsePointError;

impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num_plot: Vec<&str> = s
            .trim_matches(|p| p == '(' || p == ')')
            .split(',')
            .collect();

        if num_plot.len() != 2 {
            return Err(ParsePointError);
        }
        let x = num_plot[0]
            .trim()
            .parse::<i32>()
            .map_err(|_| ParsePointError)?;
        let y: i32 = num_plot[1].trim().parse().map_err(|_| ParsePointError)?;

        Ok(Point { x, y })
    }
}
fn main() {
    let point1 = Point { x: 1, y: 2 };
    let point2 = Point::new(4, 4);
    let point3 = "(9,9)".parse::<Point>().unwrap();
    let point4: Point = "(3, 3)".parse().unwrap();

    // let point3 = "(1, 5)".parse()?;

    let d = distance_between_points(&point1, &point2);

    let c = distance_between_points(&point3, &point4);

    let x = point2.distance_from_origin();
    println!(
        "---------------------------------------------------------------------------------------"
    );

    println!(
        "The distance from the origin of point {} is {:.3}.",
        point2, x
    );

    println!(
        "The distance between {} and {} is {:.3}.",
        point1, point2, d
    );
    println!(
        "This is Point 3:{} and Point 4: {} is {:.3}",
        point3, point4, c
    );
    println!(
        "---------------------------------------------------------------------------------------"
    );
}

fn distance_between_points(a: &Point, b: &Point) -> f64 {
    let x_distance = (b.x - a.x).pow(2);
    let y_distance = (b.y - a.y).pow(2);

    ((x_distance + y_distance) as f64).sqrt()
}
