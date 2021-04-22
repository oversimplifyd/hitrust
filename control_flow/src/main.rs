fn main() {

    loop_with_loop();
    loop_with_for_in();
    nth_fibonnaci(4);
}

fn loop_with_loop() {

    let mut check = 0;

    let result = loop {

        check += 1;

        if check == 2 {
            break 6
        }
    };

    println!("Reslt {}", result);
}

fn loop_with_for_in() {

    let tuple: [u32; 3] = [1, 2, 3];
    for element in tuple.iter() {
        println!("This is element {}", element);
    }

    for element in 1..4 {
        println!("This is another element {}", element);
    }

    const SIZE: u32 = 15;

    for number in SIZE..SIZE+1 {
        println!("This is another element {}", number);
    }

    // Also: 
    // for element in (1...4) to traverse range
    // for element in (1...4).rev()  to reverse the range
}

fn nth_fibonnaci(num: u32) -> u32 {
    let number = num;

    for n in 1..number {
        println!("This is another element {}", n);
    }

    number
}
