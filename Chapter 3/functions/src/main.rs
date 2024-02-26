/*For function and variable names, all letters are lowercase and underscores separate words.*/

fn main() {
    println!("Hello World!");
    another_function(5,2,'A');
    let y = 6; //It is a statement
    let z = {
        let x = 5;
        x + 1 //Note that the x + 1 line doesn’t have a semicolon at the end, which is unlike most of the lines you’ve seen so far. Expressions do not include ending semicolons.
    }; //It is an expression
    //If we add a semicolon to the end of an expression, we turn it into a statement, and it will then not return a value
    println!("The value of z is: {z}");
    let increase = increase_one(10);
    println!("The value is: {increase}");
}
fn another_function(a: i32, b: i32, c: char) {
    println!("This is another function.");
    let sum = a + b;
    println!("The sum of a and b is: {sum}");
    println!("The first letter is: {c}");

    //We can not write x = y =6;
    //let x = (let y = 6); It will show error.
    
}
fn increase_one(x: i32) -> i32 {
    x + 1
}
/*Statements are instructions that perform some action and do not return a value.
Expressions evaluate to a resultant value. 
Statements do not return values.*/
//Calling a function is an expression. Calling a macro is an expression. 