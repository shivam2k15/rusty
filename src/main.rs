fn main() {
    // let even = is_even(20);
    // let fibres = fib(4);
    // let user: User = User {
    //     active: true,
    //     name: String::from("Shivam"),
    //     email: String::from("value@gmail.com"),
    //     active_count: 1,
    // };

    // let rect = Rect {
    //     width: 20,
    //     height: 30,
    // };

    print!(
        "the area of rect, sq, circle is {} {} {}",
        get_area(Shape::Rectangle(20.0, 30.0)),
        get_area(Shape::Square(20.0)),
        get_area(Shape::Circle(20.0))
    );
}

fn get_area(shape: Shape) -> f64 {
    match shape {
        Shape::Rectangle(a, b) => rect_area(a, b),
        Shape::Square(a) => a * a,
        Shape::Circle(r) => 3.14 * r * r,
    }
}

enum Shape {
    Rectangle(f64, f64),
    Square(f64),
    Circle(f64),
}

fn rect_area(a: f64, b: f64) -> f64 {
    a * b
}

// struct Rect {
//     width: u32,
//     height: u32,
// }

// impl Rect {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
//     fn perimeter(&self) -> u32 {
//         2 * (self.width + self.height)
//     }
// }

// struct User {
//     active: bool,
//     name: String,
//     email: String,
//     active_count: u32,
// }

// fn is_even(num: i32) -> bool {
//     if num % 2 == 0 {
//         return true;
//     }
//     return false;
// }

// fn non_optimized_fib(num: u32) -> u32 {
//     if num == 0 {
//         return 0;
//     }
//     if num == 1 {
//         return 1;
//     }
//     return fib(num - 1) + fib(num - 2);
// }

// fn fib(num: u32) -> u32 {
//     let mut first = 0;
//     let mut second = 1;
//     if num == 0 {
//         return 0;
//     }
//     if num == 1 {
//         return 1;
//     }

//     for _ in 2..num {
//         let temp = first + second;
//         first = second;
//         second = temp;
//     }
//     return second;
// }

// fn get_length(s: &str) -> usize {
//     s.chars().count()
// }
