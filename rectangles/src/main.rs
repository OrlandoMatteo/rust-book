struct Rect {
    height: i32,
    width: i32,
}
impl Rect {
    fn area(&self) -> i32 {
        self.height * self.width
    }
    fn perimeter(&self) -> i32 {
        2 * (self.width + self.height)
    }
}

impl Rect {
    fn square(size: i32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
fn main() {
    let rect = Rect {
        height: 2,
        width: 3,
    };
    println!("The area is {}", rect.area());
    println!("The perimeter is {}", rect.perimeter());
    let square=Rect::square(3);
    println!("The area is {}", square.area());
    println!("The perimeter is {}", square.perimeter());
}
