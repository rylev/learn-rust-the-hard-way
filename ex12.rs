// http://c.learncodethehardway.org/book/ex14.html

// define a function with the fn key word
// Each argument takes a type. This function's only argument is a char (character).
// The result type is indicated by the "-> bool" syntax. This function returns a
// bool (boolean).
fn can_print_it(ch: char) -> bool {
    ch.is_alphabetic() || ch.is_whitespace()
}

// "Vec<String>" means a vector of strings. The "<>" syntax is a type parameter. "vec"
// alone is not a type. We need to indicate that it's a vector of something. In
// this cse it's a vector of strings.
// When no return type is provided it is assumed to be () (read "unit") which is a
// type with only value (also "()"). This indicates we don't really care for the
// return value, we are just using this function for its side effects.
fn print_arguments(argv: Vec<String>) {
    for arg in argv.iter() {
        // As we'll see below iterating over a "Vec<String>" does not yield each
        // String but rather a read only "reference" or "borrow" to that string. As we have
        // previously learned only one variable can own a piece of data (or perhaps
        // better said the memory where that data lives). If iterating were to
        // yield the actual data itself instead of a reference to that data, then
        // two variables "arg" and the original argv variable. Would own the memory
        // when the argument lives.

        print_letters(arg);
    }
}

// As we learned above "arg" is not the actual argument String itself but rather a
// reference to that String. We indicate this fact in the function signature with the
// "&" character. This is a read-only immutable reference to the String not the string
// itself.
fn print_letters(arg: &String) {
    // .chars() works on a &String
    for ch in arg.chars() {
        // .chars() will yield to us a char (not a ref since Strings aren't actually
        //  composed of chars in Rust)
        if can_print_it(ch) {
            // Here we want to print the char as a utf-8 char and as its 8 bit
            // unsigned interger representation.
            print!("'{}' == {} ", ch, ch as u8);
        }
    }

    println!("");
}


fn main() {
    let argv = std::os::args();
    print_arguments(argv);
}

// 1.) Look up more functions like is_alphabetic() and is_whitespace() that are
//     defined on chars.
// 2.) Look up the docs for chars() in std::str::StrPrelude. What do you think the
//     &self refers to? Do some research on self. We'll come across it again soon.
// 3.) Change print_arguments() to take a reference instead of a Vec<String>.