//The main function is special: it is always the 
//first code that runs in every executable Rust program.
fn main() {
        println!("Hello, world!"); //println! calls a Rust macro.
}

//A ! means that you’re calling a macro instead of a normal function 
// and that macros don’t always follow the same rules as functions.

// We end the line with a semicolon (;), which indicates that 
// this expression is over and the next one is ready to begin. 
// Most lines of Rust code end with a semicolon. 

// When using Windows, we get a file containing debugging information with the .pdb extension.

//-----------More about RUST---------------//

/*Rust is an ahead-of-time compiled language, meaning you can compile a program
and give the executable to someone else, and they can run it even without having Rust installed.
But if you give someone a .rb, .py, or .js file, they need
to have a Ruby, Python, JavaScript implementation installed respectively.*/

/* But in those languages, we only need one command to compile and run the program.*/