// http://c.learncodethehardway.org/book/ex3.html

fn main(){
    // Variable assignment is done using the 'let' keyword
    // No type annotations are neceesary as Rust has local type inference
    // Here we are assigning the 8 bit signed integer 10 to the variable age.
    let age = 10i8;

    // Number literals (integers, unsigned integers, floats, etc.) are distinguished
    // by one letter after the number. We'll cover more literals later.
    let height = 72i8;

    // You can skip annotating the literal if you give an explicit type
    let shoe_size : i8 = 12;

    println!("I am {} years old.", age);
    println!("I am {} inches tall.", height);
    println!("I wear a size {} shoe.", shoe_size);

    // Variables are immutable by default. If you want to reassign a variable to
    // another value you must first declare it mutable using the 'mut' keyword.
    // TODO: Uncomment the following three lines and compile
    // Be warned this will emit a compiler warning.

    // let mut european_shoe_size = 42i8;
    // european_shoe_size = 46i8;
    // println!("I wear a size {} shoe in Europe.", european_shoe_size);
}

// Questions and Exercises:
// 1.) Are the semicolons necessary?
// 2.) What happens when you replace the '{}' with '{:k}'?
// 3.) What happens when you try to reassign a variable not declared mutable?
// 4.) Get the compiler to compile the mutable variable reassignment code without a
//     compiler warning.
// 5.) Go line by line and write down anything you don't understand and what about it
//     confuses you.

// Notes on semicolons:
//
// Generally most languages with semicolons fall into two camps.
// * C/JavaScript = semicolons are mandatory.
// * Ruby/Python  = semicolons are optional and usually ommited
// Rust is different as semicolons are sematic meaning they have meaning. Semicolons
// are generally used when some sort of side effect (e.g. variable assignment, IO) is
// happening.