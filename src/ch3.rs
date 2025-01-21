use std::io;

pub fn main() {
    //https://rust-book.cs.brown.edu/ch03-02-data-types.html
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
    let floatnum: f32 = 3.14159265358;

    // boolean
    let t: bool = true;

    //char
    let cz: char = 'a';

    //tuple  - a list of items, they don't have to be the same type..

    let mytuple: (i32, f32, String) = (1, 2.0, "asdf".to_string()); // explicit define type
    let mytuple2 = (1, 2.0, "asdf".to_string()); //imply type..

    let mut mut_tuple: (i32, bool) = (5, false);

    mut_tuple.0 = 6; // can change tuple vars.. or retrieve them using tuple.0, tuple.1....

    println!("mut_tuple = {:?}", mut_tuple);

    // can also unroll tuples into vars

    let tup = (1.0, 2.0, false);
    let (a, b, c) = tup;
    println!("a = {}, b = {}, c = {}", a, b, c);

    // arrays -- similar to tuples, but all vars must be same type and length cannot change..

    let arr1 = [1, 2, 3, 4]; // implied type and length
    let arr2: [i32; 4] = [5, 6, 7, 8]; // explicitly define type and length

    // let arr3 = [3.0, 4]; // errors out because can't have a float and an int..

    let mut arr3 = [4.0, 5.0, 6.0];

    println!("arr3 = {:?}", arr3);

    arr3[2] = 25.3324234; // can change 3rd item (2nd index..) if the arr is set as mutable.

    println!("arr3 = {:?}", arr3);

    // But rust arrays cannot grow in size - so often times the "Vector" type is what you really want..
    //Can also have string arrays

    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];


    // Checking Array Bounds on the fly (per user input) - program panics if outside of bounds
    // let a = [1, 2, 3, 4, 5];
    //
    // println!("Please enter an array index.");
    //
    // let mut index = String::new();
    //
    // io::stdin()
    //     .read_line(&mut index)
    //     .expect("Failed to read line");
    //
    // let index: usize = index
    //     .trim()
    //     .parse()
    //     .expect("Index entered was not a number");
    //
    // let element = a[index];
    //
    // println!("The value of the element at index {index} is: {element}");



    //https://rust-book.cs.brown.edu/ch03-03-how-functions-work.html
    // Functions!

    print_some_stuff(5, "min");

    // Statements - do not evaluate to a value (e.g. let statement; variable assignment)

    // expression

    let res = { // results of expression can be used to assign a variable
        let y = 5; // statements present w/in expression
        y + 5 // last item (without semicolon) of expression gets returned..
    };
    println!("Expression practice, res = {}", res);
    //functions and macros are also examples of expressions

    let added_vals = add_ints(5, 6);
    println!("Added vals: {added_vals}");

    ////
    //https://rust-book.cs.brown.edu/ch03-04-comments.html
    ////

    //regular comment

    /*

    Multi line comments

     */

    // Control Flow
    // https://rust-book.cs.brown.edu/ch03-05-control-flow.html
    //

    // if statements
    let number = 3;
    if number < 5 {
        println!(" The number was less than 5...");
    } else {
        println!(" The number was greater than 5...");
    }

    let number = if number < 5 { 10 } else { 6 }; // can let combine w/ if else
    // Note - each arm of if need to be the same type

    println!("The value of number is: {number}");

    //Loops - loop, for, while

    let mut loop_count = 1;
    println!("Lets try a loop - ");
    // so apparently in a loop you can take the result of loop and write it to a value.
    let result = loop {
        println!("again!");
        if loop_count > 4 {
            break loop_count; // just after the break say the return value..
        }
        loop_count += 1;
    };
    println!("Looped {result} times!");

    // Can also write multiple loops together and name them!
    // good to know, not sure it will be that useful?

    // while loops
    let mut loop_count = 0;
    while loop_count < 3 {
        println!("while loop_count: {loop_count}");
        loop_count += 1;
    }

    // for loop

    let a = [1,2,3,4,5];
    for i in 1..a.len() { // can loop over a, or (1..a.len()) range; can also reverse a range.
        println!("a[{i}] = {}", i);
    }

    let a = [5; 10]; // so this evaluates to [5,5,5,5,5 .. 10 of them] is what the semicolon does.

    println!("{:?}", a);
    let mut sum = 0;

    for x in a {
        sum += x;
    }

    println!("{sum}");



    //Convert temperatures between Fahrenheit and Celsius.
    println!("65F in C: {}", f_to_c(65.0));
    println!("11C in F: {}", c_to_f(11.0));

    //Generate the nth Fibonacci number.
    println!("Fibonacci(6) = {}", fibonacci(6));
    println!("Fibonacci(25) = {}", fibonacci(25));
    println!("Fibonacci(50) = {}", fibonacci(50)); // i32 ran out here; switched to i128
    println!("Fibonacci(100) = {}", fibonacci(100)); // i128


    //Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.

    use std::collections::HashMap;
    let mut suffixmap = HashMap::from([
        (1, "st"),
        (2, "nd"),
        (3, "rd"),
    ]);

    for i in 4..=12 {
        suffixmap.insert(i, "th");
    }

    let gift_strings = [
        "partridge in a pear tree",
        "turtle doves",
        "french hens",
        "calling birds",
        "golden rings",
        "geese a laying",
        "swans a swimming",
        "maids a milking",
        "ladies dancing",
        "lords a leaping",
        "pipers piping",
        "drummers drumming"
    ];
    let mut song_lyrics: String = "".to_string();
    for day in 1..=12 {
        song_lyrics += &format!("On the {}{} day of Christmas my true love gave to me:\n", day, suffixmap.get(&day).unwrap());
        for day2 in (1..=day).rev() {
            if day2 == 1 {
                let and_a = if day > 1 {" and a"} else {" A"};
                song_lyrics += &format!("{} {}.\n", and_a, gift_strings[0]);
            } else {
                song_lyrics += &format!(" {} {},", day2, gift_strings[day2 - 1]);
            }
        }
    }
    println!("{}", song_lyrics);
}

fn f_to_c(f: f32) -> f32 {
    (f - 32.0) / 1.8
}

fn c_to_f(c: f32) -> f32 {
    c * 1.8 + 32.0
}

fn fibonacci(n: i32) -> i128 {
    let mut fib_prev = 0;
    let mut fib = 1;

    for i in 1..n {
        let fib0 = fib;
        fib = fib + fib_prev;
        fib_prev = fib0;
    }
    fib
}

fn print_some_stuff(x: i32, unit_label: &str) {
    println!("Going to print some stuff here, just look at this cool number: {x} {unit_label}");
}

fn add_ints(x: i32, y: i32) -> i32 {
    x + y
}