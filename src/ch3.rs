

pub fn main() {
    //Mutability
    let mut x = 5;
    x = x + 6;
    println!("mutable x = {}", x);

    //Constants
    const MY_HEIGHT: i32 = 32;
    println!("MY HEIGHT IS {MY_HEIGHT}");

    //Shadowing
    let y = 5;
    let y = "I am a shadow of my former self!";
    println!("y = {}", y);

    //Types
    let five: i8 = 5; //(-127 to 127)
    let five: u8 = 5;//(0 to 255)
    // i8, u8 - i16/u16, i32/u32, i64/u64, i128/u128, isize/usize
    // isize / usize depends on whether you're on a 32 or 64 bit system

    let one_thousand: i32 = 1_000; // you don't have to explicitly call
    // out the i32 - rust will use that as default
    //can also use an underscore as a visual separator.

    // other types (ints)
    let hexi: i32 = 0xff;
    println!("{}", hexi);

    let octa: i32 = 0o77;
    println!("{}", octa);

    let bin = 0b11;
    println!("{}", bin);

    //floats
    let x = 2.0; //f64 (double)
    let y: f32 = 2.1; // f32 (single)

    let z = 5/3;
    println!("5/3 = {}", z);

    let z = 5.0/3.0; // cannot divide 5.0 / 3
    println!("5.0/3.0 = {}", z);

    let z = 43 % 5; // type i32
    println!("43 % 5 = {}", z);

    let z= z as f64; // convert types..

    let z = (1 as f64) * 4.0;

    let c = 'z';
    //let z: char = "Zss"; // this shows error..
    let heart_eyed_cat = '😻';

    // Floating point 

}