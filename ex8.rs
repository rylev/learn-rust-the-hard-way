fn main() {
    let args = std::env::args();
    let args_size = args.len();

    // Another way to iterate is by using a while loop. In the following cases,
    // the while loop is much easier to screw up than a for loop and is arguably
    // hard to read and understand. In general prefer for loops over while loops
    // unless there's a good reason to use them.
    let mut i = 0;
    while i < args_size {
        println!("arg {}: {}", i, args[i]);
        i = i + 1;
    }

    let states = ["California", "Oregon", "Washington", "Texas"];

    let num_states = states.len();
    let mut i = 0;
    while i < num_states {
        println!("state {}: {}", i, states[i]);
        // We can use += for a short hand add and assign.
        i += 1;
    }

    // If you need to loop forever, you can of course do 'while true' but Rust provides
    // the more intention revealing shortcut: loop.
    // println!("Let's do some counting.")
    // i = 0;
    // loop {
    //     println!("{}", i);
    //     i = i + 1;
    // }
}

// Questions and Exercises:
// 1.) Forget to initialize the first int i; so have it loop wrong.
// 2.) Forget to initialize the second loop's i so that it retains the value
//     from the end of the first loop. Now your second loop might or might not run.
// 3.) Forget to do a i++ increment at the end of the loop and you get a "forever loop"
// 4.) Use a while loop to copy the values from args into states. You will have to
//     convert the Strings in args to &str by using the .as_slice() method on String.
// 5.) Make this copy loop never fail such that if there's too many args elements
//     it won't put them all into states.
// 6.) Research if you've really copied these strings. The answer may surprise and
//     confuse you though.
