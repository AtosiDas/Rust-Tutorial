/* We’ll implement a classic beginner programming problem: 
a guessing game. Here’s how it works: the program will generate 
a random integer between 1 and 100. It will then prompt the player 
to enter a guess. After a guess is entered, the program will indicate 
whether the guess is too low or too high. If the guess is correct, 
the game will print a congratulatory message and exit.*/

/*By default, Rust has a set of items defined in the standard library
 that it brings into the scope of every program. This set is called the "prelude".
 If a type we want to use isn’t in the prelude, we have to bring that type into scope explicitly with a use statement.*/
use std::io; //To obtain user input and then print the result as output, we need to bring the io input/output library into scope. The io library comes from the standard library, known as std

use rand::Rng; //The Rng trait defines methods that random number generators implement

use std::cmp::Ordering; //The Ordering type is another enum and has the variants Less, Greater, and Equal. These are the three outcomes that are possible when you compare two values.
//The cmp method compares two values and can be called on anything that can be compared.

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");
    loop {
    println!("Please input your guess.");
// we use "let" to create a variable, "mut" means mutable.
//By default, in Rust every variables are immutable, meaning once we give the variable a value, the value won't change.
    let mut guess = String::new();  //This function 'String::new()' returns a new,empty instance of a String
// "new" is an associated function** of the string type.
// This new function creates a new, empty string.

    io::stdin()  //we call the stdin function from the io module, which will allow us to handle user input.
        .read_line(&mut guess)  //Next, the line .read_line(&mut guess) calls the "read_line" method on the standard input handle to get input from the user.
        .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    println!("You guessed: {guess}"); 

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break;
        }
    }
}
}

//The type of secret-number is i32 (it is default type of a number)
//The type of guess is string. 


/* What is **associated function?
 An associated function is a function that’s implemented on a type.*/

 /*The full job of 'read_line' is to take whatever the user types into 
 standard input and append that into a string (without overwriting its contents), 
 so we therefore pass that string as an argument. The string argument needs to be 
 mutable so the method can change the string’s content.*/

/*The & indicates that this argument is a reference, which gives us a way to let multiple parts of our code access one piece of data without needing to copy that data into memory multiple times.
Like variables, references are immutable by default. Hence, we need to write "&mut guess" rather than "&guess" to make it mutable.*/

/*Remember that a crate is a collection of Rust source code files. The project we’ve been building is a binary crate, 
which is an executable. The rand crate is a library crate, which contains code that is intended to be used in other programs and can’t be executed on its own*/

/*We call the "rand::thread_rng" function that gives us the particular random number generator we’re going to use: one that is local to the current thread of execution and is seeded by the operating system.*/

/*The "gen_range" method takes a range expression as an argument and generates a random number in the range. The kind of range expression we’re using here takes the form start..=end and is inclusive on the lower and upper bounds*/

/*A match expression is made up of arms. An arm consists of a pattern to match against, and the code that should be run if the value given to match fits that arm’s pattern.*/

/*The trim method on a String instance will eliminate any whitespace at the beginning and end, which we must do to be able to compare the string to the u32, which can only contain numerical data.*/

//The parse method on strings converts a string to another type. Here, we use it to convert from a string to a number

