fn main() {
    let number = 8;

    if number < 5 {
        println!("condition was true"); //Blocks of code associated with the conditions in if expressions are sometimes called arms.
    } else {
        println!("condition was false");
    }

    // if number  {  //In this case, there will show an error beacause the condition must be a bool.
        //The error will be "expected `bool`, found integer"
    //     println!("condition was true"); //Blocks of code associated with the conditions in if expressions are sometimes called arms.
    // } else {
    //     println!("condition was false");
    // }

    if number != 0 {
        println!("The number is not zero!");
    }
    if number % 3 == 0 {
        println!("The number is divisible by 3");
    }else if number % 3 == 1 {
        println!("The remainder is 1 when the number is divided by 3");
    }else {
        println!("The remainder is 2 when the number is divided by 3");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
// The values that have the potential to be results from each arm of the if must be the same type;

    //let number = if condition {8} else {"A"}; //This will show an error. Because the types are not same.
    println!("The value of number is: {number}");
}