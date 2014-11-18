// Example 3
// http://c.learncodethehardway.org/book/ex3.html

fn main(){
    // Variable assignment is done using the 'let' keyword
    // No type annotations are neceesary as Rust has local type inference
    // Here we are assigning the integer 10 to the variable age.
    let age = 10i;

    // Number literals (integers, unsigned integers, floats, etc.) are distinguished
    // by one letter after the number. We'll cover more literals later.
    let height = 72i;

    // You can skip annotating the literal if you give an explicit type
    let shoe_size : int = 12;

    println!("I am {:d} years old.", age);
    println!("I am {:d} inches tall.", height);
    println!("I wear a size {:d} shoe.", shoe_size);

    // Variables are immutable by default. If you want to reassign a variable to
    // another value you must first declare it mutable using the 'mut' keyword.
    // TODO: Uncomment the following three lines and compile
    // Be warned this will emit a compiler warning.

    // let mut european_shoe_size = 42i;
    // european_shoe_size = 46i;
    // println!("I wear a size {:d} shoe in Europe.", european_shoe_size);
}

// Questions and Exercises:
// 1.) Are the semicolons necessary?
// 2.) What happens when you replace the '{:d}' with '{:k}'?
// 3.) What happens when you try to reassign a variable not declared mutable?
// 4.) Get the compiler to compile the mutable variable reassignment code without a
//     compiler warning.