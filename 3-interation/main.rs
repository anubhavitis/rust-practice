// An attribute to hide warnings for unused code.
#![allow(dead_code)]


fn simple_loop() {
    // As name suggests, this is a simple loop.
    // No condition is given, loop continues until break statement is met.

    let mut i:u8 = 10;
    loop {
        if i==0 {
            break;
        }
        println!("{}", i);
        i -= 1;
    }
    print!("LIFT OFF")
}

fn while_loop() {
    // It's same as simple loop, only condition is passed too.

    let mut i:u8 = 10;
    while i>0 {
        println!("{}", i);
        i -= 1;
    }
    print!("LIFT OFF")
}

fn for_loop(){
    // it's like while loop, but initiation, condition and updates
    // which are irrelevant to the context of loop body are kept separate.

    for i in (1..11).rev() {
        println!("{}", i);
    }
    print!("LIFT OFF")
}

fn main() {
    // Uncomment to use whatever you wanna try

    // simple_loop();
    // while_loop();
    // for_loop();

}