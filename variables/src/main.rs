fn main() {

    // scalar types 
    let a = 1; 
    let b: u32 = 209393; 
    let c: i8 = -3;

    let d: f32 = 3.09;

    let e: char = '3';

    let f: bool = true;

    let first_array: [u32; 6] = [1, 2, 3, 4, 5, 6];
     // accessing fristArray[0]

    let second_array = [3; 4];
    // Filled with 3s, length 4 

    let tuple: (u32, f32, char) = (2, 4.0, 'd');
    // accessing tuple.0, tuple.1 etc..

    println!("The values {} {} {} {} {} {} {} {} {}!", a, b, c, d, e, f, first_array[0], second_array[1], tuple.1);
}
