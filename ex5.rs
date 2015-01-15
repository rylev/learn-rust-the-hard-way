// http://c.learncodethehardway.org/book/ex7.html

fn main() {
    let bugs = 100i64;
    let bug_rate = 1.2f64;

    println!("You have {} bugs at the imaginary rate of {}.", bugs, bug_rate);

    let universe_of_defects = 1i32 * 1024i32 * 1024i32 * 1024i32;
    println!("The entire universe has {} bugs.", universe_of_defects);

    // Notice that bugs is of type int. Unlike in C we cannot multiple an int with a
    // 64 bit float without first explicitly casting the int to a 64 bit float.
    let expected_bugs = (bugs as f64) * bug_rate;
    println!("You are expected to have {} bugs.", expected_bugs);

    let part_of_universe = expected_bugs / (universe_of_defects as f64);
    println!("That is only a {:e} portion of the universe.", part_of_universe);

    // This makes no sense, just a demo of something weird
    let null_byte = '\0' as i64;
    let care_percentage = bugs * null_byte;
    println!("Which means you should care {}%.", care_percentage);
}

// Questions and Exercises:
// 1.) Multiple the number you assign to universe_of_defects various times until it
//     reaches a size bigger than a 32 int can hold.
// 2.) Change universe_of_defects to use 64 bit ints (i64) and unsigned 32 and
//     64 bit ints (u32, u64). How does this affect how big universe_of_defects
//     can go?
// 3.) Do some research on unsigned and signed integers. Also do some research
//     on how Rust handles integer "overflow".
// 4.) What does it mean to cast a character (the null byte) to an int? Why can
//     we do this?