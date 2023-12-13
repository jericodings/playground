mod my_funcs;
mod other_funcs;

use crate::my_funcs::add_five;
use crate::other_funcs::minus_funcs::subtract_10;

fn main() {
    let mut x: u32 = 50;
    println!("x is {}", x);

    let y: u32 = add_five(x);
    println!("y is {}", y);

    let z: u32 = subtract_10(x);
    println!("z is {}", z);

    x = 70;
    println!("x is {}", x);
}

fn my_function() {
    let x: i32 = 50;
    let y: i32 = 20;
    println!("y {}", y);
}
