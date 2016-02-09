// http://c.learncodethehardway.org/book/ex5.html

// The following imports a the std::fmt module.
// The import is unnecessary and will produce a warning!
use std::fmt;

/* This is a
    multiline comment.*/

// Start the main function.
fn main() {
    let distance = 100i8; // assign the 8 bit signed integer 100 to the immutable variable distance.

    // this is a single line comment

    // println! is a macro. that prints the formated string to stdout
    println!("You are {} miles away.", distance);

    // Notice that unlike C we do not return some int from the main function.
    // Main's return type is '()' read as "unit" which as we shall see is a much
    // nicer way to indicate nothingness than void.
}

// Questions and Exercises:
// 1.) There is also a println function. See if you can find the difference in
//     functionality between the macro 'println!' and the function 'println'
// 2.) For each line, write out the symbols you don't understand and see if you can
//     guess what they mean. Write a little chart on paper with your guess that
//     you can use to check later and see if you get it right.
