// http://c.learncodethehardway.org/book/ex8.html

fn main() {
    // this is the syntax for an array literal. Arrays are arrays in the traditional
    // sense. They are of a fixed size. If you want to have a "growable" array use
    // vector. We will explore vectors later on.
    let areas = [10i8, 12i8, 13i8, 14i8, 20i8];

    // An array's type signature looks like the following.
    // Notice the explicit mention of the length.
    let ages : [u8; 3] = [17, 19, 89];
    // Rust also lets you get views of dynamic length into arrays called slices.
    // Notice that the type signature of the slice does not contain a length. A slice's
    // size is not known at compile time.
    // Notice also the strange '&'. This symbol is very important and you'll be
    // seeing a lot in the future. For now, think about what it could possibly mean,
    // and move on. We'll cover it soon.
    let my_slice = &ages[0..1];
    // '{:?}' uses the Show trait (we will cover traits later) to "show" the slice.
    println!("My slice looks like: {:?}", my_slice);

    let name = "Rust";

    // Next we'll call some functions.
    // size_of_val returns the size of the given value in bytes as a u8. Again,
    // don't worry for now why we pass size_of_val '&areas' instead of just 'areas'.
    println!("The size of areas: {}", std::mem::size_of_val(&areas));
    // size_of returns the size of the given type in bytes as a u8. Notice how we
    // are calling this function using '<int>' and providing no argument to the fuction.
    // We are parameterizing this function based on a type and not a value.
    // When dealing with types we use '<>' and when dealing with values we use '()'.
    println!("The size of an isize: {}", std::mem::size_of::<isize>());

    // To get the size of the array we can get its total size in bytes
    // and divide by the size of an individual int
    println!("The number of ints in areas: {}", std::mem::size_of_val(&areas) / std::mem::size_of::<i8>());

    // Rust also allows calling methods on values similar to object oriented languages
    // So in order to get the length of areas, we can simply call the .len() function on it.
    println!("The number of ints in areas: {}", areas.len());

    // Array access mimics most traditional languages.
    println!("The first area is {}, the 2nd {}.", areas[0], areas[1]);

    println!("The size of a char: {}", std::mem::size_of::<char>());
    println!("The size of name: {}", std::mem::size_of_val(&name));
    println!("The length of name: {}", name.len());
    println!("The length of ages: {}", ages.len());
}

// Questions and Exercises:
// 1.) Change it so that instead of areas[0] you try to print areas[10]. Do some
//      research on why you get what you get.
// 2.) Try assigning to elements in the areas array with 'areas[0] = 100;'. What happens?
// 3.) Trying assigning an element to area that is not an i8.