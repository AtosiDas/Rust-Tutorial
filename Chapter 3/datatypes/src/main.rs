/* Rust is a statically typed language, which means that it must know the types of all variables at compile time.*/
// We’ll look at two data type subsets: scalar and compound.

//<---------------SCALAR TYPES------------------>
/*A scalar type represents a single value. 
Rust has four primary scalar types: integers, floating-point numbers, 
Booleans, and characters.*/


//<-----------------Integer Types------------------>
//u32 type takes up unsigned integer that 32 bits of space
//i32 type takes up signed integer that 32 bits of space.
/*Each signed variant can store numbers from -(2n - 1) to 2n - 1 - 1 inclusive, where n is the number of bits that variant uses.
Unsigned variants can store numbers from 0 to 2n - 1 */
//Additionally, the isize and usize types depend on the architecture of the computer our program is running on, which is denoted as “arch”.

//Integer types default to i32

//Integer Overflow: 
/*Let’s say we have a variable of type u8 that can hold values between 0 and 255. If we try to change the variable to a value outside that range, 
such as 256, integer overflow will occur, which can result in one of two behaviors. When we’re compiling in debug mode, Rust includes checks 
for integer overflow that cause our program to panic at runtime if this behavior occurs*/

/*When we’re compiling in release mode with the --release flag, Rust does not include checks for integer overflow that cause panics. 
Instead, if overflow occurs, Rust performs two’s complement wrapping. 
In the case of a u8, the value 256 becomes 0, the value 257 becomes 1, and so on.*/


//<---------------------Floating-Point Types----------------->
/*Rust also has two primitive types for floating-point numbers, which are numbers with decimal points. 
Rust’s floating-point types are f32 and f64, which are 32 bits and 64 bits in size, respectively.*/

//The default type is f64 and all floating-point types are signed.
use std::io;
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    let t = true;

    let f: bool = false; // with explicit type annotation

    let c = 'z'; //char literals with single quotes
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (500, 6.4, 1); //tuples
    //let tup = (500,6.4,1);
    let (x, y, z) = tup; //Destucturing because it breaks the single tuple into three parts. 

    println!("The value of y is: {y}");

    let tup1: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = tup1.0;

    let six_point_four = tup1.1;

    let one = tup1.2;

    let a = [1, 2, 3, 4, 5]; //array
    
    let first = a[0];
    let second = a[1];
    let b: [i32; 5] = [1, 2, 3, 4, 5];  //i32 is the type of each element and 5 is the length of the array
    let a1 = [3; 5];
    let a2 = [3, 3, 3, 3, 3]; // The arrays a1 and a2 are same.

    let arr = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
//The f32 type is a single-precision float, and f64 has double precision.

//<----------------Boolean Types------------------->

//As in most other programming languages, a Boolean type in Rust has two possible values: true and false. 
//Booleans are one byte in size. The Boolean type in Rust is specified using bool 

//<--------------------Character Type-------------------->

/* Rust’s char type is the language’s most primitive alphabetic type. */
//we specify char literals with single quotes, as opposed to string literals, which use double quotes.

/*Rust’s char type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII.*/
/*However, a “character” isn’t really a concept in Unicode, so your human intuition for what a “character” is may not match up with what a char is in Rust.*/

//<------------------Compound Types------------>
//Compound types can group multiple values into one type. 
//Rust has two primitive compound types: tuples and arrays.

/* A tuple is a general way of grouping together a number of values with a variety of types into one compound type. 
Tuples have a fixed length: once declared, they cannot grow or shrink in size.*/

//The tuple without any values has a special name, unit. 
/*This value and its corresponding type are both written () and represent an empty value or an empty return type. 
Expressions implicitly return the unit value if they don’t return any other value.*/


//<-------------Array type------------->
/*Another way to have a collection of multiple values is with an array. 
Every element of an array must have the same type. Arrays in Rust have a fixed length.*/

/*Arrays are useful when we want our data allocated on the stack rather than the heap or when we want to ensure us always have a fixed number of elements.*/ 
/*An array isn’t as flexible as the vector type, though. A vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size.*/