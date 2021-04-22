#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let test_rectangle = Rectangle {
        width: 10,
        height: 5
    };

    let area = find_area(&test_rectangle);

    println!("Area...! {}", area);

    println!("Rectangle width: {:?}", test_rectangle);
    println!("Rectangle width: {:#?}", test_rectangle);
}

fn find_area(rectangle: &Rectangle) -> u32 {
    rectangle.width + rectangle.height
}
