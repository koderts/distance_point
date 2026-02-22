
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

