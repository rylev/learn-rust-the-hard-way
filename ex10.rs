use std::os;

fn main() {
    // argv and argc are traditional names used in C and C++ for command line
    // arguments and number of command line arguments respectively
    let argv = os::args();
    let argc = argv.len();

    if argc != 2 {
        // this is how you exit a program early
        // notice the ! sense panic is not a function but rather a macro
        panic!("ERROR: You need exactly one argument.");
    }

    // get the second command line argument as a slice
    let argument = &argv[1][];
    // ** IMPORTANT **
    // We must take access the second element from the Vector as a slice because of
    // the concept of ownership in Rust. Variables in Rust are in charge of freeing
    // the resources (the memory) that they occupy. Resources (memory) can only have
    // one owner at a time. If this were not true then two variables could try to
    // free the same memory at different times. Disaster! Because the variable argv
    // owns the memory where the second argument is stored we can't also give that
    // memory to the the variable argument too. If we did both argv and argument
    // would own the same piece of memory. The final [] allows us to get a read only
    // view that piece of memory. So "argument" does own that memory (argv still
    // does) but it has the ability to read from that memory until argv goes out of
    // scope. We'll dedicate some more time to the idea in the future.

    for letter in argument.chars() {
        // the following is a construct called pattern matching. if you're familiar
        // with switch or case statements from other languages pattern matching
        // works similarly but is much more expersive. Instead of simply matching
        // on equality pattern matching looks at the strucutre and shape of what it's
        // matching against. The '|' means "or".
        match letter {
            'a' | 'A' | 'e' | 'E' | 'i' |
            'I' | 'o' | 'O' | 'u' | 'U' => println!("{} is a vowel!", letter),
        // The "_" is the "all matcher" and matches anything that didn't match before.
            _                           => println!("{} is not a vowel!", letter)
        }

    }
}
// Questions and Exercises:
// 1.) Try moving the underscore matching pattern above the first match possibility.
//     (Don't forgot to move the comma at the end of the line!) Run the program and
//     see what happens. Explain why the program changed this way.
// 2.) Google more information about pattern matching in Rust. What other patterns
//     can you match against in Rust?
// 3.) On the line where we get the first command line argument and assign it to
//     the "argument" remove the final []. What error do you get? Google that error.
//     Do some more research on the idea of "ownership", "moves" and "lifetimes" in Rust.