#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn overloaded_area(&self, another_height: u32) -> u32 {
        self.height * self.width * another_height
    }
}

fn main() {
    let first_rectangle = Rectangle {
        width: 10,
        height: 20
    };


    println!("This is the area of rectange! {}", first_rectangle.area());
    println!("This is the area of rectange! {}", first_rectangle.overloaded_area(10));
    println!("This is the height of the rectange! {}", first_rectangle.height);
}
