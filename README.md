# distance_point

## Associative
```rust
//this creates a associative for the Point type to so when you create a new Point it will be shorter! e.g:

//let point1 = Point { x: 1, y: 1 };
//let point2 = Point::new(4, 4); <- You don't need to explain the x and y coordinate!

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}
```

## Changelog

- Today 2/4/26, I implemented the `FromStr` trait because if I were to take the input the terminal produces a string. And the `FromStr` allows a string to parse into your custom type.
  1. Create a new struct for the Error for Parse design for Point
  2. I implemented the trait and I used the `std::str::FromStr;` module added the required method + create an alias for the new struct error using "type"
  1. I created a fn for `FromStr` so that it will do all the parse and the trimming
  1. trimmed all the spaces, the parentheses and removed the comma + I stored all the string in a vec
  1. Used a if statment, incase the input has too many numbr it will return the Err else it will create x and y to store it
  6. Then the Ok is for indicating it was successful
  7. now to us eit it will look like this: `let point4 = "(3, 3)".parse::<Point>().unwrap();`
  8. print it normally and the end 🙃
