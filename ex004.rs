// http://c.learncodethehardway.org/book/ex6.html

fn main() {
    // Let's meet some literals shall we?

    // The literal for signed ints is a number followed by an 'i' and some number.
    // This number is the size of the integer in bits. For example '10i32' is the
    // literal for the number 10 as a signed 32 bit integer.

    let small_distance = 10i32;
    println!("10 equals {}.", small_distance);

    // The literal for unsigned ints is a number followed by an 'u' and some number -
    // again the size of the integer in bits. For example '10u16' is the
    // literal for the number 10 as a 16 bit unsigned integer.

    let slightly_bigger_distance = 10u16;
    println!("10 equals {}.", slightly_bigger_distance);

    // If you need an integer that is dependent on the architecture on which the
    // program runs you can use the isize and usize types of integers which are
    // specified by the is and us postfixes. For example, 100is will be the 64 bit
    // signed representation of 100 if running on a 64 bit architecture and
    // 32 bit if running on a 32 bit architecture.
    let distance = 100isize;
    // We could have also written:
     //let distance : isize = 100;
    println!("You are {} miles away.", distance);

    // Literals can also have arbitray '_'s which are useful for readability:
    // 100000000i vs 100_000_000i
    let bigger_distance = 100_000_000i64;
    println!("You are {} miles away.", bigger_distance);

    // The literal for floats is a number followed by an 'f32' meaning a float that
    // uses 32 bits to represent the number.
    let power = 2.345f32;
    println!("You have {} levels of power.", power);

    // Known as doubles in C land, 64 bit floats are a number followed by 'f64'.
    let super_power = 56789.4532f64;
    println!("You have {} awesome super powers.", super_power);

    // Characters in Rust are unicode codepoints.
    // C uses u8/i8 for char (i.e. 8 bit unsigned/signed integers)
    let initial = 'L';
    println!("I have an initial {}.", initial);

    let smile = '😊';
    println!("Look how happy I am! {}.", smile);

    // String literals are of type &str. We'll cover this type of string and its
    // companion type String later on.
    let first_name = "Ryan";
    let last_name = "Levick";

    println!("I have a first name {}.", first_name);
    println!("I have a last name {}.", last_name);
    println!("My whole name is {} {}. {}.", first_name, initial, last_name);

    // The booleans true and false are pretty straight forward.
    let answer = true;
    println!("Rust is fun! {}", answer);
}

// Questions and Exercises:
// 1.) Come up with other ways to break this code by changing the 'println!'s,
//     then fix them.
// 2.) Search for the literals for more types (e.g. hexadecimal, octal and binary).
//     How do you print these types using the 'println!' macro.
