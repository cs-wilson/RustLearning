fn main() {
    let _circle = Shape::Circle{radius: 3.0};
    let _square = Shape::Square{side: 3.0};
    let _triangle = Shape::Triangle{base: 3.0, height: 4.0};
    let _rectangle = Shape::Rectangle{length: 3.0, height: 4.0};
    println!("Circle area: {}", print_area(&_circle));
    println!("Square area: {}", print_area(&_square));
    println!("Triangle area: {}", print_area(&_triangle));
    println!("Rectangle area: {}", print_area(&_rectangle));
}

// 实现一个打印图形面积的函数，它接收一个可以计算面积的类型作为参数，比如圆形，三角形，正方形，需要用到泛型和泛型约束
enum Shape{
    Circle{radius: f64},
    Square{side: f64},
    Triangle{base: f64, height: f64},
    Rectangle{length: f64, height: f64}
}

pub trait CalArea {
    fn area(&self) -> f64;
}

impl CalArea for Shape {
    fn area(&self) -> f64 {
        match &self {
            Shape::Circle { radius } => {
                3.14*radius*radius
            },
            Shape::Square { side } => {
                side*side
            },
            Shape::Triangle { base, height } => {
                base*height/2.0
            },
            Shape::Rectangle { length, height } => {
                length*height
            }
        }
    }
}

fn print_area<T: CalArea>(t: &T) -> f64 {
    t.area()
}