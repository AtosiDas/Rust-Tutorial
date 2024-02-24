// By default, variables are immutable.
/*However, you still have the option to 
make your variables mutable. Let’s explore how and 
why Rust encourages you to favor immutability and why sometimes 
you might want to opt out.*/

//We can make them mutable by adding mut in front of the variable name.
/*Adding mut also conveys intent to future readers of the code by indicating that 
other parts of the code will be changing this variable’s value.*/
fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    {
        let x = x + 1;  // It creates a new variable x by repeating let x =, taking the original value and adding 1 so the value of x is then 6.
        println!("The value of x is : {x}");  // inner scope
    }
    let x = x + 10; // In outer scope x = 5 
    println!("The value of x is: {x}");
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    const HELLO: u32 = 5;
    println!("The constants are: {THREE_HOURS_IN_SECONDS},{HELLO}");
    let mut spaces = "   ";
    let spaces = spaces.len();
}

/*Ultimately, deciding whether to use mutability or not is up to us and 
depends on what We think is clearest in that particular situation.*/

/*There are some differences between Immutable variables and Constant variables.
1)First, we aren’t allowed to use mut with constants. Constants aren’t just immutable by default—they’re always immutable. 
2)We declare constants using the const keyword instead of the let keyword, and the type of the value must be annotated.
3)Constants can be declared in any scope, including the global scope.
4)The last difference is that constants may be set only to a constant expression, not the result of a value that could only be computed at runtime.
*/

//Rust’s naming convention for constants is to use all uppercase with underscores between words.
 /*If we are shadowing, we do not need to declare a variable as mut and we add "let" when we shadow.*/

/*1) Shadowing is different from marking a variable as mut because we’ll get a compile-time error if we accidentally try to reassign 
to this variable without using the let keyword. By using let, we can perform a few transformations on a value but have the variable 
be immutable after those transformations have been completed.
2) The other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again, 
we can change the type of the value but reuse the same name.*/