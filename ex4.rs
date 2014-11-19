// http://c.learncodethehardway.org/book/ex6.html

fn main() {
    // Let's meet some literals shall we?

    // The literal for ints is a number followed by an 'i'
    // (the size of the int depends on the machine - 32 bit vs. 64 bit)
    // int and uint (unsigned int) are the only types that depend on the specific
    // machine's architecture.
    let distance = 100i;
    // We could have also written:
    // let distance : int = 100;
    println!("You are {:d} miles away.", distance);

    // Literals can also have arbitray '_'s which are useful for readability:
    // 100000000i vs 100_000_000i
    let bigger_distance = 100_000_000i;
    println!("You are {:d} miles away.", bigger_distance);

    // The literal for floats is a number followed by an 'f32' meaning a float that
    // uses 32 bits to represent the number.
    let power = 2.345f32;
    println!("You have {:f} levels of power.", power);

    // Known as doubles in C land, 64 bit floats are a number followed by 'f64'.
    let super_power = 56789.4532f64;
    println!("You have {:f} awesome super powers.", super_power);

    // Characters in Rust are unicode codepoints.
    // C uses u8/i8 for char (i.e. 8 bit unsigned/signed integers)
    let initial = 'L';
    println!("I have an initial {:c}.", initial);

    let smile = '😊';
    println!("Look how happy I am! {:c}.", smile);

    // String literals are of type &str. We'll cover this type of string and its
    // companion type String later on.
    let first_name = "Ryan";
    let last_name = "Levick";

    println!("I have a first name {}.", first_name);
    println!("I have a last name {}.", last_name);
    println!("My whole name is {} {:c}. {}.", first_name, initial, last_name);

    // The booleans true and false are pretty straight forward.
    let answer = true;
    println!("Rust is fun! {}", answer);
}

// Questions and Exercises:
// 1.) Come up with other ways to break this C code by changing the 'println!'s,
//     then fix them.
// 2.) Search for the literals for more types (e.g. hexadecimal, octal and binary).
//     How do you print these types using the 'println!' macro.